use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use deno_ast::{ParseParams, ParsedSource, SourceTextInfo};
use dprint_swc_ext::view::*;
use lazy_static::__Deref;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use url::Url;

use crate::typescript::{Declaration, DeclarationTables, KEY_OF_KEYWORD};

use super::declaration_helpers::store_declaration_maybe;

#[derive(Serialize, Debug)]
pub struct OpenApi<'n> {
    pub components: ApiComponents,
    #[serde(skip)]
    deferred_schemas: Vec<(String, String, String, Option<String>)>,
    paths: HashMap<String, ApiPath>,
    #[serde(skip)]
    symbol_tables: DeclarationTables<'n>,
}
impl<'n> OpenApi<'n> {
    pub(crate) fn new() -> Self {
        OpenApi {
            components: ApiComponents::new(),
            deferred_schemas: Vec::new(),
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

        with_ast_view_for_module(
            ModuleInfo {
                module: result.module(),
                comments: None,
                text_info: Some(result.text_info()),
                tokens: None,
            },
            |module| self.find_paths(&module.as_node(), file_path),
        );

        let deferred_schemas: Vec<(String, String, String, Option<String>)> = self.deferred_schemas.drain(..).collect();
        for deferred in deferred_schemas {
            self.add_reference_schema(&deferred.0, &deferred.1, &deferred.2, deferred.3)
        }
    }

    fn find_paths(&mut self, node: &Node<'n>, file_path: &str) {
        store_declaration_maybe(node, file_path, &mut self.symbol_tables);

        for child in node.children() {
            match child.kind() {
                NodeKind::CallExpr => match child.to::<CallExpr>() {
                    Some(call_expr) => match call_expr.callee {
                        Callee::Expr(Expr::Ident(ident)) if ident.sym().eq("Path") => {
                            self.symbol_tables.add_child_scope(file_path);
                            self.add_path(&call_expr, file_path);
                            self.symbol_tables.parent_scope(file_path);
                        }
                        _ => self.find_paths(&child, file_path),
                    },
                    None => self.find_paths(&child, file_path),
                },
                _ => self.find_paths(&child, file_path),
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

    fn add_path(&mut self, node: &CallExpr<'n>, file_path: &str) -> () {
        let args = &node.args;
        let route_handler = args.first().copied();
        let route_options = args.last().copied();
        let options = get_path_options(route_options);

        let mut operation = ApiPathOperation::new();

        let route_handler = route_handler.unwrap();
        self.add_request_details(operation.tags(options.tags), Node::from(route_handler), file_path);
        self.path(&options.path.unwrap())
            .add_operation(&options.method.unwrap(), operation);
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

                    self.symbol_tables.add_child_scope(file_path);
                    self.find_response(operation, Node::from(arrow_expression.body), file_path);
                    self.symbol_tables.parent_scope(file_path);
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
                        self.add_param_details(operation, "header", type_ref, file_path);
                    }
                    TsEntityName::Ident(identifier) if identifier.sym().eq("QueryParam") => {
                        self.add_param_details(operation, "query", type_ref, file_path);
                    }
                    TsEntityName::Ident(identifier) if identifier.sym().eq("RouteParam") => {
                        self.add_param_details(operation, "path", type_ref, file_path);
                    }
                    _ => self.add_request_params(operation, type_ref.as_node(), file_path),
                },
                _ => self.add_request_params(operation, child, file_path),
            }
        }
    }

