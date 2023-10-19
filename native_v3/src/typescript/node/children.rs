use std::{
    cell::RefCell,
    collections::BTreeMap,
    mem::transmute,
    sync::{Arc, RwLock, Weak},
};

use swc_ecma_ast::*;

use crate::messaging::MessageBus;

use super::{Node, NodeKind};

impl Node<'static> {
    pub fn children(&self, parent: Weak<Self>) -> Vec<Arc<Node<'static>>> {
        let mut children = self.children.write().unwrap();
        if children.is_some() {
            children.as_ref().unwrap().iter().map(|c| c.clone()).collect()
        } else {
            let mut children = children.get_or_insert(vec![]);
            match self.kind {
                NodeKind::AwaitExpr(raw_await) => get_await_expr_children(raw_await, &mut children, parent),
                NodeKind::ArrowExpr(arrow) => get_arrow_expr_children(arrow, &mut children, parent),
                NodeKind::BlockStmt(raw) => get_block_statement_children(raw, &mut children, parent),
                NodeKind::BlockStmtOrExpr(temp) => match temp {
                    BlockStmtOrExpr::BlockStmt(raw) => get_block_statement_children(raw, &mut children, parent),
                    BlockStmtOrExpr::Expr(raw) => get_expr_children(raw, &mut children, parent),
                },
                NodeKind::Callee(raw) => get_callee_children(raw, &mut children, parent),
                NodeKind::CallExpr(raw) => get_call_expr_children(raw, &mut children, parent),
                NodeKind::Decl(raw) => get_decl_children(raw, &mut children, parent),
                NodeKind::ExportDecl(raw) => get_export_declartion_children(raw, &mut children, parent),
                NodeKind::ExportDefaultDecl(raw) => get_export_default_decl_children(raw, &mut children, parent),
                NodeKind::ExportDefaultExpr(raw) => get_export_default_expr_children(raw, &mut children, parent),
                NodeKind::Expr(raw) => get_expr_children(raw, &mut children, parent),
                NodeKind::ExprOrSpread(raw) => get_expr_children(&*raw.expr, &mut children, parent),
                NodeKind::ExprStmt(raw) => get_expr_children(&*raw.expr, &mut children, parent),
                NodeKind::IfStmt(raw) => get_if_statement_children(raw, &mut children, parent),
                NodeKind::ImportDecl(raw) => get_import_decl_children(raw, &mut children, parent),
                NodeKind::Lit(raw) => get_lit_children(raw, &mut children, parent),
                NodeKind::MemberExpr(raw) => get_member_expr_children(raw, &mut children, parent),
                NodeKind::MemberProp(raw) => get_member_prop_children(raw, &mut children, parent),
                NodeKind::Module(ref module) => get_module_children(
                    unsafe { transmute::<&Module, &'static Module>(module) },
                    &mut children,
                    parent,
                ),
                NodeKind::ModuleItem(raw) => get_module_item_children(raw, &mut children, parent),
                NodeKind::NewExpr(raw) => get_new_expr_children(raw, &mut children, parent),
                NodeKind::Pat(raw) => get_pat_children(raw, &mut children, parent),
                NodeKind::ReturnStmt(raw) => get_return_statement_children(raw, &mut children, parent),
                NodeKind::TryStmt(raw) => get_try_statement_children(raw, &mut children, parent),
                NodeKind::TsAsExpr(raw) => get_ts_as_expr_children(raw, &mut children, parent),
                NodeKind::TsEntityName(raw) => get_ts_entity_name_children(raw, &mut children, parent),
                NodeKind::TsInterfaceDecl(raw) => get_ts_interface_decl_children(raw, &mut children, parent),
                NodeKind::TsIntersectionType(raw) => get_ts_intersection_type_children(raw, &mut children, parent),
                NodeKind::TsLitType(raw) => get_ts_lit_type_chilren(raw, &mut children, parent),
                NodeKind::TsModuleDecl(raw) => get_ts_module_decl_children(raw, &mut children, parent),
                NodeKind::TsPropertySignature(raw) => get_ts_property_signature_children(raw, &mut children, parent),
                NodeKind::TsType(raw) => get_ts_type_children(raw, &mut children, parent),
                NodeKind::TsTypeAliasDecl(raw) => get_ts_type_alias_declaration(raw, &mut children, parent),
                NodeKind::TsTypeAnnotation(raw) => get_type_annotation_children(raw, &mut children, parent),
                NodeKind::TsTypeAssertionExpr(raw) => get_ts_type_assertion_expr_children(raw, &mut children, parent),
                NodeKind::TsTypeElement(raw) => get_ts_type_element_children(raw, &mut children, parent),
                NodeKind::TsTypeLit(raw) => get_type_lit_children(raw, &mut children, parent),
                NodeKind::TsTypeParam(raw) => get_ts_type_param(raw, &mut children, parent),
                NodeKind::TsTypeParamInstantiation(raw) => {
                    get_ts_type_param_instantiation_children(raw, &mut children, parent)
                }
                NodeKind::TsUnionType(raw) => get_ts_union_type_children(raw, &mut children, parent),
                NodeKind::TsUnionOrIntersectionType(raw) => {
                    get_ts_union_or_intersection_children(raw, &mut children, parent)
                }
                NodeKind::TsTypeRef(raw) => get_ts_type_ref_children(raw, &mut children, parent),
                NodeKind::VarDecl(raw) => get_var_decl_children(raw, &mut children, parent),
                NodeKind::VarDeclarator(raw) => get_var_declarator_children(raw, &mut children, parent),
                _ => {}
            }
            children.iter().cloned().collect()
        }
    }
}

