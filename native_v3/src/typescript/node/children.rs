use std::{mem::transmute, sync::Arc};

use crossbeam::channel::Sender;
use swc_ecma_ast::*;

use crate::messaging::Message;

use super::{Node, NodeKind};

impl Node<'static> {
    pub fn children(&self) -> Vec<Arc<Node<'static>>> {
        let parent_id = self.id();
        let mut children = self.children.write().unwrap();
        if children.is_some() {
            children.as_ref().unwrap().iter().map(|c| c.clone()).collect()
        } else {
            let mut children = children.get_or_insert(vec![]);
            match self.kind {
                NodeKind::AwaitExpr(raw_await) => {
                    get_await_expr_children(raw_await, &mut children, parent_id, self.message.clone())
                }
                NodeKind::ArrowExpr(arrow) => {
                    get_arrow_expr_children(arrow, &mut children, parent_id, self.message.clone())
                }
                NodeKind::BlockStmt(raw) => {
                    get_block_statement_children(raw, &mut children, parent_id, self.message.clone())
                }
                NodeKind::BlockStmtOrExpr(temp) => match temp {
                    BlockStmtOrExpr::BlockStmt(raw) => {
                        get_block_statement_children(raw, &mut children, parent_id, self.message.clone())
                    }
                    BlockStmtOrExpr::Expr(raw) => {
                        get_expr_children(raw, &mut children, parent_id, self.message.clone())
                    }
                },
                NodeKind::Callee(raw) => get_callee_children(raw, &mut children, parent_id, self.message.clone()),
                NodeKind::CallExpr(raw) => get_call_expr_children(raw, &mut children, parent_id, self.message.clone()),
                NodeKind::Decl(raw) => get_decl_children(raw, &mut children, parent_id, self.message.clone()),
                NodeKind::ExportDecl(raw) => {
                    get_export_declartion_children(raw, &mut children, parent_id, self.message.clone())
                }
                NodeKind::ExportDefaultDecl(raw) => {
                    get_export_default_decl_children(raw, &mut children, parent_id, self.message.clone())
                }
                NodeKind::ExportDefaultExpr(raw) => {
                    get_export_default_expr_children(raw, &mut children, parent_id, self.message.clone())
                }
                NodeKind::Expr(raw) => get_expr_children(raw, &mut children, parent_id, self.message.clone()),
                NodeKind::ExprOrSpread(raw) => {
                    get_expr_children(&*raw.expr, &mut children, parent_id, self.message.clone())
                }
                NodeKind::ExprStmt(raw) => {
                    get_expr_children(&*raw.expr, &mut children, parent_id, self.message.clone())
                }
                NodeKind::IfStmt(raw) => get_if_statement_children(raw, &mut children, parent_id, self.message.clone()),
                NodeKind::ImportDecl(raw) => {
                    get_import_decl_children(raw, &mut children, parent_id, self.message.clone())
                }
                NodeKind::Lit(raw) => get_lit_children(raw, &mut children, parent_id, self.message.clone()),
                NodeKind::MemberExpr(raw) => {
                    get_member_expr_children(raw, &mut children, parent_id, self.message.clone())
                }
                NodeKind::MemberProp(raw) => {
                    get_member_prop_children(raw, &mut children, parent_id, self.message.clone())
                }
                NodeKind::Module(ref module) => get_module_children(
                    unsafe { transmute::<&Module, &'static Module>(module) },
                    &mut children,
                    parent_id,
                    self.message.clone(),
                ),
                NodeKind::ModuleItem(raw) => {
                    get_module_item_children(raw, &mut children, parent_id, self.message.clone())
                }
                NodeKind::NewExpr(raw) => get_new_expr_children(raw, &mut children, parent_id, self.message.clone()),
                NodeKind::Pat(raw) => get_pat_children(raw, &mut children, parent_id, self.message.clone()),
                NodeKind::ReturnStmt(raw) => {
                    get_return_statement_children(raw, &mut children, parent_id, self.message.clone())
                }
                NodeKind::TryStmt(raw) => {
                    get_try_statement_children(raw, &mut children, parent_id, self.message.clone())
                }
                NodeKind::TsAsExpr(raw) => get_ts_as_expr_children(raw, &mut children, parent_id, self.message.clone()),
                NodeKind::TsEntityName(raw) => {
                    get_ts_entity_name_children(raw, &mut children, parent_id, self.message.clone())
                }
                NodeKind::TsInterfaceDecl(raw) => {
                    get_ts_interface_decl_children(raw, &mut children, parent_id, self.message.clone())
                }
                NodeKind::TsIntersectionType(raw) => {
                    get_ts_intersection_type_children(raw, &mut children, parent_id, self.message.clone())
                }
                NodeKind::TsLitType(raw) => {
                    get_ts_lit_type_chilren(raw, &mut children, parent_id, self.message.clone())
                }
                NodeKind::TsModuleDecl(raw) => {
                    get_ts_module_decl_children(raw, &mut children, parent_id, self.message.clone())
                }
                NodeKind::TsPropertySignature(raw) => {
                    get_ts_property_signature_children(raw, &mut children, parent_id, self.message.clone())
                }
                NodeKind::TsType(raw) => get_ts_type_children(raw, &mut children, parent_id, self.message.clone()),
                NodeKind::TsTypeAliasDecl(raw) => {
                    get_ts_type_alias_declaration(raw, &mut children, parent_id, self.message.clone())
                }
                NodeKind::TsTypeAnnotation(raw) => {
                    get_type_annotation_children(raw, &mut children, parent_id, self.message.clone())
                }
                NodeKind::TsTypeAssertionExpr(raw) => {
                    get_ts_type_assertion_expr_children(raw, &mut children, parent_id, self.message.clone())
                }
                NodeKind::TsTypeElement(raw) => {
                    get_ts_type_element_children(raw, &mut children, parent_id, self.message.clone())
                }
                NodeKind::TsTypeLit(raw) => get_type_lit_children(raw, &mut children, parent_id, self.message.clone()),
                NodeKind::TsTypeParam(raw) => get_ts_type_param(raw, &mut children, parent_id, self.message.clone()),
                NodeKind::TsTypeParamInstantiation(raw) => {
                    get_ts_type_param_instantiation_children(raw, &mut children, parent_id, self.message.clone())
                }
                NodeKind::TsUnionType(raw) => {
                    get_ts_union_type_children(raw, &mut children, parent_id, self.message.clone())
                }
                NodeKind::TsUnionOrIntersectionType(raw) => {
                    get_ts_union_or_intersection_children(raw, &mut children, parent_id, self.message.clone())
                }
                NodeKind::TsTypeRef(raw) => {
                    get_ts_type_ref_children(raw, &mut children, parent_id, self.message.clone())
                }
                NodeKind::VarDecl(raw) => get_var_decl_children(raw, &mut children, parent_id, self.message.clone()),
                NodeKind::VarDeclarator(raw) => {
                    get_var_declarator_children(raw, &mut children, parent_id, self.message.clone())
                }
                _ => {}
            }
            children.iter().cloned().collect()
        }
    }
}

