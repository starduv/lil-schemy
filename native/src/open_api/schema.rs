use ahash::{HashMap, HashSet, HashSetExt, HashMapExt};
use serde::{Serialize, Serializer, ser::SerializeStruct};

#[derive(Serialize, Debug)]
pub struct OpenApi {
    pub components: ApiComponents,
    pub paths: HashMap<String, ApiPath>,
}

impl OpenApi {
    pub fn new() -> Self {
        OpenApi {
            components: ApiComponents::new(),
            paths: HashMap::new(),
        }
    }

    pub fn path(&mut self, key: &str) -> &mut ApiPath {
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

    pub fn add_operation(&mut self, method: &str, operation: ApiPathOperation) -> &mut ApiPathOperation {
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