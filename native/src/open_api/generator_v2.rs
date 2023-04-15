// use std::{fs::File, io::Write};

use dprint_swc_ext::view::*;
use lazy_static::__Deref;

use crate::typescript::{Declaration, DeclarationTables};

use super::{
    open_api::{ApiPathOperation, OpenApi, PathOptions, ResponseOptions},
    symbol_table_helpers::store_symbol_maybe,
};

use deno_ast::{ParseParams, ParsedSource, SourceTextInfo};
use dprint_swc_ext::view::{with_ast_view_for_module, ModuleInfo, NodeTrait};
use url::Url;

pub struct GeneratorV2<'n> {
    symbol_tables: DeclarationTables<'n>,
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

        with_ast_view_for_module(
            ModuleInfo {
                module: result.module(),
                comments: None,
                text_info: Some(result.text_info()),
                tokens: None,
            },
            |module| self.find_paths(&mut open_api, &module.as_node(), file_path),
        );

        open_api
    }

    fn find_paths(&mut self, open_api: &mut OpenApi, node: &Node<'n>, file_path: &str) {
        store_symbol_maybe(node, file_path, &mut self.symbol_tables);

        for child in node.children() {
            match child.kind() {
                NodeKind::CallExpr => match child.to::<CallExpr>() {
                    Some(call_expr) => match call_expr.callee {
                        Callee::Expr(Expr::Ident(ident)) if ident.sym().eq("Path") => {
                            self.symbol_tables.push_scope(file_path);
                            self.add_path(open_api, &call_expr, file_path);
                            self.symbol_tables.pop_scope(file_path);
                        }
                        _ => self.find_paths(open_api, &child, file_path),
                    },
                    None => self.find_paths(open_api, &child, file_path),
                },
                _ => self.find_paths(open_api, &child, file_path),
            }
        }
    }

    fn store_symbol_maybe(&mut self, node: &Node<'n>, file_path: &str) -> () {
        match node {
            Node::ClassDecl(class_declaration) => {
                println!("{:?}", class_declaration.inner);
            }
            Node::ExportDecl(export_declaration) => {
                println!("{:?}", export_declaration.inner);
            }
            Node::ExportDefaultDecl(export_default_declaration) => {
                println!("{:?}", export_default_declaration.inner);
            }
            Node::FnDecl(function_declaration) => {
                println!("{:?}", function_declaration.inner);
            }
            Node::ImportDecl(import_declaration) => {
                for child in import_declaration.children() {
                    match child {
                        Node::ImportNamedSpecifier(specifier) => {
                            // TODO this will need module resolution
                            let src = import_declaration.src.value().to_string();
                            let name = specifier.local.sym().to_string();
                            self.symbol_tables.insert(
                                file_path,
                                name.to_string(),
                                Declaration::Import {
                                    name,
                                    source_file_name: src,
                                },
                            )
                        }
                        _ => {}
                    }
                }
            }
            Node::TsEnumDecl(ts_enum_declaration) => {
                println!("{:?}", ts_enum_declaration.inner);
            }
            Node::TsInterfaceDecl(ts_interface_declaration) => {
                println!("{:?}", ts_interface_declaration.inner);
            }
            Node::TsTypeAliasDecl(ts_type_alias_declaration) => {
                println!("{:?}", ts_type_alias_declaration.inner);
            }
            Node::VarDecl(variable_declaration) => {
                for declaration in &variable_declaration.decls {
                    match declaration.name {
                        Pat::Ident(identifier) => {
                            let name = identifier.id.sym().to_string();
                            match identifier.type_ann {
                                Some(type_annotation) => match type_annotation.type_ann {
                                    dprint_swc_ext::view::TsType::TsTypeRef(type_ref) => match type_ref.type_name {
                                        dprint_swc_ext::view::TsEntityName::Ident(identifier) => {
                                            let type_name = identifier.sym().to_string();
                                            self.symbol_tables.insert(
                                                file_path,
                                                name.to_string(),
                                                Declaration::Alias {
                                                    from: name,
                                                    to: type_name,
                                                },
                                            )
                                        }
                                        _ => {}
                                    },
                                    _ => {}
                                },
                                None => match declaration.init {
                                    Some(initializer) => {
                                        self.store_variable(&name, initializer.as_node(), file_path);
                                    }
                                    None => {}
                                },
                            }
                        }
                        _ => {}
                    };
                }
            }
            _ => {}
        }
    }

    fn store_variable(&mut self, name: &str, node: Node, file_path: &str) -> () {
        for child in node.children() {
            match child {
                Node::Ident(identifier) => {
                    let type_name = identifier.sym().to_string();
                    self.symbol_tables.insert(
                        file_path,
                        name.to_string(),
                        Declaration::Alias {
                            from: name.to_string(),
                            to: type_name,
                        },
                    )
                }
                Node::TsTypeRef(type_ref) => match type_ref.type_name {
                    dprint_swc_ext::view::TsEntityName::Ident(identifier) => {
                        let type_name = identifier.sym().to_string();
                        self.symbol_tables.insert(
                            file_path,
                            name.to_string(),
                            Declaration::Alias {
                                from: name.to_string(),
                                to: type_name,
                            },
                        )
                    }
                    _ => {}
                },
                _ => self.store_variable(name, child, file_path),
            }
        }
    }

    fn add_path(&mut self, open_api: &mut OpenApi, node: &CallExpr<'n>, file_path: &str) -> () {
        let args = &node.args;
        let route_handler = args.first().copied();
        let route_options = args.last().copied();
        let options = get_path_options(route_options);

        let operation = open_api
            .path(&options.path.unwrap())
            .method(&options.method.unwrap())
            .tags(options.tags);

        let route_handler = route_handler.unwrap();
        self.add_request_details(operation, Node::from(route_handler), file_path);
    }

    fn add_request_details(
        &mut self,
        operation: &mut ApiPathOperation,
        route_handler: Node<'n>,
        file_path: &str,
    ) -> () {
        for child in route_handler.children() {
            match child {
                Node::ArrowExpr(arrow_expression) => {
                    for param in &arrow_expression.params {
                        self.add_request_params(operation, Node::from(param), file_path);
                    }

                    self.symbol_tables.push_scope(file_path);
                    self.find_response(operation, Node::from(arrow_expression.body), file_path);
                    self.symbol_tables.pop_scope(file_path);
                }
                _ => {}
            }
        }
    }

    fn add_request_params(&mut self, operation: &mut ApiPathOperation, node: Node, file_path: &str) {
        for child in node.children() {
            match child {
                Node::TsTypeRef(type_ref) => match type_ref.type_name {
                    TsEntityName::Ident(identifier) if identifier.sym().eq("BodyParam") => {
                        self.add_body_param_details(operation, type_ref, file_path);
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
                    _ => self.add_request_params(operation, type_ref.as_node(), file_path),
                },
                _ => self.add_request_params(operation, child, file_path),
            }
        }
    }

    fn add_body_param_details(
        &mut self,
        operation: &mut ApiPathOperation,
        type_ref: &TsTypeRef,
        file_path: &str,
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
                        let reference = self
                            .symbol_tables
                            .get_root_declaration(file_path, identifier.sym().to_string());

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

    fn find_response(&mut self, operation: &mut ApiPathOperation, body: Node<'n>, file_path: &str) -> () {
        for child in body.children() {
            store_symbol_maybe(&child, file_path, &mut self.symbol_tables);

            match child {
                Node::Ident(identifier) if identifier.sym().eq("Response") => {
                    self.add_response(operation, identifier.parent(), file_path)
                }
                other => self.find_response(operation, other, file_path),
            }
        }
    }

    fn add_response(&mut self, operation: &mut ApiPathOperation, node: Node, file_path: &str) -> () {
        if let Some(call_expression) = node.to::<CallExpr>() {
            let response_type = match call_expression.args.get(0) {
                Some(arg) => match arg.expr {
                    Expr::New(new_expression) => match new_expression.callee {
                        Expr::Ident(identifier) => Some(
                            self.symbol_tables
                                .get_root_declaration(file_path, identifier.sym().to_string()),
                        ),
                        _ => None,
                    },
                    Expr::Ident(response_type) => Some(
                        self.symbol_tables
                            .get_root_declaration(file_path, response_type.sym().to_string()),
                    ),
                    Expr::TsAs(ts_as) => match ts_as.type_ann {
                        TsType::TsTypeRef(type_ref) => match type_ref.type_name {
                            TsEntityName::Ident(identifier) => Some(
                                self.symbol_tables
                                    .get_root_declaration(file_path, identifier.sym().to_string()),
                            ),
                            _ => None,
                        },
                        _ => None,
                    },
                    Expr::TsTypeAssertion(type_assertion) => match type_assertion.type_ann {
                        TsType::TsTypeRef(type_ref) => match type_ref.type_name {
                            TsEntityName::Ident(identifier) => Some(
                                self.symbol_tables
                                    .get_root_declaration(file_path, identifier.sym().to_string()),
                            ),
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
