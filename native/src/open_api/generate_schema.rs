use deno_ast::swc::ast::*;
use lazy_static::__Deref;

use crate::typescript::{Declaration, DeclarationTables, SchemyNode};

use super::{
    declaration_helpers::store_declaration_maybe,
    deferred_schemas::DeferredSchemas,
    open_api::{ApiPathOperation, ApiSchema, OpenApi, PathOptions, ResponseOptions},
};

pub fn from_source_file<'m>(
    open_api: &mut OpenApi,
    file_path: &str,
    get_syntax_tree: &mut impl FnMut(&str) -> &'m Module,
    symbol_tables: &mut DeclarationTables<'m>,
) -> () {
    let result = get_syntax_tree(file_path);
    let mut deferred_schemas = DeferredSchemas::new();
    let node = SchemyNode::Module {
        node: result,
        parent: None,
    };

    find_paths(open_api, node, file_path, &mut deferred_schemas, symbol_tables);

    while let Some(source_file_name) = deferred_schemas.next_module() {
        let result = get_syntax_tree(&source_file_name);
        let module = SchemyNode::Module {
            node: result,
            parent: None,
        };
        for child in module.children() {
            define_deferred_schemas(
                open_api,
                &child,
                &source_file_name,
                &mut deferred_schemas,
                symbol_tables,
            )
        }
    }
}

fn find_paths<'m>(
    open_api: &mut OpenApi,
    node: SchemyNode<'m>,
    file_path: &str,
    deferred_schemas: &mut DeferredSchemas,
    symbol_tables: &mut DeclarationTables<'m>,
) {
    store_declaration_maybe(&node, file_path, symbol_tables);

    for child in node.children() {
        match child {
            call_expression @ SchemyNode::CallExpr { node: _, parent: _ } => match call_expression.callee() {
                Some(SchemyNode::Callee {
                    node: Callee::Expr(expression),
                    parent: _,
                }) => match &**expression {
                    Expr::Ident(ident) if ident.sym.eq("Path") => {
                        symbol_tables.add_child_scope(file_path);
                        add_path(open_api, &call_expression, file_path, deferred_schemas, symbol_tables);
                        symbol_tables.parent_scope(file_path);
                    }
                    _ => find_paths(open_api, child.clone(), file_path, deferred_schemas, symbol_tables),
                },
                _ => {}
            },
            _ => find_paths(open_api, child.clone(), file_path, deferred_schemas, symbol_tables),
        }
    }
}

fn add_path(
    open_api: &mut OpenApi,
    node: &SchemyNode,
    file_path: &str,
    deferred_schemas: &mut DeferredSchemas,
    symbol_tables: &mut DeclarationTables,
) -> () {
    let args = node.args();
    let route_handler = args.first();
    let route_options = args.last();
    let options = get_path_options(route_options);

    let mut operation = ApiPathOperation::new();

    let route_handler = route_handler.unwrap();
    add_request_details(
        open_api,
        operation.tags(options.tags),
        route_handler,
        file_path,
        deferred_schemas,
        symbol_tables,
    );
    open_api
        .path(&options.path.unwrap())
        .add_operation(&options.method.unwrap(), operation);
}

fn add_request_details(
    open_api: &mut OpenApi,
    operation: &mut ApiPathOperation,
    route_handler: &SchemyNode,
    file_path: &str,
    deferred_schemas: &mut DeferredSchemas,
    symbol_tables: &mut DeclarationTables,
) -> () {
    if let arrow_expression @ SchemyNode::ArrowExpr { node: _, parent: _ } = route_handler {
        for param in &arrow_expression.params() {
            add_request_params(open_api, operation, param, file_path, deferred_schemas, symbol_tables);
        }

        symbol_tables.add_child_scope(file_path);
        find_response(
            open_api,
            operation,
            arrow_expression.body().unwrap(),
            file_path,
            deferred_schemas,
            symbol_tables,
        );
        symbol_tables.parent_scope(file_path);
    }
}

