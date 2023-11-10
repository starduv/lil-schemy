use std::{
    rc::{Rc, Weak},
    vec,
};

use super::{NodeKind, SchemyNode};

impl<'m> SchemyNode<'m> {
    pub fn args(self: &Rc<Self>) -> Vec<Rc<SchemyNode<'m>>> {
        let mut args = vec![];
        for child in self.children() {
            match child.kind {
                NodeKind::ExprOrSpread(_) => args.push(child.clone()),
                _ => {}
            }
        }
        args
    }

    pub fn callee(self: &Rc<Self>) -> Option<Rc<SchemyNode<'m>>> {
        for child in self.children() {
            match child.kind {
                NodeKind::Callee(_) => return Some(child.clone()),
                _ => {}
            }
        }

        None
    }

    pub fn class(self: &Rc<Self>) -> Option<Rc<SchemyNode<'m>>> {
        for child in self.children() {
            match child.kind {
                NodeKind::Class(_) => return Some(child.clone()),
                _ => {}
            }
        }

        None
    }

    pub fn decl(self: &Rc<Self>) -> Option<Rc<SchemyNode<'m>>> {
        for child in self.children() {
            match child.kind {
                NodeKind::Decl(_) => return Some(child.clone()),
                _ => {}
            }
        }

        None
    }

    pub fn elem_type(self: &Rc<Self>) -> Option<Rc<SchemyNode<'m>>> {
        for child in self.children() {
            match child.kind {
                NodeKind::TsType(_) => return Some(child.clone()),
                _ => {}
            }
        }

        None
    }

    pub fn extends(self: &Rc<Self>) -> Vec<Rc<SchemyNode<'m>>> {
        let mut extends = vec![];
        for child in self.children() {
            match child.kind {
                NodeKind::TsExprWithTypeArgs(_) => extends.push(child.clone()),
                _ => {}
            }
        }
        extends
    }

    pub fn interface_body(self: &Rc<Self>) -> Option<Rc<SchemyNode<'m>>> {
        for child in self.children() {
            match child.kind {
                NodeKind::TsInterfaceBody(_) => return Some(child.clone()),
                _ => {}
            }
        }

        None
    }

    pub fn members(self: &Rc<Self>) -> Vec<Rc<SchemyNode<'m>>> {
        let mut members = vec![];
        for child in self.children() {
            match child.kind {
                NodeKind::ClassMember(_) => members.push(child.clone()),
                NodeKind::TsEnumMember(_) => members.push(child.clone()),
                NodeKind::TsTypeElement(_) => members.push(child.clone()),
                _ => {}
            }
        }
        members
    }

    pub fn type_params(self: &Rc<Self>) -> Vec<Rc<SchemyNode<'m>>> {
        let mut params = vec![];
        for child in self.children() {
            match child.kind {
                NodeKind::TsTypeParamInstantiation(_) => for child in child.children() {
                    params.push(child.clone());
                }
                _ => {}
            }
        }
        params
    }

    pub fn params(self: &Rc<Self>) -> Vec<Rc<SchemyNode<'m>>> {
        let mut params = vec![];
        for child in self.children() {
            match child.kind {
                NodeKind::Pat(_) => params.push(child.clone()),
                NodeKind::TsTypeParamInstantiation(_) => {
                    for child in child.children() {
                        params.push(child.clone());
                    }
                }
                _ => {}
            }
        }
        params
    }

    pub fn parent(self: &Rc<Self>) -> Option<Rc<SchemyNode<'m>>> {
        self.parent.as_ref().map(|p| Weak::upgrade(p).unwrap())
    }

    pub fn specifiers(self: &Rc<Self>) -> Vec<Rc<SchemyNode<'m>>> {
        let mut specificers = vec![];
        for child in self.children() {
            match child.kind {
                NodeKind::ExportSpecifier(_) => specificers.push(child.clone()),
                _ => {}
            }
        }
        specificers
    }

    pub fn class_props(self: &Rc<Self>) -> Vec<Rc<SchemyNode<'m>>> {
        let mut props = vec![];
        for child in self.children() {
            for child in child.children() {
                match child.kind {
                    NodeKind::Constructor(_) => {
                        for child in child.children() {
                            match child.kind {
                                NodeKind::TsParamProp(_) => {
                                    for child in child.children() {
                                        match child.kind {
                                            NodeKind::BindingIdent(_) => props.push(child.clone()),
                                            _ => {}
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    NodeKind::ClassProp(_) => props.push(child.clone()),
                    _ => {}
                }
            }
        }
        props
    }

    pub fn type_ann(self: &Rc<Self>) -> Option<Rc<SchemyNode<'m>>> {
        for child in self.children() {
            match child.kind {
                NodeKind::TsType(_) => return Some(child.clone()),
                NodeKind::TsTypeAnnotation(_) => return Some(child.clone()),
                NodeKind::TsPropertySignature(_) => {
                    for child in child.children() {
                        match child.kind {
                            NodeKind::TsTypeAnnotation(_) => return Some(child.clone()),
                            _ => {}
                        }
                    }
                }
                NodeKind::TsTypeElement(_) => {
                    for child in child.children() {
                        match child.kind {
                            NodeKind::TsPropertySignature(_) => {
                                for child in child.children() {
                                    match child.kind {
                                        NodeKind::TsTypeAnnotation(_) => return Some(child.clone()),
                                        _ => {}
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn as_arrow_expr(self: &Rc<Self>) -> Option<Rc<SchemyNode<'m>>> {
        for child in self.children() {
            match child.kind {
                NodeKind::ArrowExpr(_) => return Some(child.clone()),
                _ => {}
            }
        }

        None
    }
}
