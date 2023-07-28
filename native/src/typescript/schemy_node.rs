use std::vec;

use deno_ast::swc::ast::*;

#[derive(Debug)]
pub enum SchemyNode<'m> {
    BlockStmt {
        node: &'m BlockStmt,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    EmptyStmt {
        node: &'m EmptyStmt,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    Decl {
        node: &'m Decl,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    DebuggerStmt {
        node: &'m DebuggerStmt,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    WithStmt {
        node: &'m WithStmt,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    ReturnStmt {
        node: &'m ReturnStmt,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    LabeledStmt {
        node: &'m LabeledStmt,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    BreakStmt {
        node: &'m BreakStmt,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    ContinueStmt {
        node: &'m ContinueStmt,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    IfStmt {
        node: &'m IfStmt,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    SwitchStmt {
        node: &'m SwitchStmt,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    ThrowStmt {
        node: &'m ThrowStmt,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    TryStmt {
        node: &'m TryStmt,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    WhileStmt {
        node: &'m WhileStmt,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    DoWhileStmt {
        node: &'m DoWhileStmt,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    ForStmt {
        node: &'m ForStmt,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    ForInStmt {
        node: &'m ForInStmt,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    ForOfStmt {
        node: &'m ForOfStmt,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    ExprStmt {
        node: &'m ExprStmt,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    TsPropertySignature {
        node: &'m TsPropertySignature,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    TsKeywordType {
        node: &'m TsKeywordType,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    TsThisType {
        node: &'m TsThisType,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    TsFnOrConstructorType {
        node: &'m TsFnOrConstructorType,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    TsTypeRef {
        node: &'m TsTypeRef,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    TsTypeQuery {
        node: &'m TsTypeQuery,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    TsTypeLit {
        node: &'m TsTypeLit,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    TsArrayType {
        node: &'m TsArrayType,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    TsTupleType {
        node: &'m TsTupleType,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    TsOptionalType {
        node: &'m TsOptionalType,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    TsRestType {
        node: &'m TsRestType,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    TsUnionOrIntersectionType {
        node: &'m TsUnionOrIntersectionType,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    TsConditionalType {
        node: &'m TsConditionalType,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    TsInferType {
        node: &'m TsInferType,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    TsParenthesizedType {
        node: &'m TsParenthesizedType,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    TsTypeOperator {
        node: &'m TsTypeOperator,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    TsIndexedAccessType {
        node: &'m TsIndexedAccessType,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    TsMappedType {
        node: &'m TsMappedType,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    TsLitType {
        node: &'m TsLitType,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    TsTypePredicate {
        node: &'m TsTypePredicate,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    TsImportType {
        node: &'m TsImportType,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    TsType {
        node: &'m TsType,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    TsTypeAnnotation {
        node: &'m TsTypeAnn,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    ObjectLit {
        node: &'m ObjectLit,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    BlockStmtOrExpr {
        node: &'m BlockStmtOrExpr,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    Pat {
        node: &'m Pat,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    ArrowExpr {
        node: &'m ArrowExpr,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    ExprOrSpread {
        node: &'m ExprOrSpread,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    Callee {
        node: &'m Callee,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    CallExpr {
        node: &'m CallExpr,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    Ident {
        node: &'m Ident,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    Expr {
        node: &'m Expr,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    VarDecl {
        node: &'m VarDecl,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    NamedExport {
        node: &'m NamedExport,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    ImportNamedSpecifier {
        node: &'m ImportNamedSpecifier,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    ClassDecl {
        node: &'m ClassDecl,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    ClassExpr {
        node: &'m ClassExpr,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    ExportDecl {
        node: &'m ExportDecl,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    ExportDefaultDecl {
        node: &'m ExportDefaultDecl,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    ExportDefaultExpr {
        node: &'m ExportDefaultExpr,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    ImportDecl {
        node: &'m ImportDecl,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    ImportDefaultSpecifier {
        node: &'m ImportDefaultSpecifier,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    Module {
        node: &'m Module,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    ModuleItem {
        node: &'m ModuleItem,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    TsEnumDecl {
        node: &'m TsEnumDecl,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    TsInterfaceDecl {
        node: &'m TsInterfaceDecl,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    TsTypeAliasDecl {
        node: &'m TsTypeAliasDecl,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
    ImportSpecifier {
        node: &'m ImportSpecifier,
        parent: Option<Box<&'m SchemyNode<'m>>>,
    },
}

