use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct OpenApiV3 {
    pub components: ApiComponents,
    pub paths: ApiPaths
}
impl OpenApiV3 {
    pub(crate) fn new() -> Self {
        OpenApiV3 { components: ApiComponents {  }, paths: ApiPaths {  } }
    }
}

#[derive(Serialize, Debug)]
pub struct ApiComponents {}

#[derive(Serialize, Debug)]
pub struct ApiPaths {}