    fn add_param_details(
        &mut self,
        operation: &mut ApiPathOperation,
        location: &str,
        type_ref: &TsTypeRef,
        file_path: &str,
    ) {
        let parameter_name = get_parameter_name(Node::from(type_ref));
        let operation_param = operation.param(&parameter_name, location);
        if let Some(type_params) = type_ref.type_params {
            let namespace = match type_params.params.get(2) {
                Some(TsType::TsLitType(namespace)) => match &namespace.lit {
                    TsLit::Str(literal_string) => Some(literal_string.value().to_string()),
                    _ => None,
                },
                _ => None,
            };

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
                        let reference = identifier.sym().to_string();
                        self.add_reference_schema(&reference, &reference, file_path, namespace);
                        operation_param.content().schema().reference(reference.into(), false);
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
                Some(TsType::TsLitType(namespace)) => match &namespace.lit {
                    TsLit::Str(literal_string) => Some(literal_string.value().to_string()),
                    _ => None,
                },
                _ => None,
            };

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
                            .get_root_declaration_name(file_path, identifier.sym().to_string());
                        self.add_reference_schema(&reference, &reference, file_path, namespace);
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
        }
    }

    fn find_response(&mut self, operation: &mut ApiPathOperation, body: Node<'n>, file_path: &str) -> () {
        for child in body.children() {
            store_declaration_maybe(&child, file_path, &mut self.symbol_tables);

            match child {
                Node::Ident(identifier) if identifier.sym().eq("Response") => {
                    self.add_response(operation, identifier.parent(), file_path)
                }
                other => self.find_response(operation, other, file_path),
            }
        }
    }

    // TODO add schema for ref here
    fn add_response(&mut self, operation: &mut ApiPathOperation, node: Node, file_path: &str) -> () {
        if let Some(call_expression) = node.to::<CallExpr>() {
            let options = match call_expression.args.get(1) {
                Some(arg) => match arg.expr {
                    Expr::Object(options) => Some(get_response_options(options)),
                    _ => None,
                },
                None => None,
            };

            let namespace = match &options {
                Some(options) => options.namespace.clone(),
                None => None,
            };

            let response_type = match call_expression.args.get(0) {
                Some(arg) => match arg.expr {
                    Expr::New(new_expression) => match new_expression.callee {
                        Expr::Ident(identifier) => {
                            let type_reference = identifier.sym().to_string();
                            self.add_reference_schema(&type_reference, &type_reference, file_path, namespace);
                            Some(self.symbol_tables.get_root_declaration_name(file_path, type_reference))
                        }
                        _ => None,
                    },
                    Expr::Ident(response_type) => Some(
                        self.symbol_tables
                            .get_root_declaration_name(file_path, response_type.sym().to_string()),
                    ),
                    Expr::TsAs(ts_as) => match ts_as.type_ann {
                        TsType::TsTypeRef(type_ref) => match type_ref.type_name {
                            TsEntityName::Ident(identifier) => Some(
                                self.symbol_tables
                                    .get_root_declaration_name(file_path, identifier.sym().to_string()),
                            ),
                            _ => None,
                        },
                        _ => None,
                    },
                    Expr::TsTypeAssertion(type_assertion) => match type_assertion.type_ann {
                        TsType::TsTypeRef(type_ref) => match type_ref.type_name {
                            TsEntityName::Ident(identifier) => Some(
                                self.symbol_tables
                                    .get_root_declaration_name(file_path, identifier.sym().to_string()),
                            ),
                            _ => None,
                        },
                        _ => None,
                    },
                    _ => None,
                },
                None => None,
            };

            if let Some(response_options) = options {
                operation.response(&response_type, response_options);
            }
        }
    }

    fn add_reference_schema(
        &mut self,
        schema_name: &str,
        type_reference: &str,
        file_path: &str,
        namespace: Option<String>,
    ) -> () {
        if !self.symbol_tables.has_table(&file_path) {
            self.load_symbols_from_module(&file_path);
        }

        match self.symbol_tables.get_root_declaration(file_path, type_reference) {
            Some(Declaration::Export {
                name: type_name,
                source_file_name,
            }) => {
                self.deferred_schemas
                    .push((schema_name.into(), type_name, source_file_name, namespace));
            }
            Some(Declaration::Import {
                name: type_name,
                source_file_name,
            }) => {
                self.deferred_schemas
                    .push((schema_name.into(), type_name, source_file_name, namespace));
            }
            Some(Declaration::Type { node }) => match node {
                Node::ClassDecl(declaration) => println!("{:?}", declaration.inner),
                Node::TsArrayType(declaration) => println!("{:?}", declaration.inner),
                Node::TsEnumDecl(declaration) => println!("{:?}", declaration.inner),
                Node::TsInterfaceDecl(ts_interface_declaration) => {
                    println!("I found a node for {} in namespace {:?}", type_reference, namespace);
                    let schema = match namespace {
                        Some(ns) => self.components.schema(&ns).property(schema_name),
                        None => self.components.schema(schema_name)
                    };

                    schema._type = Some("object".into());

                    for property in &ts_interface_declaration.body.body {
                        match property {
                            TsTypeElement::TsPropertySignature(signature) => {
                                let property = match signature.key {
                                    Expr::Ident(identifier) => Some(schema.property(&identifier.sym().to_string())),
                                    _ => None
                                };

                                if let Some(property) = property {
                                    match signature.type_ann {
                                        Some(annotation) => match annotation.type_ann {
                                            // TODO handle enum types
                                            TsType::TsKeywordType(keyword_type) => match keyword_type.inner.kind {
                                                TsKeywordTypeKind::TsNumberKeyword => {
                                                    property._type = Some("number".into());
                                                },
                                                TsKeywordTypeKind::TsBooleanKeyword => {
                                                    property._type = Some("boolean".into());
                                                },
                                                TsKeywordTypeKind::TsBigIntKeyword => {
                                                    property._type = Some("number".into());
                                                },
                                                TsKeywordTypeKind::TsStringKeyword => {
                                                    property._type = Some("string".into());
                                                },
                                                _ => {}
                                            },
                                            _ => {}
                                        },
                                        None => {}
                                    }
                                }
                            },
                            _ => {}
                        }
                    }
                }
                Node::TsTypeAliasDecl(declaration) => println!("{:?}", declaration.inner),
                _ => {}
            },
            _ => {}
        };
    }

    fn load_symbols_from_module(&mut self, source_file_name: &str) -> () {
        let result = get_syntax_tree(source_file_name);

        with_ast_view_for_module(
            ModuleInfo {
                module: result.module(),
                comments: None,
                text_info: Some(result.text_info()),
                tokens: None,
            },
            |module| self.load_symbols(&module.as_node(), source_file_name),
        );
    }

    fn load_symbols(&mut self, node: &Node<'n>, source_file_name: &str) -> () {
        for child in node.children() {
            store_declaration_maybe(node, source_file_name, &mut self.symbol_tables);
            self.load_symbols(&child, source_file_name);
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
    _type: Option<String>,
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
        if let Some(ref _type) = self._type {
            state.serialize_field("type", _type)?;
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
            _type: None,
            reference: None,
            namespace: None,
            is_example: false,
            items: None,
            properties: None,
            required: HashSet::new(),
        }
    }

    pub fn format(&mut self, format: Option<String>) -> &mut ApiSchema {
        // TODO add format tests
        self.format = format.clone();
        self
    }

    pub fn namespace(&mut self, namespace: Option<String>) -> &mut ApiSchema {
        self.namespace = namespace.clone();
        self
    }

    pub fn primitive(&mut self, type_name: &str) -> &mut ApiSchema {
        self._type = Some(type_name.to_string());
        self
    }

    pub fn reference(&mut self, reference: Option<String>, is_example: bool) -> &mut ApiSchema {
        self.is_example = is_example;
        self.reference = reference;
        self
    }

    pub fn object(&mut self) -> &mut ApiSchema {
        self._type = Some(String::from("object"));
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
        self._type = Some(String::from("array"));
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