fn push_child(
    kind: NodeKind<'static>,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    let child_node = Arc::new(Node::new(kind, Some(parent_id), message.clone()));
    message.send(Message::RegisterNode(child_node.clone())).unwrap();
    children.push(child_node);
}

fn get_return_statement_children(
    raw: &'static ReturnStmt,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    if let Some(arg) = &raw.arg {
        get_expr_children(arg, children, parent_id, message);
    }
}

fn get_if_statement_children(
    raw: &'static IfStmt,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    get_expr_children(&*raw.test, children, parent_id, message.clone());
    get_statement_children(&*raw.cons, children, parent_id, message.clone());
    if let Some(alt) = &raw.alt {
        get_statement_children(alt, children, parent_id, message);
    }
}

fn get_try_statement_children(
    raw: &'static TryStmt,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    get_block_statement_children(&raw.block, children, parent_id, message.clone());

    if let Some(catch) = &raw.handler {
        get_catch_clause_children(catch, children, parent_id, message.clone());
    }

    if let Some(finalizer) = &raw.finalizer {
        get_block_statement_children(finalizer, children, parent_id, message);
    }
}

fn get_catch_clause_children(
    raw: &'static CatchClause,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    get_block_statement_children(&raw.body, children, parent_id, message);
}

fn get_ts_lit_type_chilren(
    raw: &'static TsLitType,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    get_ts_lit_children(&raw.lit, children, parent_id, message);
}

fn get_ts_lit_children(
    raw: &'static TsLit,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    match raw {
        TsLit::Number(raw) => {
            push_child(NodeKind::Num(raw), children, parent_id, message);
        }
        TsLit::Str(raw) => {
            push_child(NodeKind::Str(raw), children, parent_id, message);
        }
        TsLit::Tpl(raw) => {
            push_child(NodeKind::TsTplLit(raw), children, parent_id, message);
        }
        TsLit::Bool(raw) => {
            push_child(NodeKind::Bool(raw), children, parent_id, message);
        }
        TsLit::BigInt(raw) => {
            push_child(NodeKind::BigInt(raw), children, parent_id, message);
        }
    }
}

fn get_ts_union_type_children(
    raw: &'static TsUnionType,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    for type_ann in &raw.types {
        get_ts_type_children(type_ann, children, parent_id, message.clone());
    }
}

fn get_ts_intersection_type_children(
    raw: &'static TsIntersectionType,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    for type_ann in &raw.types {
        get_ts_type_children(type_ann, children, parent_id, message.clone());
    }
}

fn get_ts_union_or_intersection_children(
    raw: &'static TsUnionOrIntersectionType,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    match raw {
        TsUnionOrIntersectionType::TsUnionType(raw_union) => {
            push_child(NodeKind::TsUnionType(raw_union), children, parent_id, message);
        }
        TsUnionOrIntersectionType::TsIntersectionType(raw_intersection) => {
            push_child(
                NodeKind::TsIntersectionType(raw_intersection),
                children,
                parent_id,
                message,
            );
        }
    }
}

fn get_await_expr_children(
    raw_await: &'static AwaitExpr,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    get_expr_children(&*raw_await.arg, children, parent_id, message);
}

fn get_ts_type_param_instantiation_children(
    type_params: &'static TsTypeParamInstantiation,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    for param in &type_params.params {
        get_ts_type_children(param, children, parent_id, message.clone());
    }
}

fn get_ts_type_param(
    type_param: &'static TsTypeParam,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    if let Some(constraint) = &type_param.constraint {
        get_ts_type_children(constraint, children, parent_id, message.clone());
    }

    if let Some(default) = &type_param.default {
        get_ts_type_children(default, children, parent_id, message.clone());
    }
}

