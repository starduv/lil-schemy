use std::{
    collections::HashSet,
    fmt::Debug,
    sync::{
        atomic::{AtomicU8, Ordering},
        Arc,
    },
};

use crate::typescript::Node;

use std::time::Duration;

use crossbeam::channel::{Receiver, Sender};

static IDENTITY: AtomicU8 = AtomicU8::new(0);

pub enum Message {
    Mapper(u8),
    MapperDone(u8),
    RequestModule(u8, String),
    RegisterNode(Arc<Node<'static>>),
    RequestNode(u8, u8),
    NoOp,
}

pub enum Reply {
    SendModule(u8, Arc<Node<'static>>),
    SendNode(u8, Option<Arc<Node<'static>>>),
}

pub struct MessageBus {
    id: u8,
    message: Sender<Message>,
    on_message: Receiver<Message>,
    on_reply: Receiver<Reply>,
    reply: Sender<Reply>,
    mappers: HashSet<u8>,
    is_mapper: bool,
}

impl Drop for MessageBus {
    fn drop(&mut self) {
        if self.is_mapper {
            self.message.send(Message::MapperDone(self.id)).unwrap();
        }
    }
}

impl Debug for MessageBus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MessageBus").field("id", &self.id).finish()
    }
}

impl MessageBus {
    pub fn new() -> Self {
        let id = IDENTITY.fetch_add(1, Ordering::AcqRel);

        let (message, on_message) = crossbeam::channel::unbounded::<Message>();
        let (reply, on_reply) = crossbeam::channel::unbounded::<Reply>();

        Self {
            id,
            message,
            reply,
            on_message,
            on_reply,
            mappers: Default::default(),
            is_mapper: false,
        }
    }

    pub(crate) fn for_mapper(&self) -> Self {
        let id = IDENTITY.fetch_add(1, Ordering::AcqRel);

        self.message.send(Message::Mapper(id)).unwrap();

        Self {
            id,
            message: self.message.clone(),
            reply: self.reply.clone(),
            on_message: self.on_message.clone(),
            on_reply: self.on_reply.clone(),
            mappers: Default::default(),
            is_mapper: true,
        }
    }

    pub(crate) fn for_node(&self) -> Self {
        let id = IDENTITY.fetch_add(1, Ordering::AcqRel);

        Self {
            id,
            message: self.message.clone(),
            reply: self.reply.clone(),
            on_message: self.on_message.clone(),
            on_reply: self.on_reply.clone(),
            mappers: Default::default(),
            is_mapper: false,
        }
    }

    pub(crate) fn request_module(&self, filepath: String) -> Arc<Node<'static>> {
        self.message.send(Message::RequestModule(self.id, filepath)).unwrap();

        loop {
            match self.on_reply.recv_timeout(Duration::from_secs(5)) {
                Ok(Reply::SendModule(ref id, node)) if self.id.eq(id) => return node,
                Err(err) => panic!("Request module error: {}", err),
                _ => continue,
            }
        }
    }

    pub(crate) fn request_node(&self, parent_id: Option<u8>) -> Option<Arc<Node<'static>>> {
        if parent_id.is_none() {
            return None;
        }

        self.message
            .send(Message::RequestNode(self.id, parent_id.unwrap()))
            .unwrap();

        loop {
            match self.on_reply.recv_timeout(Duration::from_secs(5)) {
                Ok(Reply::SendNode(ref id, node)) if self.id.eq(id) => return node,
                Err(err) => panic!("Request node error: {}", err),
                _ => continue,
            }
        }
    }

    pub(crate) fn register_node(&self, node: Arc<Node<'static>>) -> () {
        self.message.send(Message::RegisterNode(node)).unwrap();
    }

    pub(crate) fn send_module(&self, id: u8, module: Arc<Node<'static>>) -> () {
        self.reply.send(Reply::SendModule(id, module)).unwrap();
    }

    pub(crate) fn send_node(&self, id: u8, node: Option<Arc<Node<'static>>>) -> () {
        self.reply.send(Reply::SendNode(id, node)).unwrap();
    }
}

impl Iterator for MessageBus {
    type Item = Message;

    fn next(&mut self) -> Option<Self::Item> {
        match self.on_message.recv_timeout(Duration::from_secs(5)) {
            Ok(Message::Mapper(id)) => {
                self.mappers.insert(id);
                return Some(Message::NoOp);
            }
            Ok(Message::MapperDone(id)) => {
                self.mappers.remove(&id);
                if self.mappers.is_empty() {
                    return None;
                } else {
                    return Some(Message::NoOp);
                }
            }
            Ok(message) => return Some(message),
            Err(err) => {
                println!("Error: {}", err);
                return None;
            }
        }
    }
}
