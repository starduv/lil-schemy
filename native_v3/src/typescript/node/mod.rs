mod children;

use std::{
    collections::{BTreeMap, HashMap},
    sync::{
        atomic::{AtomicU8, Ordering},
        Arc, RwLock, Weak,
    },
};

use crate::messaging::MessageBus;

use self::children::get_expr_children;

use super::node_kind::NodeKind;

static IDENTITY: AtomicU8 = AtomicU8::new(0);

#[derive(Debug)]
pub struct Node<'m> {
    children: RwLock<Option<Vec<Arc<Node<'m>>>>>,
    id: u8,
    parent: Option<Weak<Node<'m>>>,
    pub kind: NodeKind<'m>,
}

impl<'m> Node<'m> {
    pub fn new(kind: NodeKind<'m>, parent: Option<Weak<Node<'m>>>) -> Node<'m> {
        Self {
            id: IDENTITY.fetch_add(1, Ordering::AcqRel),
            kind,
            parent,
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

    pub fn parent(&self) -> Option<Arc<Node<'m>>> {
        self.parent.map(|p| p.upgrade().unwrap())
    }

    pub fn with_parent(&self, callback: impl FnOnce(Arc<Node>) -> ()) -> () {
        if let Some(parent) = self.parent {
            callback(parent.upgrade().unwrap());
        }
    }

    pub fn find_child(&self, predicate: impl Fn(&Node) -> bool) -> Option<Arc<Node>> {
        let children = self.children.read().unwrap();
        if let Some(children) = &*children {
            for child in children {
                if predicate(child) {
                    return Some(child.clone());
                }
            }
        }
        None
    }

    pub fn call_arguments(&self, parent: Weak<Self>) -> Vec<Arc<Node>> {
        let mut args = vec![];
        match self.kind {
            NodeKind::CallExpr(expr) => expr
                .args
                .iter()
                .for_each(|arg| get_expr_children(&arg.expr, &mut args, parent)),
            _ => {}
        }
        args
    }
}

pub trait GetProperty {
    fn get_property<R>(&self, accessor: impl Fn(&Self) -> Option<R>) -> Option<R>;
}

impl GetProperty for Node<'static> {
    fn get_property<R>(&self, accessor: impl Fn(&Self) -> Option<R>) -> Option<R> {
        accessor(self)
    }
}