pub(super) fn push_child(kind: NodeKind<'static>, children: &mut Vec<Arc<Node<'static>>>, parent: Weak<Node<'static>>) {
    let child_node = Arc::new(Node::new(kind, Some(parent)));
    children.push(child_node);
}

pub(super) fn get_return_statement_children(
    raw: &'static ReturnStmt,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    if let Some(arg) = &raw.arg {
        get_expr_children(arg, children, parent);
    }
}

pub(super) fn get_if_statement_children(
    raw: &'static IfStmt,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    get_expr_children(&*raw.test, children, parent);
    get_statement_children(&*raw.cons, children, parent);
    if let Some(alt) = &raw.alt {
        get_statement_children(alt, children, parent);
    }
}

pub(super) fn get_try_statement_children(
    raw: &'static TryStmt,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    get_block_statement_children(&raw.block, children, parent);

    if let Some(catch) = &raw.handler {
        get_catch_clause_children(catch, children, parent);
    }

    if let Some(finalizer) = &raw.finalizer {
        get_block_statement_children(finalizer, children, parent);
    }
}

pub(super) fn get_catch_clause_children(
    raw: &'static CatchClause,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    get_block_statement_children(&raw.body, children, parent);
}

pub(super) fn get_ts_lit_type_chilren(
    raw: &'static TsLitType,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    get_ts_lit_children(&raw.lit, children, parent);
}

pub(super) fn get_ts_lit_children(
    raw: &'static TsLit,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    match raw {
        TsLit::Number(raw) => {
            push_child(NodeKind::Num(raw), children, parent);
        }
        TsLit::Str(raw) => {
            push_child(NodeKind::Str(raw), children, parent);
        }
        TsLit::Tpl(raw) => {
            push_child(NodeKind::TsTplLit(raw), children, parent);
        }
        TsLit::Bool(raw) => {
            push_child(NodeKind::Bool(raw), children, parent);
        }
        TsLit::BigInt(raw) => {
            push_child(NodeKind::BigInt(raw), children, parent);
        }
    }
}

pub(super) fn get_ts_union_type_children(
    raw: &'static TsUnionType,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    for type_ann in &raw.types {
        get_ts_type_children(type_ann, children, parent);
    }
}

pub(super) fn get_ts_intersection_type_children(
    raw: &'static TsIntersectionType,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    for type_ann in &raw.types {
        get_ts_type_children(type_ann, children, parent);
    }
}

pub(super) fn get_ts_union_or_intersection_children(
    raw: &'static TsUnionOrIntersectionType,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    match raw {
        TsUnionOrIntersectionType::TsUnionType(raw_union) => {
            push_child(NodeKind::TsUnionType(raw_union), children, parent);
        }
        TsUnionOrIntersectionType::TsIntersectionType(raw_intersection) => {
            push_child(NodeKind::TsIntersectionType(raw_intersection), children, parent);
        }
    }
}

pub(super) fn get_await_expr_children(
    raw_await: &'static AwaitExpr,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    get_expr_children(&*raw_await.arg, children, parent);
}

pub(super) fn get_ts_type_param_instantiation_children(
    type_params: &'static TsTypeParamInstantiation,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    for param in &type_params.params {
        get_ts_type_children(param, children, parent);
    }
}

pub(super) fn get_ts_type_param(
    type_param: &'static TsTypeParam,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    if let Some(constraint) = &type_param.constraint {
        get_ts_type_children(constraint, children, parent);
    }

    if let Some(default) = &type_param.default {
        get_ts_type_children(default, children, parent);
    }
}

pub(super) fn get_lit_children(lit: &'static Lit, children: &mut Vec<Arc<Node<'static>>>, parent: Weak<Node<'static>>) {
    match lit {
        Lit::Str(raw) => {
            push_child(NodeKind::Str(raw), children, parent);
        }
        Lit::Bool(raw) => {
            push_child(NodeKind::Bool(raw), children, parent);
        }
        Lit::Null(raw) => {
            push_child(NodeKind::Null(raw), children, parent);
        }
        Lit::Num(raw) => {
            push_child(NodeKind::Num(raw), children, parent);
        }
        Lit::BigInt(raw) => {
            push_child(NodeKind::BigInt(raw), children, parent);
        }
        Lit::Regex(raw) => {
            push_child(NodeKind::Regex(raw), children, parent);
        }
        _ => {}
    }
}

