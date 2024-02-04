mod accessors;
mod children;
mod node_kind;

use std::rc::Rc;
use std::rc::Weak;

use swc_ecma_ast::Module;

pub use self::node_kind::NodeKind;

#[derive(Debug)]
pub struct SchemyNode<'m> {
    pub parent: Option<Weak<SchemyNode<'m>>>,
    pub kind: NodeKind<'m>,
}

impl<'m> SchemyNode<'m> {
    pub fn from_module(module: Module) -> Rc<SchemyNode<'m>> {
        let parent = None;
        Rc::new(SchemyNode {
            parent,
            kind: NodeKind::Module(module),
        })
    }

    pub fn to_child(&self, kind: NodeKind<'m>) -> Rc<SchemyNode<'m>> {
        Rc::new(SchemyNode { parent: None, kind })
    }
}
