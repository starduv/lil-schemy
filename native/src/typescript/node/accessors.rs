use std::{cell::RefCell, rc::Rc, vec};

use swc_ecma_ast::{Expr, TsTypeElement};

use super::{Context, NodeKind, SchemyNode};

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
            NodeKind::ClassExpr(raw_expr) => {
                let mut borrow = self.context.borrow_mut();
                let class_index = borrow.nodes.len();
                borrow.nodes.push(Rc::new(SchemyNode {
                    index: class_index,
                    parent_index: Some(self.index),
                    kind: NodeKind::Class(&*raw_expr.class),
                    context: self.context.clone(),
                }));
                borrow.nodes.get(class_index).map(|n| n.clone())
            }
            NodeKind::ClassDecl(raw_decl) => {
                let mut borrow = self.context.borrow_mut();
                let class_index = borrow.nodes.len();
                borrow.nodes.push(Rc::new(SchemyNode {
                    index: class_index,
                    parent_index: Some(self.index),
                    kind: NodeKind::Class(&*raw_decl.class),
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

    pub fn extends(&self) -> Vec<Rc<SchemyNode<'m>>> {
        let mut borrow = self.context.borrow_mut();
        match self.kind {
            NodeKind::TsInterfaceDecl(raw_decl) => raw_decl
            .extends
            .iter()
            .map(|raw_extend| {
                let child_index = borrow.nodes.len();
                borrow.nodes.push(Rc::new(SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index),
                    kind: NodeKind::TsExprWithTypeArgs(raw_extend),
                    context: self.context.clone(),
                }));
                borrow.nodes.get(child_index).map(|n| n.clone()).unwrap()
            })
            .collect(),
            _ => vec![]
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
            NodeKind::TsEnumDecl(raw_decl) => {
                let mut borrow = self.context.borrow_mut();
                raw_decl
                    .members
                    .iter()
                    .map(|raw_member| {
                        let child_index = borrow.nodes.len();
                        borrow.nodes.push(Rc::new(SchemyNode {
                            index: child_index,
                            parent_index: Some(self.index),
                            kind: NodeKind::TsEnumMember(raw_member),
                            context: self.context.clone(),
                        }));
                        borrow.nodes.get(child_index).map(|n| n.clone()).unwrap()
                    })
                    .collect()
            }
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

    pub fn type_params(&self) -> Option<Rc<SchemyNode<'m>>> {
        match self.kind {
            NodeKind::TsTypeRef(raw) => raw.type_params.as_ref().map(|type_params| {
                let mut borrow = self.context.borrow_mut();
                let params_index = borrow.nodes.len();
                borrow.nodes.push(Rc::new(SchemyNode {
                    index: params_index,
                    parent_index: Some(self.index),
                    kind: NodeKind::TsTypeParamInstantiation(&*type_params),
                    context: self.context.clone(),
                }));
                borrow.nodes.get(params_index).map(|n| n.clone()).unwrap()
            }),
            _ => None,
        }
    }

    pub fn params(&self) -> Vec<Rc<SchemyNode<'m>>> {
        match self.kind {
            NodeKind::TsTypeRef(raw_ref) => {
                let mut params = vec![];
                if let Some(raw_params) = &raw_ref.type_params {
                    let mut borrow = self.context.borrow_mut();
                    raw_params.params.iter().for_each(|param| {
                        let child_index = borrow.nodes.len();
                        borrow.nodes.push(Rc::new(SchemyNode {
                            index: child_index,
                            parent_index: Some(self.index),
                            kind: NodeKind::TsType(&*param),
                            context: self.context.clone(),
                        }));
                        params.push(borrow.nodes.get(child_index).unwrap().clone())
                    });
                }
                params
            }
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

    pub fn class_props(&self) -> Vec<Rc<SchemyNode<'m>>> {
        let mut props = vec![];
        match self.kind {
            NodeKind::Class(raw_class) => {
                for raw_member in &raw_class.body {
                    match raw_member {
                        swc_ecma_ast::ClassMember::Constructor(raw_ctor) => {
                            for raw_param in &raw_ctor.params {
                                match raw_param {
                                    swc_ecma_ast::ParamOrTsParamProp::TsParamProp(raw_prop) => match &raw_prop.param {
                                        swc_ecma_ast::TsParamPropParam::Ident(raw_ident) => {
                                            let mut borrow = self.context.borrow_mut();
                                            let class_index = borrow.nodes.len();
                                            borrow.nodes.push(Rc::new(SchemyNode {
                                                index: class_index,
                                                parent_index: Some(self.index),
                                                kind: NodeKind::BindingIdent(&raw_ident),
                                                context: self.context.clone(),
                                            }));
                                            props.push(borrow.nodes.get(class_index).map(|n| n.clone()).unwrap());
                                        }
                                        _ => {}
                                    },
                                    _ => {}
                                }
                            }
                        }
                        swc_ecma_ast::ClassMember::ClassProp(class_prop) => {
                            let mut borrow = self.context.borrow_mut();
                            let class_index = borrow.nodes.len();
                            borrow.nodes.push(Rc::new(SchemyNode {
                                index: class_index,
                                parent_index: Some(self.index),
                                kind: NodeKind::ClassProp(&class_prop),
                                context: self.context.clone(),
                            }));
                            props.push(borrow.nodes.get(class_index).map(|n| n.clone()).unwrap());
                        }
                        swc_ecma_ast::ClassMember::Method(_) => todo!(),
                        swc_ecma_ast::ClassMember::PrivateMethod(_) => todo!(),
                        swc_ecma_ast::ClassMember::PrivateProp(_) => todo!(),
                        swc_ecma_ast::ClassMember::TsIndexSignature(_) => todo!(),
                        swc_ecma_ast::ClassMember::Empty(_) => todo!(),
                        swc_ecma_ast::ClassMember::StaticBlock(_) => todo!(),
                        swc_ecma_ast::ClassMember::AutoAccessor(_) => todo!(),
                    }
                }
            }
            _ => {}
        }
        props
    }

    pub fn type_ann(&self) -> Option<Rc<SchemyNode<'m>>> {
        match self.kind {
            NodeKind::BindingIdent(raw_ident) => {
                if let Some(raw_ann) = &raw_ident.type_ann {
                    let mut borrow = self.context.borrow_mut();
                    let class_index = borrow.nodes.len();
                    borrow.nodes.push(Rc::new(SchemyNode {
                        index: class_index,
                        parent_index: Some(self.index),
                        kind: NodeKind::TsTypeAnnotation(&raw_ann),
                        context: self.context.clone(),
                    }));
                    borrow.nodes.get(class_index).map(|n| n.clone())
                } else {
                    None
                }
            }
            NodeKind::TsTypeAliasDecl(raw_decl) => {
                let mut borrow = self.context.borrow_mut();
                let class_index = borrow.nodes.len();
                borrow.nodes.push(Rc::new(SchemyNode {
                    index: class_index,
                    parent_index: Some(self.index),
                    kind: NodeKind::TsType(&*raw_decl.type_ann),
                    context: self.context.clone(),
                }));
                borrow.nodes.get(class_index).map(|n| n.clone())
            }
            NodeKind::TsTypeElement(TsTypeElement::TsPropertySignature(raw_prop)) => match &raw_prop.type_ann {
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

    pub fn as_arrow_expr(&self) -> Option<Rc<SchemyNode<'m>>> {
        match self.kind {
            NodeKind::ExprOrSpread(raw) => match &*raw.expr {
                Expr::Arrow(raw_arrow) => {
                    let mut borrow = self.context.borrow_mut();
                    let child_index = borrow.nodes.len();
                    let child_node = SchemyNode {
                        kind: NodeKind::ArrowExpr(&*raw_arrow),
                        index: child_index,
                        parent_index: Some(self.index),
                        context: self.context.clone(),
                    };
                    borrow.nodes.push(Rc::new(child_node));
                    borrow.nodes.get(child_index).map(|n| n.clone())
                }
                _ => None,
            },
            _ => None,
        }
    }
}
