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

use crate::messaging::{Message, Reply};

use super::{node_kind::NodeKind, Node};

pub struct Application {
    context: HashMap<u8, Arc<Node<'static>>>,
    modules: BTreeMap<String, u8>,
    source_map: Lrc<SourceMap>,
    senders: HashSet<u8>,
    message: Sender<Message>,
    on_message: crossbeam::channel::Receiver<Message>,
    reply: Sender<Reply>,
}

impl Application {
    pub(crate) fn new(
        message: Sender<Message>,
        on_message: crossbeam::channel::Receiver<Message>,
        reply: Sender<Reply>,
    ) -> Self {
        Application {
            context: Default::default(),
            modules: Default::default(),
            source_map: Default::default(),
            senders: Default::default(),
            message,
            on_message,
            reply,
        }
    }

    pub(crate) fn get_module(&mut self, path: &str, message: Sender<Message>) -> Arc<Node<'static>> {
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

    pub(crate) fn register_node(&mut self, node: Arc<Node<'static>>) -> () {
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

    pub(crate) fn run(&mut self) -> () {
        loop {
            match self.on_message.recv() {
                Ok(Message::RequestModule(id, path)) => {
                    let module = self.get_module(&path, self.message.clone());
                    self.reply.send(Reply::SendModule(id, module)).unwrap();
                }
                Ok(Message::RegisterNode(node)) => {
                    self.register_node(node);
                }
                Ok(Message::RegisterSender(id)) => {
                    self.register_sender(id);
                }
                Ok(Message::UnregisterSender(ref id)) => {
                    self.unregister_sender(id);
                }
                Err(err) => {
                    println!("Error: {}", err);
                    break;
                }
                _ => {}
            }

            if !self.has_senders() {
                break;
            }
        }
    }
}