fn add_request_params(
    open_api: &mut OpenApi,
    operation: &mut ApiPathOperation,
    node: &SchemyNode,
    file_path: &str,
    deferred_schemas: &mut DeferredSchemas,
    symbol_tables: &mut DeclarationTables,
) {
    for node in node.children() {
        match node {
            SchemyNode::TsTypeRef {
                node: type_ref,
                parent: None,
            } => match type_ref.type_name {
                TsEntityName::Ident(identifier) if identifier.sym.eq("BodyParam") => {
                    add_body_param_details(
                        open_api,
                        operation,
                        type_ref,
                        file_path,
                        deferred_schemas,
                        symbol_tables,
                    );
                }
                TsEntityName::Ident(identifier) if identifier.sym.eq("Header") => {
                    add_param_details(
                        open_api,
                        operation,
                        "header",
                        &node,
                        file_path,
                        deferred_schemas,
                        symbol_tables,
                    );
                }
                TsEntityName::Ident(identifier) if identifier.sym.eq("QueryParam") => {
                    add_param_details(
                        open_api,
                        operation,
                        "query",
                        &node,
                        file_path,
                        deferred_schemas,
                        symbol_tables,
                    );
                }
                TsEntityName::Ident(identifier) if identifier.sym.eq("RouteParam") => {
                    add_param_details(
                        open_api,
                        operation,
                        "path",
                        &node,
                        file_path,
                        deferred_schemas,
                        symbol_tables,
                    );
                }
                // TODO support route handler params in separate module
                TsEntityName::Ident(identifier) => {
                    add_param_from_referenced_type(
                        open_api,
                        operation,
                        &identifier.sym,
                        file_path,
                        deferred_schemas,
                        symbol_tables,
                    );
                }
                _ => add_request_params(open_api, operation, &node, file_path, deferred_schemas, symbol_tables),
            },
            _ => add_request_params(open_api, operation, &node, file_path, deferred_schemas, symbol_tables),
        }
    }
}

