use swc_common::{
    errors::{ColorConfig, Handler},
    sync::Lrc,
    SourceMap,
};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};

use std::{collections::BTreeMap, path::Path, sync::Arc, cell::RefCell, vec};

use super::{node_kind::NodeKind, Node};

#[derive(Default)]
pub struct Application<'m> {
    source_map: Lrc<SourceMap>,
    modules: RefCell<BTreeMap<String, usize>>,
    context: RefCell<Vec<Arc<Node<'m>>>>,
}

impl<'m> Application<'m> {
    pub(crate) fn get_module(&self, path: &str) -> Arc<Node<'m>> {
        let context = &mut self.context.borrow_mut();
        let modules = &mut self.modules.borrow_mut();
        if !modules.contains_key(path) {
            let fm = self
                .source_map
                .load_file(Path::new(path))
                .expect(format!("Could not load file '{}'", path).as_str());

            let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(self.source_map.clone()));

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

            let node_index = context.len();
            context.push(Arc::new(Node::new(node_index, NodeKind::Module(module))));
            modules.insert(path.to_string(), node_index);
        }

        let index = modules.get(path).unwrap();
        return context[*index].clone();
    }

    pub(crate) fn get_children(&self, node_id: usize) -> Vec<Arc<Node<'m>>> {
        // TODO: implement this
        vec![]
    }
}
