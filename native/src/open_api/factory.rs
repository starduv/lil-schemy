use std::{cell::RefCell, rc::Rc};

use lazy_static::__Deref;

use swc_ecma_ast::*;

use case::CaseExt;

use crate::typescript::{Declaration, DeclarationTables, ModuleCache, NodeKind, SchemyNode};

use super::{
    caching::store_declaration_maybe,
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

        while self.deferred_schemas.has_unrecognized_local_types(file_path) {
            self.define_local_schemas(file_path, &root, open_api);
        }
    }

    pub fn append_deferred_schemas(&mut self, open_api: &mut OpenApi, module_cache: &mut ModuleCache) {
        while let Some(file_path) = self.deferred_schemas.next_module() {
            let deferred_root = module_cache.parse(&file_path);
            for item_index in deferred_root.children() {
                let item = deferred_root.get(item_index).unwrap();
                self.define_external_schema(open_api, item, &file_path);
            }

            while self.deferred_schemas.has_unrecognized_local_types(&file_path) {
                self.define_local_schemas(&file_path, &deferred_root, open_api);
            }
        }
    }

    fn define_local_schemas(&mut self, file_path: &str, root: &Rc<SchemyNode>, open_api: &mut OpenApi) {
        for local_type in self.deferred_schemas.recognize_local_types(file_path) {
            let root = root.get(local_type.index).unwrap();
            self.define_local_schema(
                root,
                &local_type.type_name,
                &local_type.schema_name,
                open_api,
                file_path,
            )
        }
    }

    fn find_paths<'m>(&mut self, open_api: &mut OpenApi, root: Rc<SchemyNode<'m>>, file_path: &str) {
        store_declaration_maybe(root.clone(), file_path, &mut self.symbol_tables);

        for child_index in root.children() {
            let child = root.get(child_index).unwrap();
            match &child.kind {
                NodeKind::Ident(raw_ident) if raw_ident.sym.eq("LilPath") => {
                    let parent = child.parent().unwrap().parent().unwrap();
                    self.symbol_tables.add_child_scope(file_path);
                    self.add_path(open_api, parent, file_path);
                    self.symbol_tables.parent_scope(file_path);
                }
                _ => self.find_paths(open_api, child, file_path),
            }
        }
    }

    fn add_path(&mut self, open_api: &mut OpenApi, root: Rc<SchemyNode>, file_path: &str) -> () {
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

                self.add_request_details(&operation, route_handler.clone(), file_path, &options);
            }
        }
    }

    fn add_request_details(
        &mut self,
        operation: &Rc<RefCell<ApiPathOperation>>,
        route_handler: Rc<SchemyNode>,
        file_path: &str,
        path_options: &PathOptions,
    ) -> () {
        for param in route_handler.params() {
            self.add_request_params(operation, param, file_path, path_options);
        }

        self.symbol_tables.add_child_scope(file_path);

        self.find_response(operation, route_handler, file_path, path_options, &mut "".into());

        self.symbol_tables.parent_scope(file_path);
    }

    fn add_request_params(
        &mut self,
        operation: &Rc<RefCell<ApiPathOperation>>,
        root: Rc<SchemyNode>,
        file_path: &str,
        path_options: &PathOptions,
    ) {
        match root.kind {
            NodeKind::Ident(identifier) if identifier.sym.eq("LilBodyParam") => {
                self.add_body_param_details(operation, find_parent_type_ref(root), file_path);
            }
            NodeKind::Ident(identifier) if identifier.sym.eq("LilHeader") => {
                self.add_param_details(
                    operation,
                    "header",
                    find_parent_type_ref(root),
                    file_path,
                    false,
                    path_options,
                );
            }
            NodeKind::Ident(identifier) if identifier.sym.eq("LilQueryParam") => {
                self.add_param_details(
                    operation,
                    "query",
                    find_parent_type_ref(root),
                    file_path,
                    false,
                    path_options,
                );
            }
            NodeKind::Ident(identifier) if identifier.sym.eq("LilRouteParam") => {
                self.add_param_details(
                    operation,
                    "path",
                    find_parent_type_ref(root),
                    file_path,
                    true,
                    path_options,
                );
            }
            NodeKind::Ident(identifier) => match self.symbol_tables.get_root_declaration(file_path, &identifier.sym) {
                Some(Declaration::Import { name, source_file_name }) => {
                    self.deferred_schemas
                        .defer_operation_type(&source_file_name, operation, &name);
                }
                Some(Declaration::Type { node }) => {
                    if let Some(root) = root.get(node) {
                        self.add_request_params(operation, root, file_path, path_options);
                    }
                }
                _ => {}
            },
            _ => {
                for child_index in root.children() {
                    self.add_request_params(operation, root.get(child_index).unwrap(), file_path, path_options);
                }
            }
        }
    }

    fn add_param_details(
        &mut self,
        operation: &Rc<RefCell<ApiPathOperation>>,
        location: &str,
        root: Rc<SchemyNode>,
        file_path: &str,
        required_default: bool,
        path_options: &PathOptions,
    ) {
        let mut operation = (**operation).borrow_mut();
        let parameter_name = get_parameter_name(root.clone());
        let operation_param = operation.param(&parameter_name, location);

        let type_params = root.params();
        if let Some(type_param) = type_params.get(0) {
            let param_schema = operation_param.content().schema();
            self.define_schema_details(param_schema, &type_param, file_path, false, path_options);
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
                                .content()
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
        &mut self,
        operation: &Rc<RefCell<ApiPathOperation>>,
        root: Rc<SchemyNode>,
        file_path: &str,
        path_options: &PathOptions,
        depth: &mut String,
    ) -> () {
        for child_index in root.children() {
            let child = root.get(child_index.clone()).unwrap();
            store_declaration_maybe(child.clone(), file_path, &mut self.symbol_tables);
            match child.kind {
                NodeKind::Ident(raw) if raw.sym.eq("LilResponse") => {
                    self.add_response(operation, root.parent().unwrap(), file_path, path_options)
                }
                _ => self.find_response(operation, child, file_path, path_options, &mut depth.clone()),
            }
        }
    }

    fn add_response(
        &mut self,
        operation: &Rc<RefCell<ApiPathOperation>>,
        root: Rc<SchemyNode>,
        file_path: &str,
        path_options: &PathOptions,
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
                self.add_response_details(&response_type, &options.unwrap(), file_path, operation, path_options);
            }
            _ => {}
        };
    }

    fn add_response_details(
        &mut self,
        root: &Rc<SchemyNode>,
        options: &ResponseOptions,
        file_path: &str,
        operation: &Rc<RefCell<ApiPathOperation>>,
        path_options: &PathOptions,
    ) {
        let status_code = options.status_code.as_ref().unwrap();
        let description = options.description.as_ref().unwrap();

        let mut operation = (**operation).borrow_mut();
        let response = operation.response(&status_code, &description);
        let content = response.content();

        self.define_schema_details(content.schema(), root, file_path, true, path_options);

        content.example(options.example.clone());
    }

    fn add_body_param_details(
        &mut self,
        operation: &Rc<RefCell<ApiPathOperation>>,
        root: Rc<SchemyNode>,
        file_path: &str,
    ) -> () {
        let mut operation = (**operation).borrow_mut();
        let operation_param = operation.body();
        let type_params = root.params();

        match type_params.get(0) {
            Some(param) => match param.kind {
                NodeKind::TsType(raw_type) => match raw_type {
                    TsType::TsTypeLit(raw_lit) => {
                        let child = root.to_child(NodeKind::TsTypeLit(raw_lit));
                        self.define_schema_details(
                            operation_param.content().schema(),
                            &child,
                            file_path,
                            false,
                            &PathOptions::default(),
                        );
                    }
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
                            let root_name = self.symbol_tables.get_root_declaration_name(file_path, &identifier.sym);

                            self.deferred_schemas
                                .defer_local_type(file_path, &root_name, &root_name, param.index);

                            operation_param.content().schema().reference(root_name.into(), false);
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
                _ => {}
            },
            None => {}
        }
    }

    fn define_local_schema(
        &mut self,
        type_node: Rc<SchemyNode>,
        type_name: &str,
        schema_name: &str,
        open_api: &mut OpenApi,
        file_path: &str,
    ) -> () {
        if open_api.components.contains_schema(schema_name) {
            return;
        }

        match self.symbol_tables.get_root_declaration(file_path, type_name) {
            Some(Declaration::Export {
                name: type_name,
                source_file_name,
            }) => {
                self.deferred_schemas
                    .defer_external_type(&source_file_name, schema_name.into(), &type_name);
            }
            Some(Declaration::Import {
                name: type_name,
                source_file_name,
            }) => {
                self.deferred_schemas
                    .defer_external_type(&source_file_name, schema_name.into(), &type_name);
            }
            Some(Declaration::Type { node: node_index }) => {
                let schema = open_api.components.schema(schema_name);

                if let Some(node) = type_node.get(node_index) {
                    self.define_schema_details(schema, &node, file_path, false, &PathOptions::default());
                }
            }
            _ => {}
        };
    }

    fn define_external_schema(&mut self, open_api: &mut OpenApi, root: Rc<SchemyNode>, file_path: &str) -> () {
        store_declaration_maybe(root.clone(), file_path, &mut self.symbol_tables);
        match &root.kind {
            NodeKind::ModuleItem(ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(_))) => {
                self.define_external_schema_maybe(&root, open_api, "default", file_path)
            }
            NodeKind::ModuleItem(ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultDecl(raw_decl))) => {
                let root = root.to_child(NodeKind::DefaultDecl(&raw_decl.decl));
                self.define_external_schema_maybe(&root, open_api, "default", file_path)
            }
            NodeKind::ModuleItem(ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(raw_decl))) => match &raw_decl.decl {
                Decl::Class(ref raw_class) => {
                    let root = root.to_child(NodeKind::ClassDecl(raw_class));
                    self.define_external_schema_maybe(&root, open_api, &raw_class.ident.sym, file_path)
                }
                Decl::TsInterface(ref raw_interface) => {
                    let root = root.to_child(NodeKind::TsInterfaceDecl(raw_interface));
                    self.define_external_schema_maybe(&root, open_api, &raw_interface.id.sym, file_path)
                }
                Decl::TsTypeAlias(ref raw_alias) => {
                    let root = root.to_child(NodeKind::TsTypeAliasDecl(raw_alias));
                    self.define_external_schema_maybe(&root, open_api, &raw_alias.id.sym, file_path)
                }
                Decl::TsEnum(ref raw_alias) => {
                    let root = root.to_child(NodeKind::TsEnumDecl(raw_alias));
                    self.define_external_schema_maybe(&root, open_api, &raw_alias.id.sym, file_path)
                }
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

                            self.define_external_schema_maybe(&specifier, open_api, &name, file_path);
                        }
                        _ => {}
                    }
                }
            }
            NodeKind::ModuleItem(ModuleItem::Stmt(Stmt::Decl(Decl::TsEnum(raw_enum)))) => {
                let root = root.to_child(NodeKind::TsEnumDecl(raw_enum));
                self.define_external_schema_maybe(&root, open_api, &raw_enum.id.sym, file_path)
            }
            NodeKind::ModuleItem(ModuleItem::Stmt(Stmt::Decl(Decl::Class(raw_class)))) => {
                let root = root.to_child(NodeKind::ClassDecl(raw_class));
                self.define_external_schema_maybe(&root, open_api, &raw_class.ident.sym, file_path)
            }
            NodeKind::ModuleItem(ModuleItem::Stmt(Stmt::Decl(Decl::TsInterface(raw_interface)))) => {
                let root = root.to_child(NodeKind::TsInterfaceDecl(raw_interface));
                self.define_external_schema_maybe(&root, open_api, &raw_interface.id.sym, file_path)
            }
            NodeKind::ModuleItem(ModuleItem::Stmt(Stmt::Decl(Decl::TsTypeAlias(raw_alias)))) => {
                let root = root.to_child(NodeKind::TsTypeAliasDecl(raw_alias));
                self.define_external_schema_maybe(&root, open_api, &raw_alias.id.sym, file_path)
            }
            NodeKind::ModuleItem(ModuleItem::Stmt(Stmt::Decl(Decl::TsModule(raw_module)))) => {
                let root = root.to_child(NodeKind::TsModuleDecl(raw_module));
                for child in root.children() {
                    let child = root.get(child).unwrap();
                    self.define_external_schema(open_api, child, file_path);
                }
            }
            _ => {}
        }
    }

    fn define_external_schema_maybe(
        &mut self,
        root: &Rc<SchemyNode>,
        open_api: &mut OpenApi,
        type_name: &str,
        file_path: &str,
    ) -> () {
        if open_api.components.contains_schema(type_name) {
            return;
        }

        if let Some(deferred_operation_type) = self.deferred_schemas.recognize_operation_type(type_name, file_path) {
            match self.symbol_tables.get_root_declaration(file_path, type_name) {
                Some(Declaration::Type { node }) => {
                    let root = root.get(node).unwrap();
                    self.add_request_params(
                        &deferred_operation_type.operation,
                        root,
                        file_path,
                        &PathOptions::default(),
                    );
                }
                Some(Declaration::Import {
                    name: imported_name,
                    source_file_name: module_file_name,
                }) => {
                    self.deferred_schemas.defer_operation_type(
                        &module_file_name,
                        &deferred_operation_type.operation,
                        &imported_name,
                    );
                }
                _ => {}
            }
        }

        if let Some(deferred_type) = self.deferred_schemas.recognize_external_type(type_name, file_path) {
            match self.symbol_tables.get_root_declaration(file_path, &type_name) {
                Some(Declaration::Type { node }) => {
                    let schema = open_api.components.schema(&deferred_type.schema_name);

                    let node = root.get(node).unwrap();

                    self.define_schema_details(schema, &node, file_path, false, &PathOptions::default());
                }
                Some(Declaration::Import {
                    name: imported_name,
                    source_file_name: module_file_name,
                }) => {
                    self.deferred_schemas
                        .defer_external_type(&module_file_name, type_name, &imported_name);
                }
                _ => {}
            }
        }
    }

    fn define_schema_details(
        &mut self,
        root_schema: &mut ApiSchema,
        root: &Rc<SchemyNode>,
        file_path: &str,
        is_required: bool,
        path_options: &PathOptions,
    ) -> () {
        match root.kind {
            NodeKind::Ident(raw_ident) => match self.symbol_tables.get_root_declaration(file_path, &raw_ident.sym) {
                Some(Declaration::Import { name, source_file_name }) => {
                    let schema_name = match name.eq("default") {
                        true => raw_ident.sym.to_string(),
                        false => name.to_capitalized(),
                    };

                    self.deferred_schemas
                        .defer_external_type(&source_file_name, &schema_name, &name);

                    root_schema.reference(Some(schema_name), false);
                }
                Some(Declaration::Type { node: index }) => {
                    let node = root.get(index).unwrap();
                    self.define_schema_details(root_schema, &node, file_path, false, path_options);
                }
                _ => {
                    let name = self.symbol_tables.get_root_declaration_name(file_path, &raw_ident.sym);
                    self.define_schema_from_identifier(&name, root_schema, file_path, path_options, root, is_required);
                }
            },
            NodeKind::TsUnionOrIntersectionType(_) => {
                for child in root.children() {
                    let child = root.get(child).unwrap();
                    self.define_schema_details(root_schema, &child, file_path, is_required, path_options);
                }
            }
            NodeKind::TsUnionType(_) => {
                let any_of = root_schema.any_of();
                let mut enum_schema = ApiSchema::new();
                for child in root.children() {
                    let child = root.get(child).unwrap();
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
                            self.define_schema_details(&mut schema, &child, file_path, is_required, path_options);
                            any_of.push(schema);
                        }
                        NodeKind::Ident(_) => {
                            let mut schema = ApiSchema::new();
                            self.define_schema_details(&mut schema, &child, file_path, is_required, path_options);
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
                    let child = root.get(child).unwrap();
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
                            self.define_schema_details(&mut schema, &child, file_path, is_required, path_options);
                            all_of.push(schema);
                        }
                        NodeKind::Ident(_) => {
                            let mut schema = ApiSchema::new();
                            self.define_schema_details(&mut schema, &child, file_path, is_required, path_options);
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
                        self.define_schema_details(root_schema, &param, file_path, is_required, path_options);
                    } else {
                        match self.symbol_tables.get_root_declaration(file_path, &raw_ident.sym) {
                            Some(Declaration::Import { name, source_file_name }) => {
                                self.deferred_schemas
                                    .defer_external_type(&source_file_name, &name, &name);

                                root_schema.reference(Some(name), false);
                            }
                            _ => {
                                self.deferred_schemas.defer_local_type(
                                    file_path,
                                    &raw_ident.sym,
                                    &raw_ident.sym,
                                    root.index,
                                );

                                root_schema.reference(Some(raw_ident.sym.to_string()), false);
                            }
                        }
                    }
                }
                _ => {}
            },
            NodeKind::TsTypeRef(raw_type) => match &raw_type.type_name {
                TsEntityName::Ident(identifier) => {
                    self.define_schema_from_identifier(
                        &identifier.sym,
                        root_schema,
                        file_path,
                        path_options,
                        root,
                        is_required,
                    );
                }
                _ => println!("found some strang type ref"),
            },
            NodeKind::TsTypeAnnotation(_) => {
                for child_index in root.children() {
                    let child = root.get(child_index).unwrap();
                    self.define_schema_details(root_schema, &child, file_path, is_required, path_options);
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

                                        self.define_schema_details(
                                            property_schema,
                                            &annotation,
                                            file_path,
                                            is_required,
                                            path_options,
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

                                    self.define_schema_details(
                                        property_schema,
                                        &annotation,
                                        file_path,
                                        is_required,
                                        path_options,
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

                                        self.define_schema_details(
                                            property_schema,
                                            &annotation,
                                            file_path,
                                            is_required,
                                            path_options,
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
                self.define_schema_details(root_schema.items(), &elem_type, file_path, is_required, path_options);
            }
            NodeKind::TsInterfaceDecl(_) => {
                let extends = root.extends();
                if extends.len().gt(&0) {
                    let all_of = root_schema.all_of();
                    for extend in &extends {
                        let mut schema = ApiSchema::new();
                        self.define_schema_details(&mut schema, &extend, file_path, is_required, path_options);
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
                                        self.define_schema_details(
                                            property_schema,
                                            &annotation,
                                            file_path,
                                            is_required,
                                            path_options,
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
                                        self.define_schema_details(
                                            property_schema,
                                            &annotation,
                                            file_path,
                                            is_required,
                                            path_options,
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
                    self.define_schema_details(root_schema, &annotation, file_path, is_required, path_options);
                }
            }
            NodeKind::TsType(_) => {
                for child_index in root.children() {
                    let child = root.get(child_index).unwrap();
                    self.define_schema_details(root_schema, &child, file_path, is_required, path_options);
                }
            }
            NodeKind::TsEnumDecl(_) => {
                for member in root.members() {
                    root_schema.data_type("string");
                    self.define_schema_details(root_schema, &member, file_path, is_required, path_options);
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
                    let child = root.get(child).unwrap();
                    self.define_schema_details(root_schema, &child, file_path, is_required, path_options);
                }
            }
        }
    }

    fn define_schema_from_identifier(
        &mut self,
        identifier: &str,
        root_schema: &mut ApiSchema,
        file_path: &str,
        path_options: &PathOptions,
        root: &Rc<SchemyNode>,
        is_required: bool,
    ) -> () {
        if identifier.eq("LilSub") {
            let params = root.params();
            let param = params.last().unwrap();
            self.define_schema_details(root_schema, &param.clone(), file_path, true, path_options);
        } else if identifier.eq("LilRequiredField") {
            let params = root.params();
            let param = params.first().unwrap();
            self.define_schema_details(root_schema, &param.clone(), file_path, true, path_options);
        } else if identifier.eq("Array") {
            let items_schema = root_schema.data_type("array").items();
            if let Some(type_params) = root.type_params() {
                match type_params.kind {
                    NodeKind::TsTypeParamInstantiation(raw) => {
                        if let Some(type_param) = raw.params.get(0) {
                            let type_param = type_params.to_child(NodeKind::TsType(type_param));
                            self.define_schema_details(items_schema, &type_param, file_path, is_required, path_options);
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
            match self.symbol_tables.get_root_declaration(file_path, identifier) {
                Some(Declaration::Import { name, source_file_name }) => {
                    root_schema.reference(Some(identifier.to_string()), false);
                    self.deferred_schemas
                        .defer_external_type(&source_file_name, identifier, &name);
                }
                _ => {
                    root_schema.reference(Some(identifier.to_string()), false);
                    self.deferred_schemas
                        .defer_local_type(file_path, identifier, identifier, root.index);
                }
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