pub(super) fn get_ts_type_assertion_expr_children(
    expr: &'static TsTypeAssertion,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    get_expr_children(&*expr.expr, children, parent);
    get_ts_type_children(&*expr.type_ann, children, parent);
}

pub(super) fn get_ts_entity_name_children(
    entity_name: &'static TsEntityName,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    match entity_name {
        TsEntityName::Ident(raw) => {
            push_child(NodeKind::Ident(raw), children, parent);
        }
        TsEntityName::TsQualifiedName(raw) => {
            push_child(NodeKind::TsQualifiedName(raw), children, parent);
        }
    }
}

pub(super) fn get_ts_type_ref_children(
    type_ref: &'static TsTypeRef,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    push_child(NodeKind::TsEntityName(&type_ref.type_name), children, parent);

    if let Some(type_params) = &type_ref.type_params {
        push_child(NodeKind::TsTypeParamInstantiation(type_params), children, parent);
    }
}

pub(super) fn get_ts_type_children(
    ts_type: &'static TsType,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    match ts_type {
        TsType::TsKeywordType(raw) => {
            push_child(NodeKind::TsKeywordType(raw), children, parent);
        }
        TsType::TsThisType(raw) => {
            push_child(NodeKind::TsThisType(raw), children, parent);
        }
        TsType::TsFnOrConstructorType(raw) => {
            push_child(NodeKind::TsFnOrConstructorType(raw), children, parent);
        }
        TsType::TsTypeRef(raw) => {
            push_child(NodeKind::TsTypeRef(raw), children, parent);
        }
        TsType::TsTypeQuery(raw) => {
            push_child(NodeKind::TsTypeQuery(raw), children, parent);
        }
        TsType::TsTypeLit(raw) => {
            push_child(NodeKind::TsTypeLit(raw), children, parent);
        }
        TsType::TsArrayType(raw) => {
            push_child(NodeKind::TsArrayType(raw), children, parent);
        }
        TsType::TsTupleType(raw) => {
            push_child(NodeKind::TsTupleType(raw), children, parent);
        }
        TsType::TsOptionalType(raw) => {
            push_child(NodeKind::TsOptionalType(raw), children, parent);
        }
        TsType::TsRestType(raw) => {
            push_child(NodeKind::TsRestType(raw), children, parent);
        }
        TsType::TsUnionOrIntersectionType(raw) => {
            push_child(NodeKind::TsUnionOrIntersectionType(raw), children, parent);
        }
        TsType::TsConditionalType(raw) => {
            push_child(NodeKind::TsConditionalType(raw), children, parent);
        }
        TsType::TsInferType(raw) => {
            push_child(NodeKind::TsInferType(raw), children, parent);
        }
        TsType::TsParenthesizedType(raw) => {
            push_child(NodeKind::TsParenthesizedType(raw), children, parent);
        }
        TsType::TsTypeOperator(raw) => {
            push_child(NodeKind::TsTypeOperator(raw), children, parent);
        }
        TsType::TsIndexedAccessType(raw) => {
            push_child(NodeKind::TsIndexedAccessType(raw), children, parent);
        }
        TsType::TsMappedType(raw) => {
            push_child(NodeKind::TsMappedType(raw), children, parent);
        }
        TsType::TsLitType(raw) => {
            push_child(NodeKind::TsLitType(raw), children, parent);
        }
        TsType::TsTypePredicate(raw) => {
            push_child(NodeKind::TsTypePredicate(raw), children, parent);
        }
        TsType::TsImportType(raw) => {
            push_child(NodeKind::TsImportType(raw), children, parent);
        }
    }
}

pub(super) fn get_new_expr_children(
    expr: &'static NewExpr,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    get_expr_children(&expr.callee, children, parent);
}

