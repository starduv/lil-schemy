use std::rc::Rc;

use swc_ecma_ast::*;

use super::{NodeKind, SchemyNode};

impl<'n> SchemyNode<'n> {
    pub fn children(&self) -> Vec<usize> {
        let mut children = vec![];
        match self.kind {
            NodeKind::AwaitExpr(raw_await) => self.get_await_expr_children(raw_await, &mut children),
            NodeKind::ArrowExpr(arrow) => self.get_arrow_expr_children(arrow, &mut children),
            NodeKind::BlockStmt(raw) => self.get_block_statement_children(raw, &mut children),
            NodeKind::BlockStmtOrExpr(temp) => match temp {
                BlockStmtOrExpr::BlockStmt(raw) => self.get_block_statement_children(raw, &mut children),
                BlockStmtOrExpr::Expr(raw) => self.get_expr_children(raw, &mut children),
            },
            NodeKind::Callee(raw) => self.get_callee_children(raw, &mut children),
            NodeKind::CallExpr(raw) => self.get_call_expr_children(raw, &mut children),
            NodeKind::Decl(raw) => self.get_decl_children(raw, &mut children),
            NodeKind::ExportDecl(raw) => self.get_export_declartion_children(raw, &mut children),
            NodeKind::ExportDefaultDecl(raw) => self.get_export_default_decl_children(raw, &mut children),
            NodeKind::ExportDefaultExpr(raw) => self.get_export_default_expr_children(raw, &mut children),
            NodeKind::Expr(raw) => self.get_expr_children(raw, &mut children),
            NodeKind::ExprOrSpread(raw) => self.get_expr_children(&*raw.expr, &mut children),
            NodeKind::ExprStmt(raw) => self.get_expr_children(&*raw.expr, &mut children),
            NodeKind::ImportDecl(raw) => self.get_import_decl_children(raw, &mut children),
            NodeKind::Lit(raw) => self.get_lit_children(raw, &mut children),
            NodeKind::MemberExpr(raw) => self.get_member_expr_children(raw, &mut children),
            NodeKind::MemberProp(raw) => self.get_member_prop_children(raw, &mut children),
            NodeKind::Module(module) => self.get_module_children(module, &mut children),
            NodeKind::ModuleItem(raw) => self.get_module_item_children(raw, &mut children),
            NodeKind::NewExpr(raw) => self.get_new_expr_children(raw, &mut children),
            NodeKind::Pat(raw) => self.get_pat_children(raw, &mut children),
            NodeKind::TsAsExpr(raw) => self.get_ts_as_expr_children(raw, &mut children),
            NodeKind::TsEntityName(raw) => self.get_ts_entity_name_children(raw, &mut children),
            NodeKind::TsInterfaceDecl(raw) => self.get_ts_interface_decl_children(raw, &mut children),
            NodeKind::TsPropertySignature(raw) => self.get_ts_property_signature_children(raw, &mut children),
            NodeKind::TsType(raw) => self.get_ts_type_children(raw, &mut children),
            NodeKind::TsTypeAliasDecl(raw) => self.get_ts_type_alias_declaration(raw, &mut children),
            NodeKind::TsTypeAnnotation(raw) => self.get_type_annotation_children(raw, &mut children),
            NodeKind::TsTypeAssertionExpr(raw) => self.get_ts_type_assertion_expr_children(raw, &mut children),
            NodeKind::TsTypeElement(raw) => self.get_ts_type_element_children(raw, &mut children),
            NodeKind::TsTypeLit(raw) => self.get_type_lit_children(raw, &mut children),
            NodeKind::TsLitType(raw) => self.get_ts_lit_type_chilren(raw, &mut children),
            NodeKind::TsTypeParam(raw) => self.get_ts_type_param(raw, &mut children),
            NodeKind::TsTypeParamInstantiation(raw) => {
                self.get_ts_type_param_instantiation_children(raw, &mut children)
            }
            NodeKind::TsUnionType(raw) => self.get_ts_union_type_children(raw, &mut children),
            NodeKind::TsIntersectionType(raw) => self.get_ts_intersection_type_children(raw, &mut children),
            NodeKind::TsUnionOrIntersectionType(raw) => self.get_ts_union_or_intersection_children(raw, &mut children),
            NodeKind::TsTypeRef(raw) => self.get_ts_type_ref_children(raw, &mut children),
            NodeKind::VarDecl(raw) => self.get_var_decl_children(raw, &mut children),
            NodeKind::VarDeclarator(raw) => self.get_var_declarator_children(raw, &mut children),
            _ => {}
        }
        children
    }

    fn get_ts_lit_type_chilren(&self, raw: &'n TsLitType, children: &mut Vec<usize>) {
        self.get_ts_lit_children(&raw.lit, children);
    }