fn add_param_details(
    open_api: &mut OpenApi,
    operation: &mut ApiPathOperation,
    location: &str,
    type_ref: &SchemyNode,
    file_path: &str,
    deferred_schemas: &mut DeferredSchemas,
    symbol_tables: &mut DeclarationTables,
) {
    let parameter_name = get_parameter_name(type_ref);
    let operation_param = operation.param(&parameter_name, location);
    if let SchemyNode::TsTypeRef {
        node: type_ref,
        parent: _,
    } = type_ref
    {
        if let Some(type_params) = type_ref.type_params {
            let namespace = match type_params.params.get(2) {
                Some(namespace) => match **namespace {
                    TsType::TsLitType(namespace) => match &namespace.lit {
                        TsLit::Str(literal_string) => Some(literal_string.value.to_string()),
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            };

            match type_params.params.get(0) {
                Some(param) => match **param {
                    TsType::TsKeywordType(param_type) => match param_type.kind {
                        TsKeywordTypeKind::TsNumberKeyword => {
                            operation_param.content().schema().data_type("number");
                        }
                        TsKeywordTypeKind::TsBooleanKeyword => {
                            operation_param.content().schema().data_type("boolean");
                        }
                        TsKeywordTypeKind::TsStringKeyword => {
                            operation_param.content().schema().data_type("string");
                        }
                        _ => {}
                    },
                    TsType::TsTypeRef(type_ref) => match type_ref.type_name {
                        TsEntityName::Ident(identifier) => {
                            let reference = identifier.sym.to_string();

                            define_referenced_schema(
                                open_api,
                                &reference,
                                &reference,
                                file_path,
                                namespace.clone(),
                                deferred_schemas,
                                symbol_tables,
                            );

                            operation_param
                                .content()
                                .schema()
                                .reference(reference.into(), false)
                                .namespace(namespace);
                        }
                        _ => {}
                    },
                    _ => {}
                },
                None => {}
            }

            match type_params.params.get(1) {
                Some(param) => match **param {
                    TsType::TsLitType(required) => match required.lit {
                        TsLit::Bool(boolean) => {
                            operation_param.required(boolean.value);
                        }
                        _ => {}
                    },
                    _ => {}
                },
                None => {}
            }

            match type_params.params.get(3) {
                Some(param) => match **param {
                    TsType::TsLitType(format) => match &format.lit {
                        TsLit::Str(literal_string) => {
                            operation_param
                                .content()
                                .schema()
                                .format(Some(literal_string.value.to_string()));
                        }
                        _ => {}
                    },
                    _ => {}
                },
                None => {}
            }
        }
    }
}

// TODO add schema for ref here
fn add_body_param_details(
    open_api: &mut OpenApi,
    operation: &mut ApiPathOperation,
    type_ref: &TsTypeRef,
    file_path: &str,
    deferred_schemas: &mut DeferredSchemas,
    symbol_tables: &mut DeclarationTables,
) -> () {
    let operation_param = operation.body();
    if let Some(type_params) = type_ref.type_params {
        let namespace = match type_params.params.get(2) {
            Some(namespace) => match **namespace {
                TsType::TsLitType(namespace) => match &namespace.lit {
                    TsLit::Str(literal_string) => Some(literal_string.value.to_string()),
                    _ => None,
                },
                _ => None,
            },
            None => None,
        };

        match type_params.params.get(0) {
            Some(param_type) => match **param_type {
                TsType::TsKeywordType(param_type) => match param_type.kind {
                    TsKeywordTypeKind::TsNumberKeyword => {
                        operation_param.content().schema().data_type("number");
                    }
                    TsKeywordTypeKind::TsBooleanKeyword => {
                        operation_param.content().schema().data_type("boolean");
                    }
                    TsKeywordTypeKind::TsStringKeyword => {
                        operation_param.content().schema().data_type("string");
                    }
                    _ => {}
                },
                TsType::TsTypeRef(type_ref) => match type_ref.type_name {
                    TsEntityName::Ident(identifier) => {
                        let reference = symbol_tables.get_root_declaration_name(file_path, identifier.sym.to_string());

                        define_referenced_schema(
                            open_api,
                            &reference,
                            &reference,
                            file_path,
                            namespace.clone(),
                            deferred_schemas,
                            symbol_tables,
                        );

                        operation_param
                            .content()
                            .schema()
                            .reference(Some(reference), false)
                            .namespace(namespace);
                    }
                    _ => {}
                },
                _ => {}
            },
            None => {}
        }

        match type_params.params.get(1) {
            Some(required) => match **required {
                TsType::TsLitType(required) => match required.lit {
                    TsLit::Bool(boolean) => {
                        operation_param.required(boolean.value);
                    }
                    _ => {}
                },
                _ => {}
            },
            None => {}
        }
    }
}

fn find_response(
    open_api: &mut OpenApi,
    operation: &mut ApiPathOperation,
    body: SchemyNode,
    file_path: &str,
    deferred_schemas: &mut DeferredSchemas,
    symbol_tables: &mut DeclarationTables,
) -> () {
    for child in body.children() {
        store_declaration_maybe(&child, file_path, symbol_tables);

        match child {
            SchemyNode::Ident {
                node: identifier,
                parent: _,
            } if identifier.sym.eq("Response") => add_response(
                open_api,
                operation,
                child.parent().unwrap(),
                file_path,
                deferred_schemas,
                symbol_tables,
            ),
            other => find_response(open_api, operation, other, file_path, deferred_schemas, symbol_tables),
        }
    }
}

// TODO add schema for ref here
fn add_response(
    open_api: &mut OpenApi,
    operation: &mut ApiPathOperation,
    node: &SchemyNode,
    file_path: &str,
    deferred_schemas: &mut DeferredSchemas,
    symbol_tables: &mut DeclarationTables,
) -> () {
    if let SchemyNode::CallExpr {
        node: call_expression,
        parent: _,
    } = node
    {
        let options = match call_expression.args.get(1) {
            Some(arg) => match *arg.expr {
                Expr::Object(options) => Some(get_response_options(&options)),
                _ => None,
            },
            None => None,
        };

        let namespace = match &options {
            Some(options) => options.namespace.clone(),
            None => None,
        };

        let response_type = match call_expression.args.get(0) {
            Some(arg) => match *arg.expr {
                Expr::New(new_expression) => match *new_expression.callee {
                    Expr::Ident(identifier) => {
                        Some(symbol_tables.get_root_declaration_name(file_path, identifier.sym.to_string()))
                    }
                    _ => None,
                },
                Expr::Ident(response_type) => {
                    Some(symbol_tables.get_root_declaration_name(file_path, response_type.sym.to_string()))
                }
                Expr::TsAs(ts_as) => match *ts_as.type_ann {
                    TsType::TsTypeRef(type_ref) => match type_ref.type_name {
                        TsEntityName::Ident(identifier) => {
                            Some(symbol_tables.get_root_declaration_name(file_path, identifier.sym.to_string()))
                        }
                        _ => None,
                    },
                    _ => None,
                },
                Expr::TsTypeAssertion(type_assertion) => match *type_assertion.type_ann {
                    TsType::TsTypeRef(type_ref) => match type_ref.type_name {
                        TsEntityName::Ident(identifier) => {
                            Some(symbol_tables.get_root_declaration_name(file_path, identifier.sym.to_string()))
                        }
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            },
            None => None,
        };

        if let Some(response_type) = &response_type {
            define_referenced_schema(
                open_api,
                &response_type,
                &response_type,
                file_path,
                namespace,
                deferred_schemas,
                symbol_tables,
            );
        }

        if let Some(response_options) = options {
            operation.response(&response_type, response_options);
        }
    }
}

fn define_referenced_schema(
    open_api: &mut OpenApi,
    schema_name: &str,
    type_reference: &str,
    file_path: &str,
    namespace: Option<String>,
    deferred_schemas: &mut DeferredSchemas,
    symbol_tables: &mut DeclarationTables,
) -> () {
    match symbol_tables.get_root_declaration(file_path, type_reference) {
        Some(Declaration::Export {
            name: type_name,
            source_file_name,
        }) => {
            deferred_schemas.add_deferred_type(source_file_name, schema_name.into(), type_name, namespace);
        }
        Some(Declaration::Import {
            name: type_name,
            source_file_name,
        }) => {
            deferred_schemas.add_deferred_type(source_file_name, schema_name.into(), type_name, namespace);
        }
        Some(Declaration::Type { node }) => {
            let schema = match namespace {
                Some(ns) => open_api.components.schema(&ns).property(schema_name.into()),
                None => open_api.components.schema(schema_name),
            };

            define_referenced_schema_details(schema, node);
        }
        _ => {}
    };
}

fn define_deferred_schemas(
    open_api: &mut OpenApi,
    node: &SchemyNode,
    source_file_name: &str,
    deferred_schemas: &mut DeferredSchemas,
    symbol_tables: &mut DeclarationTables,
) -> () {
    store_declaration_maybe(node, source_file_name, symbol_tables);

    match node {
        SchemyNode::ExportDefaultExpr { node: _, parent: _ } => {
            define_deferred_type_maybe(open_api, "default", source_file_name, deferred_schemas, symbol_tables)
        }
        SchemyNode::ExportDecl {
            node: export_declaration,
            parent: _,
        } => match export_declaration.decl {
            Decl::Class(class_declaration) => {
                let name = class_declaration.ident.sym.to_string();
                define_deferred_type_maybe(open_api, &name, source_file_name, deferred_schemas, symbol_tables);
            }
            Decl::TsInterface(interface_declaration) => {
                let name = interface_declaration.id.sym.to_string();
                define_deferred_type_maybe(open_api, &name, source_file_name, deferred_schemas, symbol_tables);
            }
            Decl::TsTypeAlias(alias_declaration) => {
                let name = alias_declaration.id.sym.to_string();
                define_deferred_type_maybe(open_api, &name, source_file_name, deferred_schemas, symbol_tables);
            }
            _ => {}
        },
        SchemyNode::NamedExport {
            node: named_export,
            parent: _,
        } => {
            for specifier in &named_export.specifiers {
                match specifier {
                    ExportSpecifier::Named(named) => {
                        let name = match named.exported {
                            Some(exported) => match exported {
                                ModuleExportName::Ident(id) => id.sym.to_string(),
                                ModuleExportName::Str(id) => id.value.to_string(),
                            },
                            None => match named.orig {
                                ModuleExportName::Ident(id) => id.sym.to_string(),
                                ModuleExportName::Str(id) => id.value.to_string(),
                            },
                        };

                        define_deferred_type_maybe(open_api, &name, source_file_name, deferred_schemas, symbol_tables);
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

fn define_deferred_type_maybe(
    open_api: &mut OpenApi,
    type_name: &str,
    source_file_name: &str,
    deferred_schemas: &mut DeferredSchemas,
    symbol_tables: &mut DeclarationTables,
) -> () {
    if let Some(deferred_type) = deferred_schemas.get_deferred_type(type_name, source_file_name) {
        match symbol_tables.get_root_declaration(source_file_name, &type_name) {
            Some(Declaration::Type { node }) => {
                let schema = match &deferred_type.namespace {
                    Some(ns) => open_api
                        .components
                        .schema(&ns)
                        .data_type("object")
                        .property(&deferred_type.schema_name),
                    None => open_api.components.schema(&deferred_type.schema_name),
                };

                define_referenced_schema_details(schema, node);
            }
            Some(Declaration::Import {
                name: imported_name,
                source_file_name: module_file_name,
            }) => {
                deferred_schemas.add_deferred_type(
                    module_file_name,
                    type_name.to_string(),
                    imported_name,
                    deferred_type.namespace.clone(),
                );
            }
            _ => {}
        }
    }
}

fn add_param_from_referenced_type(
    open_api: &mut OpenApi,
    operation: &mut ApiPathOperation,
    type_ref: &str,
    file_path: &str,
    deferred_schemas: &mut DeferredSchemas,
    symbol_tables: &mut DeclarationTables,
) -> () {
    match symbol_tables.get_root_declaration(file_path, type_ref) {
        // Some(Declaration::Import { name, source_file_name }) => find_referenced_type(file_path, name, |declaration: dprint_swc_ext::swc::ast::Decl| {
        //     match declaration {
        //         dprint_swc_ext::swc::ast::Decl::TsInterface(interface_declaration) => {
        //             let name = interface_declaration.id.sym().to_string();
        //             add_param_from_referenced_type(operation, &name, source_file_name);
        //         }
        //         dprint_swc_ext::swc::ast::Decl::TsTypeAlias(alias_declaration) => {
        //             let name = alias_declaration.id.sym().to_string();
        //             add_param_from_referenced_type(operation, &name, source_file_name);
        //         }
        //         _ => {}
        //     }
        // }),
        Some(Declaration::Type { node }) => {
            add_request_params(open_api, operation, &node, file_path, deferred_schemas, symbol_tables)
        }
        _ => {}
    }
}

// helpers
fn define_referenced_schema_details(root_schema: &mut ApiSchema, node: SchemyNode) {
    match node {
        SchemyNode::TsKeywordType {
            node: keyword_type,
            parent: _,
        } => match keyword_type.kind {
            TsKeywordTypeKind::TsNumberKeyword => {
                root_schema.data_type("number".into());
            }
            TsKeywordTypeKind::TsBooleanKeyword => {
                root_schema.data_type("boolean".into());
            }
            TsKeywordTypeKind::TsBigIntKeyword => {
                root_schema.data_type("number".into());
            }
            TsKeywordTypeKind::TsStringKeyword => {
                root_schema.data_type("string".into());
            }
            TsKeywordTypeKind::TsSymbolKeyword => {
                root_schema.data_type("string".into());
            }
            _ => {}
        },
        SchemyNode::ClassDecl {
            node: class_declaration,
            parent: _,
        } => {
            root_schema.data_type("object".into());
            for property in &class_declaration.class.body {
                match property {
                    ClassMember::ClassProp(class_property) => {
                        let name = match class_property.key {
                            PropName::Ident(identifier) => Some(identifier.sym.to_string()),
                            _ => None,
                        };

                        if let Some(name) = name {
                            if let Some(annotation) = class_property.type_ann {
                                define_referenced_schema_details(
                                    root_schema.property(&name),
                                    SchemyNode::TsType {
                                        node: &annotation.type_ann,
                                        parent: None,
                                    },
                                );
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        SchemyNode::ClassExpr {
            node: class_declaration,
            parent: _,
        } => {
            root_schema.data_type("object".into());
            for property in &class_declaration.class.body {
                match property {
                    ClassMember::ClassProp(class_property) => {
                        let name = match class_property.key {
                            PropName::Ident(identifier) => Some(identifier.sym.to_string()),
                            _ => None,
                        };

                        if let Some(name) = name {
                            if let Some(annotation) = class_property.type_ann {
                                define_referenced_schema_details(
                                    root_schema.property(&name),
                                    SchemyNode::TsType {
                                        node: &annotation.type_ann,
                                        parent: None,
                                    },
                                );
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        SchemyNode::TsArrayType {
            node: array_type,
            parent: _,
        } => {
            root_schema.data_type("array".into());
            define_referenced_schema_details(
                root_schema.items(),
                SchemyNode::TsType {
                    node: &array_type.elem_type,
                    parent: None,
                },
            );
        }
        SchemyNode::TsInterfaceDecl {
            node: interface_declaration,
            parent: _,
        } => {
            root_schema.data_type("object".into());

            for property in &interface_declaration.body.body {
                match property {
                    TsTypeElement::TsPropertySignature(signature) => {
                        let property_schema = match *signature.key {
                            Expr::Ident(identifier) => {
                                let name = identifier.sym.to_string();
                                Some(root_schema.property(&name))
                            }
                            _ => None,
                        };

                        if let Some(property_schema) = property_schema {
                            if let Some(annotation) = signature.type_ann {
                                define_referenced_schema_details(
                                    property_schema,
                                    SchemyNode::TsType {
                                        node: &annotation.type_ann,
                                        parent: None,
                                    },
                                );
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        SchemyNode::TsTypeLit {
            node: type_literal,
            parent: _,
        } => {
            root_schema.data_type("object".into());
            for member in &type_literal.members {
                match member {
                    TsTypeElement::TsPropertySignature(signature) => {
                        let property_schema = match *signature.key {
                            Expr::Ident(identifier) => {
                                let name = identifier.sym.to_string();
                                Some(root_schema.property(&name))
                            }
                            _ => None,
                        };

                        if let Some(property_schema) = property_schema {
                            if let Some(annotation) = signature.type_ann {
                                define_referenced_schema_details(
                                    property_schema,
                                    SchemyNode::TsType {
                                        node: &annotation.type_ann,
                                        parent: None,
                                    },
                                );
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        SchemyNode::TsTypeAliasDecl {
            node: type_alias_declaration,
            parent: _,
        } => define_referenced_schema_details(
            root_schema,
            SchemyNode::TsType {
                node: &type_alias_declaration.type_ann,
                parent: None,
            },
        ),
        _ => {}
    }
}

// fn get_syntax_tree(path: &str) -> ParsedSource {
//     let specifier = Url::from_file_path(path).unwrap();
//     let source_text = std::fs::read_to_string(specifier.path()).unwrap();
//     let parsed_source = deno_ast::parse_module(ParseParams {
//         capture_tokens: true,
//         maybe_syntax: None,
//         media_type: deno_ast::MediaType::TypeScript,
//         scope_analysis: false,
//         specifier: specifier.to_string(),
//         text_info: SourceTextInfo::new(source_text.into()),
//     })
//     .unwrap();

//     parsed_source
// }

fn get_path_options(options: Option<&SchemyNode>) -> PathOptions {
    let mut path_options = PathOptions::new();
    if let Some(options) = options {
        load_options(&mut path_options, options);
    }
    path_options
}

fn load_options(path_options: &mut PathOptions, node: &SchemyNode) {
    for child in node.children() {
        match child {
            SchemyNode::ObjectLit {
                node: type_literal,
                parent,
            } => {
                for prop_or_spread in &type_literal.props {
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

fn get_parameter_name(node: &SchemyNode) -> String {
    match node {
        SchemyNode::TsPropertySignature {
            node: property,
            parent: _,
        } if property.key.is_ident() => {
            let identifier = property.key.as_ident().unwrap();
            identifier.sym.to_string()
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
            PropOrSpread::Prop(prop) => match **prop {
                Prop::KeyValue(key_value) => {
                    let key = match key_value.key {
                        PropName::Ident(identifier) => Some(identifier.sym.to_string()),
                        _ => None,
                    };

                    let value = match *key_value.value {
                        Expr::Lit(Lit::Str(value)) => Some(value.value.to_string()),
                        Expr::Lit(Lit::Num(value)) => value.raw.as_ref().map(|v| v.to_string()),
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
            },
            _ => {}
        }
    }

    response_options
}