pub(super) fn get_ts_as_expr_children(
    expr: &'static TsAsExpr,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    match &*expr.type_ann {
        TsType::TsKeywordType(raw) => {
            push_child(NodeKind::TsKeywordType(raw), children, parent);
        }
        TsType::TsThisType(raw) => {
            push_child(NodeKind::TsThisType(raw), children, parent);
        }
        TsType::TsFnOrConstructorType(raw) => {
            push_child(NodeKind::TsFnOrConstructorType(raw), children, parent);
        }
        TsType::TsTypeRef(raw) => {
            push_child(NodeKind::TsTypeRef(raw), children, parent);
        }
        TsType::TsTypeQuery(raw) => {
            push_child(NodeKind::TsTypeQuery(raw), children, parent);
        }
        TsType::TsTypeLit(raw) => {
            push_child(NodeKind::TsTypeLit(raw), children, parent);
        }
        TsType::TsArrayType(raw) => {
            push_child(NodeKind::TsArrayType(raw), children, parent);
        }
        TsType::TsTupleType(raw) => {
            push_child(NodeKind::TsTupleType(raw), children, parent);
        }
        TsType::TsOptionalType(raw) => {
            push_child(NodeKind::TsOptionalType(raw), children, parent);
        }
        TsType::TsRestType(raw) => {
            push_child(NodeKind::TsRestType(raw), children, parent);
        }
        TsType::TsUnionOrIntersectionType(raw) => {
            push_child(NodeKind::TsUnionOrIntersectionType(raw), children, parent);
        }
        TsType::TsConditionalType(raw) => {
            push_child(NodeKind::TsConditionalType(raw), children, parent);
        }
        TsType::TsInferType(raw) => {
            push_child(NodeKind::TsInferType(raw), children, parent);
        }
        TsType::TsParenthesizedType(raw) => {
            push_child(NodeKind::TsParenthesizedType(raw), children, parent);
        }
        TsType::TsTypeOperator(raw) => {
            push_child(NodeKind::TsTypeOperator(raw), children, parent);
        }
        TsType::TsIndexedAccessType(raw) => {
            push_child(NodeKind::TsIndexedAccessType(raw), children, parent);
        }
        TsType::TsMappedType(raw) => {
            push_child(NodeKind::TsMappedType(raw), children, parent);
        }
        TsType::TsLitType(raw) => {
            push_child(NodeKind::TsLitType(raw), children, parent);
        }
        TsType::TsTypePredicate(raw) => {
            push_child(NodeKind::TsTypePredicate(raw), children, parent);
        }
        TsType::TsImportType(raw) => {
            push_child(NodeKind::TsImportType(raw), children, parent);
        }
    }
    get_expr_children(&*expr.expr, children, parent);
}

pub(super) fn get_var_declarator_children(
    var_declarator: &'static VarDeclarator,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    get_pat_children(&var_declarator.name, children, parent);

    if let Some(init) = &var_declarator.init {
        push_child(NodeKind::Expr(init), children, parent);
    }
}

pub(super) fn get_member_prop_children(
    prop: &'static MemberProp,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    match prop {
        MemberProp::Ident(raw) => {
            push_child(NodeKind::Ident(raw), children, parent);
        }
        _ => {}
    }
}

pub(super) fn get_member_expr_children(
    expr: &'static MemberExpr,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    push_child(NodeKind::Expr(&expr.obj), children, parent);

    push_child(NodeKind::MemberProp(&expr.prop), children, parent);
}

pub(super) fn get_callee_children(
    callee: &'static Callee,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    match callee {
        Callee::Expr(expr) => get_expr_children(expr, children, parent),
        _ => {}
    }
}

pub(super) fn get_decl_children(
    decl: &'static Decl,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    match decl {
        Decl::Class(raw) => {
            push_child(NodeKind::ClassDecl(raw), children, parent);
        }
        Decl::Fn(raw) => {
            push_child(NodeKind::FnDecl(raw), children, parent);
        }
        Decl::TsEnum(raw) => {
            push_child(NodeKind::TsEnumDecl(raw), children, parent);
        }
        Decl::TsInterface(raw) => {
            push_child(NodeKind::TsInterfaceDecl(raw), children, parent);
        }
        Decl::TsModule(raw) => {
            push_child(NodeKind::TsModuleDecl(raw), children, parent);
        }
        Decl::Var(raw) => {
            push_child(NodeKind::VarDecl(raw), children, parent);
        }
        Decl::TsTypeAlias(raw) => {
            push_child(NodeKind::TsTypeAliasDecl(raw), children, parent);
        }
        _ => {}
    }
}

pub(super) fn get_ts_module_decl_children(
    decl: &'static TsModuleDecl,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    match &decl.body {
        Some(TsNamespaceBody::TsModuleBlock(raw)) => get_ts_module_block_children(raw, children, parent),
        Some(TsNamespaceBody::TsNamespaceDecl(raw)) => get_ts_namespace_decl_children(raw, children, parent),
        _ => {}
    }
}

pub(super) fn get_ts_namespace_decl_children(
    decl: &'static TsNamespaceDecl,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    match &*decl.body {
        TsNamespaceBody::TsModuleBlock(raw) => get_ts_module_block_children(raw, children, parent),
        TsNamespaceBody::TsNamespaceDecl(raw) => get_ts_namespace_decl_children(raw, children, parent),
    }
}

pub(super) fn get_ts_module_block_children(
    block: &'static TsModuleBlock,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    block.body.iter().for_each(|item| {
        push_child(NodeKind::ModuleItem(item), children, parent);
    });
}

pub(super) fn get_class_decl_children(
    decl: &'static ClassDecl,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    decl.class.body.iter().for_each(|member| {
        push_child(NodeKind::ClassMember(member), children, parent);
    });
}
pub(super) fn get_fn_decl_children(
    decl: &'static FnDecl,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    decl.function.body.iter().for_each(|member| {
        push_child(NodeKind::BlockStmt(member), children, parent);
    });
}
pub(super) fn get_ts_enum_decl_children(
    decl: &'static TsEnumDecl,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    decl.members.iter().for_each(|member| {
        push_child(NodeKind::TsEnumMember(member), children, parent);
    });
}
pub(super) fn get_ts_interface_decl_children(
    decl: &'static TsInterfaceDecl,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    if let Some(type_params) = &decl.type_params {
        for param in &type_params.params {
            push_child(NodeKind::TsTypeParam(&param), children, parent);
        }
    }

    decl.body.body.iter().for_each(|member| {
        push_child(NodeKind::TsTypeElement(member), children, parent);
    });
}