fn get_lit_children(
    lit: &'static Lit,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    match lit {
        Lit::Str(raw) => {
            push_child(NodeKind::Str(raw), children, parent_id, message);
        }
        Lit::Bool(raw) => {
            push_child(NodeKind::Bool(raw), children, parent_id, message);
        }
        Lit::Null(raw) => {
            push_child(NodeKind::Null(raw), children, parent_id, message);
        }
        Lit::Num(raw) => {
            push_child(NodeKind::Num(raw), children, parent_id, message);
        }
        Lit::BigInt(raw) => {
            push_child(NodeKind::BigInt(raw), children, parent_id, message);
        }
        Lit::Regex(raw) => {
            push_child(NodeKind::Regex(raw), children, parent_id, message);
        }
        _ => {}
    }
}

fn get_ts_type_assertion_expr_children(
    expr: &'static TsTypeAssertion,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    get_expr_children(&*expr.expr, children, parent_id, message.clone());
    get_ts_type_children(&*expr.type_ann, children, parent_id, message);
}

fn get_ts_entity_name_children(
    entity_name: &'static TsEntityName,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    match entity_name {
        TsEntityName::Ident(raw) => {
            push_child(NodeKind::Ident(raw), children, parent_id, message);
        }
        TsEntityName::TsQualifiedName(raw) => {
            push_child(NodeKind::TsQualifiedName(raw), children, parent_id, message);
        }
    }
}

fn get_ts_type_ref_children(
    type_ref: &'static TsTypeRef,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    push_child(
        NodeKind::TsEntityName(&type_ref.type_name),
        children,
        parent_id,
        message.clone(),
    );

    if let Some(type_params) = &type_ref.type_params {
        push_child(
            NodeKind::TsTypeParamInstantiation(type_params),
            children,
            parent_id,
            message,
        );
    }
}

fn get_ts_type_children(
    ts_type: &'static TsType,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    match ts_type {
        TsType::TsKeywordType(raw) => {
            push_child(NodeKind::TsKeywordType(raw), children, parent_id, message);
        }
        TsType::TsThisType(raw) => {
            push_child(NodeKind::TsThisType(raw), children, parent_id, message);
        }
        TsType::TsFnOrConstructorType(raw) => {
            push_child(NodeKind::TsFnOrConstructorType(raw), children, parent_id, message);
        }
        TsType::TsTypeRef(raw) => {
            push_child(NodeKind::TsTypeRef(raw), children, parent_id, message);
        }
        TsType::TsTypeQuery(raw) => {
            push_child(NodeKind::TsTypeQuery(raw), children, parent_id, message);
        }
        TsType::TsTypeLit(raw) => {
            push_child(NodeKind::TsTypeLit(raw), children, parent_id, message);
        }
        TsType::TsArrayType(raw) => {
            push_child(NodeKind::TsArrayType(raw), children, parent_id, message);
        }
        TsType::TsTupleType(raw) => {
            push_child(NodeKind::TsTupleType(raw), children, parent_id, message);
        }
        TsType::TsOptionalType(raw) => {
            push_child(NodeKind::TsOptionalType(raw), children, parent_id, message);
        }
        TsType::TsRestType(raw) => {
            push_child(NodeKind::TsRestType(raw), children, parent_id, message);
        }
        TsType::TsUnionOrIntersectionType(raw) => {
            push_child(NodeKind::TsUnionOrIntersectionType(raw), children, parent_id, message);
        }
        TsType::TsConditionalType(raw) => {
            push_child(NodeKind::TsConditionalType(raw), children, parent_id, message);
        }
        TsType::TsInferType(raw) => {
            push_child(NodeKind::TsInferType(raw), children, parent_id, message);
        }
        TsType::TsParenthesizedType(raw) => {
            push_child(NodeKind::TsParenthesizedType(raw), children, parent_id, message);
        }
        TsType::TsTypeOperator(raw) => {
            push_child(NodeKind::TsTypeOperator(raw), children, parent_id, message);
        }
        TsType::TsIndexedAccessType(raw) => {
            push_child(NodeKind::TsIndexedAccessType(raw), children, parent_id, message);
        }
        TsType::TsMappedType(raw) => {
            push_child(NodeKind::TsMappedType(raw), children, parent_id, message);
        }
        TsType::TsLitType(raw) => {
            push_child(NodeKind::TsLitType(raw), children, parent_id, message);
        }
        TsType::TsTypePredicate(raw) => {
            push_child(NodeKind::TsTypePredicate(raw), children, parent_id, message);
        }
        TsType::TsImportType(raw) => {
            push_child(NodeKind::TsImportType(raw), children, parent_id, message);
        }
    }
}

fn get_new_expr_children(
    expr: &'static NewExpr,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    get_expr_children(&expr.callee, children, parent_id, message);
}