    fn get_ts_lit_children(&self, raw: &'n TsLit, children: &mut Vec<usize>) {
        match raw {
            TsLit::Number(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::Num(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsLit::Str(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::Str(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsLit::Tpl(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::TsTplLit(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsLit::Bool(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::Bool(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsLit::BigInt(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::BigInt(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
        }
    }

    fn get_ts_union_type_children(&self, raw: &'n TsUnionType, children: &mut Vec<usize>) {
        for type_ann in &raw.types {
            self.get_ts_type_children(type_ann, children);
        }
    }

    fn get_ts_intersection_type_children(&self, raw: &'n TsIntersectionType, children: &mut Vec<usize>) {
        for type_ann in &raw.types {
            self.get_ts_type_children(type_ann, children);
        }
    }

    fn get_ts_union_or_intersection_children(&self, raw: &'n TsUnionOrIntersectionType, children: &mut Vec<usize>) {
        match raw {
            TsUnionOrIntersectionType::TsUnionType(raw_union) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::TsUnionType(raw_union),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsUnionOrIntersectionType::TsIntersectionType(raw_intersection) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::TsIntersectionType(raw_intersection),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
        }
    }

    fn get_await_expr_children(&self, raw_await: &'n AwaitExpr, children: &mut Vec<usize>) {
        self.get_expr_children(&*raw_await.arg, children);
    }

    fn get_ts_type_param_instantiation_children(
        &self,
        type_params: &'n TsTypeParamInstantiation,
        children: &mut Vec<usize>,
    ) {
        for param in &type_params.params {
            self.get_ts_type_children(param, children);
        }
    }

    fn get_ts_type_param(&self, type_param: &'n TsTypeParam, children: &mut Vec<usize>) {
        if let Some(constraint) = &type_param.constraint {
            self.get_ts_type_children(constraint, children);
        }

        if let Some(default) = &type_param.default {
            self.get_ts_type_children(default, children);
        }
    }

    fn get_lit_children(&self, lit: &'n Lit, children: &mut Vec<usize>) {
        match lit {
            Lit::Str(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::Str(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Lit::Bool(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::Bool(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Lit::Null(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::Null(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Lit::Num(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::Num(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Lit::BigInt(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::BigInt(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Lit::Regex(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::Regex(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            _ => {}
        }
    }

    fn get_ts_type_assertion_expr_children(&self, expr: &'n TsTypeAssertion, children: &mut Vec<usize>) {
        self.get_expr_children(&*expr.expr, children);
        self.get_ts_type_children(&*expr.type_ann, children);
    }

    fn get_ts_entity_name_children(&self, entity_name: &'n TsEntityName, children: &mut Vec<usize>) {
        match entity_name {
            TsEntityName::Ident(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::Ident(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsEntityName::TsQualifiedName(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::TsQualifiedName(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
        }
    }

    fn get_ts_type_ref_children(&self, type_ref: &'n TsTypeRef, children: &mut Vec<usize>) {
        let mut borrow = self.context.borrow_mut();
        let child_index = borrow.nodes.len();
        let child_node = SchemyNode {
            index: child_index,
            parent_index: Some(self.index.clone()),
            kind: NodeKind::TsEntityName(&type_ref.type_name),
            context: self.context.clone(),
        };
        borrow.nodes.push(Rc::new(child_node));
        children.push(child_index);

        if let Some(type_params) = &type_ref.type_params {
            let child_index = borrow.nodes.len();
            let child_node = SchemyNode {
                index: child_index,
                parent_index: Some(self.index.clone()),
                kind: NodeKind::TsTypeParamInstantiation(type_params),
                context: self.context.clone(),
            };
            borrow.nodes.push(Rc::new(child_node));
            children.push(child_index);
        }
    }

    fn get_ts_type_children(&self, ts_type: &'n TsType, children: &mut Vec<usize>) {
        match ts_type {
            TsType::TsKeywordType(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsKeywordType(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsThisType(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsThisType(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsFnOrConstructorType(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsFnOrConstructorType(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsTypeRef(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsTypeRef(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsTypeQuery(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsTypeQuery(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsTypeLit(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsTypeLit(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsArrayType(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsArrayType(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsTupleType(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsTupleType(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsOptionalType(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsOptionalType(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsRestType(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsRestType(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsUnionOrIntersectionType(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsUnionOrIntersectionType(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsConditionalType(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsConditionalType(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsInferType(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsInferType(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsParenthesizedType(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsParenthesizedType(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsTypeOperator(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsTypeOperator(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsIndexedAccessType(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsIndexedAccessType(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsMappedType(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsMappedType(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsLitType(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsLitType(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsTypePredicate(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsTypePredicate(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsImportType(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsImportType(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
        }
    }

    fn get_new_expr_children(&self, expr: &'n NewExpr, children: &mut Vec<usize>) {
        self.get_expr_children(&expr.callee, children);
    }

    fn get_ts_as_expr_children(&self, expr: &'n TsAsExpr, children: &mut Vec<usize>) {
        match &*expr.type_ann {
            TsType::TsKeywordType(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsKeywordType(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsThisType(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsThisType(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsFnOrConstructorType(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsFnOrConstructorType(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsTypeRef(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsTypeRef(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsTypeQuery(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsTypeQuery(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsTypeLit(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsTypeLit(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsArrayType(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsArrayType(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsTupleType(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsTupleType(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsOptionalType(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsOptionalType(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsRestType(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsRestType(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsUnionOrIntersectionType(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsUnionOrIntersectionType(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsConditionalType(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsConditionalType(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsInferType(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsInferType(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsParenthesizedType(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsParenthesizedType(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsTypeOperator(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsTypeOperator(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsIndexedAccessType(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsIndexedAccessType(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsMappedType(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsMappedType(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsLitType(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsLitType(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsTypePredicate(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsTypePredicate(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsImportType(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                    kind: NodeKind::TsImportType(raw),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
        }
        self.get_expr_children(&*expr.expr, children);
    }

    // fn get_var_declarator_children(&self, var_declarator: &'n VarDeclarator, children: &mut Vec<usize>) {
    //     let mut borrow = self.context.borrow_mut();
    //     if let Some(init) = &var_declarator.init {
    //         let child_index = borrow.nodes.len();
    //         let child_node = SchemyNode {
    //             index: child_index,
    //             parent_index: Some(self.index.clone()),
    //             kind: NodeKind::Expr(init),
    //             context: self.context.clone(),
    //         };
    //         borrow.nodes.push(Rc::new(child_node));
    //         children.push(child_index);
    //     }
    // }

    fn get_var_declarator_children(&self, var_declarator: &'n VarDeclarator, children: &mut Vec<usize>) {
        let mut borrow = self.context.borrow_mut();
        if let Some(init) = &var_declarator.init {
            let child_index = borrow.nodes.len();
            let child_node = SchemyNode {
                index: child_index,
                parent_index: Some(self.index.clone()),
                kind: NodeKind::Expr(init),
                context: self.context.clone(),
            };
            borrow.nodes.push(Rc::new(child_node));
            children.push(child_index);
        }
    }

    fn get_member_prop_children(&self, prop: &'n MemberProp, children: &mut Vec<usize>) {
        match prop {
            MemberProp::Ident(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::Ident(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            _ => {}
        }
    }

    fn get_member_expr_children(&self, expr: &'n MemberExpr, children: &mut Vec<usize>) {
        let mut borrow = self.context.borrow_mut();
        let child_index = borrow.nodes.len();
        let child_node = SchemyNode {
            index: child_index,
            parent_index: Some(self.index.clone()),
            kind: NodeKind::Expr(&expr.obj),
            context: self.context.clone(),
        };
        borrow.nodes.push(Rc::new(child_node));
        children.push(child_index);

        let child_index = borrow.nodes.len();
        let child_node = SchemyNode {
            index: child_index,
            parent_index: Some(self.index.clone()),
            kind: NodeKind::MemberProp(&expr.prop),
            context: self.context.clone(),
        };
        borrow.nodes.push(Rc::new(child_node));
        children.push(child_index);
    }

    fn get_callee_children(&self, callee: &'n Callee, children: &mut Vec<usize>) {
        match callee {
            Callee::Expr(expr) => self.get_expr_children(expr, children),
            _ => {}
        }
    }

    fn get_decl_children(&self, decl: &'n Decl, children: &mut Vec<usize>) {
        match decl {
            Decl::Class(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::ClassDecl(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Decl::Fn(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::FnDecl(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Decl::TsEnum(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::TsEnumDecl(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Decl::TsInterface(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::TsInterfaceDecl(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Decl::TsModule(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::TsModuleDecl(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Decl::Var(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::VarDecl(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Decl::TsTypeAlias(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    kind: NodeKind::TsTypeAliasDecl(raw),
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            _ => {}
        }
    }

    fn get_ts_module_decl_children(&self, decl: &'n TsModuleDecl, children: &mut Vec<usize>) {
        match &decl.body {
            Some(TsNamespaceBody::TsModuleBlock(raw)) => self.get_ts_module_block_children(raw, children),
            Some(TsNamespaceBody::TsNamespaceDecl(raw)) => self.get_ts_namespace_decl_children(raw, children),
            _ => {}
        }
    }

    fn get_ts_namespace_decl_children(&self, decl: &'n TsNamespaceDecl, children: &mut Vec<usize>) {
        match &*decl.body {
            TsNamespaceBody::TsModuleBlock(raw) => self.get_ts_module_block_children(raw, children),
            TsNamespaceBody::TsNamespaceDecl(raw) => self.get_ts_namespace_decl_children(raw, children),
        }
    }

    fn get_ts_module_block_children(&self, block: &'n TsModuleBlock, children: &mut Vec<usize>) {
        let mut borrow = self.context.borrow_mut();
        block.body.iter().for_each(|item| {
            let child_index = borrow.nodes.len();
            let child_node = SchemyNode {
                index: child_index,
                parent_index: Some(self.index.clone()),
                kind: NodeKind::ModuleItem(item),
                context: self.context.clone(),
            };
            borrow.nodes.push(Rc::new(child_node));
            children.push(child_index);
        });
    }

    fn get_class_decl_children(&self, decl: &'n ClassDecl, children: &mut Vec<usize>) {
        let mut borrow = self.context.borrow_mut();
        decl.class.body.iter().for_each(|member| {
            let child_index = borrow.nodes.len();
            let child_node = SchemyNode {
                index: child_index,
                parent_index: Some(self.index.clone()),
                kind: NodeKind::ClassMember(member),
                context: self.context.clone(),
            };
            borrow.nodes.push(Rc::new(child_node));
            children.push(child_index);
        });
    }
    fn get_fn_decl_children(&self, decl: &'n FnDecl, children: &mut Vec<usize>) {
        let mut borrow = self.context.borrow_mut();
        decl.function.body.iter().for_each(|member| {
            let child_index = borrow.nodes.len();
            let child_node = SchemyNode {
                index: child_index,
                parent_index: Some(self.index.clone()),
                kind: NodeKind::BlockStmt(member),
                context: self.context.clone(),
            };
            borrow.nodes.push(Rc::new(child_node));
            children.push(child_index);
        });
    }
    fn get_ts_enum_decl_children(&self, decl: &'n TsEnumDecl, children: &mut Vec<usize>) {
        let mut borrow = self.context.borrow_mut();
        decl.members.iter().for_each(|member| {
            let child_index = borrow.nodes.len();
            let child_node = SchemyNode {
                index: child_index,
                parent_index: Some(self.index.clone()),
                kind: NodeKind::TsEnumMember(member),
                context: self.context.clone(),
            };
            borrow.nodes.push(Rc::new(child_node));
            children.push(child_index);
        });
    }
    fn get_ts_interface_decl_children(&self, decl: &'n TsInterfaceDecl, children: &mut Vec<usize>) {
        let mut borrow = self.context.borrow_mut();
        if let Some(type_params) = &decl.type_params {
            for param in &type_params.params {
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::TsTypeParam(&param),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
        }

        decl.body.body.iter().for_each(|member| {
            let child_index = borrow.nodes.len();
            let child_node = SchemyNode {
                index: child_index,
                parent_index: Some(self.index.clone()),
                kind: NodeKind::TsTypeElement(member),
                context: self.context.clone(),
            };
            borrow.nodes.push(Rc::new(child_node));
            children.push(child_index);
        });
    }

    fn get_ts_type_alias_declaration(&self, decl: &'n TsTypeAliasDecl, children: &mut Vec<usize>) {
        let mut borrow = self.context.borrow_mut();
        let child_index = borrow.nodes.len();
        let child_node = SchemyNode {
            index: child_index,
            parent_index: Some(self.index.clone()),
            kind: NodeKind::TsType(&decl.type_ann),
            context: self.context.clone(),
        };
        borrow.nodes.push(Rc::new(child_node));
        children.push(child_index);
    }

    fn get_module_decl_children(&self, decl: &'n ModuleDecl, children: &mut Vec<usize>) {
        match decl {
            ModuleDecl::Import(raw) => self.get_import_decl_children(raw, children),
            ModuleDecl::ExportDecl(raw) => self.get_export_declartion_children(raw, children),
            ModuleDecl::ExportNamed(raw) => self.get_named_export_children(raw, children),
            ModuleDecl::ExportDefaultDecl(raw) => self.get_export_default_decl_children(raw, children),
            ModuleDecl::ExportDefaultExpr(raw) => self.get_export_default_expr_children(raw, children),
            _ => {}
        }
    }

    fn get_named_export_children(&self, named_export: &'n NamedExport, children: &mut Vec<usize>) {
        let mut borrow = self.context.borrow_mut();
        for specifier in &named_export.specifiers {
            let child_index = borrow.nodes.len();
            let child_node = SchemyNode {
                index: child_index,
                parent_index: Some(self.index.clone()),
                kind: NodeKind::ExportSpecifier(specifier),
                context: self.context.clone(),
            };
            borrow.nodes.push(Rc::new(child_node));
            children.push(child_index);
        }
    }

    fn get_ts_type_element_children(&self, type_element: &'n TsTypeElement, children: &mut Vec<usize>) {
        match type_element {
            TsTypeElement::TsCallSignatureDecl(raw) => self.get_ts_call_signature_decl_children(raw, children),
            TsTypeElement::TsConstructSignatureDecl(raw) => {
                self.get_ts_construct_signature_decl_children(raw, children)
            }
            TsTypeElement::TsIndexSignature(raw) => self.get_ts_index_signature_children(raw, children),
            TsTypeElement::TsMethodSignature(raw) => self.get_ts_method_signature_children(raw, children),
            TsTypeElement::TsPropertySignature(raw) => self.get_ts_property_signature_children(raw, children),
            TsTypeElement::TsGetterSignature(raw) => self.get_ts_getter_signature_children(raw, children),
            _ => {}
        }
    }

    fn get_ts_call_signature_decl_children(&self, thing: &'n TsCallSignatureDecl, children: &mut Vec<usize>) {
        {
            let mut borrow = self.context.borrow_mut();
            let child_index = borrow.nodes.len();
            for param in &thing.params {
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::TsFnParam(param),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
        }

        if let Some(type_ann) = &thing.type_ann {
            self.get_type_annotation_children(type_ann, children);
        }
    }
    fn get_ts_construct_signature_decl_children(&self, thing: &'n TsConstructSignatureDecl, children: &mut Vec<usize>) {
        {
            let mut borrow = self.context.borrow_mut();
            let child_index = borrow.nodes.len();
            for param in &thing.params {
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::TsFnParam(param),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
        }

        if let Some(type_ann) = &thing.type_ann {
            self.get_type_annotation_children(type_ann, children);
        }
    }
    fn get_ts_index_signature_children(&self, thing: &'n TsIndexSignature, children: &mut Vec<usize>) {
        {
            let mut borrow = self.context.borrow_mut();
            let child_index = borrow.nodes.len();
            for param in &thing.params {
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::TsFnParam(param),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
        }

        if let Some(type_ann) = &thing.type_ann {
            self.get_type_annotation_children(type_ann, children);
        }
    }
    fn get_ts_method_signature_children(&self, thing: &'n TsMethodSignature, children: &mut Vec<usize>) {
        {
            let mut borrow = self.context.borrow_mut();
            let child_index = borrow.nodes.len();
            for param in &thing.params {
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::TsFnParam(param),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
        }

        if let Some(type_ann) = &thing.type_ann {
            self.get_type_annotation_children(type_ann, children);
        }
    }
    fn get_ts_property_signature_children(&self, signature: &'n TsPropertySignature, children: &mut Vec<usize>) {
        {
            let mut borrow = self.context.borrow_mut();
            let child_index = borrow.nodes.len();

            for param in &signature.params {
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::TsFnParam(param),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
        }

        if let Some(type_ann) = &signature.type_ann {
            self.get_type_annotation_children(type_ann, children);
        }
    }
    fn get_ts_getter_signature_children(&self, thing: &'n TsGetterSignature, children: &mut Vec<usize>) {
        if let Some(type_ann) = &thing.type_ann {
            self.get_type_annotation_children(type_ann, children);
        }
    }

    fn get_type_lit_children(&self, type_lit: &'n TsTypeLit, children: &mut Vec<usize>) {
        let mut borrow = self.context.borrow_mut();
        for member in &type_lit.members {
            let child_index = borrow.nodes.len();
            let child_node = SchemyNode {
                index: child_index,
                parent_index: Some(self.index.clone()),
                kind: NodeKind::TsTypeElement(member),
                context: self.context.clone(),
            };
            borrow.nodes.push(Rc::new(child_node));
            children.push(child_index);
        }
    }

    fn get_call_expr_children(&self, expr: &'n CallExpr, children: &mut Vec<usize>) {
        let mut borrow = self.context.borrow_mut();
        let child_index = borrow.nodes.len();
        let child_node = SchemyNode {
            index: child_index,
            parent_index: Some(self.index.clone()),
            kind: NodeKind::Callee(&expr.callee),
            context: self.context.clone(),
        };
        borrow.nodes.push(Rc::new(child_node));
        children.push(child_index);

        expr.args.iter().for_each(|arg| {
            let child_index = borrow.nodes.len();
            let child_node = SchemyNode {
                index: child_index,
                parent_index: Some(self.index.clone()),
                kind: NodeKind::ExprOrSpread(arg),
                context: self.context.clone(),
            };
            borrow.nodes.push(Rc::new(child_node));
            children.push(child_index);
        });
    }

    fn get_expr_children(&self, expr: &'n Expr, children: &mut Vec<usize>) {
        match expr {
            Expr::This(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::ThisExpr(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Expr::Array(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::ArrayLit(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Expr::Object(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::ObjectLit(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Expr::Fn(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::FnExpr(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Expr::Unary(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::UnaryExpr(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Expr::Update(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::UpdateExpr(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Expr::Bin(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::BinExpr(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Expr::Assign(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::AssignExpr(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Expr::Member(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::MemberExpr(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Expr::SuperProp(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::SuperPropExpr(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Expr::Cond(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::CondExpr(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Expr::Call(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::CallExpr(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Expr::New(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::NewExpr(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Expr::Seq(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::SeqExpr(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Expr::Ident(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::Ident(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Expr::Lit(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::Lit(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Expr::Tpl(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::Tpl(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Expr::TaggedTpl(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::TaggedTpl(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Expr::Arrow(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::ArrowExpr(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Expr::Class(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::ClassExpr(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Expr::Yield(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::YieldExpr(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Expr::MetaProp(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::MetaPropExpr(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Expr::Await(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::AwaitExpr(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Expr::Paren(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::ParenExpr(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Expr::TsTypeAssertion(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::TsTypeAssertionExpr(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Expr::TsConstAssertion(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::TsConstAssertionExpr(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Expr::TsNonNull(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::TsNonNullExpr(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Expr::TsAs(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::TsAsExpr(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Expr::TsInstantiation(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::TsInstantiationExpr(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Expr::TsSatisfies(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::TsSatisfiesExpr(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Expr::PrivateName(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::PrivateNameExpr(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Expr::OptChain(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::OptChainExpr(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Expr::Invalid(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: (Some(self.index.clone())),
                    kind: NodeKind::InvalidExpr(raw),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            _ => {}
        }
    }

    fn get_arrow_expr_children(&self, expr: &'n ArrowExpr, children: &mut Vec<usize>) {
        let mut borrow = self.context.borrow_mut();
        let child_index = borrow.nodes.len();
        let child_node = SchemyNode {
            index: child_index,
            parent_index: Some(self.index.clone()),
            kind: NodeKind::BlockStmtOrExpr(&*expr.body),
            context: self.context.clone(),
        };
        borrow.nodes.push(Rc::new(child_node));
        children.push(child_index);

        expr.params.iter().for_each(|param| {
            let child_index = borrow.nodes.len();
            let child_node = SchemyNode {
                index: child_index,
                parent_index: Some(self.index.clone()),
                kind: NodeKind::Pat(param),
                context: self.context.clone(),
            };
            borrow.nodes.push(Rc::new(child_node));
            children.push(child_index);
        });
    }

    fn get_module_children(&self, module: &'n Module, children: &mut Vec<usize>) {
        let mut borrow = self.context.borrow_mut();
        for item in &module.body {
            let child_index = borrow.nodes.len();
            let child_node = SchemyNode {
                index: child_index,
                parent_index: Some(self.index.clone()),
                kind: NodeKind::ModuleItem(item),
                context: self.context.clone(),
            };
            borrow.nodes.push(Rc::new(child_node));
            children.push(child_index);
        }
    }

    fn get_export_declartion_children(&self, export_decl: &'n ExportDecl, children: &mut Vec<usize>) {
        match &export_decl.decl {
            Decl::Class(declaration) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::ClassDecl(declaration),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Decl::Fn(declaration) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::FnDecl(declaration),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Decl::Var(declaration) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::VarDecl(declaration),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Decl::TsInterface(declaration) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::TsInterfaceDecl(declaration),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Decl::TsTypeAlias(declaration) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::TsTypeAliasDecl(declaration),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Decl::TsEnum(declaration) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::TsEnumDecl(declaration),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Decl::TsModule(declaration) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    kind: NodeKind::TsModuleDecl(declaration),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            _ => {}
        }
    }

    fn get_export_default_expr_children(&self, expression: &'n ExportDefaultExpr, children: &mut Vec<usize>) {
        self.get_expr_children(&expression.expr, children)
    }

    fn get_export_default_decl_children(&self, export_declaration: &'n ExportDefaultDecl, children: &mut Vec<usize>) {
        match &export_declaration.decl {
            DefaultDecl::Class(declaration) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::ClassExpr(&declaration),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            DefaultDecl::TsInterfaceDecl(declaration) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::TsInterfaceDecl(&declaration),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            _ => {}
        }
    }

    fn get_import_decl_children(&self, import_declaration: &'n ImportDecl, children: &mut Vec<usize>) {
        let mut borrow = self.context.borrow_mut();
        for specifier in &import_declaration.specifiers {
            let child_index = borrow.nodes.len();
            let child_node = SchemyNode {
                kind: NodeKind::ImportSpecifier(&specifier),
                index: child_index,
                parent_index: Some(self.index.clone()),
                context: self.context.clone(),
            };
            borrow.nodes.push(Rc::new(child_node));
            children.push(child_index);
        }
    }

    fn get_module_item_children(&self, module_item: &'n ModuleItem, children: &mut Vec<usize>) {
        match module_item {
            ModuleItem::ModuleDecl(declaration) => match declaration {
                ModuleDecl::Import(declaration) => {
                    let mut borrow = self.context.borrow_mut();
                    let child_index = borrow.nodes.len();
                    let child_node = SchemyNode {
                        kind: NodeKind::ImportDecl(&declaration),
                        index: child_index,
                        parent_index: Some(self.index.clone()),
                        context: self.context.clone(),
                    };
                    borrow.nodes.push(Rc::new(child_node));
                    children.push(child_index);
                }
                ModuleDecl::ExportDecl(declaration) => {
                    let mut borrow = self.context.borrow_mut();
                    let child_index = borrow.nodes.len();
                    let child_node = SchemyNode {
                        kind: NodeKind::ExportDecl(&declaration),
                        index: child_index,
                        parent_index: Some(self.index.clone()),
                        context: self.context.clone(),
                    };
                    borrow.nodes.push(Rc::new(child_node));
                    children.push(child_index);
                }
                ModuleDecl::ExportNamed(declaration) => {
                    let mut borrow = self.context.borrow_mut();
                    let child_index = borrow.nodes.len();
                    let child_node = SchemyNode {
                        kind: NodeKind::NamedExport(&declaration),
                        index: child_index,
                        parent_index: Some(self.index.clone()),
                        context: self.context.clone(),
                    };
                    borrow.nodes.push(Rc::new(child_node));
                    children.push(child_index);
                }
                ModuleDecl::ExportDefaultDecl(declaration) => {
                    let mut borrow = self.context.borrow_mut();
                    let child_index = borrow.nodes.len();
                    let child_node = SchemyNode {
                        kind: NodeKind::ExportDefaultDecl(&declaration),
                        index: child_index,
                        parent_index: Some(self.index.clone()),
                        context: self.context.clone(),
                    };
                    borrow.nodes.push(Rc::new(child_node));
                    children.push(child_index);
                }
                ModuleDecl::ExportDefaultExpr(declaration) => {
                    let mut borrow = self.context.borrow_mut();
                    let child_index = borrow.nodes.len();
                    let child_node = SchemyNode {
                        kind: NodeKind::ExportDefaultExpr(&declaration),
                        index: child_index,
                        parent_index: Some(self.index.clone()),
                        context: self.context.clone(),
                    };
                    borrow.nodes.push(Rc::new(child_node));
                    children.push(child_index);
                }
                ModuleDecl::ExportAll(declaration) => {
                    let mut borrow = self.context.borrow_mut();
                    let child_index = borrow.nodes.len();
                    let child_node = SchemyNode {
                        kind: NodeKind::ExportAll(&declaration),
                        index: child_index,
                        parent_index: Some(self.index.clone()),
                        context: self.context.clone(),
                    };
                    borrow.nodes.push(Rc::new(child_node));
                    children.push(child_index);
                }
                ModuleDecl::TsImportEquals(declaration) => {
                    let mut borrow = self.context.borrow_mut();
                    let child_index = borrow.nodes.len();
                    let child_node = SchemyNode {
                        kind: NodeKind::TsImportEquals(&declaration),
                        index: child_index,
                        parent_index: Some(self.index.clone()),
                        context: self.context.clone(),
                    };
                    borrow.nodes.push(Rc::new(child_node));
                    children.push(child_index);
                }
                ModuleDecl::TsExportAssignment(declaration) => {
                    let mut borrow = self.context.borrow_mut();
                    let child_index = borrow.nodes.len();
                    let child_node = SchemyNode {
                        kind: NodeKind::TsExportAssignment(&declaration),
                        index: child_index,
                        parent_index: Some(self.index.clone()),
                        context: self.context.clone(),
                    };
                    borrow.nodes.push(Rc::new(child_node));
                    children.push(child_index);
                }
                ModuleDecl::TsNamespaceExport(declaration) => {
                    let mut borrow = self.context.borrow_mut();
                    let child_index = borrow.nodes.len();
                    let child_node = SchemyNode {
                        kind: NodeKind::TsNamespaceExport(&declaration),
                        index: child_index,
                        parent_index: Some(self.index.clone()),
                        context: self.context.clone(),
                    };
                    borrow.nodes.push(Rc::new(child_node));
                    children.push(child_index);
                }
            },
            ModuleItem::Stmt(statement) => self.get_statement_children(statement, children),
        }
    }

    fn get_pat_children(&self, pat: &'n Pat, children: &mut Vec<usize>) {
        match pat {
            Pat::Ident(ident) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::TsTypeAnnotation(ident.type_ann.as_ref().unwrap()),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Pat::Array(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::ArrayPat(raw),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Pat::Rest(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::RestPat(raw),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Pat::Object(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::ObjectPat(raw),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Pat::Assign(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::AssignPat(raw),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Pat::Expr(raw) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::Expr(raw),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            _ => {}
        }
    }

    fn get_statement_children(&self, statement: &'n Stmt, children: &mut Vec<usize>) {
        match statement {
            Stmt::Block(block_stmt) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::BlockStmt(block_stmt),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Stmt::Empty(empty_stmt) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::EmptyStmt(empty_stmt),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Stmt::Debugger(debugger_stmt) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::DebuggerStmt(debugger_stmt),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Stmt::With(with_stmt) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::WithStmt(with_stmt),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Stmt::Return(return_stmt) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::ReturnStmt(return_stmt),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Stmt::Labeled(labeled_stmt) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::LabeledStmt(labeled_stmt),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Stmt::Break(break_stmt) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::BreakStmt(break_stmt),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Stmt::Continue(continue_stmt) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::ContinueStmt(continue_stmt),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Stmt::If(if_stmt) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::IfStmt(if_stmt),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Stmt::Switch(switch_stmt) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::SwitchStmt(switch_stmt),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Stmt::Throw(throw_stmt) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::ThrowStmt(throw_stmt),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Stmt::Try(try_stmt) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::TryStmt(try_stmt),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Stmt::While(while_stmt) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::WhileStmt(while_stmt),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Stmt::DoWhile(do_while_stmt) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::DoWhileStmt(do_while_stmt),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Stmt::For(for_stmt) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::ForStmt(for_stmt),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Stmt::ForIn(for_in_stmt) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::ForInStmt(for_in_stmt),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Stmt::ForOf(for_of_stmt) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::ForOfStmt(for_of_stmt),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Stmt::Decl(decl_stmt) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::Decl(decl_stmt),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            Stmt::Expr(expr_stmt) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::ExprStmt(expr_stmt),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
        }
    }

    fn get_block_statement_children(&self, block_stmnt: &'n BlockStmt, children: &mut Vec<usize>) {
        block_stmnt
            .stmts
            .iter()
            .for_each(|statement| self.get_statement_children(statement, children))
    }

    fn get_type_annotation_children(&self, type_annotation: &'n TsTypeAnn, children: &mut Vec<usize>) {
        match &*type_annotation.type_ann {
            TsType::TsKeywordType(ts_keyword_type) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::TsKeywordType(&ts_keyword_type),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsThisType(ts_this_type) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::TsThisType(&ts_this_type),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsFnOrConstructorType(ts_fn_or_constructor_type) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::TsFnOrConstructorType(&ts_fn_or_constructor_type),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsTypeRef(ts_type_ref) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::TsTypeRef(&ts_type_ref),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsTypeQuery(ts_type_query) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::TsTypeQuery(&ts_type_query),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsTypeLit(ts_type_lit) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::TsTypeLit(&ts_type_lit),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsArrayType(ts_array_type) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::TsArrayType(&ts_array_type),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsTupleType(ts_tuple_type) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::TsTupleType(&ts_tuple_type),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsOptionalType(ts_optional_type) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::TsOptionalType(&ts_optional_type),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsRestType(ts_rest_type) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::TsRestType(&ts_rest_type),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsUnionOrIntersectionType(ts_union_or_intersection_type) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::TsUnionOrIntersectionType(&ts_union_or_intersection_type),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsConditionalType(ts_conditional_type) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::TsConditionalType(&ts_conditional_type),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsInferType(ts_infer_type) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::TsInferType(&ts_infer_type),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsParenthesizedType(ts_parenthesized_type) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::TsParenthesizedType(&ts_parenthesized_type),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsTypeOperator(ts_type_operator) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::TsTypeOperator(&ts_type_operator),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsIndexedAccessType(ts_indexed_access_type) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::TsIndexedAccessType(&ts_indexed_access_type),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsMappedType(ts_mapped_type) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::TsMappedType(&ts_mapped_type),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsLitType(ts_lit_type) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::TsLitType(&ts_lit_type),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsTypePredicate(ts_type_predicate) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::TsTypePredicate(&ts_type_predicate),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            TsType::TsImportType(ts_import_type) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::TsImportType(&ts_import_type),
                    index: child_index,
                    parent_index: Some(self.index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
        }
    }

    fn get_var_decl_children(&self, variable_declaration: &'n VarDecl, children: &mut Vec<usize>) {
        let mut borrow = self.context.borrow_mut();
        variable_declaration.decls.iter().for_each(|decl| {
            let child_index = borrow.nodes.len();
            let child_node = SchemyNode {
                kind: NodeKind::VarDeclarator(decl),
                index: child_index,
                parent_index: Some(self.index.clone()),
                context: self.context.clone(),
            };
            borrow.nodes.push(Rc::new(child_node));
            children.push(child_index);
        })
    }
}