pub(super) fn get_ts_type_alias_declaration(
    decl: &'static TsTypeAliasDecl,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    push_child(NodeKind::TsType(&decl.type_ann), children, parent);
}

pub(super) fn get_module_decl_children(
    decl: &'static ModuleDecl,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    match decl {
        ModuleDecl::Import(raw) => get_import_decl_children(raw, children, parent),
        ModuleDecl::ExportDecl(raw) => get_export_declartion_children(raw, children, parent),
        ModuleDecl::ExportNamed(raw) => get_named_export_children(raw, children, parent),
        ModuleDecl::ExportDefaultDecl(raw) => get_export_default_decl_children(raw, children, parent),
        ModuleDecl::ExportDefaultExpr(raw) => get_export_default_expr_children(raw, children, parent),
        _ => {}
    }
}

pub(super) fn get_named_export_children(
    named_export: &'static NamedExport,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    for specifier in &named_export.specifiers {
        push_child(NodeKind::ExportSpecifier(specifier), children, parent);
    }
}

pub(super) fn get_ts_type_element_children(
    type_element: &'static TsTypeElement,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    match type_element {
        TsTypeElement::TsCallSignatureDecl(raw) => get_ts_call_signature_decl_children(raw, children, parent),
        TsTypeElement::TsConstructSignatureDecl(raw) => get_ts_construct_signature_decl_children(raw, children, parent),
        TsTypeElement::TsIndexSignature(raw) => get_ts_index_signature_children(raw, children, parent),
        TsTypeElement::TsMethodSignature(raw) => get_ts_method_signature_children(raw, children, parent),
        TsTypeElement::TsPropertySignature(raw) => get_ts_property_signature_children(raw, children, parent),
        TsTypeElement::TsGetterSignature(raw) => get_ts_getter_signature_children(raw, children, parent),
        _ => {}
    }
}

pub(super) fn get_ts_call_signature_decl_children(
    thing: &'static TsCallSignatureDecl,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    for param in &thing.params {
        push_child(NodeKind::TsFnParam(param), children, parent);
    }

    if let Some(type_ann) = &thing.type_ann {
        get_type_annotation_children(type_ann, children, parent);
    }
}
pub(super) fn get_ts_construct_signature_decl_children(
    thing: &'static TsConstructSignatureDecl,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    {
        for param in &thing.params {
            push_child(NodeKind::TsFnParam(param), children, parent);
        }
    }

    if let Some(type_ann) = &thing.type_ann {
        get_type_annotation_children(type_ann, children, parent);
    }
}
pub(super) fn get_ts_index_signature_children(
    thing: &'static TsIndexSignature,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    {
        for param in &thing.params {
            push_child(NodeKind::TsFnParam(param), children, parent);
        }
    }

    if let Some(type_ann) = &thing.type_ann {
        get_type_annotation_children(type_ann, children, parent);
    }
}
pub(super) fn get_ts_method_signature_children(
    thing: &'static TsMethodSignature,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    {
        for param in &thing.params {
            push_child(NodeKind::TsFnParam(param), children, parent);
        }
    }

    if let Some(type_ann) = &thing.type_ann {
        get_type_annotation_children(type_ann, children, parent);
    }
}
pub(super) fn get_ts_property_signature_children(
    signature: &'static TsPropertySignature,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    {
        for param in &signature.params {
            push_child(NodeKind::TsFnParam(param), children, parent);
        }
    }

    if let Some(type_ann) = &signature.type_ann {
        get_type_annotation_children(type_ann, children, parent);
    }
}
pub(super) fn get_ts_getter_signature_children(
    thing: &'static TsGetterSignature,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    if let Some(type_ann) = &thing.type_ann {
        get_type_annotation_children(type_ann, children, parent);
    }
}

pub(super) fn get_type_lit_children(
    type_lit: &'static TsTypeLit,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    for member in &type_lit.members {
        push_child(NodeKind::TsTypeElement(member), children, parent);
    }
}

pub(super) fn get_call_expr_children(
    expr: &'static CallExpr,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    push_child(NodeKind::Callee(&expr.callee), children, parent);

    expr.args.iter().for_each(|arg| {
        push_child(NodeKind::ExprOrSpread(arg), children, parent);
    });
}

