use std::rc::Rc;

use lazy_static::__Deref;

use swc_ecma_ast::*;

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

        for immediate in self.deferred_schemas.take_immediate_types(file_path) {
            let root = root.get(immediate.index).unwrap();
            self.define_referenced_schema(
                root,
                &immediate.schema_name,
                &immediate.schema_name,
                open_api,
                file_path,
            )
        }

        while let Some(source_file_name) = self.deferred_schemas.next_module() {
            let deferred_root = module_cache.parse(&source_file_name);
            for item_index in deferred_root.children() {
                let item = deferred_root.get(item_index).unwrap();
                self.define_deferred_schemas(open_api, item, &source_file_name);
            }
        }
    }

    fn find_paths<'m>(&mut self, open_api: &mut OpenApi, root: Rc<SchemyNode<'m>>, file_path: &str) {
        store_declaration_maybe(root.clone(), file_path, &mut self.symbol_tables);

        for child_index in root.children() {
            let child = root.get(child_index).unwrap();
            match &child.kind {
                NodeKind::Ident(raw_ident) if raw_ident.sym.eq("Path") => {
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

        let operation = open_api
            .path(&options.path.unwrap())
            .add_operation(&options.method.unwrap());

        unsafe {
            let operation: &mut ApiPathOperation = &mut *operation;
            operation.tags(options.tags.clone());
        }

        self.add_request_details(open_api, operation, route_handler.clone(), file_path);
    }

    fn add_request_details(
        &mut self,
        open_api: &mut OpenApi,
        operation: *mut ApiPathOperation,
        route_handler: Rc<SchemyNode>,
        file_path: &str,
    ) -> () {
        for param in route_handler.params() {
            self.add_request_params(open_api, operation, param, file_path);
        }

        self.symbol_tables.add_child_scope(file_path);

        self.find_response(open_api, operation, route_handler, file_path);

        self.symbol_tables.parent_scope(file_path);
    }

    fn add_request_params(
        &mut self,
        open_api: &mut OpenApi,
        operation: *mut ApiPathOperation,
        root: Rc<SchemyNode>,
        file_path: &str,
    ) {
        match root.kind {
            NodeKind::Ident(identifier) if identifier.sym.eq("BodyParam") => {
                self.add_body_param_details(open_api, operation, root.parent().unwrap().parent().unwrap(), file_path);
            }
            NodeKind::Ident(identifier) if identifier.sym.eq("Header") => {
                self.add_param_details(operation, "header", root.parent().unwrap().parent().unwrap(), file_path);
            }
            NodeKind::Ident(identifier) if identifier.sym.eq("QueryParam") => {
                self.add_param_details(operation, "query", root.parent().unwrap().parent().unwrap(), file_path);
            }
            NodeKind::Ident(identifier) if identifier.sym.eq("RouteParam") => {
                self.add_param_details(operation, "path", root.parent().unwrap().parent().unwrap(), file_path);
            }
            NodeKind::Ident(identifier) => match self.symbol_tables.get_root_declaration(file_path, &identifier.sym) {
                Some(Declaration::Import { name, source_file_name }) => {
                    self.deferred_schemas
                        .add_deferred_operation_type(&source_file_name, operation, &name);
                }
                _ => {}
            },
            _ => {
                for child_index in root.children() {
                    self.add_request_params(open_api, operation, root.get(child_index).unwrap(), file_path);
                }
            }
        }
    }

    fn add_param_details(
        &mut self,
        operation: *mut ApiPathOperation,
        location: &str,
        root: Rc<SchemyNode>,
        file_path: &str,
    ) {
        unsafe {
            let operation: &mut ApiPathOperation = &mut *operation;
            let parameter_name = get_parameter_name(root.clone());
            let operation_param = operation.param(&parameter_name, location);

            let type_params = root.params();

            if let Some(type_param) = type_params.get(0) {
                self.define_schema_details(operation_param.content().schema(), type_param.clone(), file_path);
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
    }

    fn find_response(
        &mut self,
        open_api: &mut OpenApi,
        operation: *mut ApiPathOperation,
        root: Rc<SchemyNode>,
        file_path: &str,
    ) -> () {
        for child_index in root.children() {
            let child = root.get(child_index.clone()).unwrap();
            store_declaration_maybe(child.clone(), file_path, &mut self.symbol_tables);
            match child.kind {
                NodeKind::Ident(raw) if raw.sym.eq("Response") => {
                    self.add_response(open_api, operation, root.parent().unwrap(), file_path)
                }
                _ => self.find_response(open_api, operation, child, file_path),
            }
        }
    }

    fn add_response(
        &mut self,
        open_api: &mut OpenApi,
        operation: *mut ApiPathOperation,
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

        match args.get(0) {
            Some(response_type) if options.is_some() => {
                self.add_response_details(&response_type, &options.unwrap(), file_path, operation, open_api);
            }
            _ => {}
        };
    }

    fn add_response_details(
        &mut self,
        root: &Rc<SchemyNode>,
        options: &ResponseOptions,
        file_path: &str,
        operation: *mut ApiPathOperation,
        open_api: &mut OpenApi,
    ) {
        match root.kind {
            NodeKind::Ident(raw_ident) => {
                if raw_ident.sym.eq("Array") {
                    let status_code = options.status_code.as_ref().unwrap();
                    let description = options.description.as_ref().unwrap();

                    unsafe {
                        let operation: &mut ApiPathOperation = &mut *operation;
                        let response = operation.response(&status_code, &description);
                        let content = response.content();
                        let items_schema = content.schema().data_type("array").items();

                        if let Some(type_params) = root.parent().unwrap().parent().unwrap().type_params() {
                            match type_params.kind {
                                NodeKind::TsTypeParamInstantiation(raw) => {
                                    if let Some(type_param) = raw.params.get(0) {
                                        let type_param = type_params.to_child(NodeKind::TsType(type_param));
                                        self.define_schema_details(items_schema, type_param, file_path);
                                    }
                                }
                                _ => {}
                            }
                        }

                        content.example(options.example.clone(), options.namespace.clone());
                    }
                } else {
                    let name = self
                        .symbol_tables
                        .get_root_declaration_name(file_path, raw_ident.sym.to_string());

                    let status_code = options.status_code.as_ref().unwrap();
                    let description = options.description.as_ref().unwrap();

                    unsafe {
                        let operation: &mut ApiPathOperation = &mut *operation;
                        let response = operation.response(&status_code, &description);
                        let content = response.content();
                        content.schema().reference(Some(name.clone()), false);
                        content.example(options.example.clone(), options.namespace.clone());
                    }

                    self.define_referenced_schema(root.clone(), &name, &name, open_api, file_path);
                }
            }
            NodeKind::Str(_) => {
                let status_code = options.status_code.as_ref().unwrap();
                let description = options.description.as_ref().unwrap();

                unsafe {
                    let operation: &mut ApiPathOperation = &mut *operation;
                    let response = operation.response(&status_code, &description);
                    let content = response.content();
                    content.schema().data_type("string");
                    content.example(options.example.clone(), options.namespace.clone());
                }
            }
            NodeKind::Bool(_) => {
                let status_code = options.status_code.as_ref().unwrap();
                let description = options.description.as_ref().unwrap();

                unsafe {
                    let operation: &mut ApiPathOperation = &mut *operation;
                    let response = operation.response(&status_code, &description);
                    let content = response.content();
                    content.schema().data_type("boolean");
                    content.example(options.example.clone(), options.namespace.clone());
                }
            }
            NodeKind::Null(_) => {
                let status_code = options.status_code.as_ref().unwrap();
                let description = options.description.as_ref().unwrap();

                unsafe {
                    let operation: &mut ApiPathOperation = &mut *operation;
                    let response = operation.response(&status_code, &description);
                    let content = response.content();
                    content.example(options.example.clone(), options.namespace.clone());
                }
            }
            NodeKind::Num(_) => {
                let status_code = options.status_code.as_ref().unwrap();
                let description = options.description.as_ref().unwrap();

                unsafe {
                    let operation: &mut ApiPathOperation = &mut *operation;
                    let response = operation.response(&status_code, &description);
                    let content = response.content();
                    content.schema().data_type("number");
                    content.example(options.example.clone(), options.namespace.clone());
                }
            }
            NodeKind::BigInt(_) => {
                let status_code = options.status_code.as_ref().unwrap();
                let description = options.description.as_ref().unwrap();

                unsafe {
                    let operation: &mut ApiPathOperation = &mut *operation;
                    let response = operation.response(&status_code, &description);
                    let content = response.content();
                    content.schema().data_type("number");
                    content.example(options.example.clone(), options.namespace.clone());
                }
            }
            NodeKind::Regex(_) => {
                let status_code = options.status_code.as_ref().unwrap();
                let description = options.description.as_ref().unwrap();

                unsafe {
                    let operation: &mut ApiPathOperation = &mut *operation;
                    let response = operation.response(&status_code, &description);
                    let content = response.content();
                    content.schema().data_type("string");
                    content.example(options.example.clone(), options.namespace.clone());
                }
            }

            _ => {
                for child_index in root.children() {
                    let child = root.get(child_index).unwrap();
                    self.add_response_details(&child, options, file_path, operation, open_api);
                }
            }
        };
    }

    fn add_body_param_details(
        &mut self,
        open_api: &mut OpenApi,
        operation: *mut ApiPathOperation,
        root: Rc<SchemyNode>,
        file_path: &str,
    ) -> () {
        unsafe {
            let operation: &mut ApiPathOperation = &mut *operation;
            let operation_param = operation.body();
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
    }

    fn define_referenced_schema(
        &mut self,
        type_node: Rc<SchemyNode>,
        type_name: &str,
        schema_name: &str,
        open_api: &mut OpenApi,
        file_path: &str,
    ) -> () {
        match self.symbol_tables.get_root_declaration(file_path, type_name) {
            Some(Declaration::Export {
                name: type_name,
                source_file_name,
            }) => {
                self.deferred_schemas
                    .add_deferred_type(&source_file_name, schema_name.into(), &type_name);
            }
            Some(Declaration::Import {
                name: type_name,
                source_file_name,
            }) => {
                self.deferred_schemas
                    .add_deferred_type(&source_file_name, schema_name.into(), &type_name);
            }
            Some(Declaration::Type { node: node_index }) => {
                let schema = open_api.components.schema(schema_name);

                if let Some(node) = type_node.get(node_index) {
                    self.define_schema_details(schema, node, file_path);
                } else {
                    // we may not have found this type yet as we traverse the module
                    // self.deferred_schemas.add_deferred_type(file_path, schema_name, type_name, namespace);
                    panic!("we didn't find the type node for {}", type_name);
                }
            }
            _ => {}
        };
    }

    fn define_deferred_schemas(&mut self, open_api: &mut OpenApi, root: Rc<SchemyNode>, source_file_name: &str) -> () {
        store_declaration_maybe(root.clone(), source_file_name, &mut self.symbol_tables);
        match &root.kind {
            NodeKind::ModuleItem(ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(raw_expr))) => {
                let root = root.to_child(NodeKind::ExportDefaultExpr(raw_expr));
                println!("- found a default export");
                self.define_deferred_type_maybe(root, open_api, "default", source_file_name)
            }
            NodeKind::ModuleItem(ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(raw_decl))) => match &raw_decl.decl {
                Decl::Class(ref raw_class) => {
                    println!("- found a class export");
                    let root = root.to_child(NodeKind::ClassDecl(raw_class));
                    self.define_deferred_type_maybe(root, open_api, &raw_class.ident.sym, source_file_name)
                }
                Decl::TsInterface(ref raw_interface) => {
                    println!("- found a interface export");
                    let root = root.to_child(NodeKind::TsInterfaceDecl(raw_interface));
                    self.define_deferred_type_maybe(root, open_api, &raw_interface.id.sym, source_file_name)
                }
                Decl::TsTypeAlias(ref raw_alias) => {
                    println!("- found a type alias export");
                    let root = root.to_child(NodeKind::TsTypeAliasDecl(raw_alias));
                    self.define_deferred_type_maybe(root, open_api, &raw_alias.id.sym, source_file_name)
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

                            println!("- found named export");

                            self.define_deferred_type_maybe(specifier, open_api, &name, source_file_name);
                        }
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
        if let Some(deferred_operation_type) = self
            .deferred_schemas
            .get_deferred_operation_type(type_name, source_file_name)
        {
            match self.symbol_tables.get_root_declaration(source_file_name, type_name) {
                Some(Declaration::Type { node }) => unsafe {
                    let root = root.get(node).unwrap();
                    let operation: &mut ApiPathOperation = &mut *deferred_operation_type.operation;
                    self.add_request_params(open_api, operation, root, source_file_name);
                },
                Some(Declaration::Import {
                    name: imported_name,
                    source_file_name: module_file_name,
                }) => {
                    self.deferred_schemas.add_deferred_operation_type(
                        &module_file_name,
                        deferred_operation_type.operation,
                        &imported_name,
                    );
                }
                _ => {}
            }
        }

        if let Some(deferred_type) = self.deferred_schemas.get_deferred_type(type_name, source_file_name) {
            match self.symbol_tables.get_root_declaration(source_file_name, &type_name) {
                Some(Declaration::Type { node }) => {
                    let schema = open_api.components.schema(&deferred_type.schema_name);

                    let node = root.get(node).unwrap();

                    self.define_schema_details(schema, node, source_file_name);
                }
                Some(Declaration::Import {
                    name: imported_name,
                    source_file_name: module_file_name,
                }) => {
                    self.deferred_schemas
                        .add_deferred_type(&module_file_name, type_name, &imported_name);
                }
                _ => {}
            }
        }
    }

    fn define_schema_details(&mut self, root_schema: &mut ApiSchema, root: Rc<SchemyNode>, file_path: &str) -> () {
        match root.kind {
            NodeKind::TsTypeRef(raw_type) => match &raw_type.type_name {
                TsEntityName::Ident(identifier) => {
                    if identifier.sym.eq("Array") {
                        let items_schema = root_schema.data_type("array").items();
                        if let Some(type_params) = root.type_params() {
                            match type_params.kind {
                                NodeKind::TsTypeParamInstantiation(raw) => {
                                    if let Some(type_param) = raw.params.get(0) {
                                        let type_param = type_params.to_child(NodeKind::TsType(type_param));
                                        self.define_schema_details(items_schema, type_param, file_path);
                                    }
                                }
                                _ => {}
                            }
                        }
                    } else {
                        let root_name = self
                            .symbol_tables
                            .get_root_declaration_name(file_path, identifier.sym.to_string());

                        root_schema.reference(Some(root_name.clone()), false);

                        self.deferred_schemas
                            .add_deferred_immediate(file_path, &root_name, root.index);
                    }
                }
                _ => println!("found some strang type ref"),
            },
            NodeKind::TsTypeAnnotation(_) => {
                for child_index in root.children() {
                    let child = root.get(child_index).unwrap();
                    self.define_schema_details(root_schema, child, file_path);
                }
            }
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
                            NodeKind::ClassMember(_) => {
                                let class_prop = property.class_prop().unwrap();
                                if let NodeKind::ClassProp(raw_prop) = class_prop.kind {
                                    let name = match &raw_prop.key {
                                        PropName::Ident(identifier) => Some(identifier.sym.to_string()),
                                        _ => None,
                                    };

                                    if let Some(name) = name {
                                        if let Some(annotation) = class_prop.type_ann() {
                                            self.define_schema_details(
                                                root_schema.property(&name),
                                                annotation.clone(),
                                                file_path,
                                            );
                                        }
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
                                        self.define_schema_details(
                                            root_schema.property(&name),
                                            annotation.clone(),
                                            file_path,
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
                root_schema.data_type("array".into());
                let elem_type = root.elem_type().unwrap();
                self.define_schema_details(root_schema.items(), elem_type.clone(), file_path);
            }
            NodeKind::TsInterfaceDecl(_) => {
                root_schema.data_type("object".into());
                if let Some(interface_body) = root.interface_body() {
                    for interface_member in interface_body.body() {
                        match interface_member.kind {
                            NodeKind::TsTypeElement(TsTypeElement::TsPropertySignature(raw_prop)) => {
                                let property_schema = match &*raw_prop.key {
                                    Expr::Ident(identifier) => {
                                        let name = identifier.sym.to_string();
                                        Some(root_schema.property(&name))
                                    }
                                    _ => None,
                                };

                                if let Some(property_schema) = property_schema {
                                    if let Some(annotation) = interface_member.type_ann() {
                                        self.define_schema_details(property_schema, annotation.clone(), file_path);
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
                        NodeKind::TsTypeElement(raw_element) => match raw_element {
                            TsTypeElement::TsPropertySignature(raw_prop) => {
                                let property_schema = match &*raw_prop.key {
                                    Expr::Ident(identifier) => {
                                        let name = identifier.sym.to_string();
                                        Some(root_schema.property(&name))
                                    }
                                    _ => None,
                                };

                                if let Some(property_schema) = property_schema {
                                    if let Some(annotation) = member.type_ann() {
                                        self.define_schema_details(property_schema, annotation.clone(), file_path);
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
                    self.define_schema_details(root_schema, annotation.clone(), file_path);
                }
            }
            NodeKind::TsType(_) => {
                for child_index in root.children() {
                    let child = root.get(child_index).unwrap();
                    self.define_schema_details(root_schema, child, file_path);
                }
            }
            _ => {}
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
