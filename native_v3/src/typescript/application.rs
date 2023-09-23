use crossbeam::channel::Sender;
use swc_common::{
    errors::{ColorConfig, Handler},
    sync::Lrc,
    SourceMap,
};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};

use std::{
    collections::{BTreeMap, HashMap, HashSet},
    path::Path,
    sync::Arc,
};

use crate::messaging::Message;

use super::{node_kind::NodeKind, Node};

#[derive(Default)]
pub struct Application<'m> {
    context: HashMap<u8, Arc<Node<'m>>>,
    modules: BTreeMap<String, u8>,
    source_map: Lrc<SourceMap>,
    senders: HashSet<u8>,
}

impl<'m> Application<'m> {
    pub(crate) fn get_module(&mut self, path: &str, message: Sender<Message>) -> Arc<Node<'m>> {
        if !self.modules.contains_key(path) {
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

            let node = Arc::new(Node::new(NodeKind::Module(module), None, message));
            self.modules.insert(path.to_string(), node.id());
            self.context.insert(node.id(), node);
        }

        let id = self.modules.get(path).unwrap();
        self.context.get(id).unwrap().clone()
    }

    pub(crate) fn register_node(&mut self, node: Arc<Node<'m>>) -> () {
        self.context.insert(node.id(), node.clone());
    }

    pub(crate) fn register_sender(&mut self, id: u8) -> () {
        self.senders.insert(id);
    }

    pub(crate) fn unregister_sender(&mut self, id: &u8) -> () {
        self.senders.remove(id);
    }

    pub(crate) fn has_senders(&self) -> bool {
        self.senders.len() > 0
    }
}
