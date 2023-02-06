use ahash::{HashMap, HashMapExt};
use serde::Serialize;

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

    pub(crate) fn path(&mut self, key: String) -> Option<&mut ApiPath> {
        let path = ApiPath::new();
        self.paths.insert(key.clone(), path);
        self.paths.get_mut(&key)
    }
}

#[derive(Serialize, Debug)]
pub struct ApiComponents {}

#[derive(Serialize, Debug)]
pub struct ApiPath {}
impl ApiPath {
    fn new() -> ApiPath {
        ApiPath {}
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
