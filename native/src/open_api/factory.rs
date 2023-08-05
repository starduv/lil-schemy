use std::{path::PathBuf, rc::Rc};

use es_resolve::{EsResolver, TargetEnv};
use lazy_static::__Deref;
use neon::macro_internal::runtime::raw;
use swc_ecma_ast::*;

use crate::typescript::{Declaration, DeclarationTables, ModuleCache, NodeKind, SchemyNode};

use super::{
    deferred::DeferredSchemas,
    schema::{ApiPathOperation, ApiSchema, OpenApi, PathOptions, ResponseOptions},
};

pub struct OpenApiFactory {
    deferred_schemas: DeferredSchemas,
    symbol_tables: DeclarationTables,
}

impl OpenApiFactory {
    pub fn new() -> Self {
        OpenApiFactory {
            deferred_schemas: DeferredSchemas::default(),
            symbol_tables: DeclarationTables::default(),
        }
    }

    pub fn append_schema(&mut self, open_api: &mut OpenApi, file_path: &str, module_cache: &mut ModuleCache) -> () {
        let root = module_cache.parse(&file_path);
        self.find_paths(open_api, root.clone(), file_path);

        while let Some(source_file_name) = self.deferred_schemas.next_module() {
            let deferred_root = module_cache.parse(&source_file_name);
            self.define_deferred_schemas(open_api, deferred_root, &source_file_name);
        }
    }

