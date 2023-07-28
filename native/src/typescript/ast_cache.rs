use ahash::{HashMap, HashMapExt};
use deno_ast::{swc::ast::Module, ParseParams, ParsedSource, SourceTextInfo};
use url::Url;

#[derive(Debug)]
pub struct AstCache {
    cache: HashMap<String, ParsedSource>,
}

impl AstCache {
    pub fn new() -> Self {
        Self { cache: HashMap::new() }
    }

    pub fn get_ast(&mut self, path: &str) -> &Module {
        let parsed_source = self.cache.entry(path.to_string()).or_insert_with(|| {
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
        });

        parsed_source.module()
    }
}
