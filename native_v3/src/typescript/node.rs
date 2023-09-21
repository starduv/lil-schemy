use super::node_kind::NodeKind;

#[derive(Debug)]
pub struct Node<'m> {
    id: usize,
    pub kind: NodeKind<'m>,
}

impl<'m> Node<'m> {
    pub fn new(id: usize, kind: NodeKind<'m>) -> Self {
        Self { id, kind }
    }

    pub fn is_path(&self) -> bool {
        match &self.kind {
            NodeKind::Ident(raw) => raw.sym.eq("LilPath"),
            _ => false,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}
