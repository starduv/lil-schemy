use std::rc::Rc;

use super::SchemyNode;

#[derive(Debug, Default)]
pub struct Context<'n> {
    pub(super) nodes: Vec<Rc<SchemyNode<'n>>>,
}
