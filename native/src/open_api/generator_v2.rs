// use std::{fs::File, io::Write};
use std::collections::BTreeMap;

use dprint_swc_ext::view::*;
use lazy_static::__Deref;

use crate::typescript::DeclarationTable;

use super::{
    open_api::{ApiPathOperation, OpenApi, PathOptions, ResponseOptions},
    symbol_table_helpers::store_symbol_maybe,
};

use deno_ast::{ParseParams, ParsedSource, SourceTextInfo};
use dprint_swc_ext::view::{with_ast_view_for_module, ModuleInfo, NodeTrait};
use url::Url;

pub struct GeneratorV2<'n> {
    symbol_tables: BTreeMap<String, DeclarationTable<'n>>,
}
impl<'n> GeneratorV2<'n> {
    pub fn new() -> GeneratorV2<'n> {
        GeneratorV2 {
            symbol_tables: Default::default(),
        }
    }

    pub(crate) fn from_source_file(&mut self, file_path: &str) -> OpenApi {
        let mut open_api = OpenApi::new();
        let result = get_syntax_tree(file_path);
        let symbol_table = self
            .symbol_tables
            .entry(file_path.to_owned())
            .or_insert(Default::default());

        with_ast_view_for_module(
            ModuleInfo {
                module: result.module(),
                comments: None,
                text_info: Some(result.text_info()),
                tokens: None,
            },
            |module| find_paths(&mut open_api, &module.as_node(), symbol_table),
        );

        open_api
    }
}

fn get_syntax_tree(path: &str) -> ParsedSource {
    let specifier = Url::from_file_path(path).unwrap();
    let source_text = std::fs::read_to_string(specifier.path()).unwrap();
    let parsed_source = deno_ast::parse_module(ParseParams {
        capture_tokens: true,
        maybe_syntax: None,
        media_type: deno_ast::MediaType::TypeScript,
        scope_analysis: false,
        specifier: specifier.to_string(),
        text_info: SourceTextInfo::new(source_text.into()),
    })
    .unwrap();

    parsed_source
}

pub fn find_paths<'n>(open_api: &mut OpenApi, node: &Node<'n>, symbol_table: &mut DeclarationTable<'n>) {
    store_symbol_maybe(node, symbol_table);

    for child in node.children() {
        match child.kind() {
            NodeKind::CallExpr => match child.to::<CallExpr>() {
                Some(call_expr) => match call_expr.callee {
                    Callee::Expr(Expr::Ident(ident)) if ident.sym().eq("Path") => {
                        symbol_table.push_scope();
                        add_path(open_api, &call_expr, symbol_table);
                        symbol_table.pop_scope();
                    }
                    _ => find_paths(open_api, &child, symbol_table),
                },
                None => find_paths(open_api, &child, symbol_table),
            },
            _ => find_paths(open_api, &child, symbol_table),
        }
    }
}

fn add_path<'n>(open_api: &mut OpenApi, node: &CallExpr<'n>, symbol_table: &mut DeclarationTable<'n>) -> () {
    let args = &node.args;
    let route_handler = args.first().copied();
    let route_options = args.last().copied();
    let options = get_path_options(route_options);

    let mut operation = open_api
        .path(&options.path.unwrap())
        .method(&options.method.unwrap())
        .tags(options.tags);

    let route_handler = route_handler.unwrap();
    add_request_details(&mut operation, Node::from(route_handler), symbol_table);
}

fn get_path_options(options: Option<&ExprOrSpread>) -> PathOptions {
    let mut path_options = PathOptions::new();
    if let Some(options) = options {
        load_options(&mut path_options, &options.as_node());
    }
    path_options
}

fn load_options(path_options: &mut PathOptions, node: &Node) {
    for child in node.children() {
        match child.kind() {
            NodeKind::ObjectLit => {
                let type_literal = child.to::<ObjectLit>().unwrap();
                for prop_or_spread in &type_literal.inner.props {
                    match prop_or_spread.as_prop() {
                        Some(prop) => match prop.as_ref() {
                            deno_ast::swc::ast::Prop::KeyValue(key_value) => match &key_value.key {
                                deno_ast::swc::ast::PropName::Ident(i) if i.sym.eq("method") => {
                                    path_options.method = match key_value.value.deref() {
                                        deno_ast::swc::ast::Expr::Lit(deno_ast::swc::ast::Lit::Str(s)) => {
                                            Some(s.value.to_string())
                                        }
                                        _ => None,
                                    }
                                }
                                deno_ast::swc::ast::PropName::Ident(i) if i.sym.eq("path") => {
                                    path_options.path = match key_value.value.deref() {
                                        deno_ast::swc::ast::Expr::Lit(deno_ast::swc::ast::Lit::Str(s)) => {
                                            Some(s.value.to_string())
                                        }
                                        _ => None,
                                    }
                                }
                                deno_ast::swc::ast::PropName::Ident(i) if i.sym.eq("tags") => {
                                    let mut tags = vec![];
                                    match key_value.value.deref() {
                                        deno_ast::swc::ast::Expr::Array(literal) => {
                                            for element in &literal.elems {
                                                if let Some(element) = element {
                                                    match element.expr.deref() {
                                                        deno_ast::swc::ast::Expr::Lit(
                                                            deno_ast::swc::ast::Lit::Str(s),
                                                        ) => tags.push(s.value.to_string()),
                                                        _ => {}
                                                    }
                                                }
                                            }
                                        }
                                        _ => {}
                                    }

                                    if tags.len() > 0 {
                                        path_options.tags = Some(tags);
                                    }
                                }
                                _ => {}
                            },
                            _ => {}
                        },
                        None => {}
                    }
                }
            }
            _ => load_options(path_options, &child),
        }
    }
}

