use swc_common::{
    errors::{ColorConfig, Handler},
    sync::Lrc,
    SourceMap,
};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};

use std::{
    collections::{BTreeMap, HashMap},
    path::Path,
    sync::Arc,
};

use crate::messaging::{Message, MessageBus};

use super::{node_kind::NodeKind, Node};

pub struct Application {
    context: HashMap<u8, Arc<Node<'static>>>,
    modules: BTreeMap<String, u8>,
    source_map: Lrc<SourceMap>,
}

impl Application {
    pub(crate) fn new() -> Self {
        Application {
            context: Default::default(),
            modules: Default::default(),
            source_map: Default::default(),
        }
    }

    pub(crate) fn get_module(&mut self, path: &str, bus: MessageBus) -> Arc<Node<'static>> {
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

            let node = Arc::new(Node::new(NodeKind::Module(module), None, bus));
            self.modules.insert(path.to_string(), node.id());
            self.context.insert(node.id(), node);
        }

        let id = self.modules.get(path).unwrap();
        self.context.get(id).unwrap().clone()
    }

    pub(crate) fn register_node(&mut self, node: Arc<Node<'static>>) -> () {
        self.context.insert(node.id(), node.clone());
    }

    pub(crate) fn run(&mut self, mut bus: MessageBus) -> () {
        while let Some(message) = bus.next() {
            match message {
                Message::RequestModule(id, path) => {
                    let module = self.get_module(&path, bus.for_node());
                    bus.send_module(id, module);
                }
                Message::RequestNode(id, node_id) => {
                    let node = self.context.get(&node_id).map(|n| n.clone());
                    bus.send_node(id, node);
                }
                Message::RegisterNode(node) => {
                    self.register_node(node);
                }
                _ => {}
            }
        }
    }
}