fn get_ts_as_expr_children(
    expr: &'static TsAsExpr,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    match &*expr.type_ann {
        TsType::TsKeywordType(raw) => {
            push_child(NodeKind::TsKeywordType(raw), children, parent_id, message.clone());
        }
        TsType::TsThisType(raw) => {
            push_child(NodeKind::TsThisType(raw), children, parent_id, message.clone());
        }
        TsType::TsFnOrConstructorType(raw) => {
            push_child(
                NodeKind::TsFnOrConstructorType(raw),
                children,
                parent_id,
                message.clone(),
            );
        }
        TsType::TsTypeRef(raw) => {
            push_child(NodeKind::TsTypeRef(raw), children, parent_id, message.clone());
        }
        TsType::TsTypeQuery(raw) => {
            push_child(NodeKind::TsTypeQuery(raw), children, parent_id, message.clone());
        }
        TsType::TsTypeLit(raw) => {
            push_child(NodeKind::TsTypeLit(raw), children, parent_id, message.clone());
        }
        TsType::TsArrayType(raw) => {
            push_child(NodeKind::TsArrayType(raw), children, parent_id, message.clone());
        }
        TsType::TsTupleType(raw) => {
            push_child(NodeKind::TsTupleType(raw), children, parent_id, message.clone());
        }
        TsType::TsOptionalType(raw) => {
            push_child(NodeKind::TsOptionalType(raw), children, parent_id, message.clone());
        }
        TsType::TsRestType(raw) => {
            push_child(NodeKind::TsRestType(raw), children, parent_id, message.clone());
        }
        TsType::TsUnionOrIntersectionType(raw) => {
            push_child(
                NodeKind::TsUnionOrIntersectionType(raw),
                children,
                parent_id,
                message.clone(),
            );
        }
        TsType::TsConditionalType(raw) => {
            push_child(NodeKind::TsConditionalType(raw), children, parent_id, message.clone());
        }
        TsType::TsInferType(raw) => {
            push_child(NodeKind::TsInferType(raw), children, parent_id, message.clone());
        }
        TsType::TsParenthesizedType(raw) => {
            push_child(NodeKind::TsParenthesizedType(raw), children, parent_id, message.clone());
        }
        TsType::TsTypeOperator(raw) => {
            push_child(NodeKind::TsTypeOperator(raw), children, parent_id, message.clone());
        }
        TsType::TsIndexedAccessType(raw) => {
            push_child(NodeKind::TsIndexedAccessType(raw), children, parent_id, message.clone());
        }
        TsType::TsMappedType(raw) => {
            push_child(NodeKind::TsMappedType(raw), children, parent_id, message.clone());
        }
        TsType::TsLitType(raw) => {
            push_child(NodeKind::TsLitType(raw), children, parent_id, message.clone());
        }
        TsType::TsTypePredicate(raw) => {
            push_child(NodeKind::TsTypePredicate(raw), children, parent_id, message.clone());
        }
        TsType::TsImportType(raw) => {
            push_child(NodeKind::TsImportType(raw), children, parent_id, message.clone());
        }
    }
    get_expr_children(&*expr.expr, children, parent_id, message);
}

fn get_var_declarator_children(
    var_declarator: &'static VarDeclarator,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    if let Some(init) = &var_declarator.init {
        push_child(NodeKind::Expr(init), children, parent_id, message);
    }
}

fn get_member_prop_children(
    prop: &'static MemberProp,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    match prop {
        MemberProp::Ident(raw) => {
            push_child(NodeKind::Ident(raw), children, parent_id, message);
        }
        _ => {}
    }
}

fn get_member_expr_children(
    expr: &'static MemberExpr,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    push_child(NodeKind::Expr(&expr.obj), children, parent_id, message.clone());

    push_child(NodeKind::MemberProp(&expr.prop), children, parent_id, message);
}

fn get_callee_children(
    callee: &'static Callee,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    match callee {
        Callee::Expr(expr) => get_expr_children(expr, children, parent_id, message),
        _ => {}
    }
}

fn get_decl_children(
    decl: &'static Decl,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    match decl {
        Decl::Class(raw) => {
            push_child(NodeKind::ClassDecl(raw), children, parent_id, message);
        }
        Decl::Fn(raw) => {
            push_child(NodeKind::FnDecl(raw), children, parent_id, message);
        }
        Decl::TsEnum(raw) => {
            push_child(NodeKind::TsEnumDecl(raw), children, parent_id, message);
        }
        Decl::TsInterface(raw) => {
            push_child(NodeKind::TsInterfaceDecl(raw), children, parent_id, message);
        }
        Decl::TsModule(raw) => {
            push_child(NodeKind::TsModuleDecl(raw), children, parent_id, message);
        }
        Decl::Var(raw) => {
            push_child(NodeKind::VarDecl(raw), children, parent_id, message);
        }
        Decl::TsTypeAlias(raw) => {
            push_child(NodeKind::TsTypeAliasDecl(raw), children, parent_id, message);
        }
        _ => {}
    }
}

fn get_ts_module_decl_children(
    decl: &'static TsModuleDecl,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    match &decl.body {
        Some(TsNamespaceBody::TsModuleBlock(raw)) => get_ts_module_block_children(raw, children, parent_id, message),
        Some(TsNamespaceBody::TsNamespaceDecl(raw)) => {
            get_ts_namespace_decl_children(raw, children, parent_id, message)
        }
        _ => {}
    }
}

fn get_ts_namespace_decl_children(
    decl: &'static TsNamespaceDecl,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    match &*decl.body {
        TsNamespaceBody::TsModuleBlock(raw) => get_ts_module_block_children(raw, children, parent_id, message),
        TsNamespaceBody::TsNamespaceDecl(raw) => get_ts_namespace_decl_children(raw, children, parent_id, message),
    }
}

fn get_ts_module_block_children(
    block: &'static TsModuleBlock,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    block.body.iter().for_each(|item| {
        push_child(NodeKind::ModuleItem(item), children, parent_id, message.clone());
    });
}

