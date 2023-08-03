use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use deno_ast::{ParseParams, SourceTextInfo, ParsedSource};
use url::Url;

use super::{Context, SchemyNode};

pub struct ModuleCache<'m> {
    cache: RefCell<BTreeMap<String, ParsedSource>>,
    context: Rc<RefCell<Context<'m>>>,
}

impl<'m> ModuleCache<'m> {
    pub fn new() -> Self {
        Self {
            cache: RefCell::new(BTreeMap::new()),
            context: Rc::new(RefCell::new(Context::default())),
        }
    }

    pub fn get_syntax_tree(&self, path: &str) -> &ParsedSource {
        let mut borrow = self.cache.borrow_mut();
        borrow.entry(path.to_string()).or_insert_with(|| {
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
        borrow.get(path).unwrap()
    }
}
