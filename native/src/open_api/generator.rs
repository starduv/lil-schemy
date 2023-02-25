use ahash::HashMap;

use crate::typescript::{AstNode, SourceFile};

use super::open_api::OpenApi;

pub struct OpenApiGenerator<'m> {
    open_api: OpenApi,
    ast_map: &'m HashMap<String, SourceFile>,
}
impl<'m> OpenApiGenerator<'m> {
    pub fn new(ast_map: &'m HashMap<String, SourceFile>) -> Self {
        OpenApiGenerator {
            ast_map,
            open_api: OpenApi::new(),
        }
    }

    pub(crate) fn result(&self) -> &super::open_api::OpenApi {
        &self.open_api
    }

    pub(crate) fn api_paths_from(&self, path: String) -> () {
        let source_file = self
            .ast_map
            .get(&path)
            .expect(&format!("Could not find ast for path '{}'", path));

        source_file.do_recursively(|node| self.find_api_paths(node));
    }

    fn find_api_paths(&self, node: &AstNode) -> () {
        if self.is_api_path(node) {
            // TODO do something here
        }
    }

    fn is_api_path(&self, node: &AstNode) -> bool {
        match node.expression {
            Some(ref expression) => match expression.escaped_text {
                Some(ref text) => text.eq("Path"),
                None => false,
            },
            _ => false,
        }
    }
}