impl<'m> SchemyNode<'m> {
    pub fn children(&'m self) -> Vec<SchemyNode> {
        match self {
            SchemyNode::Module {
                node: module,
                parent: _,
            } => module
                .body
                .iter()
                .map(|item| SchemyNode::ModuleItem {
                    node: item,
                    parent: Some(Box::from(self)),
                })
                .collect(),
            SchemyNode::ImportDecl {
                node: import_decl,
                parent: _,
            } => import_decl
                .specifiers
                .iter()
                .map(|specifier| SchemyNode::ImportSpecifier {
                    node: specifier,
                    parent: Some(Box::from(self)),
                })
                .collect(),
            SchemyNode::Pat { node, parent: _ } => match node {
                Pat::Ident(ident) if ident.type_ann.is_some() => {
                    vec![SchemyNode::TsTypeAnnotation {
                        node: ident.type_ann.as_ref().unwrap(),
                        parent: Some(Box::from(self)),
                    }]
                }
                _ => vec![],
            },
            SchemyNode::BlockStmt { node, parent: _ } => node
                .stmts
                .iter()
                .map(|statement| match statement {
                    Stmt::Block(block_stmt) => SchemyNode::BlockStmt {
                        node: &block_stmt,
                        parent: Some(Box::from(self)),
                    },
                    Stmt::Empty(empty_stmt) => SchemyNode::EmptyStmt {
                        node: &empty_stmt,
                        parent: Some(Box::from(self)),
                    },
                    Stmt::Debugger(debugger_stmt) => SchemyNode::DebuggerStmt {
                        node: &debugger_stmt,
                        parent: Some(Box::from(self)),
                    },
                    Stmt::With(with_stmt) => SchemyNode::WithStmt {
                        node: &with_stmt,
                        parent: Some(Box::from(self)),
                    },
                    Stmt::Return(return_stmt) => SchemyNode::ReturnStmt {
                        node: &return_stmt,
                        parent: Some(Box::from(self)),
                    },
                    Stmt::Labeled(labeled_stmt) => SchemyNode::LabeledStmt {
                        node: &labeled_stmt,
                        parent: Some(Box::from(self)),
                    },
                    Stmt::Break(break_stmt) => SchemyNode::BreakStmt {
                        node: &break_stmt,
                        parent: Some(Box::from(self)),
                    },
                    Stmt::Continue(continue_stmt) => SchemyNode::ContinueStmt {
                        node: &continue_stmt,
                        parent: Some(Box::from(self)),
                    },
                    Stmt::If(if_stmt) => SchemyNode::IfStmt {
                        node: &if_stmt,
                        parent: Some(Box::from(self)),
                    },
                    Stmt::Switch(switch_stmt) => SchemyNode::SwitchStmt {
                        node: &switch_stmt,
                        parent: Some(Box::from(self)),
                    },
                    Stmt::Throw(throw_stmt) => SchemyNode::ThrowStmt {
                        node: &throw_stmt,
                        parent: Some(Box::from(self)),
                    },
                    Stmt::Try(try_stmt) => SchemyNode::TryStmt {
                        node: &try_stmt,
                        parent: Some(Box::from(self)),
                    },
                    Stmt::While(while_stmt) => SchemyNode::WhileStmt {
                        node: &while_stmt,
                        parent: Some(Box::from(self)),
                    },
                    Stmt::DoWhile(do_while_stmt) => SchemyNode::DoWhileStmt {
                        node: &do_while_stmt,
                        parent: Some(Box::from(self)),
                    },
                    Stmt::For(for_stmt) => SchemyNode::ForStmt {
                        node: &for_stmt,
                        parent: Some(Box::from(self)),
                    },
                    Stmt::ForIn(for_in_stmt) => SchemyNode::ForInStmt {
                        node: &for_in_stmt,
                        parent: Some(Box::from(self)),
                    },
                    Stmt::ForOf(for_of_stmt) => SchemyNode::ForOfStmt {
                        node: &for_of_stmt,
                        parent: Some(Box::from(self)),
                    },
                    Stmt::Decl(decl_stmt) => SchemyNode::Decl {
                        node: &decl_stmt,
                        parent: Some(Box::from(self)),
                    },
                    Stmt::Expr(expr_stmt) => SchemyNode::ExprStmt {
                        node: &expr_stmt,
                        parent: Some(Box::from(self)),
                    },
                })
                .collect(),
            SchemyNode::TsTypeAnnotation { node, parent: _ } => match &*node.type_ann {
                TsType::TsKeywordType(ts_keyword_type) => vec![SchemyNode::TsKeywordType {
                    node: &ts_keyword_type,
                    parent: Some(Box::from(self)),
                }],
                TsType::TsThisType(ts_this_type) => vec![SchemyNode::TsThisType {
                    node: &ts_this_type,
                    parent: Some(Box::from(self)),
                }],
                TsType::TsFnOrConstructorType(ts_fn_or_constructor_type) => vec![SchemyNode::TsFnOrConstructorType {
                    node: &ts_fn_or_constructor_type,
                    parent: Some(Box::from(self)),
                }],
                TsType::TsTypeRef(ts_type_ref) => vec![SchemyNode::TsTypeRef {
                    node: &ts_type_ref,
                    parent: Some(Box::from(self)),
                }],
                TsType::TsTypeQuery(ts_type_query) => vec![SchemyNode::TsTypeQuery {
                    node: &ts_type_query,
                    parent: Some(Box::from(self)),
                }],
                TsType::TsTypeLit(ts_type_lit) => vec![SchemyNode::TsTypeLit {
                    node: &ts_type_lit,
                    parent: Some(Box::from(self)),
                }],
                TsType::TsArrayType(ts_array_type) => vec![SchemyNode::TsArrayType {
                    node: &ts_array_type,
                    parent: Some(Box::from(self)),
                }],
                TsType::TsTupleType(ts_tuple_type) => vec![SchemyNode::TsTupleType {
                    node: &ts_tuple_type,
                    parent: Some(Box::from(self)),
                }],
                TsType::TsOptionalType(ts_optional_type) => vec![SchemyNode::TsOptionalType {
                    node: &ts_optional_type,
                    parent: Some(Box::from(self)),
                }],
                TsType::TsRestType(ts_rest_type) => vec![SchemyNode::TsRestType {
                    node: &ts_rest_type,
                    parent: Some(Box::from(self)),
                }],
                TsType::TsUnionOrIntersectionType(ts_union_or_intersection_type) => {
                    vec![SchemyNode::TsUnionOrIntersectionType {
                        node: &ts_union_or_intersection_type,
                        parent: Some(Box::from(self)),
                    }]
                }
                TsType::TsConditionalType(ts_conditional_type) => vec![SchemyNode::TsConditionalType {
                    node: &ts_conditional_type,
                    parent: Some(Box::from(self)),
                }],
                TsType::TsInferType(ts_infer_type) => vec![SchemyNode::TsInferType {
                    node: &ts_infer_type,
                    parent: Some(Box::from(self)),
                }],
                TsType::TsParenthesizedType(ts_parenthesized_type) => vec![SchemyNode::TsParenthesizedType {
                    node: &ts_parenthesized_type,
                    parent: Some(Box::from(self)),
                }],
                TsType::TsTypeOperator(ts_type_operator) => vec![SchemyNode::TsTypeOperator {
                    node: &ts_type_operator,
                    parent: Some(Box::from(self)),
                }],
                TsType::TsIndexedAccessType(ts_indexed_access_type) => vec![SchemyNode::TsIndexedAccessType {
                    node: &ts_indexed_access_type,
                    parent: Some(Box::from(self)),
                }],
                TsType::TsMappedType(ts_mapped_type) => vec![SchemyNode::TsMappedType {
                    node: &ts_mapped_type,
                    parent: Some(Box::from(self)),
                }],
                TsType::TsLitType(ts_lit_type) => vec![SchemyNode::TsLitType {
                    node: &ts_lit_type,
                    parent: Some(Box::from(self)),
                }],
                TsType::TsTypePredicate(ts_type_predicate) => vec![SchemyNode::TsTypePredicate {
                    node: &ts_type_predicate,
                    parent: Some(Box::from(self)),
                }],
                TsType::TsImportType(ts_import_type) => vec![SchemyNode::TsImportType {
                    node: &ts_import_type,
                    parent: Some(Box::from(self)),
                }],
            },
            _ => vec![],
        }
    }