fn get_class_decl_children(
    decl: &'static ClassDecl,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    decl.class.body.iter().for_each(|member| {
        push_child(NodeKind::ClassMember(member), children, parent_id, message.clone());
    });
}
fn get_fn_decl_children(
    decl: &'static FnDecl,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    decl.function.body.iter().for_each(|member| {
        push_child(NodeKind::BlockStmt(member), children, parent_id, message.clone());
    });
}
fn get_ts_enum_decl_children(
    decl: &'static TsEnumDecl,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    decl.members.iter().for_each(|member| {
        push_child(NodeKind::TsEnumMember(member), children, parent_id, message.clone());
    });
}
fn get_ts_interface_decl_children(
    decl: &'static TsInterfaceDecl,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    if let Some(type_params) = &decl.type_params {
        for param in &type_params.params {
            push_child(NodeKind::TsTypeParam(&param), children, parent_id, message.clone());
        }
    }

    decl.body.body.iter().for_each(|member| {
        push_child(NodeKind::TsTypeElement(member), children, parent_id, message.clone());
    });
}

fn get_ts_type_alias_declaration(
    decl: &'static TsTypeAliasDecl,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    push_child(NodeKind::TsType(&decl.type_ann), children, parent_id, message);
}

fn get_module_decl_children(
    decl: &'static ModuleDecl,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    match decl {
        ModuleDecl::Import(raw) => get_import_decl_children(raw, children, parent_id, message),
        ModuleDecl::ExportDecl(raw) => get_export_declartion_children(raw, children, parent_id, message),
        ModuleDecl::ExportNamed(raw) => get_named_export_children(raw, children, parent_id, message),
        ModuleDecl::ExportDefaultDecl(raw) => get_export_default_decl_children(raw, children, parent_id, message),
        ModuleDecl::ExportDefaultExpr(raw) => get_export_default_expr_children(raw, children, parent_id, message),
        _ => {}
    }
}

fn get_named_export_children(
    named_export: &'static NamedExport,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    for specifier in &named_export.specifiers {
        push_child(
            NodeKind::ExportSpecifier(specifier),
            children,
            parent_id,
            message.clone(),
        );
    }
}

fn get_ts_type_element_children(
    type_element: &'static TsTypeElement,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    match type_element {
        TsTypeElement::TsCallSignatureDecl(raw) => {
            get_ts_call_signature_decl_children(raw, children, parent_id, message)
        }
        TsTypeElement::TsConstructSignatureDecl(raw) => {
            get_ts_construct_signature_decl_children(raw, children, parent_id, message)
        }
        TsTypeElement::TsIndexSignature(raw) => get_ts_index_signature_children(raw, children, parent_id, message),
        TsTypeElement::TsMethodSignature(raw) => get_ts_method_signature_children(raw, children, parent_id, message),
        TsTypeElement::TsPropertySignature(raw) => {
            get_ts_property_signature_children(raw, children, parent_id, message)
        }
        TsTypeElement::TsGetterSignature(raw) => get_ts_getter_signature_children(raw, children, parent_id, message),
        _ => {}
    }
}

fn get_ts_call_signature_decl_children(
    thing: &'static TsCallSignatureDecl,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    for param in &thing.params {
        push_child(NodeKind::TsFnParam(param), children, parent_id, message.clone());
    }

    if let Some(type_ann) = &thing.type_ann {
        get_type_annotation_children(type_ann, children, parent_id, message);
    }
}
fn get_ts_construct_signature_decl_children(
    thing: &'static TsConstructSignatureDecl,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    {
        for param in &thing.params {
            push_child(NodeKind::TsFnParam(param), children, parent_id, message.clone());
        }
    }

    if let Some(type_ann) = &thing.type_ann {
        get_type_annotation_children(type_ann, children, parent_id, message);
    }
}
fn get_ts_index_signature_children(
    thing: &'static TsIndexSignature,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    {
        for param in &thing.params {
            push_child(NodeKind::TsFnParam(param), children, parent_id, message.clone());
        }
    }

    if let Some(type_ann) = &thing.type_ann {
        get_type_annotation_children(type_ann, children, parent_id, message);
    }
}
fn get_ts_method_signature_children(
    thing: &'static TsMethodSignature,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    {
        for param in &thing.params {
            push_child(NodeKind::TsFnParam(param), children, parent_id, message.clone());
        }
    }

    if let Some(type_ann) = &thing.type_ann {
        get_type_annotation_children(type_ann, children, parent_id, message);
    }
}
fn get_ts_property_signature_children(
    signature: &'static TsPropertySignature,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    {
        for param in &signature.params {
            push_child(NodeKind::TsFnParam(param), children, parent_id, message.clone());
        }
    }

    if let Some(type_ann) = &signature.type_ann {
        get_type_annotation_children(type_ann, children, parent_id, message);
    }
}
fn get_ts_getter_signature_children(
    thing: &'static TsGetterSignature,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    if let Some(type_ann) = &thing.type_ann {
        get_type_annotation_children(type_ann, children, parent_id, message);
    }
}

fn get_type_lit_children(
    type_lit: &'static TsTypeLit,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    for member in &type_lit.members {
        push_child(NodeKind::TsTypeElement(member), children, parent_id, message.clone());
    }
}

fn get_call_expr_children(
    expr: &'static CallExpr,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    push_child(NodeKind::Callee(&expr.callee), children, parent_id, message.clone());

    expr.args.iter().for_each(|arg| {
        push_child(NodeKind::ExprOrSpread(arg), children, parent_id, message.clone());
    });
}

