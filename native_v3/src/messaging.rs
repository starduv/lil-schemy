use std::sync::Arc;

use crate::typescript::Node;

pub enum Message {
    RegisterSender(u8),
    RequestModule(u8, String),
    RegisterNode(Arc<Node<'static>>),
    RequestNode(u8, usize),
    UnregisterSender(u8),
}

pub enum Reply {
    SendModule(u8, Arc<Node<'static>>),
    SendNode(u8, Arc<Node<'static>>),
}
