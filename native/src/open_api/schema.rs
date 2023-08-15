use std::{cell::RefCell, rc::Rc};

use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use serde::{ser::SerializeStruct, Serialize, Serializer};

#[derive(Serialize, Debug)]
pub struct OpenApi {
    #[serde(rename = "openapi")]
    pub open_api: String,
    pub components: ApiComponents,
    pub paths: HashMap<String, ApiPath>,
}

impl OpenApi {
    pub fn new() -> Self {
        OpenApi {
            open_api: "3.0.1".to_string(),
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
    get: Option<Rc<RefCell<ApiPathOperation>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    put: Option<Rc<RefCell<ApiPathOperation>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post: Option<Rc<RefCell<ApiPathOperation>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<Rc<RefCell<ApiPathOperation>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<Rc<RefCell<ApiPathOperation>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    head: Option<Rc<RefCell<ApiPathOperation>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    patch: Option<Rc<RefCell<ApiPathOperation>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trace: Option<Rc<RefCell<ApiPathOperation>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    servers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<Rc<RefCell<ApiPathParameter>>>,
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

    pub fn add_operation(&mut self, method: &str) -> &Rc<RefCell<ApiPathOperation>> {
        match method.to_lowercase().as_str() {
            "get" => self.get.insert(Rc::new(RefCell::new(ApiPathOperation::new()))),
            "put" => self.put.insert(Rc::new(RefCell::new(ApiPathOperation::new()))),
            "post" => self.post.insert(Rc::new(RefCell::new(ApiPathOperation::new()))),
            "delete" => self.delete.insert(Rc::new(RefCell::new(ApiPathOperation::new()))),
            "options" => self.options.insert(Rc::new(RefCell::new(ApiPathOperation::new()))),
            "head" => self.head.insert(Rc::new(RefCell::new(ApiPathOperation::new()))),
            "patch" => self.patch.insert(Rc::new(RefCell::new(ApiPathOperation::new()))),
            "trace" => self.trace.insert(Rc::new(RefCell::new(ApiPathOperation::new()))),
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

    pub(crate) fn response(&mut self, status_code: &str, description: &str) -> &mut ApiResponse {
        let response = ApiResponse::new(description);
        self.responses.entry(status_code.into()).or_insert(response)
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
    fn new(description: &str) -> Self {
        ApiResponse {
            content: None,
            description: description.to_string(),
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

    pub fn example(&mut self, example: Option<String>) -> &mut ApiConent {
        if example.is_some() {
            let mut schema = ApiSchema::new();
            schema.reference(example, true);
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
    is_example: bool,
    properties: Option<HashMap<String, ApiSchema>>,
    required: HashSet<String>,
    enums: Option<Vec<String>>,
    additional_properties: Option<AllOf>,
}

impl Serialize for ApiSchema {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ApiSchema", 5)?;
        if let Some(ref additional) = self.additional_properties {
            state.serialize_field("additionalProperties", additional)?;
        }
        if let Some(ref enums) = self.enums {
            state.serialize_field("enum", enums)?;
        }
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

            path.push_str(reference);
            state.serialize_field("$ref", &path)?;
        }
        state.end()
    }
}

impl ApiSchema {
    pub fn new() -> Self {
        ApiSchema {
            additional_properties: None,
            format: None,
            data_type: None,
            reference: None,
            is_example: false,
            items: None,
            properties: None,
            enums: None,
            required: HashSet::new(),
        }
    }

    pub fn additional_properties(&mut self) -> &mut ApiSchema {
        let properties = self.additional_properties.get_or_insert(AllOf { all_of: Vec::new() });

        properties.all_of.push(ApiSchema::new());

        properties.all_of.last_mut().unwrap()
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

    pub fn reference(&mut self, reference: Option<String>, is_example: bool) -> &mut ApiSchema {
        self.is_example = is_example;
        self.reference = reference.clone();
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

    pub(crate) fn enum_value(&mut self, value: &str) {
        self.enums.get_or_insert(Vec::new()).push(value.to_string());
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
    required: bool,
}

impl ApiParam {
    fn new(name: Option<&str>, location: Option<&str>) -> ApiParam {
        ApiParam {
            content: None,
            location: location.map(|l| l.to_string()),
            name: name.map(|n| n.to_string()),
            required: false,
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
        self.required = required;
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
    pub status_code: Option<String>,
}
impl ResponseOptions {
    pub(crate) fn new() -> Self {
        ResponseOptions {
            description: None,
            example: None,
            status_code: None,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct AllOf {
    #[serde(rename = "allOf")]
    all_of: Vec<ApiSchema>,
}