fn get_expr_children(
    expr: &'static Expr,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    match expr {
        Expr::This(raw) => {
            push_child(NodeKind::ThisExpr(raw), children, parent_id, message);
        }
        Expr::Array(raw) => {
            push_child(NodeKind::ArrayLit(raw), children, parent_id, message);
        }
        Expr::Object(raw) => {
            push_child(NodeKind::ObjectLit(raw), children, parent_id, message);
        }
        Expr::Fn(raw) => {
            push_child(NodeKind::FnExpr(raw), children, parent_id, message);
        }
        Expr::Unary(raw) => {
            push_child(NodeKind::UnaryExpr(raw), children, parent_id, message);
        }
        Expr::Update(raw) => {
            push_child(NodeKind::UpdateExpr(raw), children, parent_id, message);
        }
        Expr::Bin(raw) => {
            push_child(NodeKind::BinExpr(raw), children, parent_id, message);
        }
        Expr::Assign(raw) => {
            push_child(NodeKind::AssignExpr(raw), children, parent_id, message);
        }
        Expr::Member(raw) => {
            push_child(NodeKind::MemberExpr(raw), children, parent_id, message);
        }
        Expr::SuperProp(raw) => {
            push_child(NodeKind::SuperPropExpr(raw), children, parent_id, message);
        }
        Expr::Cond(raw) => {
            push_child(NodeKind::CondExpr(raw), children, parent_id, message);
        }
        Expr::Call(raw) => {
            push_child(NodeKind::CallExpr(raw), children, parent_id, message);
        }
        Expr::New(raw) => {
            push_child(NodeKind::NewExpr(raw), children, parent_id, message);
        }
        Expr::Seq(raw) => {
            push_child(NodeKind::SeqExpr(raw), children, parent_id, message);
        }
        Expr::Ident(raw) => {
            push_child(NodeKind::Ident(raw), children, parent_id, message);
        }
        Expr::Lit(raw) => {
            push_child(NodeKind::Lit(raw), children, parent_id, message);
        }
        Expr::Tpl(raw) => {
            push_child(NodeKind::TemplateLiteral(raw), children, parent_id, message);
        }
        Expr::TaggedTpl(raw) => {
            push_child(NodeKind::TaggedTpl(raw), children, parent_id, message);
        }
        Expr::Arrow(raw) => {
            push_child(NodeKind::ArrowExpr(raw), children, parent_id, message);
        }
        Expr::Class(raw) => {
            push_child(NodeKind::ClassExpr(raw), children, parent_id, message);
        }
        Expr::Yield(raw) => {
            push_child(NodeKind::YieldExpr(raw), children, parent_id, message);
        }
        Expr::MetaProp(raw) => {
            push_child(NodeKind::MetaPropExpr(raw), children, parent_id, message);
        }
        Expr::Await(raw) => {
            push_child(NodeKind::AwaitExpr(raw), children, parent_id, message);
        }
        Expr::Paren(raw) => {
            push_child(NodeKind::ParenExpr(raw), children, parent_id, message);
        }
        Expr::TsTypeAssertion(raw) => {
            push_child(NodeKind::TsTypeAssertionExpr(raw), children, parent_id, message);
        }
        Expr::TsConstAssertion(raw) => {
            push_child(NodeKind::TsConstAssertionExpr(raw), children, parent_id, message);
        }
        Expr::TsNonNull(raw) => {
            push_child(NodeKind::TsNonNullExpr(raw), children, parent_id, message);
        }
        Expr::TsAs(raw) => {
            push_child(NodeKind::TsAsExpr(raw), children, parent_id, message);
        }
        Expr::TsInstantiation(raw) => {
            push_child(NodeKind::TsInstantiationExpr(raw), children, parent_id, message);
        }
        Expr::TsSatisfies(raw) => {
            push_child(NodeKind::TsSatisfiesExpr(raw), children, parent_id, message);
        }
        Expr::PrivateName(raw) => {
            push_child(NodeKind::PrivateNameExpr(raw), children, parent_id, message);
        }
        Expr::OptChain(raw) => {
            push_child(NodeKind::OptChainExpr(raw), children, parent_id, message);
        }
        Expr::Invalid(raw) => {
            push_child(NodeKind::InvalidExpr(raw), children, parent_id, message);
        }
        _ => {}
    }
}

fn get_arrow_expr_children(
    expr: &'static ArrowExpr,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    push_child(
        NodeKind::BlockStmtOrExpr(&*expr.body),
        children,
        parent_id,
        message.clone(),
    );

    expr.params.iter().for_each(|param| {
        push_child(NodeKind::Pat(param), children, parent_id, message.clone());
    });
}

fn get_module_children(
    module: &'static Module,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    for item in &module.body {
        push_child(NodeKind::ModuleItem(item), children, parent_id, message.clone());
    }
}

fn get_export_declartion_children(
    export_decl: &'static ExportDecl,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    match &export_decl.decl {
        Decl::Class(declaration) => {
            push_child(NodeKind::ClassDecl(declaration), children, parent_id, message);
        }
        Decl::Fn(declaration) => {
            push_child(NodeKind::FnDecl(declaration), children, parent_id, message);
        }
        Decl::Var(declaration) => {
            push_child(NodeKind::VarDecl(declaration), children, parent_id, message);
        }
        Decl::TsInterface(declaration) => {
            push_child(NodeKind::TsInterfaceDecl(declaration), children, parent_id, message);
        }
        Decl::TsTypeAlias(declaration) => {
            push_child(NodeKind::TsTypeAliasDecl(declaration), children, parent_id, message);
        }
        Decl::TsEnum(declaration) => {
            push_child(NodeKind::TsEnumDecl(declaration), children, parent_id, message);
        }
        Decl::TsModule(declaration) => {
            push_child(NodeKind::TsModuleDecl(declaration), children, parent_id, message);
        }
        _ => {}
    }
}