pub(super) fn get_expr_children(
    expr: &'static Expr,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    match expr {
        Expr::This(raw) => {
            push_child(NodeKind::ThisExpr(raw), children, parent);
        }
        Expr::Array(raw) => {
            push_child(NodeKind::ArrayLit(raw), children, parent);
        }
        Expr::Object(raw) => {
            push_child(NodeKind::ObjectLit(raw), children, parent);
        }
        Expr::Fn(raw) => {
            push_child(NodeKind::FnExpr(raw), children, parent);
        }
        Expr::Unary(raw) => {
            push_child(NodeKind::UnaryExpr(raw), children, parent);
        }
        Expr::Update(raw) => {
            push_child(NodeKind::UpdateExpr(raw), children, parent);
        }
        Expr::Bin(raw) => {
            push_child(NodeKind::BinExpr(raw), children, parent);
        }
        Expr::Assign(raw) => {
            push_child(NodeKind::AssignExpr(raw), children, parent);
        }
        Expr::Member(raw) => {
            push_child(NodeKind::MemberExpr(raw), children, parent);
        }
        Expr::SuperProp(raw) => {
            push_child(NodeKind::SuperPropExpr(raw), children, parent);
        }
        Expr::Cond(raw) => {
            push_child(NodeKind::CondExpr(raw), children, parent);
        }
        Expr::Call(raw) => {
            push_child(NodeKind::CallExpr(raw), children, parent);
        }
        Expr::New(raw) => {
            push_child(NodeKind::NewExpr(raw), children, parent);
        }
        Expr::Seq(raw) => {
            push_child(NodeKind::SeqExpr(raw), children, parent);
        }
        Expr::Ident(raw) => {
            push_child(NodeKind::Ident(raw), children, parent);
        }
        Expr::Lit(raw) => {
            push_child(NodeKind::Lit(raw), children, parent);
        }
        Expr::Tpl(raw) => {
            push_child(NodeKind::TemplateLiteral(raw), children, parent);
        }
        Expr::TaggedTpl(raw) => {
            push_child(NodeKind::TaggedTpl(raw), children, parent);
        }
        Expr::Arrow(raw) => {
            push_child(NodeKind::ArrowExpr(raw), children, parent);
        }
        Expr::Class(raw) => {
            push_child(NodeKind::ClassExpr(raw), children, parent);
        }
        Expr::Yield(raw) => {
            push_child(NodeKind::YieldExpr(raw), children, parent);
        }
        Expr::MetaProp(raw) => {
            push_child(NodeKind::MetaPropExpr(raw), children, parent);
        }
        Expr::Await(raw) => {
            push_child(NodeKind::AwaitExpr(raw), children, parent);
        }
        Expr::Paren(raw) => {
            push_child(NodeKind::ParenExpr(raw), children, parent);
        }
        Expr::TsTypeAssertion(raw) => {
            push_child(NodeKind::TsTypeAssertionExpr(raw), children, parent);
        }
        Expr::TsConstAssertion(raw) => {
            push_child(NodeKind::TsConstAssertionExpr(raw), children, parent);
        }
        Expr::TsNonNull(raw) => {
            push_child(NodeKind::TsNonNullExpr(raw), children, parent);
        }
        Expr::TsAs(raw) => {
            push_child(NodeKind::TsAsExpr(raw), children, parent);
        }
        Expr::TsInstantiation(raw) => {
            push_child(NodeKind::TsInstantiationExpr(raw), children, parent);
        }
        Expr::TsSatisfies(raw) => {
            push_child(NodeKind::TsSatisfiesExpr(raw), children, parent);
        }
        Expr::PrivateName(raw) => {
            push_child(NodeKind::PrivateNameExpr(raw), children, parent);
        }
        Expr::OptChain(raw) => {
            push_child(NodeKind::OptChainExpr(raw), children, parent);
        }
        Expr::Invalid(raw) => {
            push_child(NodeKind::InvalidExpr(raw), children, parent);
        }
        _ => {}
    }
}

pub(super) fn get_arrow_expr_children(
    expr: &'static ArrowExpr,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    push_child(NodeKind::BlockStmtOrExpr(&*expr.body), children, parent);

    expr.params.iter().for_each(|param| {
        push_child(NodeKind::Pat(param), children, parent);
    });
}

pub(super) fn get_module_children(
    module: &'static Module,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    for item in &module.body {
        push_child(NodeKind::ModuleItem(item), children, parent);
    }
}

pub(super) fn get_export_declartion_children(
    export_decl: &'static ExportDecl,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    match &export_decl.decl {
        Decl::Class(declaration) => {
            push_child(NodeKind::ClassDecl(declaration), children, parent);
        }
        Decl::Fn(declaration) => {
            push_child(NodeKind::FnDecl(declaration), children, parent);
        }
        Decl::Var(declaration) => {
            push_child(NodeKind::VarDecl(declaration), children, parent);
        }
        Decl::TsInterface(declaration) => {
            push_child(NodeKind::TsInterfaceDecl(declaration), children, parent);
        }
        Decl::TsTypeAlias(declaration) => {
            push_child(NodeKind::TsTypeAliasDecl(declaration), children, parent);
        }
        Decl::TsEnum(declaration) => {
            push_child(NodeKind::TsEnumDecl(declaration), children, parent);
        }
        Decl::TsModule(declaration) => {
            push_child(NodeKind::TsModuleDecl(declaration), children, parent);
        }
        _ => {}
    }
}

