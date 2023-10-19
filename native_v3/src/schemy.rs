#[derive(Debug, Default)]
pub struct SchemyPath {
    pub module_id: usize,
    pub method: Option<String>,
    pub path: Option<String>,
    pub tags: Vec<String>,
}

impl SchemyPath {
    pub fn new(module_id: usize) -> Self {
        Self { module_id, method: None, path: None, tags: vec![] }
    }
}

#[derive(Debug)]
pub enum Method {}