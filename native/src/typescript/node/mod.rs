mod accessors;
mod children;
mod context;
mod node_kind;

use std::{cell::RefCell, rc::Rc};

use swc_ecma_ast::Module;

pub use self::context::Context;
pub use self::node_kind::NodeKind;

#[derive(Debug)]
pub struct Node<'m> {
    pub index: usize,
    pub parent_index: Option<usize>,
    pub kind: NodeKind<'m>,
    pub context: Rc<RefCell<Context<'m>>>,
}

impl<'m, 'c> Node<'m> {
    pub fn from_module(module: &'m Module) -> Rc<Node<'m>> {
        let context = Rc::new(RefCell::new(Context::new()));
        let mut borrow = context.borrow_mut();
        let index = borrow.nodes.len();
        let parent_index = None;
        let node = Node {
            index,
            parent_index,
            kind: NodeKind::Module(module),
            context: context.clone(),
        };
        borrow.nodes.push(Rc::new(node));
        borrow.nodes.last().map(|n| n.clone()).unwrap()
    }

    pub fn to_child(&self, kind: NodeKind<'m>) -> Rc<Node<'m>> {
        let mut borrow = self.context.borrow_mut();
        let index = borrow.nodes.len();
        let parent_index = Some(self.index);
        let node = Node {
            index,
            parent_index,
            kind,
            context: self.context.clone(),
        };
        borrow.nodes.push(Rc::new(node));
        borrow.nodes.last().map(|n| n.clone()).unwrap()
    }
}