pub(super) fn get_export_default_expr_children(
    expression: &'static ExportDefaultExpr,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    get_expr_children(&expression.expr, children, parent)
}

pub(super) fn get_export_default_decl_children(
    export_declaration: &'static ExportDefaultDecl,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    match &export_declaration.decl {
        DefaultDecl::Class(declaration) => {
            push_child(NodeKind::ClassExpr(&declaration), children, parent);
        }
        DefaultDecl::TsInterfaceDecl(declaration) => {
            push_child(NodeKind::TsInterfaceDecl(&declaration), children, parent);
        }
        _ => {}
    }
}

pub(super) fn get_import_decl_children(
    import_declaration: &'static ImportDecl,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    for specifier in &import_declaration.specifiers {
        push_child(NodeKind::ImportSpecifier(&specifier), children, parent);
    }
}

pub(super) fn get_module_item_children(
    module_item: &'static ModuleItem,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    match module_item {
        ModuleItem::ModuleDecl(declaration) => match declaration {
            ModuleDecl::Import(declaration) => {
                push_child(NodeKind::ImportDecl(&declaration), children, parent);
            }
            ModuleDecl::ExportDecl(declaration) => {
                push_child(NodeKind::ExportDecl(&declaration), children, parent);
            }
            ModuleDecl::ExportNamed(declaration) => {
                push_child(NodeKind::NamedExport(&declaration), children, parent);
            }
            ModuleDecl::ExportDefaultDecl(declaration) => {
                push_child(NodeKind::ExportDefaultDecl(&declaration), children, parent);
            }
            ModuleDecl::ExportDefaultExpr(declaration) => {
                push_child(NodeKind::ExportDefaultExpr(&declaration), children, parent);
            }
            ModuleDecl::ExportAll(declaration) => {
                push_child(NodeKind::ExportAll(&declaration), children, parent);
            }
            ModuleDecl::TsImportEquals(declaration) => {
                push_child(NodeKind::TsImportEquals(&declaration), children, parent);
            }
            ModuleDecl::TsExportAssignment(declaration) => {
                push_child(NodeKind::TsExportAssignment(&declaration), children, parent);
            }
            ModuleDecl::TsNamespaceExport(declaration) => {
                push_child(NodeKind::TsNamespaceExport(&declaration), children, parent);
            }
        },
        ModuleItem::Stmt(statement) => get_statement_children(statement, children, parent),
    }
}

pub(super) fn get_pat_children(pat: &'static Pat, children: &mut Vec<Arc<Node<'static>>>, parent: Weak<Node<'static>>) {
    match pat {
        Pat::Ident(ident) => ident.type_ann.iter().for_each(|raw| {
            push_child(NodeKind::TsTypeAnnotation(raw), children, parent);
        }),
        Pat::Array(raw) => {
            push_child(NodeKind::ArrayPat(raw), children, parent);
        }
        Pat::Rest(raw) => {
            push_child(NodeKind::RestPat(raw), children, parent);
        }
        Pat::Object(raw) => {
            push_child(NodeKind::ObjectPat(raw), children, parent);
        }
        Pat::Assign(raw) => {
            push_child(NodeKind::AssignPat(raw), children, parent);
        }
        Pat::Expr(raw) => {
            push_child(NodeKind::Expr(raw), children, parent);
        }
        _ => {}
    }
}

pub(super) fn get_statement_children(
    statement: &'static Stmt,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    match statement {
        Stmt::Block(block_stmt) => {
            push_child(NodeKind::BlockStmt(block_stmt), children, parent);
        }
        Stmt::Empty(empty_stmt) => {
            push_child(NodeKind::EmptyStmt(empty_stmt), children, parent);
        }
        Stmt::Debugger(debugger_stmt) => {
            push_child(NodeKind::DebuggerStmt(debugger_stmt), children, parent);
        }
        Stmt::With(with_stmt) => {
            push_child(NodeKind::WithStmt(with_stmt), children, parent);
        }
        Stmt::Return(return_stmt) => {
            push_child(NodeKind::ReturnStmt(return_stmt), children, parent);
        }
        Stmt::Labeled(labeled_stmt) => {
            push_child(NodeKind::LabeledStmt(labeled_stmt), children, parent);
        }
        Stmt::Break(break_stmt) => {
            push_child(NodeKind::BreakStmt(break_stmt), children, parent);
        }
        Stmt::Continue(continue_stmt) => {
            push_child(NodeKind::ContinueStmt(continue_stmt), children, parent);
        }
        Stmt::If(if_stmt) => {
            push_child(NodeKind::IfStmt(if_stmt), children, parent);
        }
        Stmt::Switch(switch_stmt) => {
            push_child(NodeKind::SwitchStmt(switch_stmt), children, parent);
        }
        Stmt::Throw(throw_stmt) => {
            push_child(NodeKind::ThrowStmt(throw_stmt), children, parent);
        }
        Stmt::Try(try_stmt) => {
            push_child(NodeKind::TryStmt(try_stmt), children, parent);
        }
        Stmt::While(while_stmt) => {
            push_child(NodeKind::WhileStmt(while_stmt), children, parent);
        }
        Stmt::DoWhile(do_while_stmt) => {
            push_child(NodeKind::DoWhileStmt(do_while_stmt), children, parent);
        }
        Stmt::For(for_stmt) => {
            push_child(NodeKind::ForStmt(for_stmt), children, parent);
        }
        Stmt::ForIn(for_in_stmt) => {
            push_child(NodeKind::ForInStmt(for_in_stmt), children, parent);
        }
        Stmt::ForOf(for_of_stmt) => {
            push_child(NodeKind::ForOfStmt(for_of_stmt), children, parent);
        }
        Stmt::Decl(decl_stmt) => {
            push_child(NodeKind::Decl(decl_stmt), children, parent);
        }
        Stmt::Expr(expr_stmt) => {
            push_child(NodeKind::ExprStmt(expr_stmt), children, parent);
        }
    }
}

