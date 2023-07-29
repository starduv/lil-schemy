use std::rc::Rc;

use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use deno_ast::{swc::ast::*, ParseParams, ParsedSource, SourceTextInfo};
use lazy_static::__Deref;

use serde::{ser::SerializeStruct, Serialize, Serializer};
use url::Url;

use crate::typescript::{Declaration, DeclarationTables, SchemyNode};

use super::{declaration_helpers::store_declaration_maybe, deferred_schemas::DeferredSchemas};

pub struct OpenApiFactory<'n> {
    ast_cache: HashMap<String, ParsedSource>,
    symbol_tables: DeclarationTables<'n>,
}

impl<'n> OpenApiFactory<'n> {
    pub(crate) fn from_source_file(&mut self, file_path: &str) -> OpenApi {
        let mut open_api = OpenApi::new();
        let mut deferred_schemas = DeferredSchemas::new();
        let symbol_tables = &mut DeclarationTables::default();

        let result = self.get_syntax_tree(file_path);
        let module = SchemyNode::Module {
            node: Rc::new(result),
            parent: None,
        };

        find_paths(&mut open_api, module, file_path, &mut deferred_schemas, symbol_tables);

        while let Some(source_file_name) = deferred_schemas.next_module() {
            let result = self.get_syntax_tree(&source_file_name);
            let module = SchemyNode::Module {
                node: Rc::new(result),
                parent: None,
            };
            for child in module.children() {
                define_deferred_schemas(
                    &mut open_api,
                    child,
                    &source_file_name,
                    &mut deferred_schemas,
                    symbol_tables,
                );
            }
        }

        open_api
    }

    fn get_syntax_tree(&mut self, path: &str) -> &Module {
        self.ast_cache
            .entry(path.to_string())
            .or_insert_with(|| {
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
            })
            .module()
    }

    pub(crate) fn new() -> Self {
        Self {
            ast_cache: HashMap::new(),
            symbol_tables: DeclarationTables::default(),
        }
    }

    // fn add_param_from_referenced_type(
    //     &mut self,
    //     schema_type_name: &str,
    //     operation: &mut ApiPathOperation,
    //     file_path: &str,
    // ) -> () {
    // let root_declaration = self.symbol_tables.get_root_declaration(file_path, type_ref);
    // match  root_declaration{
    //     // Some(Declaration::Import { name, source_file_name }) => find_referenced_type(file_path, name, |declaration: dprint_swc_ext::swc::ast::Decl| {
    //     //     match declaration {
    //     //         dprint_swc_ext::swc::ast::Decl::TsInterface(interface_declaration) => {
    //     //             let name = interface_declaration.id.sym().to_string();
    //     //             self.add_param_from_referenced_type(operation, &name, source_file_name);
    //     //         }
    //     //         dprint_swc_ext::swc::ast::Decl::TsTypeAlias(alias_declaration) => {
    //     //             let name = alias_declaration.id.sym().to_string();
    //     //             self.add_param_from_referenced_type(operation, &name, source_file_name);
    //     //         }
    //     //         _ => {}
    //     //     }
    //     // }),
    //     Some(Declaration::Type { node }) => self.add_request_params(operation, &node, file_path),
    //     _ => {}
    // }
    // }
}

// BEGIN HELPER FUNCTINOS
fn find_paths<'n>(
    open_api: &mut OpenApi,
    node: SchemyNode<'n>,
    file_path: &str,
    deferred_schemas: &mut DeferredSchemas,
    symbol_tables: &mut DeclarationTables<'n>,
) {
    store_declaration_maybe(node.clone(), file_path, symbol_tables);
    let children = node.children();
    for child in children {
        match child {
            ref call_expression @ SchemyNode::CallExpr { node: _, parent: _ } => match call_expression.callee() {
                Some(SchemyNode::Callee {
                    node: expression,
                    parent: _,
                }) => match expression.deref() {
                    Callee::Expr(ident) => match &**ident {
                        Expr::Ident(ident) if ident.sym.eq("Path") => {
                            symbol_tables.add_child_scope(file_path);
                            add_path(
                                open_api,
                                call_expression.clone(),
                                file_path,
                                deferred_schemas,
                                symbol_tables,
                            );
                            symbol_tables.parent_scope(file_path);
                        }
                        _ => find_paths(open_api, child, file_path, deferred_schemas, symbol_tables),
                    },
                    _ => {}
                },
                _ => {}
            },
            _ => find_paths(open_api, child, file_path, deferred_schemas, symbol_tables),
        }
    }
}

