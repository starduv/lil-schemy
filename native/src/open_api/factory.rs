use std::{cell::RefCell, ops::Deref, rc::Rc};

use swc_ecma_ast::*;

use crate::typescript::{ModuleCache, NodeKind, SchemyNode};

use super::{
    schema::{ApiPathOperation, ApiSchema, OpenApi, PathOptions, ResponseOptions},
    state::{Declaration, Store},
};

pub fn append_schema(open_api: &mut OpenApi, file_path: &str, module_cache: &mut ModuleCache, store: &mut Store) -> () {
    let root = module_cache.parse(&file_path);
    find_paths(open_api, root.clone(), file_path, store);

    while store.has_unrecognized_local_types(file_path) {
        define_local_schemas(file_path, open_api, store);
    }
}

pub fn append_deferred_schemas(open_api: &mut OpenApi, module_cache: &mut ModuleCache, store: &mut Store) -> () {
    while let Some(file_path) = store.next_module() {
        let deferred_root = module_cache.parse(&file_path);
        for item in deferred_root.children() {
            define_external_schema(open_api, item, &file_path, store);
        }

        while store.has_unrecognized_local_types(&file_path) {
            define_local_schemas(&file_path, open_api, store);
        }
    }
}

fn define_local_schemas(file_path: &str, open_api: &mut OpenApi, store: &mut Store) {
    for local_type in store.recognize_local_types(file_path) {
        define_local_schema(
            &local_type.type_name,
            &local_type.schema_name,
            open_api,
            file_path,
            store,
        )
    }
}

fn find_paths(open_api: &mut OpenApi, root: Rc<SchemyNode<'static>>, file_path: &str, store: &mut Store) {
    store.store_declaration_maybe(root.clone(), file_path);

    for child in root.children() {
        match &child.kind {
            NodeKind::Ident(raw_ident) if raw_ident.sym.eq("LilPath") => {
                let parent = child.parent().unwrap().parent().unwrap();
                store.add_child_scope(file_path);
                add_path(open_api, parent, file_path, store);
                store.parent_scope(file_path);
            }
            _ => find_paths(open_api, child, file_path, store),
        }
    }
}

fn add_path(open_api: &mut OpenApi, root: Rc<SchemyNode<'static>>, file_path: &str, store: &mut Store) -> () {
    let args = root.args();
    let route_handler = args.first().unwrap().as_arrow_expr().unwrap();
    let route_options = args.last().unwrap();
    let options = get_path_options(route_options.clone());

    if let Some(path) = &options.path {
        if let Some(method) = &options.method {
            let operation = open_api.path(&path).add_operation(&method).clone();

            {
                let mut borrow = (*operation).borrow_mut();
                borrow.tags(options.tags.clone());
            }

            add_request_details(&operation, route_handler.clone(), file_path, &options, store);
        }
    }
}

fn add_request_details(
    operation: &Rc<RefCell<ApiPathOperation>>,
    route_handler: Rc<SchemyNode<'static>>,
    file_path: &str,
    path_options: &PathOptions,
    store: &mut Store,
) -> () {
    for param in route_handler.params() {
        add_request_params(operation, param, file_path, path_options, store);
    }

    store.add_child_scope(file_path);

    find_response(operation, route_handler, file_path, path_options, &mut "".into(), store);

    store.parent_scope(file_path);
}

fn add_request_params(
    operation: &Rc<RefCell<ApiPathOperation>>,
    root: Rc<SchemyNode<'static>>,
    file_path: &str,
    path_options: &PathOptions,
    store: &mut Store,
) {
    match root.kind {
        NodeKind::Ident(identifier) if identifier.sym.eq("LilBodyParam") => {
            add_body_param_details(operation, find_parent_type_ref(root), file_path, path_options, store);
        }
        NodeKind::Ident(identifier) if identifier.sym.eq("LilHeader") => {
            add_param_details(
                operation,
                "header",
                find_parent_type_ref(root),
                file_path,
                true,
                path_options,
                store,
            );
        }
        NodeKind::Ident(identifier) if identifier.sym.eq("LilQueryParam") => {
            add_param_details(
                operation,
                "query",
                find_parent_type_ref(root),
                file_path,
                false,
                path_options,
                store,
            );
        }
        NodeKind::Ident(identifier) if identifier.sym.eq("LilRouteParam") => {
            add_param_details(
                operation,
                "path",
                find_parent_type_ref(root),
                file_path,
                true,
                path_options,
                store,
            );
        }
        NodeKind::Ident(identifier) => match store.get_root_declaration(file_path, &identifier.sym) {
            Some(Declaration::Import { name, source_file_name }) => {
                store.defer_operation_type(&source_file_name, operation, &name);
            }
            Some(Declaration::Type { node }) => {
                add_request_params(operation, node, file_path, path_options, store);
            }
            _ => {}
        },
        _ => {
            for child in root.children() {
                add_request_params(operation, child, file_path, path_options, store);
            }
        }
    }
}

