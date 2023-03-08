use ahash::{HashMap, HashMapExt};
use serde::{ser::SerializeStruct, Serialize, Serializer};

#[derive(Serialize, Debug)]
pub struct OpenApi {
    pub components: ApiComponents,
    paths: HashMap<String, ApiPath>,
}
impl<'v> OpenApi {
    pub(crate) fn new() -> Self {
        OpenApi {
            components: ApiComponents::new(),
            paths: HashMap::new(),
        }
    }

    pub(crate) fn path(&mut self, key: &str) -> &mut ApiPath {
        self.paths.entry(key.to_string()).or_insert(ApiPath::new())
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
    #[serde(rename = "requestBody", skip_serializing_if = "Option::is_none")]
    body_parameter: Option<BodyParam>,
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

    pub(crate) fn response(&mut self, name: &Option<String>, response_args: ResponseOptions) -> &mut ApiResponse {
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

        if name.is_some() {
            content
                .schema()
                .reference(name.to_owned(), false)
                .namespace(response_args.namespace);
        }

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

    pub(crate) fn body(&mut self) -> &mut BodyParam {
        self.body_parameter.get_or_insert(BodyParam::new())
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

#[derive(Debug)]
pub struct ApiSchema {
    items: Option<Box<ApiSchema>>,
    format: Option<String>,
    _type: Option<String>,
    reference: Option<String>,
    namespace: Option<String>,
    is_example: bool,
    properties: Option<HashMap<String, ApiSchema>>,
    required: Vec<String>,
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
            required: Vec::new(),
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
        self.required.push(name_text.to_string());

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
#[derive(Serialize, Debug)]
pub struct BodyParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<HashMap<String, ApiConent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<bool>,
}

impl BodyParam {
    fn new() -> BodyParam {
        BodyParam {
            content: None,
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

    pub(crate) fn required(&mut self, required: bool) -> &mut BodyParam {
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