fn add_path<'n>(
    open_api: &mut OpenApi,
    node: SchemyNode,
    file_path: &str,
    deferred_schemas: &mut DeferredSchemas,
    symbol_tables: &mut DeclarationTables<'n>,
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
        route_handler.clone(),
        file_path,
        deferred_schemas,
        symbol_tables,
    );

    open_api
        .path(&options.path.unwrap())
        .add_operation(&options.method.unwrap(), operation);
}

fn add_request_details<'n>(
    open_api: &mut OpenApi,
    operation: &mut ApiPathOperation,
    route_handler: SchemyNode,
    file_path: &str,
    deferred_schemas: &mut DeferredSchemas,
    symbol_tables: &mut DeclarationTables<'n>,
) -> () {
    if let arrow_expression @ SchemyNode::ArrowExpr { node: _, parent: _ } = route_handler {
        for param in &arrow_expression.params() {
            add_request_params(
                open_api,
                operation,
                param.clone(),
                file_path,
                deferred_schemas,
                symbol_tables,
            );
        }

        symbol_tables.add_child_scope(file_path);

        if let Some(body) = arrow_expression.body() {
            find_response(
                open_api,
                operation,
                body.clone(),
                file_path,
                deferred_schemas,
                symbol_tables,
            );
        }

        symbol_tables.parent_scope(file_path);
    }
}

fn add_request_params(
    open_api: &mut OpenApi,
    operation: &mut ApiPathOperation,
    node: SchemyNode,
    file_path: &str,
    deferred_schemas: &mut DeferredSchemas,
    symbol_tables: &mut DeclarationTables,
) {
    for node in node.children() {
        match node {
            SchemyNode::TsTypeRef {
                node: ref type_ref,
                parent: None,
            } => match &type_ref.deref().type_name {
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
                        node,
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
                        node,
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
                        node,
                        file_path,
                        deferred_schemas,
                        symbol_tables,
                    );
                }
                // TODO support route handler params in separate module
                // TsEntityName::Ident(identifier) => {
                //     self.add_param_from_referenced_type(&identifier.sym, operation, file_path);
                // }
                _ => add_request_params(open_api, operation, node, file_path, deferred_schemas, symbol_tables),
            },
            _ => add_request_params(open_api, operation, node, file_path, deferred_schemas, symbol_tables),
        }
    }
}

