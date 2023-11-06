use std::{collections::BTreeMap, path::Path, rc::Rc};
use swc_common::{
    errors::{ColorConfig, Handler},
    sync::Lrc,
    SourceMap,
};

use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};

use super::SchemyNode;

pub struct ModuleCache {
    cm: Lrc<SourceMap>,
    cache: BTreeMap<String, Rc<SchemyNode<'static>>>,
}

impl<'m> ModuleCache {
    pub fn new() -> Self {
        Self {
            cm: Default::default(),
            cache: BTreeMap::new(),
        }
    }

    pub fn parse(&mut self, path: &str) -> Rc<SchemyNode<'static>> {
        self.cache.entry(path.to_string()).or_insert_with(|| {
            let fm = self
                .cm
                .load_file(Path::new(path))
                .expect(format!("Could not load file '{}'", path).as_str());
            let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(self.cm.clone()));

            let lexer = Lexer::new(
                Syntax::Typescript(Default::default()),
                Default::default(),
                StringInput::from(&*fm),
                None,
            );

            let mut parser = Parser::new_from(lexer);

            for e in parser.take_errors() {
                e.into_diagnostic(&handler).emit();
            }

            let module = parser
                .parse_module()
                .map_err(|e| {
                    // Unrecoverable fatal error occurred
                    e.into_diagnostic(&handler).emit()
                })
                .expect(format!("Could not parse module '{}'", path).as_str());

            SchemyNode::from_module(module)
        });

        self.cache.get(path).unwrap().clone()
    }
}
