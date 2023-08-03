use std::{rc::Rc, vec, cell::RefCell};

use super::{NodeKind, SchemyNode, Context};

impl<'m> SchemyNode<'m> {
    pub fn args(&self) -> Vec<Rc<SchemyNode<'m>>> {
        match self.kind {
            NodeKind::CallExpr(expr) => {
                let mut borrow = self.context.borrow_mut();
                expr.args
                    .iter()
                    .map(|arg| {
                        let args_index = borrow.nodes.len();
                        borrow.nodes.push(Rc::new(SchemyNode {
                            index: args_index,
                            parent_index: Some(self.index),
                            kind: NodeKind::ExprOrSpread(arg),
                            context: self.context.clone(),
                        }));
                        borrow.nodes.get(args_index).map(|n| n.clone()).unwrap()
                    })
                    .collect()
            }
            _ => vec![],
        }
    }

    pub fn body(&self) -> Vec<Rc<SchemyNode<'m>>> {
        match self.kind {
            NodeKind::Class(class) => {
                let mut borrow = self.context.borrow_mut();
                class
                    .body
                    .iter()
                    .map(|statement| {
                        let args_index = borrow.nodes.len();
                        borrow.nodes.push(Rc::new(SchemyNode {
                            index: args_index,
                            parent_index: Some(self.index),
                            kind: NodeKind::ClassMember(statement),
                            context: self.context.clone(),
                        }));
                        borrow.nodes.get(args_index).map(|n| n.clone()).unwrap()
                    })
                    .collect()
            }
            NodeKind::TsInterfaceBody(body) => {
                let mut borrow = self.context.borrow_mut();
                body.body
                    .iter()
                    .map(|statement| {
                        let args_index = borrow.nodes.len();
                        borrow.nodes.push(Rc::new(SchemyNode {
                            index: args_index,
                            parent_index: Some(self.index),
                            kind: NodeKind::TsTypeElement(statement),
                            context: self.context.clone(),
                        }));
                        borrow.nodes.get(args_index).map(|n| n.clone()).unwrap()
                    })
                    .collect()
            }
            _ => vec![],
        }
    }

    pub fn callee(&self) -> Option<Rc<SchemyNode<'m>>> {
        match self.kind {
            NodeKind::CallExpr(expr) => {
                let mut borrow = self.context.borrow_mut();
                let callee = &expr.callee;
                let callee_index = borrow.nodes.len();
                borrow.nodes.push(Rc::new(SchemyNode {
                    index: callee_index,
                    parent_index: Some(self.index),
                    kind: NodeKind::Callee(callee),
                    context: self.context.clone(),
                }));
                borrow.nodes.get(callee_index).map(|n| n.clone())
            }
            _ => None,
        }
    }

    pub fn class(&self) -> Option<Rc<SchemyNode<'m>>> {
        match self.kind {
            NodeKind::ClassDecl(decl) => {
                let mut borrow = self.context.borrow_mut();
                let class_index = borrow.nodes.len();
                borrow.nodes.push(Rc::new(SchemyNode {
                    index: class_index,
                    parent_index: Some(self.index),
                    kind: NodeKind::Class(&*decl.class),
                    context: self.context.clone(),
                }));
                borrow.nodes.get(class_index).map(|n| n.clone())
            }
            _ => None,
        }
    }

    pub fn decl(&self) -> Option<Rc<SchemyNode<'m>>> {
        match self.kind {
            NodeKind::ExportDecl(export_decl) => {
                let mut borrow = self.context.borrow_mut();
                let class_index = borrow.nodes.len();
                borrow.nodes.push(Rc::new(SchemyNode {
                    index: class_index,
                    parent_index: Some(self.index),
                    kind: NodeKind::Decl(&export_decl.decl),
                    context: self.context.clone(),
                }));
                borrow.nodes.get(class_index).map(|n| n.clone())
            }
            _ => None,
        }
    }

    pub fn elem_type(&self) -> Option<Rc<SchemyNode<'m>>> {
        match self.kind {
            NodeKind::TsArrayType(array_type) => {
                let mut borrow = self.context.borrow_mut();
                let class_index = borrow.nodes.len();
                borrow.nodes.push(Rc::new(SchemyNode {
                    index: class_index,
                    parent_index: Some(self.index),
                    kind: NodeKind::TsType(&*array_type.elem_type),
                    context: self.context.clone(),
                }));
                borrow.nodes.get(class_index).map(|n| n.clone())
            }
            _ => None,
        }
    }

    pub fn get(&self, index: usize) -> Option<Rc<SchemyNode<'m>>> {
        let borrow = self.context.borrow();
        match borrow.nodes.get(index) {
            Some(node) => Some(node.clone()),
            None => None,
        }
    }

    pub fn get_context(&self) -> Rc<RefCell<Context<'m>>> {
        self.context.clone()
    }

    pub fn interface_body(&self) -> Option<Rc<SchemyNode<'m>>> {
        match self.kind {
            NodeKind::TsInterfaceDecl(decl) => {
                let mut borrow = self.context.borrow_mut();
                let callee_index = borrow.nodes.len();
                borrow.nodes.push(Rc::new(SchemyNode {
                    index: callee_index,
                    parent_index: Some(self.index),
                    kind: NodeKind::TsInterfaceBody(&decl.body),
                    context: self.context.clone(),
                }));
                borrow.nodes.get(callee_index).map(|n| n.clone())
            }
            _ => None,
        }
    }

    pub fn members(&self) -> Vec<Rc<SchemyNode<'m>>> {
        match self.kind {
            NodeKind::TsTypeLit(type_lit) => {
                let mut borrow = self.context.borrow_mut();
                type_lit
                    .members
                    .iter()
                    .map(|type_element| {
                        let params_index = borrow.nodes.len();
                        borrow.nodes.push(Rc::new(SchemyNode {
                            index: params_index,
                            parent_index: Some(self.index),
                            kind: NodeKind::TsTypeElement(type_element),
                            context: self.context.clone(),
                        }));
                        borrow.nodes.get(params_index).map(|n| n.clone()).unwrap()
                    })
                    .collect()
            }
            _ => vec![],
        }
    }

    pub fn params(&self) -> Vec<Rc<SchemyNode<'m>>> {
        match self.kind {
            NodeKind::ArrowExpr(expr) => expr
                .params
                .iter()
                .map(|param| {
                    let mut borrow = self.context.borrow_mut();
                    let params_index = borrow.nodes.len();
                    borrow.nodes.push(Rc::new(SchemyNode {
                        index: params_index,
                        parent_index: Some(self.index),
                        kind: NodeKind::Pat(param),
                        context: self.context.clone(),
                    }));
                    borrow.nodes.get(params_index).map(|n| n.clone()).unwrap()
                })
                .collect(),
            _ => vec![],
        }
    }

    pub fn parent(&self) -> Option<Rc<SchemyNode<'m>>> {
        let borrow = self.context.borrow();
        match self.parent_index {
            Some(index) => borrow.nodes.get(index).map(|n| n.clone()),
            None => None,
        }
    }

    pub fn specifiers(&self) -> Vec<Rc<SchemyNode<'m>>> {
        match self.kind {
            NodeKind::NamedExport(named_export) => {
                let mut borrow = self.context.borrow_mut();
                named_export
                    .specifiers
                    .iter()
                    .map(|type_element| {
                        let params_index = borrow.nodes.len();
                        borrow.nodes.push(Rc::new(SchemyNode {
                            index: params_index,
                            parent_index: Some(self.index),
                            kind: NodeKind::ExportSpecifier(type_element),
                            context: self.context.clone(),
                        }));
                        borrow.nodes.get(params_index).map(|n| n.clone()).unwrap()
                    })
                    .collect()
            }
            _ => vec![],
        }
    }

    pub fn type_ann(&self) -> Option<Rc<SchemyNode<'m>>> {
        match self.kind {
            NodeKind::ClassProp(class_prop) => match &class_prop.type_ann {
                Some(type_ann) => {
                    let mut borrow = self.context.borrow_mut();
                    let class_index = borrow.nodes.len();
                    borrow.nodes.push(Rc::new(SchemyNode {
                        index: class_index,
                        parent_index: Some(self.index),
                        kind: NodeKind::TsTypeAnnotation(&*type_ann),
                        context: self.context.clone(),
                    }));
                    borrow.nodes.get(class_index).map(|n| n.clone())
                }
                None => None,
            },
            _ => None,
        }
    }

    pub fn type_params(&self) -> Vec<Rc<SchemyNode<'m>>> {
        match self.kind {
            NodeKind::TsTypeRef(type_ref) => {
                let mut borrow = self.context.borrow_mut();
                type_ref
                    .type_params
                    .iter()
                    .map(|param| {
                        let params_index = borrow.nodes.len();
                        borrow.nodes.push(Rc::new(SchemyNode {
                            index: params_index,
                            parent_index: Some(self.index),
                            kind: NodeKind::TsTypeParamInstantiation(param),
                            context: self.context.clone(),
                        }));
                        borrow.nodes.get(params_index).map(|n| n.clone()).unwrap()
                    })
                    .collect()
            }
            _ => vec![],
        }
    }
}
