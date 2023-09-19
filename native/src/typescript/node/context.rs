use std::rc::Rc;

use super::Node;

#[derive(Debug, Default)]
pub struct Context<'m> {
    pub(in crate::typescript) nodes: Vec<Rc<Node<'m>>>,
}
impl<'m> Context<'m> {
    pub(crate) fn new() -> Context<'m> {
        Self { nodes: Vec::new() }
    }
}