fn add_param_details(
    operation: &Rc<RefCell<ApiPathOperation>>,
    location: &str,
    root: Rc<SchemyNode<'static>>,
    file_path: &str,
    required_default: bool,
    path_options: &PathOptions,
    store: &mut Store,
) {
    let mut operation = (**operation).borrow_mut();
    let parameter_name = get_parameter_name(root.clone());
    let operation_param = operation.param(&parameter_name, location);

    let type_params = root.params();
    if let Some(type_param) = type_params.get(0) {
        let param_schema = operation_param.content(None).schema();
        define_schema_details(param_schema, &type_param, file_path, false, path_options, store);
    }

    match type_params.get(1) {
        Some(param) => match &param.kind {
            NodeKind::TsType(required) => match required {
                TsType::TsLitType(raw) => match raw.lit {
                    TsLit::Bool(raw_bool) => {
                        operation_param.required(raw_bool.value);
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        },
        None => {
            operation_param.required(required_default);
        }
    }

    match type_params.get(2) {
        Some(param) => match param.kind {
            NodeKind::TsType(raw_type) => match raw_type {
                TsType::TsLitType(raw_lit) => match &raw_lit.lit {
                    TsLit::Str(raw_str) => {
                        operation_param
                            .content(None)
                            .schema()
                            .format(Some(raw_str.value.to_string()));
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        },
        None => {}
    }
}

fn find_response(
    operation: &Rc<RefCell<ApiPathOperation>>,
    root: Rc<SchemyNode<'static>>,
    file_path: &str,
    path_options: &PathOptions,
    depth: &mut String,
    store: &mut Store,
) -> () {
    for child in root.children() {
        store.store_declaration_maybe(child.clone(), file_path);
        match child.kind {
            NodeKind::Ident(raw) if raw.sym.eq("LilResponse") => {
                add_response(operation, root.parent().unwrap(), file_path, path_options, store)
            }
            _ => find_response(operation, child, file_path, path_options, &mut depth.clone(), store),
        }
    }
}

fn add_response(
    operation: &Rc<RefCell<ApiPathOperation>>,
    root: Rc<SchemyNode<'static>>,
    file_path: &str,
    path_options: &PathOptions,
    store: &mut Store,
) -> () {
    let args = root.args();
    let options = match args.get(1) {
        Some(arg) => match arg.kind {
            NodeKind::ExprOrSpread(raw) => match &*raw.expr {
                Expr::Object(options) => Some(get_response_options(&options)),
                _ => None,
            },
            _ => None,
        },
        None => None,
    };

    match args.get(0) {
        Some(response_type) if options.is_some() => {
            add_response_details(
                &response_type,
                &options.unwrap(),
                file_path,
                operation,
                path_options,
                store,
            );
        }
        _ => {}
    };
}

fn add_response_details(
    root: &Rc<SchemyNode<'static>>,
    options: &ResponseOptions,
    file_path: &str,
    operation: &Rc<RefCell<ApiPathOperation>>,
    path_options: &PathOptions,
    store: &mut Store,
) {
    let status_code = options.status_code.as_ref().unwrap();
    let description = options.description.as_ref().unwrap();
    let media_type = options.media_type.as_ref().map(|s| s.as_str());

    let mut operation = (**operation).borrow_mut();
    let response = operation.response(&status_code, &description);
    let content = response.content(media_type);

    define_schema_details(content.schema(), root, file_path, true, path_options, store);

    content.example(options.example.clone());
}

fn add_body_param_details(
    operation: &Rc<RefCell<ApiPathOperation>>,
    root: Rc<SchemyNode<'static>>,
    file_path: &str,
    path_options: &PathOptions,
    store: &mut Store,
) -> () {
    let mut operation = (**operation).borrow_mut();
    let operation_param = operation.body();
    let type_params = root.params();

    let content: Option<&str> = match type_params.get(2) {
        Some(param) => match &param.kind {
            NodeKind::TsType(media_type) => match media_type {
                TsType::TsLitType(raw) => match &raw.lit {
                    TsLit::Str(raw_str) => Some(&raw_str.value),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        },
        None => None,
    };

    match type_params.get(0) {
        Some(param) => match param.kind {
            NodeKind::TsType(raw_type) => match raw_type {
                TsType::TsTypeLit(raw_lit) => {
                    let child = root.to_child(NodeKind::TsTypeLit(raw_lit));
                    define_schema_details(
                        operation_param.content(content).schema(),
                        &child,
                        file_path,
                        false,
                        &PathOptions::default(),
                        store,
                    );
                }
                TsType::TsKeywordType(raw_keyword) => match raw_keyword.kind {
                    TsKeywordTypeKind::TsNumberKeyword => {
                        operation_param.content(content).schema().data_type("number");
                    }
                    TsKeywordTypeKind::TsBooleanKeyword => {
                        operation_param.content(content).schema().data_type("boolean");
                    }
                    TsKeywordTypeKind::TsStringKeyword => {
                        operation_param.content(content).schema().data_type("string");
                    }
                    _ => {}
                },
                TsType::TsTypeRef(raw_type) => match &raw_type.type_name {
                    TsEntityName::Ident(identifier) => {
                        let identifier = identifier.sym.to_string();
                        let root_schema = operation_param.content(content).schema();
                        define_schema_from_identifier(
                            &identifier,
                            root_schema,
                            file_path,
                            path_options,
                            &root,
                            true,
                            store,
                        );
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        },
        None => {}
    }

    match type_params.get(1) {
        Some(param) => match &param.kind {
            NodeKind::TsType(required) => match required {
                TsType::TsLitType(raw) => match raw.lit {
                    TsLit::Bool(raw_bool) => {
                        operation_param.required(raw_bool.value);
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        },
        None => {}
    }
}

fn define_local_schema(
    type_name: &str,
    schema_name: &str,
    open_api: &mut OpenApi,
    file_path: &str,
    store: &mut Store,
) -> () {
    if open_api.components.contains_schema(schema_name) {
        return;
    }

    match store.get_root_declaration(file_path, type_name) {
        Some(Declaration::Export {
            name: type_name,
            source_file_name,
        }) => {
            store.defer_external_type(&source_file_name, schema_name.into(), &type_name);
        }
        Some(Declaration::Import {
            name: type_name,
            source_file_name,
        }) => {
            store.defer_external_type(&source_file_name, schema_name.into(), &type_name);
        }
        Some(Declaration::Type { node }) => {
            let schema = open_api.components.schema_with_id(schema_name);
            define_schema_details(schema, &node, file_path, false, &PathOptions::default(), store);
        }
        _ => {}
    };
}

fn define_external_schema(
    open_api: &mut OpenApi,
    root: Rc<SchemyNode<'static>>,
    file_path: &str,
    store: &mut Store,
) -> () {
    store.store_declaration_maybe(root.clone(), file_path);
    match &root.kind {
        NodeKind::ModuleItem(ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(_))) => {
            define_external_schema_maybe(open_api, "default", file_path, store)
        }
        NodeKind::ModuleItem(ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultDecl(_))) => {
            define_external_schema_maybe(open_api, "default", file_path, store)
        }
        NodeKind::ModuleItem(ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(raw_decl))) => match &raw_decl.decl {
            Decl::Class(ref raw_class) => {
                define_external_schema_maybe(open_api, &raw_class.ident.sym, file_path, store)
            }
            Decl::TsInterface(ref raw_interface) => {
                define_external_schema_maybe(open_api, &raw_interface.id.sym, file_path, store)
            }
            Decl::TsTypeAlias(ref raw_alias) => {
                define_external_schema_maybe(open_api, &raw_alias.id.sym, file_path, store)
            }
            Decl::TsEnum(ref raw_alias) => define_external_schema_maybe(open_api, &raw_alias.id.sym, file_path, store),
            _ => {}
        },
        NodeKind::ModuleItem(ModuleItem::ModuleDecl(ModuleDecl::ExportNamed(raw_export))) => {
            let root = root.to_child(NodeKind::NamedExport(raw_export));
            for specifier in root.specifiers() {
                match specifier.kind {
                    NodeKind::ExportSpecifier(ExportSpecifier::Named(named)) => {
                        let name = match &named.exported {
                            Some(exported) => match exported {
                                ModuleExportName::Ident(id) => id.sym.to_string(),
                                ModuleExportName::Str(id) => id.value.to_string(),
                            },
                            None => match &named.orig {
                                ModuleExportName::Ident(id) => id.sym.to_string(),
                                ModuleExportName::Str(id) => id.value.to_string(),
                            },
                        };

                        define_external_schema_maybe(open_api, &name, file_path, store);
                    }
                    _ => {}
                }
            }
        }
        NodeKind::ModuleItem(ModuleItem::Stmt(Stmt::Decl(Decl::TsEnum(raw_enum)))) => {
            define_external_schema_maybe(open_api, &raw_enum.id.sym, file_path, store)
        }
        NodeKind::ModuleItem(ModuleItem::Stmt(Stmt::Decl(Decl::Class(raw_class)))) => {
            define_external_schema_maybe(open_api, &raw_class.ident.sym, file_path, store)
        }
        NodeKind::ModuleItem(ModuleItem::Stmt(Stmt::Decl(Decl::TsInterface(raw_interface)))) => {
            define_external_schema_maybe(open_api, &raw_interface.id.sym, file_path, store)
        }
        NodeKind::ModuleItem(ModuleItem::Stmt(Stmt::Decl(Decl::TsTypeAlias(raw_alias)))) => {
            define_external_schema_maybe(open_api, &raw_alias.id.sym, file_path, store)
        }
        NodeKind::ModuleItem(ModuleItem::Stmt(Stmt::Decl(Decl::TsModule(raw_module)))) => {
            let root = root.to_child(NodeKind::TsModuleDecl(raw_module));
            for child in root.children() {
                define_external_schema(open_api, child, file_path, store);
            }
        }
        _ => {}
    }
}

fn define_external_schema_maybe(open_api: &mut OpenApi, type_name: &str, file_path: &str, store: &mut Store) -> () {
    if open_api.components.contains_schema(type_name) {
        return;
    }

    if let Some(deferred_operation_type) = store.recognize_operation_type(type_name, file_path) {
        match store.get_root_declaration(file_path, type_name) {
            Some(Declaration::Type { node }) => {
                add_request_params(
                    &deferred_operation_type.operation,
                    node,
                    file_path,
                    &PathOptions::default(),
                    store,
                );
            }
            Some(Declaration::Import {
                name: imported_name,
                source_file_name: module_file_name,
            }) => {
                store.defer_operation_type(&module_file_name, &deferred_operation_type.operation, &imported_name);
            }
            _ => {}
        }
    }

    if let Some(deferred_type) = store.recognize_external_type(type_name, file_path) {
        match store.get_root_declaration(file_path, &type_name) {
            Some(Declaration::Type { node }) => {
                let schema = open_api.components.schema_with_id(&deferred_type.schema_name);
                define_schema_details(schema, &node, file_path, false, &PathOptions::default(), store);
            }
            Some(Declaration::Import {
                name: imported_name,
                source_file_name: module_file_name,
            }) => {
                store.defer_external_type(&module_file_name, type_name, &imported_name);
            }
            _ => {}
        }
    }
}

fn define_schema_details(
    root_schema: &mut ApiSchema,
    root: &Rc<SchemyNode<'static>>,
    file_path: &str,
    is_required: bool,
    path_options: &PathOptions,
    store: &mut Store,
) -> () {
    match root.kind {
        NodeKind::Ident(raw_ident) => match store.get_root_declaration(file_path, &raw_ident.sym) {
            Some(Declaration::Type { node }) => {
                define_schema_details(root_schema, &node, file_path, false, path_options, store);
            }
            _ => {
                let schema_name = store.get_root_declaration_name(file_path, &raw_ident.sym);

                define_schema_from_identifier(
                    &schema_name,
                    root_schema,
                    file_path,
                    path_options,
                    root,
                    is_required,
                    store,
                );
            }
        },
        NodeKind::TsUnionOrIntersectionType(_) => {
            for child in root.children() {
                define_schema_details(root_schema, &child, file_path, is_required, path_options, store);
            }
        }
        NodeKind::TsUnionType(_) => {
            let any_of = root_schema.any_of();
            let mut enum_schema = ApiSchema::new();
            for child in root.children() {
                match child.kind {
                    NodeKind::TsLitType(raw) => match &raw.lit {
                        TsLit::Number(raw_num) => enum_schema.enum_value(&format!("{}", &raw_num.value)),
                        TsLit::Str(raw_str) => enum_schema.enum_value(&format!("{}", &raw_str.value)),
                        TsLit::Bool(raw_bool) => enum_schema.enum_value(&format!("{}", &raw_bool.value)),
                        TsLit::BigInt(raw_int) => enum_schema.enum_value(&format!("{}", &raw_int.value)),
                        _ => {}
                    },
                    NodeKind::TsTypeLit(_) => {
                        let mut schema = ApiSchema::new();
                        define_schema_details(&mut schema, &child, file_path, is_required, path_options, store);
                        any_of.push(schema);
                    }
                    NodeKind::Ident(_) => {
                        let mut schema = ApiSchema::new();
                        define_schema_details(&mut schema, &child, file_path, is_required, path_options, store);
                        any_of.push(schema);
                    }
                    _ => {}
                }
            }

            if enum_schema.has_enums() {
                any_of.push(enum_schema);
            }
        }
        NodeKind::TsIntersectionType(_) => {
            let all_of = root_schema.all_of();
            let mut enum_schema = ApiSchema::new();
            for child in root.children() {
                match child.kind {
                    NodeKind::TsLitType(raw) => match &raw.lit {
                        TsLit::Number(raw_num) => enum_schema.enum_value(&format!("{}", &raw_num.value)),
                        TsLit::Str(raw_str) => enum_schema.enum_value(&format!("{}", &raw_str.value)),
                        TsLit::Bool(raw_bool) => enum_schema.enum_value(&format!("{}", &raw_bool.value)),
                        TsLit::BigInt(raw_int) => enum_schema.enum_value(&format!("{}", &raw_int.value)),
                        _ => {}
                    },
                    NodeKind::TsTypeLit(_) => {
                        let mut schema = ApiSchema::new();
                        define_schema_details(&mut schema, &child, file_path, is_required, path_options, store);
                        all_of.push(schema);
                    }
                    NodeKind::Ident(_) => {
                        let mut schema = ApiSchema::new();
                        define_schema_details(&mut schema, &child, file_path, is_required, path_options, store);
                        all_of.push(schema);
                    }
                    _ => {}
                }
            }

            if enum_schema.has_enums() {
                all_of.push(enum_schema);
            }
        }
        NodeKind::TsExprWithTypeArgs(raw_expr) => match &*raw_expr.expr {
            Expr::Ident(raw_ident) => {
                if raw_ident.sym.eq("Omit") || raw_ident.sym.eq("Pick") {
                    let params = root.params();
                    let param = params.first().unwrap();
                    define_schema_details(root_schema, &param, file_path, is_required, path_options, store);
                } else {
                    match store.get_root_declaration(file_path, &raw_ident.sym) {
                        Some(Declaration::Import { name, source_file_name }) => {
                            store.defer_external_type(&source_file_name, &name, &name);

                            root_schema.reference(Some(name), false);
                        }
                        _ => {
                            store.defer_local_type(file_path, &raw_ident.sym, &raw_ident.sym, root.clone());

                            root_schema.reference(Some(raw_ident.sym.to_string()), false);
                        }
                    }
                }
            }
            _ => {}
        },
        NodeKind::TsTypeRef(raw_type) => match &raw_type.type_name {
            TsEntityName::Ident(identifier) => {
                define_schema_from_identifier(
                    &identifier.sym,
                    root_schema,
                    file_path,
                    path_options,
                    root,
                    is_required,
                    store,
                );
            }
            _ => {}
        },
        NodeKind::TsTypeAnnotation(_) => {
            for child in root.children() {
                define_schema_details(root_schema, &child, file_path, is_required, path_options, store);
            }
        }
        NodeKind::TsKeywordType(raw) => match raw.kind {
            TsKeywordTypeKind::TsNumberKeyword => {
                root_schema.data_type("number");
            }
            TsKeywordTypeKind::TsBooleanKeyword => {
                root_schema.data_type("boolean");
            }
            TsKeywordTypeKind::TsBigIntKeyword => {
                root_schema.data_type("number");
            }
            TsKeywordTypeKind::TsStringKeyword => {
                root_schema.data_type("string");
            }
            TsKeywordTypeKind::TsSymbolKeyword => {
                root_schema.data_type("string");
            }
            _ => {}
        },
        NodeKind::ClassDecl(_) => {
            root_schema.data_type("object");
            if let Some(class_node) = root.class() {
                for property in class_node.class_props() {
                    match property.kind {
                        NodeKind::ClassProp(raw_prop) => {
                            let name = match &raw_prop.key {
                                PropName::Ident(identifier) => Some(identifier.sym.to_string()),
                                _ => None,
                            };

                            if let Some(name) = name {
                                if let Some(annotation) = property.type_ann() {
                                    let property_schema = root_schema.property(&name);

                                    if is_required {
                                        property_schema.required_field(&name);
                                    }

                                    define_schema_details(
                                        property_schema,
                                        &annotation,
                                        file_path,
                                        is_required,
                                        path_options,
                                        store,
                                    );
                                }
                            }
                        }
                        NodeKind::BindingIdent(raw_ident) => {
                            if let Some(annotation) = property.type_ann() {
                                let property_schema = root_schema.property(&raw_ident.sym.to_string());

                                if is_required {
                                    property_schema.required_field(&raw_ident.sym);
                                }

                                define_schema_details(
                                    property_schema,
                                    &annotation,
                                    file_path,
                                    is_required,
                                    path_options,
                                    store,
                                );
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        NodeKind::ClassExpr(_) => {
            root_schema.data_type("object");
            if let Some(class_node) = root.class() {
                for class_member in class_node.body() {
                    match class_member.kind {
                        NodeKind::ClassProp(raw) => {
                            let name = match &raw.key {
                                PropName::Ident(identifier) => Some(identifier.sym.to_string()),
                                _ => None,
                            };

                            if let Some(name) = name {
                                if let Some(annotation) = class_member.type_ann() {
                                    let property_schema = root_schema.property(&name);

                                    if is_required {
                                        property_schema.required_field(&name);
                                    }

                                    define_schema_details(
                                        property_schema,
                                        &annotation,
                                        file_path,
                                        is_required,
                                        path_options,
                                        store,
                                    );
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        NodeKind::TsArrayType(_) => {
            root_schema.data_type("array");
            let elem_type = root.elem_type().unwrap();
            define_schema_details(
                root_schema.items(),
                &elem_type,
                file_path,
                is_required,
                path_options,
                store,
            );
        }
        NodeKind::TsInterfaceDecl(_) => {
            let extends = root.extends();
            if extends.len().gt(&0) {
                let all_of = root_schema.all_of();
                for extend in &extends {
                    let mut schema = ApiSchema::new();
                    define_schema_details(&mut schema, &extend, file_path, is_required, path_options, store);
                    all_of.push(schema);
                }
            }

            let root_schema = match extends.len() > 0 {
                true => {
                    let all_of = root_schema.all_of();
                    all_of.push(ApiSchema::new());
                    all_of.last_mut().unwrap()
                }
                false => root_schema,
            };

            root_schema.data_type("object");

            if let Some(interface_body) = root.interface_body() {
                for interface_member in interface_body.body() {
                    match interface_member.kind {
                        NodeKind::TsTypeElement(TsTypeElement::TsPropertySignature(raw_prop)) => {
                            let property_schema = match &*raw_prop.key {
                                Expr::Ident(identifier) => {
                                    let name = identifier.sym.to_string();
                                    let property_schema = root_schema.property(&name);

                                    if is_required {
                                        property_schema.required_field(&name);
                                    }

                                    Some(property_schema)
                                }
                                _ => None,
                            };

                            if let Some(property_schema) = property_schema {
                                if let Some(annotation) = interface_member.type_ann() {
                                    define_schema_details(
                                        property_schema,
                                        &annotation,
                                        file_path,
                                        is_required,
                                        path_options,
                                        store,
                                    );
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        NodeKind::TsTypeLit(_) => {
            root_schema.data_type("object");
            for member in root.members() {
                match member.kind {
                    NodeKind::TsTypeElement(raw_element) => match raw_element {
                        TsTypeElement::TsPropertySignature(raw_prop) => {
                            let property_schema = match &*raw_prop.key {
                                Expr::Ident(identifier) => {
                                    let name = identifier.sym.to_string();
                                    let property_schema = root_schema.property(&name);

                                    if is_required {
                                        property_schema.required_field(&name);
                                    }

                                    Some(property_schema)
                                }
                                _ => None,
                            };

                            if let Some(property_schema) = property_schema {
                                if let Some(annotation) = member.type_ann() {
                                    define_schema_details(
                                        property_schema,
                                        &annotation,
                                        file_path,
                                        is_required,
                                        path_options,
                                        store,
                                    );
                                }
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
        NodeKind::TsTypeAliasDecl(_) => {
            if let Some(annotation) = root.type_ann() {
                define_schema_details(root_schema, &annotation, file_path, is_required, path_options, store);
            }
        }
        NodeKind::TsType(_) => {
            for child in root.children() {
                define_schema_details(root_schema, &child, file_path, is_required, path_options, store);
            }
        }
        NodeKind::TsEnumDecl(_) => {
            for member in root.members() {
                root_schema.data_type("string");
                define_schema_details(root_schema, &member, file_path, is_required, path_options, store);
            }
        }
        NodeKind::TsEnumMember(raw_member) => match &raw_member.init {
            Some(expr) => match &**expr {
                Expr::Lit(raw_lit) => match raw_lit {
                    Lit::Str(raw_str) => {
                        root_schema.enum_value(&raw_str.value);
                    }
                    Lit::Num(raw_num) => {
                        if let Some(raw) = &raw_num.raw {
                            root_schema.enum_value(&raw);
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
            None => {}
        },
        NodeKind::TemplateLiteral(_) => {
            root_schema.data_type("string");
        }
        NodeKind::Str(_) => {
            root_schema.data_type("string");
        }
        NodeKind::Bool(_) => {
            root_schema.data_type("boolean");
        }
        NodeKind::Num(_) => {
            root_schema.data_type("number");
        }
        NodeKind::BigInt(_) => {
            root_schema.data_type("number");
        }
        NodeKind::Regex(_) => {
            root_schema.data_type("string");
        }
        _ => {
            for child in root.children() {
                define_schema_details(root_schema, &child, file_path, is_required, path_options, store);
            }
        }
    }
}

fn define_schema_from_identifier(
    identifier: &str,
    root_schema: &mut ApiSchema,
    file_path: &str,
    path_options: &PathOptions,
    root: &Rc<SchemyNode<'static>>,
    is_required: bool,
    store: &mut Store,
) -> () {
    if identifier.eq("LilSub") {
        let params = root.params();
        let param = params.last().unwrap();
        define_schema_details(root_schema, &param.clone(), file_path, true, path_options, store);
    } else if identifier.eq("LilRequiredProp") {
        let params = root.params();
        let param = params.first().unwrap();
        define_schema_details(root_schema, &param.clone(), file_path, true, path_options, store);
    } else if identifier.eq("Array") {
        let items_schema = root_schema.data_type("array").items();
        if let Some(type_params) = root.type_params() {
            match type_params.kind {
                NodeKind::TsTypeParamInstantiation(raw) => {
                    if let Some(type_param) = raw.params.get(0) {
                        let type_param = type_params.to_child(NodeKind::TsType(type_param));
                        define_schema_details(items_schema, &type_param, file_path, is_required, path_options, store);
                    }
                }
                _ => {}
            }
        }
    } else if identifier.eq("Uint8Array") | identifier.eq("Buffer") {
        root_schema.data_type("string").format(Some("binary".into()));
    } else if identifier.eq("URL") {
        root_schema.data_type("string").format(Some("uri".into()));
    } else {
        match store.get_root_declaration(file_path, identifier) {
            Some(Declaration::Import { name, source_file_name }) => {
                root_schema.reference(Some(identifier.to_string()), false);
                store.defer_external_type(&source_file_name, identifier, &name);
            }
            _ => {
                root_schema.reference(Some(identifier.to_string()), false);
                store.defer_local_type(file_path, identifier, identifier, root.clone());
            }
        }
    }
}

fn get_path_options(options: Rc<SchemyNode>) -> PathOptions {
    let mut path_options = PathOptions::new();
    load_options(&mut path_options, options);
    path_options
}

fn load_options(path_options: &mut PathOptions, root: Rc<SchemyNode>) {
    if let NodeKind::ExprOrSpread(raw_expr) = root.kind {
        match &*raw_expr.expr {
            Expr::Object(raw_literal) => {
                for prop_or_spread in &raw_literal.props {
                    match prop_or_spread.as_prop() {
                        Some(prop) => match prop.as_ref() {
                            Prop::KeyValue(key_value) => match &key_value.key {
                                PropName::Ident(i) if i.sym.eq("method") => {
                                    path_options.method = match key_value.value.deref() {
                                        Expr::Lit(Lit::Str(s)) => Some(s.value.to_string()),
                                        _ => None,
                                    }
                                }
                                PropName::Ident(i) if i.sym.eq("path") => {
                                    path_options.path = match key_value.value.deref() {
                                        Expr::Lit(Lit::Str(s)) => Some(s.value.to_string()),
                                        _ => None,
                                    }
                                }
                                PropName::Ident(i) if i.sym.eq("tags") => {
                                    let mut tags = vec![];
                                    match key_value.value.deref() {
                                        Expr::Array(literal) => {
                                            for element in &literal.elems {
                                                if let Some(element) = element {
                                                    match element.expr.deref() {
                                                        Expr::Lit(Lit::Str(s)) => tags.push(s.value.to_string()),
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
            _ => {}
        }
    }
}

fn get_parameter_name(root: Rc<SchemyNode>) -> String {
    match &root.kind {
        NodeKind::TsTypeElement(TsTypeElement::TsPropertySignature(raw)) if raw.key.is_ident() => {
            let identifier = raw.key.as_ident().unwrap();
            identifier.sym.to_string()
        }
        _ => match root.parent() {
            Some(parent) => get_parameter_name(parent),
            None => panic!("Could not find parameter name"),
        },
    }
}

fn get_response_options(options: &ObjectLit) -> ResponseOptions {
    let mut response_options = ResponseOptions::new();

    for prop in &options.props {
        match prop {
            PropOrSpread::Prop(prop) => match &**prop {
                Prop::KeyValue(key_value) => {
                    let key = match &key_value.key {
                        PropName::Ident(identifier) => Some(identifier.sym.to_string()),
                        _ => None,
                    };

                    let value = match &*key_value.value {
                        Expr::Lit(Lit::Str(value)) => Some(value.value.to_string()),
                        Expr::Lit(Lit::Num(value)) => value.raw.as_ref().map(|v| v.to_string()),
                        _ => None,
                    };

                    match key {
                        Some(k) if k.eq("description") => response_options.description = value,
                        Some(k) if k.eq("example") => response_options.example = value,
                        Some(k) if k.eq("statusCode") => response_options.status_code = value,
                        Some(k) if k.eq("mediaType") => response_options.media_type = value,
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

fn find_parent_type_ref<'m>(root: Rc<SchemyNode<'m>>) -> Rc<SchemyNode<'m>> {
    match &root.kind {
        NodeKind::TsTypeRef(_) => root.clone(),
        _ => find_parent_type_ref(root.parent().unwrap()),
    }
}