fn get_export_default_expr_children(
    expression: &'static ExportDefaultExpr,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    get_expr_children(&expression.expr, children, parent_id, message)
}

fn get_export_default_decl_children(
    export_declaration: &'static ExportDefaultDecl,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    match &export_declaration.decl {
        DefaultDecl::Class(declaration) => {
            push_child(NodeKind::ClassExpr(&declaration), children, parent_id, message);
        }
        DefaultDecl::TsInterfaceDecl(declaration) => {
            push_child(NodeKind::TsInterfaceDecl(&declaration), children, parent_id, message);
        }
        _ => {}
    }
}

fn get_import_decl_children(
    import_declaration: &'static ImportDecl,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    for specifier in &import_declaration.specifiers {
        push_child(
            NodeKind::ImportSpecifier(&specifier),
            children,
            parent_id,
            message.clone(),
        );
    }
}

fn get_module_item_children(
    module_item: &'static ModuleItem,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    match module_item {
        ModuleItem::ModuleDecl(declaration) => match declaration {
            ModuleDecl::Import(declaration) => {
                push_child(NodeKind::ImportDecl(&declaration), children, parent_id, message);
            }
            ModuleDecl::ExportDecl(declaration) => {
                push_child(NodeKind::ExportDecl(&declaration), children, parent_id, message);
            }
            ModuleDecl::ExportNamed(declaration) => {
                push_child(NodeKind::NamedExport(&declaration), children, parent_id, message);
            }
            ModuleDecl::ExportDefaultDecl(declaration) => {
                push_child(NodeKind::ExportDefaultDecl(&declaration), children, parent_id, message);
            }
            ModuleDecl::ExportDefaultExpr(declaration) => {
                push_child(NodeKind::ExportDefaultExpr(&declaration), children, parent_id, message);
            }
            ModuleDecl::ExportAll(declaration) => {
                push_child(NodeKind::ExportAll(&declaration), children, parent_id, message);
            }
            ModuleDecl::TsImportEquals(declaration) => {
                push_child(NodeKind::TsImportEquals(&declaration), children, parent_id, message);
            }
            ModuleDecl::TsExportAssignment(declaration) => {
                push_child(NodeKind::TsExportAssignment(&declaration), children, parent_id, message);
            }
            ModuleDecl::TsNamespaceExport(declaration) => {
                push_child(NodeKind::TsNamespaceExport(&declaration), children, parent_id, message);
            }
        },
        ModuleItem::Stmt(statement) => get_statement_children(statement, children, parent_id, message),
    }
}

fn get_pat_children(
    pat: &'static Pat,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    match pat {
        Pat::Ident(ident) => ident.type_ann.iter().for_each(|raw| {
            push_child(NodeKind::TsTypeAnnotation(raw), children, parent_id, message.clone());
        }),
        Pat::Array(raw) => {
            push_child(NodeKind::ArrayPat(raw), children, parent_id, message);
        }
        Pat::Rest(raw) => {
            push_child(NodeKind::RestPat(raw), children, parent_id, message);
        }
        Pat::Object(raw) => {
            push_child(NodeKind::ObjectPat(raw), children, parent_id, message);
        }
        Pat::Assign(raw) => {
            push_child(NodeKind::AssignPat(raw), children, parent_id, message);
        }
        Pat::Expr(raw) => {
            push_child(NodeKind::Expr(raw), children, parent_id, message);
        }
        _ => {}
    }
}

fn get_statement_children(
    statement: &'static Stmt,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    match statement {
        Stmt::Block(block_stmt) => {
            push_child(NodeKind::BlockStmt(block_stmt), children, parent_id, message);
        }
        Stmt::Empty(empty_stmt) => {
            push_child(NodeKind::EmptyStmt(empty_stmt), children, parent_id, message);
        }
        Stmt::Debugger(debugger_stmt) => {
            push_child(NodeKind::DebuggerStmt(debugger_stmt), children, parent_id, message);
        }
        Stmt::With(with_stmt) => {
            push_child(NodeKind::WithStmt(with_stmt), children, parent_id, message);
        }
        Stmt::Return(return_stmt) => {
            push_child(NodeKind::ReturnStmt(return_stmt), children, parent_id, message);
        }
        Stmt::Labeled(labeled_stmt) => {
            push_child(NodeKind::LabeledStmt(labeled_stmt), children, parent_id, message);
        }
        Stmt::Break(break_stmt) => {
            push_child(NodeKind::BreakStmt(break_stmt), children, parent_id, message);
        }
        Stmt::Continue(continue_stmt) => {
            push_child(NodeKind::ContinueStmt(continue_stmt), children, parent_id, message);
        }
        Stmt::If(if_stmt) => {
            push_child(NodeKind::IfStmt(if_stmt), children, parent_id, message);
        }
        Stmt::Switch(switch_stmt) => {
            push_child(NodeKind::SwitchStmt(switch_stmt), children, parent_id, message);
        }
        Stmt::Throw(throw_stmt) => {
            push_child(NodeKind::ThrowStmt(throw_stmt), children, parent_id, message);
        }
        Stmt::Try(try_stmt) => {
            push_child(NodeKind::TryStmt(try_stmt), children, parent_id, message);
        }
        Stmt::While(while_stmt) => {
            push_child(NodeKind::WhileStmt(while_stmt), children, parent_id, message);
        }
        Stmt::DoWhile(do_while_stmt) => {
            push_child(NodeKind::DoWhileStmt(do_while_stmt), children, parent_id, message);
        }
        Stmt::For(for_stmt) => {
            push_child(NodeKind::ForStmt(for_stmt), children, parent_id, message);
        }
        Stmt::ForIn(for_in_stmt) => {
            push_child(NodeKind::ForInStmt(for_in_stmt), children, parent_id, message);
        }
        Stmt::ForOf(for_of_stmt) => {
            push_child(NodeKind::ForOfStmt(for_of_stmt), children, parent_id, message);
        }
        Stmt::Decl(decl_stmt) => {
            push_child(NodeKind::Decl(decl_stmt), children, parent_id, message);
        }
        Stmt::Expr(expr_stmt) => {
            push_child(NodeKind::ExprStmt(expr_stmt), children, parent_id, message);
        }
    }
}