pub(super) fn get_block_statement_children(
    block_stmnt: &'static BlockStmt,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    block_stmnt
        .stmts
        .iter()
        .for_each(|statement| get_statement_children(statement, children, parent))
}

pub(super) fn get_type_annotation_children(
    type_annotation: &'static TsTypeAnn,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    match &*type_annotation.type_ann {
        TsType::TsKeywordType(ts_keyword_type) => {
            push_child(NodeKind::TsKeywordType(&ts_keyword_type), children, parent);
        }
        TsType::TsThisType(ts_this_type) => {
            push_child(NodeKind::TsThisType(&ts_this_type), children, parent);
        }
        TsType::TsFnOrConstructorType(ts_fn_or_constructor_type) => {
            push_child(
                NodeKind::TsFnOrConstructorType(&ts_fn_or_constructor_type),
                children,
                parent,
            );
        }
        TsType::TsTypeRef(ts_type_ref) => {
            push_child(NodeKind::TsTypeRef(&ts_type_ref), children, parent);
        }
        TsType::TsTypeQuery(ts_type_query) => {
            push_child(NodeKind::TsTypeQuery(&ts_type_query), children, parent);
        }
        TsType::TsTypeLit(ts_type_lit) => {
            push_child(NodeKind::TsTypeLit(&ts_type_lit), children, parent);
        }
        TsType::TsArrayType(ts_array_type) => {
            push_child(NodeKind::TsArrayType(&ts_array_type), children, parent);
        }
        TsType::TsTupleType(ts_tuple_type) => {
            push_child(NodeKind::TsTupleType(&ts_tuple_type), children, parent);
        }
        TsType::TsOptionalType(ts_optional_type) => {
            push_child(NodeKind::TsOptionalType(&ts_optional_type), children, parent);
        }
        TsType::TsRestType(ts_rest_type) => {
            push_child(NodeKind::TsRestType(&ts_rest_type), children, parent);
        }
        TsType::TsUnionOrIntersectionType(ts_union_or_intersection_type) => {
            push_child(
                NodeKind::TsUnionOrIntersectionType(&ts_union_or_intersection_type),
                children,
                parent,
            );
        }
        TsType::TsConditionalType(ts_conditional_type) => {
            push_child(NodeKind::TsConditionalType(&ts_conditional_type), children, parent);
        }
        TsType::TsInferType(ts_infer_type) => {
            push_child(NodeKind::TsInferType(&ts_infer_type), children, parent);
        }
        TsType::TsParenthesizedType(ts_parenthesized_type) => {
            push_child(NodeKind::TsParenthesizedType(&ts_parenthesized_type), children, parent);
        }
        TsType::TsTypeOperator(ts_type_operator) => {
            push_child(NodeKind::TsTypeOperator(&ts_type_operator), children, parent);
        }
        TsType::TsIndexedAccessType(ts_indexed_access_type) => {
            push_child(NodeKind::TsIndexedAccessType(&ts_indexed_access_type), children, parent);
        }
        TsType::TsMappedType(ts_mapped_type) => {
            push_child(NodeKind::TsMappedType(&ts_mapped_type), children, parent);
        }
        TsType::TsLitType(ts_lit_type) => {
            push_child(NodeKind::TsLitType(&ts_lit_type), children, parent);
        }
        TsType::TsTypePredicate(ts_type_predicate) => {
            push_child(NodeKind::TsTypePredicate(&ts_type_predicate), children, parent);
        }
        TsType::TsImportType(ts_import_type) => {
            push_child(NodeKind::TsImportType(&ts_import_type), children, parent);
        }
    }
}

pub(super) fn get_var_decl_children(
    variable_declaration: &'static VarDecl,
    children: &mut Vec<Arc<Node<'static>>>,
    parent: Weak<Node<'static>>,
) {
    variable_declaration.decls.iter().for_each(|decl| {
        push_child(NodeKind::VarDeclarator(decl), children, parent);
    })
}
