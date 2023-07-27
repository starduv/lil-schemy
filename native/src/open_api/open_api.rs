use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use deno_ast::{swc::ast::*, ParseParams, ParsedSource, SourceTextInfo};
use lazy_static::__Deref;

use serde::{ser::SerializeStruct, Serialize, Serializer};
use url::Url;

use crate::typescript::{Declaration, DeclarationTables, SchemyNode};

use super::{declaration_helpers::store_declaration_maybe, deferred_schemas::DeferredSchemas};

#[derive(Serialize, Debug)]
pub struct OpenApi<'m> {
    pub components: ApiComponents,
    pub paths: HashMap<String, ApiPath>,
    #[serde(skip)]
    deferred_schemas: DeferredSchemas,
    #[serde(skip)]
    symbol_tables: DeclarationTables<'m>,
}
impl<'m> OpenApi<'m> {
    pub(crate) fn new() -> Self {
        OpenApi {
            components: ApiComponents::new(),
            deferred_schemas: DeferredSchemas::new(),
            paths: HashMap::new(),
            symbol_tables: Default::default(),
        }
    }

    pub(crate) fn path(&mut self, key: &str) -> &mut ApiPath {
        self.paths.entry(key.to_string()).or_insert(ApiPath::new())
    }

    pub(crate) fn merge(&mut self, open_api: OpenApi) -> () {
        self.components.schemas.extend(open_api.components.schemas);
        self.paths.extend(open_api.paths);
    }

    pub(crate) fn from_source_file(&mut self, file_path: &str) -> () {
        let result = get_syntax_tree(file_path);

        self.find_paths(
            SchemyNode::Module {
                node: result.module(),
                parent: None,
            },
            file_path,
        );

        while let Some(source_file_name) = self.deferred_schemas.next_module() {
            let result = get_syntax_tree(&source_file_name);
            let module = SchemyNode::Module {
                node: result.module(),
                parent: None,
            };
            for child in module.children() {
                self.define_deferred_schemas(&child, &source_file_name)
            }
        }
    }

    fn find_paths(&mut self, node: SchemyNode<'m>, file_path: &str) {
        store_declaration_maybe(&node, file_path, &mut self.symbol_tables);

        for child in node.children() {
            match child {
                call_expression @ SchemyNode::CallExpr { node: _, parent: _ } => match call_expression.callee() {
                    Some(SchemyNode::Callee {
                        node: Callee::Expr(expression),
                        parent: _,
                    }) => match **expression {
                        Expr::Ident(ident) if ident.sym.eq("Path") => {
                            self.symbol_tables.add_child_scope(file_path);
                            self.add_path(&call_expression, file_path);
                            self.symbol_tables.parent_scope(file_path);
                        }
                        _ => self.find_paths(child, file_path),
                    },
                    _ => {}
                },
                _ => self.find_paths(child, file_path),
            }
        }
    }

    fn add_path(&mut self, node: &SchemyNode, file_path: &str) -> () {
        let args = node.args();
        let route_handler = args.first();
        let route_options = args.last();
        let options = get_path_options(route_options);

        let mut operation = ApiPathOperation::new();

        let route_handler = route_handler.unwrap();
        self.add_request_details(operation.tags(options.tags), route_handler, file_path);
        self.path(&options.path.unwrap())
            .add_operation(&options.method.unwrap(), operation);
    }

    fn add_request_details(
        &mut self,
        operation: &mut ApiPathOperation,
        route_handler: &SchemyNode<'m>,
        file_path: &str,
    ) -> () {
        if let arrow_expression @ SchemyNode::ArrowExpr { node: _, parent: _ } = route_handler {
            for param in &arrow_expression.params() {
                self.add_request_params(operation, param, file_path);
            }

            self.symbol_tables.add_child_scope(file_path);
            self.find_response(operation, arrow_expression.body().unwrap(), file_path);
            self.symbol_tables.parent_scope(file_path);
        }
    }

    fn add_request_params(&mut self, operation: &mut ApiPathOperation, node: &SchemyNode<'m>, file_path: &str) {
        for node in node.children() {
            match node {
                SchemyNode::TsTypeRef {
                    node: type_ref,
                    parent: None,
                } => match type_ref.type_name {
                    TsEntityName::Ident(identifier) if identifier.sym.eq("BodyParam") => {
                        self.add_body_param_details(operation, type_ref, file_path);
                    }
                    TsEntityName::Ident(identifier) if identifier.sym.eq("Header") => {
                        self.add_param_details(operation, "header", &node, file_path);
                    }
                    TsEntityName::Ident(identifier) if identifier.sym.eq("QueryParam") => {
                        self.add_param_details(operation, "query", &node, file_path);
                    }
                    TsEntityName::Ident(identifier) if identifier.sym.eq("RouteParam") => {
                        self.add_param_details(operation, "path", &node, file_path);
                    }
                    // TODO support route handler params in separate module
                    TsEntityName::Ident(identifier) => {
                        self.add_param_from_referenced_type(operation, &identifier.sym, file_path);
                    }
                    _ => self.add_request_params(operation, &node, file_path),
                },
                _ => self.add_request_params(operation, &node, file_path),
            }
        }
    }

