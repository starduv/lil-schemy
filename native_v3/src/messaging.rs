use std::sync::Arc;

use crate::typescript::Node;

pub enum Message {
    RequestModule(u8, String),
    RequestChildren(u8, usize),
    RequestNode(u8, usize),
}

pub enum Reply {
    SendModule(u8, Arc<Node<'static>>),
    SendChildren(u8, Vec<Arc<Node<'static>>>),
    SendNode(u8, Arc<Node<'static>>),
}
