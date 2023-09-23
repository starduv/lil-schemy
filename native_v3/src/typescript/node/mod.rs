mod children;

use std::sync::{
    atomic::{AtomicU8, Ordering},
    Arc, RwLock,
};

use crossbeam::channel::Sender;

use crate::messaging::Message;

use super::node_kind::NodeKind;

static IDENTITY: AtomicU8 = AtomicU8::new(0);

#[derive(Debug)]
pub struct Node<'m> {
    id: u8,
    children: RwLock<Option<Vec<Arc<Node<'m>>>>>,
    parent: Option<u8>,
    message: Sender<Message>,
    pub kind: NodeKind<'m>,
}

impl<'m> Node<'m> {
    pub fn new(kind: NodeKind<'m>, parent: Option<u8>, message: Sender<Message>) -> Node<'m> {
        Self {
            id: IDENTITY.fetch_add(1, Ordering::AcqRel),
            kind,
            parent,
            message,
            children: RwLock::new(None),
        }
    }

    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn is_path(&self) -> bool {
        match &self.kind {
            NodeKind::Ident(raw) => raw.sym.eq("LilPath"),
            _ => false,
        }
    }
}