    pub(crate) fn callee(&'m self) -> Option<SchemyNode<'m>> {
        match self {
            SchemyNode::CallExpr {
                node: call_expr,
                parent: _,
            } => Some(SchemyNode::Callee {
                node: &call_expr.callee,
                parent: Some(Box::from(self)),
            }),
            _ => None,
        }
    }

    pub(crate) fn args(&'m self) -> Vec<SchemyNode<'m>> {
        match self {
            SchemyNode::CallExpr {
                node: call_expr,
                parent: _,
            } => call_expr
                .args
                .iter()
                .map(|arg| SchemyNode::ExprOrSpread {
                    node: arg,
                    parent: Some(Box::from(self)),
                })
                .collect(),
            _ => vec![],
        }
    }

    pub(crate) fn params(&'m self) -> Vec<SchemyNode<'m>> {
        match self {
            SchemyNode::ArrowExpr { node, parent:_ } => node
                .params
                .iter()
                .map(|param| SchemyNode::Pat {
                    node: param,
                    parent: Some(Box::from(self)),
                })
                .collect(),
            _ => vec![],
        }
    }

    pub(crate) fn body(&'m self) -> Option<SchemyNode<'m>> {
        match self {
            SchemyNode::ArrowExpr { node, parent: _ } => Some(SchemyNode::BlockStmtOrExpr {
                node: &node.body,
                parent: Some(Box::from(self)),
            }),
            _ => None,
        }
    }

    pub(crate) fn parent(&'m self) -> Option<&SchemyNode<'_>> {
        match self {
            SchemyNode::BlockStmt { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::TsPropertySignature { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::TsKeywordType { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::TsThisType { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::TsFnOrConstructorType { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::TsTypeRef { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::TsTypeQuery { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::TsTypeLit { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::TsArrayType { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::TsTupleType { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::TsOptionalType { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::TsRestType { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::TsUnionOrIntersectionType { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::TsConditionalType { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::TsInferType { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::TsParenthesizedType { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::TsTypeOperator { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::TsIndexedAccessType { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::TsMappedType { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::TsLitType { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::TsTypePredicate { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::TsImportType { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::TsType { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::TsTypeAnnotation { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::ObjectLit { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::BlockStmtOrExpr { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::Pat { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::ArrowExpr { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::ExprOrSpread { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::Callee { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::CallExpr { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::Ident { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::Expr { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::VarDecl { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::NamedExport { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::ImportNamedSpecifier { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::ClassDecl { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::ClassExpr { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::ExportDecl { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::ExportDefaultDecl { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::ExportDefaultExpr { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::ImportDecl { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::ImportDefaultSpecifier { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::Module { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::ModuleItem { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::TsEnumDecl { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::TsInterfaceDecl { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::TsTypeAliasDecl { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::ImportSpecifier { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::EmptyStmt { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::DebuggerStmt { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::WithStmt { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::ReturnStmt { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::LabeledStmt { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::BreakStmt { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::ContinueStmt { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::IfStmt { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::SwitchStmt { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::ThrowStmt { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::TryStmt { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::WhileStmt { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::DoWhileStmt { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::ForStmt { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::ForInStmt { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::ForOfStmt { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::ExprStmt { node: _, parent } => parent.as_ref().map(|p| **p),
            SchemyNode::Decl { node: _, parent } => parent.as_ref().map(|p| **p),
        }
    }
}

impl<'m> Clone for SchemyNode<'m> {
    fn clone(&self) -> Self {
        match self {
            Self::BlockStmt { node, parent } => Self::BlockStmt {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::EmptyStmt { node, parent } => Self::EmptyStmt {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::Decl { node, parent } => Self::Decl {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::DebuggerStmt { node, parent } => Self::DebuggerStmt {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::WithStmt { node, parent } => Self::WithStmt {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ReturnStmt { node, parent } => Self::ReturnStmt {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::LabeledStmt { node, parent } => Self::LabeledStmt {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::BreakStmt { node, parent } => Self::BreakStmt {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ContinueStmt { node, parent } => Self::ContinueStmt {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::IfStmt { node, parent } => Self::IfStmt {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::SwitchStmt { node, parent } => Self::SwitchStmt {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ThrowStmt { node, parent } => Self::ThrowStmt {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TryStmt { node, parent } => Self::TryStmt {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::WhileStmt { node, parent } => Self::WhileStmt {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::DoWhileStmt { node, parent } => Self::DoWhileStmt {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ForStmt { node, parent } => Self::ForStmt {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ForInStmt { node, parent } => Self::ForInStmt {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ForOfStmt { node, parent } => Self::ForOfStmt {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ExprStmt { node, parent } => Self::ExprStmt {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsPropertySignature { node, parent } => Self::TsPropertySignature {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsKeywordType { node, parent } => Self::TsKeywordType {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsThisType { node, parent } => Self::TsThisType {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsFnOrConstructorType { node, parent } => Self::TsFnOrConstructorType {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsTypeRef { node, parent } => Self::TsTypeRef {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsTypeQuery { node, parent } => Self::TsTypeQuery {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsTypeLit { node, parent } => Self::TsTypeLit {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsArrayType { node, parent } => Self::TsArrayType {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsTupleType { node, parent } => Self::TsTupleType {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsOptionalType { node, parent } => Self::TsOptionalType {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsRestType { node, parent } => Self::TsRestType {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsUnionOrIntersectionType { node, parent } => Self::TsUnionOrIntersectionType {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsConditionalType { node, parent } => Self::TsConditionalType {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsInferType { node, parent } => Self::TsInferType {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsParenthesizedType { node, parent } => Self::TsParenthesizedType {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsTypeOperator { node, parent } => Self::TsTypeOperator {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsIndexedAccessType { node, parent } => Self::TsIndexedAccessType {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsMappedType { node, parent } => Self::TsMappedType {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsLitType { node, parent } => Self::TsLitType {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsTypePredicate { node, parent } => Self::TsTypePredicate {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsImportType { node, parent } => Self::TsImportType {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsType { node, parent } => Self::TsType {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsTypeAnnotation { node, parent } => Self::TsTypeAnnotation {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ObjectLit { node, parent } => Self::ObjectLit {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::BlockStmtOrExpr { node, parent } => Self::BlockStmtOrExpr {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::Pat { node, parent } => Self::Pat {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ArrowExpr { node, parent } => Self::ArrowExpr {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ExprOrSpread { node, parent } => Self::ExprOrSpread {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::Callee { node, parent } => Self::Callee {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::CallExpr { node, parent } => Self::CallExpr {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::Ident { node, parent } => Self::Ident {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::Expr { node, parent } => Self::Expr {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::VarDecl { node, parent } => Self::VarDecl {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::NamedExport { node, parent } => Self::NamedExport {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ImportNamedSpecifier { node, parent } => Self::ImportNamedSpecifier {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ClassDecl { node, parent } => Self::ClassDecl {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ClassExpr { node, parent } => Self::ClassExpr {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ExportDecl { node, parent } => Self::ExportDecl {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ExportDefaultDecl { node, parent } => Self::ExportDefaultDecl {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ExportDefaultExpr { node, parent } => Self::ExportDefaultExpr {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ImportDecl { node, parent } => Self::ImportDecl {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ImportDefaultSpecifier { node, parent } => Self::ImportDefaultSpecifier {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::Module { node, parent } => Self::Module {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ModuleItem { node, parent } => Self::ModuleItem {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsEnumDecl { node, parent } => Self::TsEnumDecl {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsInterfaceDecl { node, parent } => Self::TsInterfaceDecl {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsTypeAliasDecl { node, parent } => Self::TsTypeAliasDecl {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ImportSpecifier { node, parent } => Self::ImportSpecifier {
                node: node,
                parent: parent.as_ref().map(|p| p.clone()),
            },
        }
    }
}