    fn add_param_details(
        &mut self,
        operation: &mut ApiPathOperation,
        location: &str,
        type_ref: &SchemyNode<'m>,
        file_path: &str,
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
                                self.define_referenced_schema(&reference, &reference, file_path, namespace.clone());
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
        &mut self,
        operation: &mut ApiPathOperation,
        type_ref: &TsTypeRef,
        file_path: &str,
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
                            let reference = self
                                .symbol_tables
                                .get_root_declaration_name(file_path, identifier.sym.to_string());
                            self.define_referenced_schema(&reference, &reference, file_path, namespace.clone());
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

    fn find_response(&mut self, operation: &mut ApiPathOperation, body: SchemyNode<'m>, file_path: &str) -> () {
        for child in body.children() {
            store_declaration_maybe(&child, file_path, &mut self.symbol_tables);

            match child {
                SchemyNode::Ident {
                    node: identifier,
                    parent: _,
                } if identifier.sym.eq("Response") => self.add_response(operation, child.parent().unwrap(), file_path),
                other => self.find_response(operation, other, file_path),
            }
        }
    }

    // TODO add schema for ref here
    fn add_response(&mut self, operation: &mut ApiPathOperation, node: &SchemyNode<'m>, file_path: &str) -> () {
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
                    Expr::TsAs(ts_as) => match *ts_as.type_ann {
                        TsType::TsTypeRef(type_ref) => match type_ref.type_name {
                            TsEntityName::Ident(identifier) => Some(
                                self.symbol_tables
                                    .get_root_declaration_name(file_path, identifier.sym.to_string()),
                            ),
                            _ => None,
                        },
                        _ => None,
                    },
                    Expr::TsTypeAssertion(type_assertion) => match *type_assertion.type_ann {
                        TsType::TsTypeRef(type_ref) => match type_ref.type_name {
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
                None => None,
            };

            if let Some(response_type) = &response_type {
                self.define_referenced_schema(&response_type, &response_type, file_path, namespace);
            }

            if let Some(response_options) = options {
                operation.response(&response_type, response_options);
            }
        }
    }

    fn define_referenced_schema(
        &mut self,
        schema_name: &str,
        type_reference: &str,
        file_path: &str,
        namespace: Option<String>,
    ) -> () {
        match self.symbol_tables.get_root_declaration(file_path, type_reference) {
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
            Some(Declaration::Type { node }) => {
                let schema = match namespace {
                    Some(ns) => self.components.schema(&ns).property(schema_name.into()),
                    None => self.components.schema(schema_name),
                };

                define_referenced_schema_details(schema, node);
            }
            _ => {}
        };
    }

    fn define_deferred_schemas(&mut self, node: &SchemyNode, source_file_name: &str) -> () {
        store_declaration_maybe(node, source_file_name, &mut self.symbol_tables);

        match node {
            SchemyNode::ExportDefaultExpr { node: _, parent: _ } => {
                self.define_deferred_type_maybe("default", source_file_name)
            }
            SchemyNode::ExportDecl {
                node: export_declaration,
                parent: _,
            } => match export_declaration.decl {
                Decl::Class(class_declaration) => {
                    let name = class_declaration.ident.sym.to_string();
                    self.define_deferred_type_maybe(&name, source_file_name);
                }
                Decl::TsInterface(interface_declaration) => {
                    let name = interface_declaration.id.sym.to_string();
                    self.define_deferred_type_maybe(&name, source_file_name);
                }
                Decl::TsTypeAlias(alias_declaration) => {
                    let name = alias_declaration.id.sym.to_string();
                    self.define_deferred_type_maybe(&name, source_file_name);
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

                            self.define_deferred_type_maybe(&name, source_file_name);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    fn define_deferred_type_maybe(&mut self, type_name: &str, source_file_name: &str) -> () {
        if let Some(deferred_type) = self.deferred_schemas.get_deferred_type(type_name, source_file_name) {
            match self.symbol_tables.get_root_declaration(source_file_name, &type_name) {
                Some(Declaration::Type { node }) => {
                    let schema = match &deferred_type.namespace {
                        Some(ns) => self
                            .components
                            .schema(&ns)
                            .data_type("object")
                            .property(&deferred_type.schema_name),
                        None => self.components.schema(&deferred_type.schema_name),
                    };

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

    fn add_param_from_referenced_type(
        &mut self,
        operation: &mut ApiPathOperation,
        type_ref: &str,
        file_path: &str,
    ) -> () {
        match self.symbol_tables.get_root_declaration(file_path, type_ref) {
            // Some(Declaration::Import { name, source_file_name }) => find_referenced_type(file_path, name, |declaration: dprint_swc_ext::swc::ast::Decl| {
            //     match declaration {
            //         dprint_swc_ext::swc::ast::Decl::TsInterface(interface_declaration) => {
            //             let name = interface_declaration.id.sym().to_string();
            //             self.add_param_from_referenced_type(operation, &name, source_file_name);
            //         }
            //         dprint_swc_ext::swc::ast::Decl::TsTypeAlias(alias_declaration) => {
            //             let name = alias_declaration.id.sym().to_string();
            //             self.add_param_from_referenced_type(operation, &name, source_file_name);
            //         }
            //         _ => {}
            //     }
            // }),
            Some(Declaration::Type { node }) => self.add_request_params(operation, &node, file_path),
            _ => {}
        }
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
            root_schema.data_type = Some("object".into());
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
            root_schema.data_type = Some("array".into());
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
            root_schema.data_type = Some("object".into());

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
            root_schema.data_type = Some("object".into());
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

fn find_referenced_type() {}
