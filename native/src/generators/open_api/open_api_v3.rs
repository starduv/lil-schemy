use ahash::{HashMap, HashMapExt};
use serde::Serialize;

use crate::typescript::ResponseOptions;

#[derive(Serialize, Debug)]
pub struct OpenApiV3 {
    components: ApiComponents,
    paths: HashMap<String, ApiPath>,
}
impl OpenApiV3 {
    pub(crate) fn new() -> Self {
        OpenApiV3 {
            components: ApiComponents {},
            paths: HashMap::new(),
        }
    }

    pub(crate) fn path(&mut self, key: String) -> &mut ApiPath {
        let path = ApiPath::new();
        self.paths.insert(key.clone(), path);
        self.paths.get_mut(&key).expect("Could access ApiPath")
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

impl ApiPath {
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

    pub(crate) fn method(&mut self, method: Option<String>) -> &mut ApiPathOperation {
        match method {
            Some(m) => match m.to_lowercase().as_str() {
                "get" => self.get.get_or_insert(ApiPathOperation::new()),
                "put" => self.put.get_or_insert(ApiPathOperation::new()),
                "post" => self.post.get_or_insert(ApiPathOperation::new()),
                "delete" => self.delete.get_or_insert(ApiPathOperation::new()),
                "options" => self.options.get_or_insert(ApiPathOperation::new()),
                "head" => self.head.get_or_insert(ApiPathOperation::new()),
                "patch" => self.patch.get_or_insert(ApiPathOperation::new()),
                "trace" => self.trace.get_or_insert(ApiPathOperation::new()),
                other => panic!("Unsupported http method '{}'", other),
            },
            None => panic!("Property 'method' of PathOptions is required "),
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

    pub(crate) fn response(&mut self, response_args: ResponseOptions) -> &mut ApiResponse {
        let status_code = response_args
            .status_code
            .expect("An ApiResponse must have a status code");

        let description = response_args
            .description
            .expect("An ApiResponse must have a description");

        let mut response = ApiResponse::new(description);

        response.content().schema().example(response_args.example);
        // .examples(response_args.examples)
        // .namespace(response_args.namespace);

        self.responses.insert(status_code.clone(), response);

        self.responses
            .get_mut(&status_code)
            .expect("Could not get recently set ApiResponse")
    }

    pub(crate) fn param(&mut self, name: String, location: &str) -> &mut ApiParam {
        let param = ApiParam::new(name.clone(), location.to_string());
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
    #[serde(skip)]
    namespace: Option<String>,
}

impl ApiResponse {
    fn new(description: String) -> Self {
        ApiResponse {
            content: None,
            description,
            examples: None,
            headers: None,
            links: None,
            namespace: None,
        }
    }

    pub(crate) fn content(&mut self) -> &mut ApiConent {
        let api_content = ApiConent::new();
        let key = "application/json";
        self.content
            .get_or_insert_with(Default::default)
            .insert(key.to_owned(), api_content);

        self.content.get_or_insert_with(Default::default).get_mut(key).unwrap()
    }
}

#[derive(Serialize, Debug)]
pub struct ApiConent {
    schema: ApiSchema,
}
impl ApiConent {
    pub fn new() -> Self {
        ApiConent {
            schema: ApiSchema::new(),
        }
    }

    pub fn schema(&mut self) -> &mut ApiSchema {
        &mut self.schema
    }
}

#[derive(Serialize, Debug)]
pub struct ApiSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    example: Option<Box<ApiSchema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    primitive: Option<String>,
    #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    reference: Option<String>,
}

impl ApiSchema {
    pub fn new() -> Self {
        ApiSchema {
            example: None,
            format: None,
            primitive: None,
            reference: None,
        }
    }

    pub fn reference(&mut self, reference: String) -> &mut ApiSchema {
        self.reference = Some(reference);
        self
    }

    pub fn primitive(&mut self, type_name: String) -> &mut ApiSchema {
        self.primitive = Some(type_name);
        self
    }

    pub fn format(&mut self, format: Option<String>) -> &mut ApiSchema {
        self.format = format;
        self
    }

    pub fn example(&mut self, example: Option<String>) -> &mut ApiSchema {
        if let Some(example) = example {
            let mut schema = ApiSchema::new();
            schema.reference(example);
            self.example = Some(Box::new(schema));
        }
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
    fn new(name: String, location: String) -> ApiParam {
        ApiParam {
            content: None,
            location,
            name,
            required: None,
        }
    }

    pub(crate) fn content(&mut self) -> &mut ApiConent {
        let api_content = ApiConent::new();
        let key = "application/json";
        self.content
            .get_or_insert(HashMap::new())
            .insert(key.to_owned(), api_content);

        self.content.get_or_insert(HashMap::new()).get_mut(key).unwrap()
    }

    pub(crate) fn required(&mut self, required: Option<bool>) -> &mut ApiParam {
        self.required = required;
        self
    }
}