fn get_block_statement_children(
    block_stmnt: &'static BlockStmt,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    block_stmnt
        .stmts
        .iter()
        .for_each(|statement| get_statement_children(statement, children, parent_id, message.clone()))
}

fn get_type_annotation_children(
    type_annotation: &'static TsTypeAnn,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    match &*type_annotation.type_ann {
        TsType::TsKeywordType(ts_keyword_type) => {
            push_child(NodeKind::TsKeywordType(&ts_keyword_type), children, parent_id, message);
        }
        TsType::TsThisType(ts_this_type) => {
            push_child(NodeKind::TsThisType(&ts_this_type), children, parent_id, message);
        }
        TsType::TsFnOrConstructorType(ts_fn_or_constructor_type) => {
            push_child(
                NodeKind::TsFnOrConstructorType(&ts_fn_or_constructor_type),
                children,
                parent_id,
                message,
            );
        }
        TsType::TsTypeRef(ts_type_ref) => {
            push_child(NodeKind::TsTypeRef(&ts_type_ref), children, parent_id, message);
        }
        TsType::TsTypeQuery(ts_type_query) => {
            push_child(NodeKind::TsTypeQuery(&ts_type_query), children, parent_id, message);
        }
        TsType::TsTypeLit(ts_type_lit) => {
            push_child(NodeKind::TsTypeLit(&ts_type_lit), children, parent_id, message);
        }
        TsType::TsArrayType(ts_array_type) => {
            push_child(NodeKind::TsArrayType(&ts_array_type), children, parent_id, message);
        }
        TsType::TsTupleType(ts_tuple_type) => {
            push_child(NodeKind::TsTupleType(&ts_tuple_type), children, parent_id, message);
        }
        TsType::TsOptionalType(ts_optional_type) => {
            push_child(
                NodeKind::TsOptionalType(&ts_optional_type),
                children,
                parent_id,
                message,
            );
        }
        TsType::TsRestType(ts_rest_type) => {
            push_child(NodeKind::TsRestType(&ts_rest_type), children, parent_id, message);
        }
        TsType::TsUnionOrIntersectionType(ts_union_or_intersection_type) => {
            push_child(
                NodeKind::TsUnionOrIntersectionType(&ts_union_or_intersection_type),
                children,
                parent_id,
                message,
            );
        }
        TsType::TsConditionalType(ts_conditional_type) => {
            push_child(
                NodeKind::TsConditionalType(&ts_conditional_type),
                children,
                parent_id,
                message,
            );
        }
        TsType::TsInferType(ts_infer_type) => {
            push_child(NodeKind::TsInferType(&ts_infer_type), children, parent_id, message);
        }
        TsType::TsParenthesizedType(ts_parenthesized_type) => {
            push_child(
                NodeKind::TsParenthesizedType(&ts_parenthesized_type),
                children,
                parent_id,
                message,
            );
        }
        TsType::TsTypeOperator(ts_type_operator) => {
            push_child(
                NodeKind::TsTypeOperator(&ts_type_operator),
                children,
                parent_id,
                message,
            );
        }
        TsType::TsIndexedAccessType(ts_indexed_access_type) => {
            push_child(
                NodeKind::TsIndexedAccessType(&ts_indexed_access_type),
                children,
                parent_id,
                message,
            );
        }
        TsType::TsMappedType(ts_mapped_type) => {
            push_child(NodeKind::TsMappedType(&ts_mapped_type), children, parent_id, message);
        }
        TsType::TsLitType(ts_lit_type) => {
            push_child(NodeKind::TsLitType(&ts_lit_type), children, parent_id, message);
        }
        TsType::TsTypePredicate(ts_type_predicate) => {
            push_child(
                NodeKind::TsTypePredicate(&ts_type_predicate),
                children,
                parent_id,
                message,
            );
        }
        TsType::TsImportType(ts_import_type) => {
            push_child(NodeKind::TsImportType(&ts_import_type), children, parent_id, message);
        }
    }
}

fn get_var_decl_children(
    variable_declaration: &'static VarDecl,
    children: &mut Vec<Arc<Node<'static>>>,
    parent_id: u8,
    message: Sender<Message>,
) {
    variable_declaration.decls.iter().for_each(|decl| {
        push_child(NodeKind::VarDeclarator(decl), children, parent_id, message.clone());
    })
}