    fn find_paths<'m>(&mut self, open_api: &mut OpenApi, root: Rc<SchemyNode<'m>>, file_path: &str) {
        store_declaration_maybe(root.clone(), file_path, &mut self.symbol_tables);

        for child_index in root.children() {
            let child = root.get(child_index).unwrap();
            match &child.kind {
                NodeKind::CallExpr(_) => {
                    if let Some(callee) = child.callee() {
                        if let NodeKind::Callee(raw) = callee.kind {
                            match raw {
                                Callee::Expr(raw_expr) => match &**raw_expr {
                                    Expr::Ident(raw_ident) if raw_ident.sym.eq("Path") => {
                                        self.symbol_tables.add_child_scope(file_path);
                                        self.add_path(open_api, child, file_path);
                                        self.symbol_tables.parent_scope(file_path);
                                    }
                                    _ => self.find_paths(open_api, child, file_path),
                                },
                                _ => {}
                            }
                        }
                    }
                }
                _ => self.find_paths(open_api, child, file_path),
            }
        }
    }

    fn add_path(&mut self, open_api: &mut OpenApi, root: Rc<SchemyNode>, file_path: &str) -> () {
        let args = root.args();
        let route_handler = args.first();
        let route_options = args.last();
        let options = get_path_options(route_options.map(|n| n.clone()));

        let mut operation = ApiPathOperation::new();

        let route_handler = route_handler.map(|n| n.clone()).unwrap();

        self.add_request_details(open_api, operation.tags(options.tags), route_handler, file_path);

        open_api
            .path(&options.path.unwrap())
            .add_operation(&options.method.unwrap(), operation);
    }

    fn add_request_details(
        &mut self,
        open_api: &mut OpenApi,
        operation: &mut ApiPathOperation,
        route_handler: Rc<SchemyNode>,
        file_path: &str,
    ) -> () {
        for param in route_handler.params() {
            self.add_request_params(open_api, operation, param, file_path);
        }

        self.symbol_tables.add_child_scope(file_path);

        self.find_response(open_api, operation, route_handler, file_path, &mut String::from(""));

        self.symbol_tables.parent_scope(file_path);
    }

    fn add_request_params(
        &mut self,
        open_api: &mut OpenApi,
        operation: &mut ApiPathOperation,
        root: Rc<SchemyNode>,
        file_path: &str,
    ) {
        for child_index in root.children() {
            let child = root.get(child_index).unwrap();
            match child.kind {
                NodeKind::TsTypeRef(raw) => match &raw.deref().type_name {
                    TsEntityName::Ident(identifier) if identifier.sym.eq("BodyParam") => {
                        self.add_body_param_details(open_api, operation, child, file_path);
                    }
                    TsEntityName::Ident(identifier) if identifier.sym.eq("Header") => {
                        self.add_param_details(open_api, operation, "header", child, file_path);
                    }
                    TsEntityName::Ident(identifier) if identifier.sym.eq("QueryParam") => {
                        self.add_param_details(open_api, operation, "query", child, file_path);
                    }
                    TsEntityName::Ident(identifier) if identifier.sym.eq("RouteParam") => {
                        self.add_param_details(open_api, operation, "path", child, file_path);
                    }
                    // TODO support route handler params in separate module
                    // TsEntityName::Ident(identifier) => {
                    //     self.add_param_from_referenced_type(&identifier.sym, operation, file_path);
                    // }
                    _ => {
                        self.add_request_params(open_api, operation, child, file_path);
                    }
                },
                _ => {
                    self.add_request_params(open_api, operation, child, file_path);
                }
            }
        }
    }

    fn add_param_details(
        &mut self,
        open_api: &mut OpenApi,
        operation: &mut ApiPathOperation,
        location: &str,
        root: Rc<SchemyNode>,
        file_path: &str,
    ) {
        let parameter_name = get_parameter_name(root.clone());
        let operation_param = operation.param(&parameter_name, location);

        let type_params = root.params();
        let namespace = match type_params.get(2) {
            Some(namespace) => match namespace.kind {
                NodeKind::TsType(raw_type) => match raw_type {
                    TsType::TsLitType(raw_lit) => match &raw_lit.lit {
                        TsLit::Str(literal_string) => Some(literal_string.value.to_string()),
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        };

        match type_params.get(0) {
            Some(param) => match param.kind {
                NodeKind::TsType(raw_type) => match raw_type {
                    TsType::TsKeywordType(raw_keyword) => match raw_keyword.kind {
                        TsKeywordTypeKind::TsNumberKeyword => {
                            operation_param.content().schema().data_type("number");
                        }
                        TsKeywordTypeKind::TsBooleanKeyword => {
                            operation_param.content().schema().data_type("boolean");
                        }
                        TsKeywordTypeKind::TsStringKeyword => {
                            operation_param.content().schema().data_type("string");
                        }
                        _ => println!("found this while looking for param type: {:?}", raw_keyword.kind),
                    },
                    TsType::TsTypeRef(raw_type) => match &raw_type.type_name {
                        TsEntityName::Ident(identifier) => {
                            let root_name = self
                                .symbol_tables
                                .get_root_declaration_name(file_path, identifier.sym.to_string());

                            self.define_referenced_schema(
                                param.clone(),
                                &root_name,
                                &root_name,
                                open_api,
                                file_path,
                                namespace.clone(),
                            );

                            operation_param
                                .content()
                                .schema()
                                .reference(root_name.into(), false)
                                .namespace(namespace);
                        }
                        _ => println!("found some strang type ref"),
                    },
                    _ => {}
                },
                _ => println!("found some abstraction around your type ref {:?}", param.kind),
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
                other => println!("found this while looking for required: {:?}", other),
            },
            None => println!("i didn't find a second param at all!"),
        }

        match type_params.get(3) {
            Some(param) => match param.kind {
                NodeKind::TsType(raw_type) => match raw_type {
                    TsType::TsLitType(raw_lit) => match &raw_lit.lit {
                        TsLit::Str(raw_str) => {
                            operation_param
                                .content()
                                .schema()
                                .format(Some(raw_str.value.to_string()));
                        }
                        _ => {}
                    },
                    _ => {}
                },
                _ => println!("found this while looking for format: {:?}", param.kind),
            },
            None => {}
        }
    }

    fn find_response(
        &mut self,
        open_api: &mut OpenApi,
        operation: &mut ApiPathOperation,
        root: Rc<SchemyNode>,
        file_path: &str,
        depth: &mut String,
    ) -> () {
        depth.push_str("-");

        for child_index in root.children() {
            let child = root.get(child_index.clone()).unwrap();
            store_declaration_maybe(child.clone(), file_path, &mut self.symbol_tables);
            match child.kind {
                NodeKind::Ident(raw) if raw.sym.eq("Response") => {
                    self.add_response(open_api, operation, root.parent().unwrap(), file_path)
                }
                _ => self.find_response(open_api, operation, child, file_path, &mut depth.clone()),
            }
        }
    }

    fn add_response(
        &mut self,
        open_api: &mut OpenApi,
        operation: &mut ApiPathOperation,
        root: Rc<SchemyNode>,
        file_path: &str,
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

        let namespace = match &options {
            Some(options) => options.namespace.clone(),
            None => None,
        };

        let response_type_name = match args.get(0) {
            Some(arg) => match &arg.kind {
                NodeKind::ExprOrSpread(raw) => match &*raw.expr {
                    Expr::New(new_expression) => match &*new_expression.callee {
                        Expr::Ident(identifier) => Some(
                            self.symbol_tables
                                .get_root_declaration_name(file_path, identifier.sym.to_string()),
                        ),
                        _ => None,
                    },
                    Expr::Ident(response_type) => Some(
                        self.symbol_tables
                            .get_root_declaration_name(file_path, response_type.sym.to_string()),
                    ),
                    Expr::TsAs(ts_as) => match &*ts_as.type_ann {
                        TsType::TsTypeRef(type_ref) => match &type_ref.type_name {
                            TsEntityName::Ident(identifier) => Some(
                                self.symbol_tables
                                    .get_root_declaration_name(file_path, identifier.sym.to_string()),
                            ),
                            _ => None,
                        },
                        _ => None,
                    },
                    Expr::TsTypeAssertion(type_assertion) => match &*type_assertion.type_ann {
                        TsType::TsTypeRef(type_ref) => match &type_ref.type_name {
                            TsEntityName::Ident(identifier) => Some(
                                self.symbol_tables
                                    .get_root_declaration_name(file_path, identifier.sym.to_string()),
                            ),
                            _ => None,
                        },
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            },
            None => None,
        };

        if let Some(response_type_name) = &response_type_name {
            self.define_referenced_schema(
                root,
                &response_type_name,
                &response_type_name,
                open_api,
                file_path,
                namespace,
            );
        }

        if let Some(response_options) = options {
            operation.response(&response_type_name, response_options);
        }
    }

    fn add_body_param_details(
        &mut self,
        open_api: &mut OpenApi,
        operation: &mut ApiPathOperation,
        root: Rc<SchemyNode>,
        file_path: &str,
    ) -> () {
        let operation_param = operation.body();
        let type_params = root.params();
        println!("type params: {:?}", type_params.len());
        let namespace = match type_params.get(2) {
            Some(namespace) => match namespace.kind {
                NodeKind::TsType(raw_type) => match raw_type {
                    TsType::TsLitType(raw_lit) => match &raw_lit.lit {
                        TsLit::Str(literal_string) => Some(literal_string.value.to_string()),
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        };

        match type_params.get(0) {
            Some(param) => match param.kind {
                NodeKind::TsType(raw_type) => match raw_type {
                    TsType::TsKeywordType(raw_keyword) => match raw_keyword.kind {
                        TsKeywordTypeKind::TsNumberKeyword => {
                            operation_param.content().schema().data_type("number");
                        }
                        TsKeywordTypeKind::TsBooleanKeyword => {
                            operation_param.content().schema().data_type("boolean");
                        }
                        TsKeywordTypeKind::TsStringKeyword => {
                            operation_param.content().schema().data_type("string");
                        }
                        _ => println!("found this while looking for param type: {:?}", raw_keyword.kind),
                    },
                    TsType::TsTypeRef(raw_type) => match &raw_type.type_name {
                        TsEntityName::Ident(identifier) => {
                            let root_name = self
                                .symbol_tables
                                .get_root_declaration_name(file_path, identifier.sym.to_string());

                            self.define_referenced_schema(
                                param.clone(),
                                &root_name,
                                &root_name,
                                open_api,
                                file_path,
                                namespace.clone(),
                            );

                            operation_param
                                .content()
                                .schema()
                                .reference(root_name.into(), false)
                                .namespace(namespace);
                        }
                        _ => println!("found some strang type ref"),
                    },
                    _ => {}
                },
                _ => println!("found some abstraction around your type ref {:?}", param.kind),
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
                other => println!("found this while looking for required: {:?}", other),
            },
            None => println!("i didn't find a second param at all!"),
        }
    }

    fn define_referenced_schema(
        &mut self,
        type_node: Rc<SchemyNode>,
        type_name: &str,
        schema_name: &str,
        open_api: &mut OpenApi,
        file_path: &str,
        namespace: Option<String>,
    ) -> () {
        match self.symbol_tables.get_root_declaration(file_path, type_name) {
            Some(Declaration::Export {
                name: type_name,
                source_file_name,
            }) => {
                self.deferred_schemas
                    .add_deferred_type(source_file_name, schema_name.into(), type_name, namespace);
            }
            Some(Declaration::Import {
                name: type_name,
                source_file_name,
            }) => {
                self.deferred_schemas
                    .add_deferred_type(source_file_name, schema_name.into(), type_name, namespace);
            }
            Some(Declaration::Type { node: node_index }) => {
                let schema = match namespace {
                    Some(ns) => open_api.components.schema(&ns).property(schema_name.into()),
                    None => open_api.components.schema(schema_name),
                };
                let node = type_node.get(node_index).unwrap();
                define_referenced_schema_details(schema, node);
            }
            _ => {}
        };
    }

    fn define_deferred_schemas(&mut self, open_api: &mut OpenApi, root: Rc<SchemyNode>, source_file_name: &str) -> () {
        store_declaration_maybe(root.clone(), source_file_name, &mut self.symbol_tables);

        match &root.kind {
            NodeKind::ExportDefaultExpr(_) => {
                self.define_deferred_type_maybe(root, open_api, "default", source_file_name)
            }
            NodeKind::ExportDecl(_) => {
                if let Some(declaration) = root.decl() {
                    match declaration.kind {
                        NodeKind::ClassDecl(class_declaration) => {
                            let name = class_declaration.ident.sym.to_string();
                            self.define_deferred_type_maybe(declaration, open_api, &name, source_file_name);
                        }
                        NodeKind::TsInterfaceDecl(interface_declaration) => {
                            let name = interface_declaration.id.sym.to_string();
                            self.define_deferred_type_maybe(declaration, open_api, &name, source_file_name);
                        }
                        NodeKind::TsTypeAliasDecl(alias_declaration) => {
                            let name = alias_declaration.id.sym.to_string();
                            self.define_deferred_type_maybe(declaration, open_api, &name, source_file_name);
                        }
                        _ => {}
                    }
                }
            }
            NodeKind::NamedExport(_) => {
                for specifier in root.specifiers() {
                    match specifier.kind {
                        NodeKind::ExportSpecifier(export_specifier) => match export_specifier {
                            ExportSpecifier::Named(named) => {
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

                                self.define_deferred_type_maybe(specifier, open_api, &name, source_file_name);
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    fn define_deferred_type_maybe(
        &mut self,
        root: Rc<SchemyNode>,
        open_api: &mut OpenApi,
        type_name: &str,
        source_file_name: &str,
    ) -> () {
        if let Some(deferred_type) = self.deferred_schemas.get_deferred_type(type_name, source_file_name) {
            match self.symbol_tables.get_root_declaration(source_file_name, &type_name) {
                Some(Declaration::Type { node }) => {
                    let schema = match &deferred_type.namespace {
                        Some(ns) => open_api
                            .components
                            .schema(&ns)
                            .data_type("object")
                            .property(&deferred_type.schema_name),
                        None => open_api.components.schema(&deferred_type.schema_name),
                    };

                    let node = root.get(node).unwrap();

                    define_referenced_schema_details(schema, node);
                }
                Some(Declaration::Import {
                    name: imported_name,
                    source_file_name: module_file_name,
                }) => {
                    self.deferred_schemas.add_deferred_type(
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
}

fn store_declaration_maybe(root: Rc<SchemyNode>, file_path: &str, symbol_tables: &mut DeclarationTables) -> () {
    match root.kind {
        // NodeKind::TsAsExpr(raw_expr) => match &*raw_expr.type_ann {
        //     TsType::TsTypeRef(raw) => {
        //         let name = match &raw.type_name {
        //             TsEntityName::Ident(identifier) => &identifier.sym,
        //             _ => "",
        //         };

        //         println!("I'm storing the type {}", name);

        //         symbol_tables.insert(
        //             file_path,
        //             name.to_string(),
        //             Declaration::Type {
        //                 node: root.index.clone(),
        //             }
        //         )
        //     },
        //     _ => {}
        // }
        NodeKind::ClassDecl(raw) => {
            let name = raw.ident.sym.to_string();
            symbol_tables.insert(
                file_path,
                name.to_string(),
                Declaration::Type {
                    node: root.index.clone(),
                },
            )
        }
        NodeKind::TsInterfaceDecl(raw) => {
            let name = raw.id.sym.to_string();
            symbol_tables.insert(
                file_path,
                name,
                Declaration::Type {
                    node: root.index.clone(),
                },
            )
        }
        NodeKind::TsTypeAliasDecl(raw) => {
            let name = raw.id.sym.to_string();
            symbol_tables.insert(
                file_path,
                name,
                Declaration::Type {
                    node: root.index.clone(),
                },
            )
        }
        NodeKind::TsEnumDecl(raw) => {
            let name = raw.id.sym.to_string();
            symbol_tables.insert(
                file_path,
                name,
                Declaration::Type {
                    node: root.index.clone(),
                },
            )
        }

        NodeKind::Ident(raw) => {
            let target_name = raw.sym.to_string();
            symbol_tables.insert(
                file_path,
                "default".into(),
                Declaration::Alias {
                    from: "default".into(),
                    to: target_name,
                },
            )
        }
        NodeKind::ClassExpr(_) => symbol_tables.insert(
            file_path,
            "default".into(),
            Declaration::Type {
                node: root.index.clone(),
            },
        ),
        NodeKind::ImportDecl(raw) => {
            for child_index in root.children() {
                let child = root.get(child_index).unwrap();
                match child.kind {
                    NodeKind::ImportDefaultSpecifier(raw_specifier) => {
                        let src = raw.src.value.to_string();
                        match EsResolver::new(&src, &PathBuf::from(file_path), TargetEnv::Node).resolve() {
                            Ok(module_path) => {
                                let name = raw_specifier.local.sym.to_string();
                                symbol_tables.insert(
                                    file_path,
                                    name,
                                    Declaration::Import {
                                        name: String::from("default"),
                                        source_file_name: module_path,
                                    },
                                )
                            }
                            Err(err) => println!("'{}', module resolution error: {:?}", file_path, err),
                        }
                    }
                    NodeKind::ImportNamedSpecifier(raw_specifier) => {
                        let src = raw.src.value.to_string();
                        match EsResolver::new(&src, &PathBuf::from(file_path), TargetEnv::Node).resolve() {
                            Ok(module_path) => {
                                let name = raw_specifier.local.sym.to_string();
                                symbol_tables.insert(
                                    file_path,
                                    name.to_string(),
                                    Declaration::Import {
                                        name,
                                        source_file_name: module_path,
                                    },
                                )
                            }
                            Err(err) => println!("'{}', module resolution error: {:?}", file_path, err),
                        }
                    }
                    _ => {}
                }
            }
        }
        NodeKind::NamedExport(raw) => {
            let src = &raw.src.as_ref().unwrap().value;
            match EsResolver::new(&src, &PathBuf::from(file_path), TargetEnv::Node).resolve() {
                Ok(module_file_name) => {
                    for specifier in &raw.specifiers {
                        match specifier {
                            ExportSpecifier::Named(named_specifier) => {
                                let type_name = match &named_specifier.orig {
                                    ModuleExportName::Ident(identifier) => &identifier.sym,
                                    ModuleExportName::Str(identifier) => &identifier.value,
                                };

                                if let Some(exported_name) = &named_specifier.exported {
                                    let exported_name = match exported_name {
                                        ModuleExportName::Ident(id) => &id.sym,
                                        ModuleExportName::Str(id) => &id.value,
                                    };

                                    symbol_tables.insert(
                                        file_path,
                                        exported_name.to_string(),
                                        Declaration::Import {
                                            name: type_name.to_string(),
                                            source_file_name: module_file_name.to_string(),
                                        },
                                    )
                                } else {
                                    symbol_tables.insert(
                                        file_path,
                                        type_name.to_string(),
                                        Declaration::Import {
                                            name: type_name.to_string(),
                                            source_file_name: module_file_name.to_string(),
                                        },
                                    )
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Err(err) => println!("'{}', module resolution error: {:?}", file_path, err),
            }
        }
        NodeKind::VarDeclarator(raw) => {
            match &raw.name {
                Pat::Ident(identifier) => {
                    let name = identifier.id.sym.to_string();
                    match &identifier.type_ann {
                        Some(type_annotation) => match &*type_annotation.type_ann {
                            TsType::TsTypeRef(type_ref) => match &type_ref.type_name {
                                TsEntityName::Ident(identifier) => symbol_tables.insert(
                                    file_path,
                                    name.to_string(),
                                    Declaration::Alias {
                                        from: name,
                                        to: identifier.sym.to_string(),
                                    },
                                ),
                                _ => {}
                            },
                            _ => {}
                        },
                        None => match &raw.init {
                            Some(initializer) => {
                                let node = root.to_child(NodeKind::Expr(initializer));
                                store_variable(&name, node, file_path, symbol_tables);
                            }
                            None => {}
                        },
                    }
                }
                _ => println!("yeah there's a pat"),
            };
        }
        _ => {}
    }
}

fn store_variable(name: &str, root: Rc<SchemyNode>, file_path: &str, symbol_tables: &mut DeclarationTables) -> () {
    for child_index in root.children() {
        let child = root.get(child_index).unwrap();
        match child.kind {
            NodeKind::Ident(raw) => {
                let type_name = raw.sym.to_string();
                symbol_tables.insert(
                    file_path,
                    name.to_string(),
                    Declaration::Alias {
                        from: name.to_string(),
                        to: type_name,
                    },
                )
            }
            NodeKind::TsTypeRef(raw) => match &raw.type_name {
                TsEntityName::Ident(identifier) => {
                    let type_name = identifier.sym.to_string();
                    symbol_tables.insert(
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
            _ => store_variable(name, child, file_path, symbol_tables),
        }
    }
}

// // fn add_param_from_referenced_type(
// //     &mut self,
// //     schema_type_name: &str,
// //     operation: &mut ApiPathOperation,
// //     file_path: &str,
// // ) -> () {
// // let root_declaration = self.symbol_tables.get_root_declaration(file_path, type_ref);
// // match  root_declaration{
// //     // Some(Declaration::Import { name, source_file_name }) => find_referenced_type(file_path, name, |declaration: dprint_swc_ext::swc::ast::Decl| {
// //     //     match declaration {
// //     //         dprint_swc_ext::swc::ast::Decl::TsInterface(interface_declaration) => {
// //     //             let name = interface_declaration.id.sym().to_string();
// //     //             self.add_param_from_referenced_type(operation, &name, source_file_name);
// //     //         }
// //     //         dprint_swc_ext::swc::ast::Decl::TsTypeAlias(alias_declaration) => {
// //     //             let name = alias_declaration.id.sym().to_string();
// //     //             self.add_param_from_referenced_type(operation, &name, source_file_name);
// //     //         }
// //     //         _ => {}
// //     //     }
// //     // }),
// //     Some(Declaration::Type { node }) => self.add_request_params(operation, &node, file_path),
// //     _ => {}
// // }
// // }

fn define_referenced_schema_details(root_schema: &mut ApiSchema, root: Rc<SchemyNode>) -> () {
    match root.kind {
        NodeKind::TsKeywordType(raw) => match raw.kind {
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
        NodeKind::ClassDecl(_) => {
            root_schema.data_type("object".into());
            if let Some(class_node) = root.class() {
                for property in class_node.body() {
                    match property.kind {
                        NodeKind::ClassProp(class_property) => {
                            let name = match &class_property.key {
                                PropName::Ident(identifier) => Some(identifier.sym.to_string()),
                                _ => None,
                            };

                            if let Some(name) = name {
                                if let Some(annotation) = &property.type_ann() {
                                    define_referenced_schema_details(root_schema.property(&name), annotation.clone());
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        NodeKind::ClassExpr(_) => {
            root_schema.data_type("object".into());
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
                                    define_referenced_schema_details(root_schema.property(&name), annotation.clone());
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        NodeKind::TsArrayType(_) => {
            root_schema.data_type("array".into());
            let elem_type = root.elem_type().unwrap();
            define_referenced_schema_details(root_schema.items(), elem_type.clone());
        }
        NodeKind::TsInterfaceDecl(_) => {
            root_schema.data_type("object".into());
            if let Some(interface_body) = root.interface_body() {
                for interface_member in interface_body.body() {
                    match interface_member.kind {
                        NodeKind::TsPropertySignature(signature) => {
                            let property_schema = match &*signature.key {
                                Expr::Ident(identifier) => {
                                    let name = identifier.sym.to_string();
                                    Some(root_schema.property(&name))
                                }
                                _ => None,
                            };

                            if let Some(property_schema) = property_schema {
                                if let Some(annotation) = interface_member.type_ann() {
                                    define_referenced_schema_details(property_schema, annotation.clone());
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        NodeKind::TsTypeLit(_) => {
            root_schema.data_type("object".into());
            for member in root.members() {
                match member.kind {
                    NodeKind::TsPropertySignature(signature) => {
                        let property_schema = match &*signature.key {
                            Expr::Ident(identifier) => {
                                let name = identifier.sym.to_string();
                                Some(root_schema.property(&name))
                            }
                            _ => None,
                        };

                        if let Some(property_schema) = property_schema {
                            if let Some(annotation) = member.type_ann() {
                                define_referenced_schema_details(property_schema, annotation.clone());
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        NodeKind::TsTypeAliasDecl(_) => {
            if let Some(annotation) = root.type_ann() {
                define_referenced_schema_details(root_schema, annotation.clone())
            }
        }
        _ => {}
    }
}

fn get_path_options(options: Option<Rc<SchemyNode>>) -> PathOptions {
    let mut path_options = PathOptions::new();
    if let Some(options) = options {
        load_options(&mut path_options, options);
    }
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