fn add_param_details(
    open_api: &mut OpenApi,
    operation: &mut ApiPathOperation,
    location: &str,
    type_ref: SchemyNode,
    file_path: &str,
    deferred_schemas: &mut DeferredSchemas,
    symbol_tables: &mut DeclarationTables,
) {
    let parameter_name = get_parameter_name(&type_ref);
    let operation_param = operation.param(&parameter_name, location);
    if let SchemyNode::TsTypeRef {
        node: ref type_ref,
        parent: _,
    } = type_ref
    {
        if let Some(type_params) = &type_ref.type_params {
            let namespace = match type_params.params.get(2) {
                Some(namespace) => match &**namespace {
                    TsType::TsLitType(namespace) => match &namespace.lit {
                        TsLit::Str(literal_string) => Some(literal_string.value.to_string()),
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            };

            match type_params.params.get(0) {
                Some(param) => match &**param {
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
                    TsType::TsTypeRef(type_ref) => match &type_ref.type_name {
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
                Some(param) => match &**param {
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
                Some(param) => match &**param {
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
    if let Some(type_params) = &type_ref.type_params {
        let namespace = match type_params.params.get(2) {
            Some(namespace) => match &**namespace {
                TsType::TsLitType(namespace) => match &namespace.lit {
                    TsLit::Str(literal_string) => Some(literal_string.value.to_string()),
                    _ => None,
                },
                _ => None,
            },
            None => None,
        };

        match type_params.params.get(0) {
            Some(param_type) => match &**param_type {
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
                TsType::TsTypeRef(type_ref) => match &type_ref.type_name {
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
            Some(required) => match &**required {
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

fn find_response<'n>(
    open_api: &mut OpenApi,
    operation: &mut ApiPathOperation,
    root: SchemyNode<'n>,
    file_path: &str,
    deferred_schemas: &mut DeferredSchemas,
    symbol_tables: &mut DeclarationTables<'n>,
) -> () {
    for ref child in root.children() {
        store_declaration_maybe(child.clone(), file_path, symbol_tables);

        match child {
            SchemyNode::Ident {
                node: identifier,
                parent: _,
            } if identifier.sym.eq("Response") => add_response(
                open_api,
                operation,
                root.clone(),
                file_path,
                deferred_schemas,
                symbol_tables,
            ),
            _ => find_response(
                open_api,
                operation,
                child.clone(),
                file_path,
                deferred_schemas,
                symbol_tables,
            ),
        }
    }
}

// TODO add schema for ref here
fn add_response(
    open_api: &mut OpenApi,
    operation: &mut ApiPathOperation,
    node: SchemyNode,
    file_path: &str,
    deferred_schemas: &mut DeferredSchemas,
    symbol_tables: &mut DeclarationTables,
) -> () {
    if let SchemyNode::CallExpr {
        node: ref call_expression,
        parent: _,
    } = node
    {
        let options = match call_expression.args.get(1) {
            Some(arg) => match &*arg.expr {
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
            Some(arg) => match &*arg.expr {
                Expr::New(new_expression) => match &*new_expression.callee {
                    Expr::Ident(identifier) => {
                        Some(symbol_tables.get_root_declaration_name(file_path, identifier.sym.to_string()))
                    }
                    _ => None,
                },
                Expr::Ident(response_type) => {
                    Some(symbol_tables.get_root_declaration_name(file_path, response_type.sym.to_string()))
                }
                Expr::TsAs(ts_as) => match &*ts_as.type_ann {
                    TsType::TsTypeRef(type_ref) => match &type_ref.type_name {
                        TsEntityName::Ident(identifier) => {
                            Some(symbol_tables.get_root_declaration_name(file_path, identifier.sym.to_string()))
                        }
                        _ => None,
                    },
                    _ => None,
                },
                Expr::TsTypeAssertion(type_assertion) => match &*type_assertion.type_ann {
                    TsType::TsTypeRef(type_ref) => match &type_ref.type_name {
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

fn define_deferred_schemas<'n>(
    open_api: &mut OpenApi,
    node: SchemyNode<'n>,
    source_file_name: &str,
    deferred_schemas: &mut DeferredSchemas,
    symbol_tables: &mut DeclarationTables<'n>,
) -> () {
    store_declaration_maybe(node.clone(), source_file_name, symbol_tables);

    match node {
        SchemyNode::ExportDefaultExpr { node: _, parent: _ } => {
            define_deferred_type_maybe(open_api, "default", source_file_name, deferred_schemas, symbol_tables)
        }
        SchemyNode::ExportDecl {
            node: ref export_declaration,
            parent: _,
        } => match &export_declaration.decl {
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

// END HELPER FUNCTIONS

#[derive(Serialize, Debug)]
pub struct OpenApi {
    pub components: ApiComponents,
    pub paths: HashMap<String, ApiPath>,
}

impl OpenApi {
    pub(crate) fn new() -> Self {
        OpenApi {
            components: ApiComponents::new(),
            paths: HashMap::new(),
        }
    }

    pub(crate) fn path(&mut self, key: &str) -> &mut ApiPath {
        self.paths.entry(key.to_string()).or_insert(ApiPath::new())
    }

    pub fn merge(&mut self, open_api: OpenApi) -> () {
        self.components.schemas.extend(open_api.components.schemas);
        self.paths.extend(open_api.paths);
    }
}

#[derive(Serialize, Debug)]
pub struct ApiComponents {
    schemas: HashMap<String, ApiSchema>,
}

impl ApiComponents {
    pub fn new() -> Self {
        ApiComponents {
            schemas: HashMap::new(),
        }
    }

    pub fn schema(&mut self, name: &str) -> &mut ApiSchema {
        self.schemas.entry(name.to_string()).or_insert(ApiSchema::new())
    }
}

#[derive(Serialize, Debug)]
pub struct ApiPath {
    #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    schema_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    get: Option<ApiPathOperation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    put: Option<ApiPathOperation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post: Option<ApiPathOperation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<ApiPathOperation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<ApiPathOperation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    head: Option<ApiPathOperation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    patch: Option<ApiPathOperation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trace: Option<ApiPathOperation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    servers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<ApiPathParameter>,
}

impl<'v> ApiPath {
    fn new() -> ApiPath {
        ApiPath {
            schema_ref: None,
            summary: None,
            description: None,
            get: None,
            put: None,
            post: None,
            delete: None,
            options: None,
            head: None,
            patch: None,
            trace: None,
            servers: None,
            parameters: None,
        }
    }

    fn add_operation(&mut self, method: &str, operation: ApiPathOperation) -> &mut ApiPathOperation {
        match method.to_lowercase().as_str() {
            "get" => self.get.insert(operation),
            "put" => self.put.insert(operation),
            "post" => self.post.insert(operation),
            "delete" => self.delete.insert(operation),
            "options" => self.options.insert(operation),
            "head" => self.head.insert(operation),
            "patch" => self.patch.insert(operation),
            "trace" => self.trace.insert(operation),
            other => panic!("Unsupported http method '{}'", other),
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct ApiPathOperation {
    #[serde(rename = "requestBody", skip_serializing_if = "Option::is_none")]
    body_parameter: Option<ApiParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    examples: Option<HashMap<String, ApiSchema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<Vec<ApiParam>>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    responses: HashMap<String, ApiResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<String>>,
}

impl ApiPathOperation {
    pub fn new() -> Self {
        ApiPathOperation {
            body_parameter: None,
            examples: None,
            responses: HashMap::new(),
            parameters: None,
            tags: None,
        }
    }

    pub(crate) fn tags(&mut self, tags: Option<Vec<String>>) -> &mut ApiPathOperation {
        self.tags = tags;
        self
    }

    pub(crate) fn response(
        &mut self,
        response_type: &Option<String>,
        response_args: ResponseOptions,
    ) -> &mut ApiResponse {
        let status_code = response_args
            .status_code
            .expect("An ApiResponse must have a status code");

        let description = response_args
            .description
            .expect("An ApiResponse must have a description");

        let mut response = ApiResponse::new(description);

        let content = response
            .content()
            .example(response_args.example, response_args.namespace.clone());

        if response_type.is_some() {
            content
                .schema()
                .reference(response_type.to_owned(), false)
                .namespace(response_args.namespace);
        }

        self.responses.entry(status_code).or_insert(response)
    }

    pub(crate) fn param(&mut self, name: &str, location: &str) -> &mut ApiParam {
        let param = ApiParam::new(Some(name), Some(location));
        self.parameters.get_or_insert_with(Default::default).push(param);
        self.parameters
            .get_or_insert_with(Default::default)
            .last_mut()
            .expect("Could not get parameter from ApiOperation")
    }

    pub(crate) fn body(&mut self) -> &mut ApiParam {
        self.body_parameter.get_or_insert(ApiParam::new(None, None))
    }
}

#[derive(Serialize, Debug)]
pub struct ApiPathParameter {}

#[derive(Clone, Debug, Serialize)]
pub struct ApiResponse {
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<HashMap<String, ApiParam>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<HashMap<String, ApiConent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<Vec<ApiSchema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    examples: Option<Vec<String>>,
}

impl ApiResponse {
    fn new(description: String) -> Self {
        ApiResponse {
            content: None,
            description,
            examples: None,
            headers: None,
            links: None,
        }
    }

    pub(crate) fn content(&mut self) -> &mut ApiConent {
        let key = "application/json";
        self.content
            .get_or_insert_with(Default::default)
            .entry(key.to_owned())
            .or_insert(ApiConent::new())
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ApiConent {
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<ApiSchema>,
    #[serde(skip_serializing_if = "Option::is_none")]
    example: Option<Box<ApiSchema>>,
}
impl ApiConent {
    pub fn new() -> Self {
        ApiConent {
            schema: None,
            example: None,
        }
    }

    pub fn schema(&mut self) -> &mut ApiSchema {
        self.schema.get_or_insert(ApiSchema::new())
    }

    pub fn example(&mut self, example: Option<String>, namespace: Option<String>) -> &mut ApiConent {
        if example.is_some() {
            let mut schema = ApiSchema::new();
            schema.reference(example, true).namespace(namespace);
            self.example = Some(Box::new(schema));
        }
        self
    }
}

#[derive(Clone, Debug)]
pub struct ApiSchema {
    items: Option<Box<ApiSchema>>,
    format: Option<String>,
    data_type: Option<String>,
    reference: Option<String>,
    namespace: Option<String>,
    is_example: bool,
    properties: Option<HashMap<String, ApiSchema>>,
    required: HashSet<String>,
}

impl Serialize for ApiSchema {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ApiSchema", 5)?;
        if let Some(ref format) = self.format {
            state.serialize_field("format", format)?;
        }
        if let Some(ref items) = self.items {
            state.serialize_field("items", items)?;
        }
        if let Some(ref properties) = self.properties {
            state.serialize_field("properties", properties)?;
        }
        if !self.required.is_empty() {
            state.serialize_field("required", &self.required)?;
        }
        if let Some(ref data_type) = self.data_type {
            state.serialize_field("type", data_type)?;
        }
        if let Some(ref reference) = self.reference {
            let mut path = String::from(match self.is_example {
                true => "#/components/examples/",
                false => "#/components/schemas/",
            });

            if let Some(ref namespace) = self.namespace {
                path.push_str(namespace);
                match self.is_example {
                    true => path.push('.'),
                    false => path.push_str("/properties/"),
                }
            }

            path.push_str(reference);
            state.serialize_field("$ref", &path)?;
        }
        state.end()
    }
}

impl ApiSchema {
    pub fn new() -> Self {
        ApiSchema {
            format: None,
            data_type: None,
            reference: None,
            namespace: None,
            is_example: false,
            items: None,
            properties: None,
            required: HashSet::new(),
        }
    }

    pub fn data_type(&mut self, data_type: &str) -> &mut ApiSchema {
        self.data_type = Some(data_type.into());
        self
    }

    pub fn format(&mut self, format: Option<String>) -> &mut ApiSchema {
        // TODO add format tests
        self.format = format;
        self
    }

    pub fn namespace(&mut self, namespace: Option<String>) -> &mut ApiSchema {
        self.namespace = namespace;
        self
    }

    pub fn reference(&mut self, reference: Option<String>, is_example: bool) -> &mut ApiSchema {
        self.is_example = is_example;
        self.reference = reference;
        self
    }

    pub fn property(&mut self, name_text: &str) -> &mut ApiSchema {
        self.required.insert(name_text.to_string());

        self.properties
            .get_or_insert(HashMap::new())
            .entry(name_text.to_string())
            .or_insert(ApiSchema::new())
    }

    pub fn array(&mut self) -> &mut ApiSchema {
        self.data_type = Some(String::from("array"));
        self
    }

    pub fn items(&mut self) -> &mut ApiSchema {
        self.items.get_or_insert(Box::new(ApiSchema::new()))
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ApiParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename = "in", skip_serializing_if = "Option::is_none")]
    location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<HashMap<String, ApiConent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<bool>,
}

impl ApiParam {
    fn new(name: Option<&str>, location: Option<&str>) -> ApiParam {
        ApiParam {
            content: None,
            location: location.map(|l| l.to_string()),
            name: name.map(|n| n.to_string()),
            required: None,
        }
    }

    pub(crate) fn content(&mut self) -> &mut ApiConent {
        let key = "application/json";
        self.content
            .get_or_insert(HashMap::new())
            .entry(key.to_owned())
            .or_insert(ApiConent::new())
    }

    pub(crate) fn required(&mut self, required: bool) -> &mut ApiParam {
        self.required = Some(required);
        self
    }
}

#[derive(Debug)]
pub struct PathOptions {
    pub method: Option<String>,
    pub path: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl PathOptions {
    pub(crate) fn new() -> Self {
        PathOptions {
            method: None,
            path: None,
            tags: None,
        }
    }
}

#[derive(Debug)]
pub struct ResponseOptions {
    pub description: Option<String>,
    pub example: Option<String>,
    pub namespace: Option<String>,
    pub status_code: Option<String>,
}
impl ResponseOptions {
    pub(crate) fn new() -> Self {
        ResponseOptions {
            description: None,
            example: None,
            namespace: None,
            status_code: None,
        }
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
                root_schema.data_type = Some("number".into());
            }
            TsKeywordTypeKind::TsBooleanKeyword => {
                root_schema.data_type = Some("boolean".into());
            }
            TsKeywordTypeKind::TsBigIntKeyword => {
                root_schema.data_type = Some("number".into());
            }
            TsKeywordTypeKind::TsStringKeyword => {
                root_schema.data_type = Some("string".into());
            }
            TsKeywordTypeKind::TsSymbolKeyword => {
                root_schema.data_type = Some("string".into());
            }
            _ => {}
        },
        SchemyNode::ClassDecl {
            node: class_declaration,
            parent: _,
        } => {
            root_schema.data_type = Some("object".into());
            for property in &class_declaration.class.body {
                match property {
                    ClassMember::ClassProp(class_property) => {
                        let name = match &class_property.key {
                            PropName::Ident(identifier) => Some(identifier.sym.to_string()),
                            _ => None,
                        };

                        if let Some(name) = name {
                            if let Some(annotation) = &class_property.type_ann {
                                define_referenced_schema_details(
                                    root_schema.property(&name),
                                    SchemyNode::TsType {
                                        node: Rc::new(&annotation.type_ann),
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
            root_schema.data_type = Some("object".into());
            for property in &class_declaration.class.body {
                match property {
                    ClassMember::ClassProp(class_property) => {
                        let name = match &class_property.key {
                            PropName::Ident(identifier) => Some(identifier.sym.to_string()),
                            _ => None,
                        };

                        if let Some(name) = name {
                            if let Some(annotation) = &class_property.type_ann {
                                define_referenced_schema_details(
                                    root_schema.property(&name),
                                    SchemyNode::TsType {
                                        node: Rc::new(&annotation.type_ann),
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
            root_schema.data_type = Some("array".into());
            define_referenced_schema_details(
                root_schema.items(),
                SchemyNode::TsType {
                    node: Rc::new(&array_type.elem_type),
                    parent: None,
                },
            );
        }
        SchemyNode::TsInterfaceDecl {
            node: interface_declaration,
            parent: _,
        } => {
            root_schema.data_type = Some("object".into());

            for property in &interface_declaration.body.body {
                match property {
                    TsTypeElement::TsPropertySignature(signature) => {
                        let property_schema = match &*signature.key {
                            Expr::Ident(identifier) => {
                                let name = identifier.sym.to_string();
                                Some(root_schema.property(&name))
                            }
                            _ => None,
                        };

                        if let Some(property_schema) = property_schema {
                            if let Some(annotation) = &signature.type_ann {
                                define_referenced_schema_details(
                                    property_schema,
                                    SchemyNode::TsType {
                                        node: Rc::new(&annotation.type_ann),
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
            root_schema.data_type = Some("object".into());
            for member in &type_literal.members {
                match member {
                    TsTypeElement::TsPropertySignature(signature) => {
                        let property_schema = match &*signature.key {
                            Expr::Ident(identifier) => {
                                let name = identifier.sym.to_string();
                                Some(root_schema.property(&name))
                            }
                            _ => None,
                        };

                        if let Some(property_schema) = property_schema {
                            if let Some(annotation) = &signature.type_ann {
                                define_referenced_schema_details(
                                    property_schema,
                                    SchemyNode::TsType {
                                        node: Rc::new(&annotation.type_ann),
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
                node: Rc::new(&type_alias_declaration.type_ann),
                parent: None,
            },
        ),
        _ => {}
    }
}

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
                parent: _,
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
            Some(ref parent) => get_parameter_name(parent),
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

fn find_referenced_type() {}