fn add_request_details<'n>(
    operation: &mut ApiPathOperation,
    route_handler: Node<'n>,
    symbol_table: &mut DeclarationTable<'n>,
) -> () {
    for child in route_handler.children() {
        match child {
            Node::ArrowExpr(arrow_expression) => {
                for param in &arrow_expression.params {
                    add_request_params(operation, Node::from(param), symbol_table);
                }

                symbol_table.push_scope();
                find_response(operation, Node::from(arrow_expression.body), symbol_table);
                symbol_table.pop_scope();
            }
            _ => {}
        }
    }
}

fn add_request_params(operation: &mut ApiPathOperation, node: Node, symbol_table: &mut DeclarationTable) {
    for child in node.children() {
        match child {
            Node::TsTypeRef(type_ref) => match type_ref.type_name {
                TsEntityName::Ident(identifier) if identifier.sym().eq("BodyParam") => {
                    add_body_param_details(operation, type_ref, symbol_table);
                }
                TsEntityName::Ident(identifier) if identifier.sym().eq("Header") => {
                    add_param_details(operation, "header", type_ref);
                }
                TsEntityName::Ident(identifier) if identifier.sym().eq("QueryParam") => {
                    add_param_details(operation, "query", type_ref);
                }
                TsEntityName::Ident(identifier) if identifier.sym().eq("RouteParam") => {
                    add_param_details(operation, "path", type_ref);
                }
                _ => add_request_params(operation, type_ref.as_node(), symbol_table),
            },
            _ => add_request_params(operation, child, symbol_table),
        }
    }
}

fn add_body_param_details(
    operation: &mut ApiPathOperation,
    type_ref: &TsTypeRef,
    symbol_table: &mut DeclarationTable,
) -> () {
    let operation_param = operation.body();
    if let Some(type_params) = type_ref.type_params {
        match type_params.params.get(0) {
            Some(TsType::TsKeywordType(param_type)) => match param_type.inner.kind {
                TsKeywordTypeKind::TsNumberKeyword => {
                    operation_param.content().schema().primitive("number");
                }
                TsKeywordTypeKind::TsBooleanKeyword => {
                    operation_param.content().schema().primitive("boolean");
                }
                TsKeywordTypeKind::TsStringKeyword => {
                    operation_param.content().schema().primitive("string");
                }
                _ => {}
            },
            Some(TsType::TsTypeRef(type_ref)) => match type_ref.type_name {
                TsEntityName::Ident(identifier) => {
                    let reference = symbol_table.get_root_declaration(identifier.sym().to_string());
                    operation_param.content().schema().reference(Some(reference), false);
                }
                _ => {}
            },
            _ => {}
        }

        match type_params.params.get(1) {
            Some(TsType::TsLitType(required)) => match required.lit {
                TsLit::Bool(boolean) => {
                    operation_param.required(boolean.value());
                }
                _ => {}
            },
            _ => {}
        }

        match type_params.params.get(2) {
            Some(TsType::TsLitType(namespace)) => match &namespace.lit {
                TsLit::Str(literal_string) => {
                    operation_param
                        .content()
                        .schema()
                        .namespace(Some(literal_string.value().to_string()));
                }
                _ => {}
            },
            _ => {}
        }
    }
}

