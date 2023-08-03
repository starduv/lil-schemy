use std::rc::Rc;

use super::SchemyNode;

#[derive(Debug, Default)]
pub struct Context<'m> {
    pub(super) nodes: Vec<Rc<SchemyNode<'m>>>,
}
