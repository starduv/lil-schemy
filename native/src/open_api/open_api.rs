use ahash::{HashMap, HashMapExt};
use serde::{
    ser::{Error, SerializeStruct},
    Serialize, Serializer,
};

use crate::typescript::{AstNode, Declaration};

#[derive(Serialize, Debug)]
pub struct OpenApi {
    components: ApiComponents,
    paths: HashMap<String, ApiPath>,
}
impl<'v> OpenApi {
    pub(crate) fn new() -> Self {
        OpenApi {
            components: ApiComponents {},
            paths: HashMap::new(),
        }
    }

    pub(crate) fn path(&mut self, key: &str) -> &mut ApiPath {
        self.paths.entry(key.to_string()).or_insert(ApiPath::new())
        // let path = ApiPath::new();
        // self.paths.insert(key.to_string(), path);
        // self.paths.get_mut(key).expect("Could access ApiPath")
    }
}

#[derive(Serialize, Debug)]
pub struct ApiComponents {}

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

    pub(crate) fn method(&mut self, method: &str) -> &mut ApiPathOperation {
        match method.to_lowercase().as_str() {
            "get" => self.get.get_or_insert(ApiPathOperation::new()),
            "put" => self.put.get_or_insert(ApiPathOperation::new()),
            "post" => self.post.get_or_insert(ApiPathOperation::new()),
            "delete" => self.delete.get_or_insert(ApiPathOperation::new()),
            "options" => self.options.get_or_insert(ApiPathOperation::new()),
            "head" => self.head.get_or_insert(ApiPathOperation::new()),
            "patch" => self.patch.get_or_insert(ApiPathOperation::new()),
            "trace" => self.trace.get_or_insert(ApiPathOperation::new()),
            other => panic!("Unsupported http method '{}'", other),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ApiPathOperation {
    #[serde(skip_serializing_if = "Option::is_none")]
    examples: Option<HashMap<String, ApiSchema>>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    responses: HashMap<String, ApiResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<Vec<ApiParam>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<String>>,
}

impl ApiPathOperation {
    pub fn new() -> Self {
        ApiPathOperation {
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

    pub(crate) fn response(&mut self, name: &str, response_args: ResponseOptions) -> &mut ApiResponse {
        let status_code = response_args
            .status_code
            .expect("An ApiResponse must have a status code");

        let description = response_args
            .description
            .expect("An ApiResponse must have a description");

        let mut response = ApiResponse::new(description);

        response
            .content()
            .example(response_args.example, response_args.namespace.clone())
            .schema()
            .reference(name.to_string(), false)
            .namespace(response_args.namespace);

        self.responses.entry(status_code).or_insert(response)
    }

    pub(crate) fn param(&mut self, name: &str, location: &str) -> &mut ApiParam {
        let param = ApiParam::new(name, location.to_string());
        self.parameters.get_or_insert_with(Default::default).push(param);
        self.parameters
            .get_or_insert_with(Default::default)
            .last_mut()
            .expect("Could not get parameter from ApiOperation")
    }
}

#[derive(Serialize, Debug)]
pub struct ApiPathParameter {}

#[derive(Serialize, Debug)]
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

#[derive(Serialize, Debug)]
pub struct ApiConent {
    schema: ApiSchema,
    #[serde(skip_serializing_if = "Option::is_none")]
    example: Option<Box<ApiSchema>>,
}
impl ApiConent {
    pub fn new() -> Self {
        ApiConent {
            schema: ApiSchema::new(),
            example: None,
        }
    }

    pub fn schema(&mut self) -> &mut ApiSchema {
        &mut self.schema
    }

    pub fn example(&mut self, example: Option<String>, namespace: Option<String>) -> &mut ApiConent {
        if let Some(example) = example {
            let mut schema = ApiSchema::new();
            schema.reference(example, true).namespace(namespace);
            self.example = Some(Box::new(schema));
        }
        self
    }
}

#[derive(Debug)]
pub struct ApiSchema {
    format: Option<String>,
    primitive: Option<String>,
    reference: Option<String>,
    namespace: Option<String>,
    is_example: bool,
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
        if let Some(ref primitive) = self.primitive {
            state.serialize_field("type", primitive)?;
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
            primitive: None,
            reference: None,
            namespace: None,
            is_example: false,
        }
    }

    pub fn format(&mut self, format: String) -> &mut ApiSchema {
        // TODO add format tests
        self.format = Some(format);
        self
    }

    pub fn namespace(&mut self, namespace: Option<String>) -> &mut ApiSchema {
        if let Some(namespace) = namespace {
            self.namespace = Some(namespace);
        }
        self
    }

    pub fn primitive(&mut self, type_name: &str) -> &mut ApiSchema {
        self.primitive = Some(type_name.to_string());
        self
    }

    pub fn reference(&mut self, reference: String, is_example: bool) -> &mut ApiSchema {
        self.is_example = is_example;
        self.reference = Some(reference);
        self
    }
}

#[derive(Serialize, Debug)]
pub struct ApiParam {
    name: String,
    #[serde(rename = "in")]
    location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<HashMap<String, ApiConent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<bool>,
}

impl ApiParam {
    fn new(name: &str, location: String) -> ApiParam {
        ApiParam {
            content: None,
            location,
            name: name.to_string(),
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

pub struct PathArgs {
    pub method: Option<String>,
    pub path: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl PathArgs {
    pub(crate) fn new() -> Self {
        PathArgs {
            method: None,
            path: None,
            tags: None,
        }
    }
}

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