fn add_param_details(operation: &mut ApiPathOperation, location: &str, type_ref: &TsTypeRef) {
    let parameter_name = get_parameter_name(Node::from(type_ref));
    let operation_param = operation.param(&parameter_name, location);
    if let Some(type_params) = type_ref.type_params {
        match type_params.params.get(0) {
            Some(TsType::TsKeywordType(param_type)) => match param_type.inner.kind {
                TsKeywordTypeKind::TsNumberKeyword => {
                    operation_param.content().schema().primitive("number");
                }
                TsKeywordTypeKind::TsBooleanKeyword => {
                    operation_param.content().schema().primitive("boolean");
                }
                TsKeywordTypeKind::TsStringKeyword => {
                    operation_param.content().schema().primitive("string");
                }
                _ => {}
            },
            Some(TsType::TsTypeRef(type_ref)) => match type_ref.type_name {
                TsEntityName::Ident(identifier) => {
                    operation_param
                        .content()
                        .schema()
                        .reference(identifier.sym().to_string().into(), false);
                }
                _ => {}
            },
            _ => {}
        }

        match type_params.params.get(1) {
            Some(TsType::TsLitType(required)) => match required.lit {
                TsLit::Bool(boolean) => {
                    operation_param.required(boolean.value());
                }
                _ => {}
            },
            _ => {}
        }

        match type_params.params.get(2) {
            Some(TsType::TsLitType(namespace)) => match &namespace.lit {
                TsLit::Str(literal_string) => {
                    operation_param
                        .content()
                        .schema()
                        .namespace(Some(literal_string.value().to_string()));
                }
                _ => {}
            },
            _ => {}
        }

        match type_params.params.get(3) {
            Some(TsType::TsLitType(format)) => match &format.lit {
                TsLit::Str(literal_string) => {
                    operation_param
                        .content()
                        .schema()
                        .format(Some(literal_string.value().to_string()));
                }
                _ => {}
            },
            _ => {}
        }
    }
}

fn get_parameter_name(node: Node) -> String {
    match node {
        Node::TsPropertySignature(property) if property.key.is::<Ident>() => {
            let identifier = property.key.to::<Ident>().unwrap();
            identifier.sym().to_string()
        }
        other => match other.parent() {
            Some(parent) => get_parameter_name(parent),
            None => panic!("Could not find parameter name"),
        },
    }
}

fn find_response<'a>(operation: &mut ApiPathOperation, body: Node<'a>, symbol_table: &mut DeclarationTable<'a>) -> () {
    for child in body.children() {
        store_symbol_maybe(&child, symbol_table);

        match child {
            Node::Ident(identifier) if identifier.sym().eq("Response") => {
                add_response(operation, identifier.parent(), symbol_table)
            }
            other => find_response(operation, other, symbol_table),
        }
    }
}

fn add_response(operation: &mut ApiPathOperation, node: Node, symbol_table: &mut DeclarationTable) -> () {
    if let Some(call_expression) = node.to::<CallExpr>() {
        let response_type = match call_expression.args.get(0) {
            Some(arg) => match arg.expr {
                Expr::New(new_expression) => match new_expression.callee {
                    Expr::Ident(identifier) => Some(symbol_table.get_root_declaration(identifier.sym().to_string())),
                    _ => None,
                },
                Expr::Ident(response_type) => Some(symbol_table.get_root_declaration(response_type.sym().to_string())),
                Expr::TsAs(ts_as) => match ts_as.type_ann {
                    TsType::TsTypeRef(type_ref) => match type_ref.type_name {
                        TsEntityName::Ident(identifier) => {
                            Some(symbol_table.get_root_declaration(identifier.sym().to_string()))
                        }
                        _ => None,
                    },
                    _ => None,
                },
                Expr::TsTypeAssertion(type_assertion) => match type_assertion.type_ann {
                    TsType::TsTypeRef(type_ref) => match type_ref.type_name {
                        TsEntityName::Ident(identifier) => {
                            Some(symbol_table.get_root_declaration(identifier.sym().to_string()))
                        }
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            },
            None => None,
        };

        let options = match call_expression.args.get(1) {
            Some(arg) => match arg.expr {
                Expr::Object(options) => Some(get_response_options(options)),
                _ => None,
            },
            None => None,
        };

        if let Some(response_options) = options {
            operation.response(&response_type, response_options);
        }
    }
}

fn get_response_options(options: &ObjectLit) -> ResponseOptions {
    let mut response_options = ResponseOptions::new();

    for prop in &options.props {
        match prop {
            PropOrSpread::Prop(Prop::KeyValue(key_value)) => {
                let key = match key_value.key {
                    PropName::Ident(identifier) => Some(identifier.sym().to_string()),
                    _ => None,
                };

                let value = match key_value.value {
                    Expr::Lit(Lit::Str(value)) => Some(value.value().to_string()),
                    Expr::Lit(Lit::Num(value)) => value.raw().as_ref().map(|v| v.to_string()),
                    _ => None,
                };

                match key {
                    Some(k) if k.eq("description") => response_options.description = value,
                    Some(k) if k.eq("example") => response_options.example = value,
                    Some(k) if k.eq("namespace") => response_options.namespace = value,
                    Some(k) if k.eq("statusCode") => response_options.status_code = value,
                    _ => {}
                }
            }
            _ => {}
        }
    }

    response_options
}
