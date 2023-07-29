use std::{vec, rc::Rc};

use deno_ast::swc::ast::*;
use lazy_static::__Deref;

#[derive(Debug)]
pub enum SchemyNode<'m> {
    BlockStmt {
        node: Rc<&'m BlockStmt>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    EmptyStmt {
        node: Rc<&'m EmptyStmt>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    Decl {
        node: Rc<&'m Decl>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    DebuggerStmt {
        node: Rc<&'m DebuggerStmt>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    WithStmt {
        node: Rc<&'m WithStmt>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    ReturnStmt {
        node: Rc<&'m ReturnStmt>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    LabeledStmt {
        node: Rc<&'m LabeledStmt>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    BreakStmt {
        node: Rc<&'m BreakStmt>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    ContinueStmt {
        node: Rc<&'m ContinueStmt>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    IfStmt {
        node: Rc<&'m IfStmt>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    SwitchStmt {
        node: Rc<&'m SwitchStmt>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    ThrowStmt {
        node: Rc<&'m ThrowStmt>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    TryStmt {
        node: Rc<&'m TryStmt>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    WhileStmt {
        node: Rc<&'m WhileStmt>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    DoWhileStmt {
        node: Rc<&'m DoWhileStmt>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    ForStmt {
        node: Rc<&'m ForStmt>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    ForInStmt {
        node: Rc<&'m ForInStmt>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    ForOfStmt {
        node: Rc<&'m ForOfStmt>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    ExprStmt {
        node: Rc<&'m ExprStmt>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    TsPropertySignature {
        node: Rc<&'m TsPropertySignature>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    TsKeywordType {
        node: Rc<&'m TsKeywordType>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    TsThisType {
        node: Rc<&'m TsThisType>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    TsFnOrConstructorType {
        node: Rc<&'m TsFnOrConstructorType>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    TsTypeRef {
        node: Rc<&'m TsTypeRef>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    TsTypeQuery {
        node: Rc<&'m TsTypeQuery>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    TsTypeLit {
        node: Rc<&'m TsTypeLit>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    TsArrayType {
        node: Rc<&'m TsArrayType>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    TsTupleType {
        node: Rc<&'m TsTupleType>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    TsOptionalType {
        node: Rc<&'m TsOptionalType>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    TsRestType {
        node: Rc<&'m TsRestType>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    TsUnionOrIntersectionType {
        node: Rc<&'m TsUnionOrIntersectionType>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    TsConditionalType {
        node: Rc<&'m TsConditionalType>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    TsInferType {
        node: Rc<&'m TsInferType>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    TsParenthesizedType {
        node: Rc<&'m TsParenthesizedType>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    TsTypeOperator {
        node: Rc<&'m TsTypeOperator>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    TsIndexedAccessType {
        node: Rc<&'m TsIndexedAccessType>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    TsMappedType {
        node: Rc<&'m TsMappedType>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    TsLitType {
        node: Rc<&'m TsLitType>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    TsTypePredicate {
        node: Rc<&'m TsTypePredicate>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    TsImportType {
        node: Rc<&'m TsImportType>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    TsType {
        node: Rc<&'m TsType>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    TsTypeAnnotation {
        node: Rc<&'m TsTypeAnn>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    ObjectLit {
        node: Rc<&'m ObjectLit>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    BlockStmtOrExpr {
        node: Rc<&'m BlockStmtOrExpr>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    Pat {
        node: Rc<&'m Pat>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    ArrowExpr {
        node: Rc<&'m ArrowExpr>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    ExprOrSpread {
        node: Rc<&'m ExprOrSpread>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    Callee {
        node: Rc<&'m Callee>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    CallExpr {
        node: Rc<&'m CallExpr>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    Ident {
        node: Rc<&'m Ident>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    Expr {
        node: Rc<&'m Expr>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    VarDecl {
        node: Rc<&'m VarDecl>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    NamedExport {
        node: Rc<&'m NamedExport>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    ImportNamedSpecifier {
        node: Rc<&'m ImportNamedSpecifier>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    ClassDecl {
        node: Rc<&'m ClassDecl>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    ClassExpr {
        node: Rc<&'m ClassExpr>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    ExportDecl {
        node: Rc<&'m ExportDecl>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    ExportDefaultDecl {
        node: Rc<&'m ExportDefaultDecl>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    ExportDefaultExpr {
        node: Rc<&'m ExportDefaultExpr>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    ImportDecl {
        node: Rc<&'m ImportDecl>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    ImportDefaultSpecifier {
        node: Rc<&'m ImportDefaultSpecifier>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    Module {
        node: Rc<&'m Module>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    ModuleItem {
        node: Rc<&'m ModuleItem>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    TsEnumDecl {
        node: Rc<&'m TsEnumDecl>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    TsInterfaceDecl {
        node: Rc<&'m TsInterfaceDecl>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    TsTypeAliasDecl {
        node: Rc<&'m TsTypeAliasDecl>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    ImportSpecifier {
        node: Rc<&'m ImportSpecifier>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
}

impl <'m> SchemyNode <'m> {
    pub fn children(&self) -> Vec<SchemyNode> {
        match self {
            SchemyNode::Module {
                node: module,
                parent: _,
            } => module
                .body
                .iter()
                .map(|item| SchemyNode::ModuleItem {
                    node: Rc::new(item),
                    parent: Some(Box::from(self.clone())),
                })
                .collect(),
            SchemyNode::ImportDecl {
                node: import_decl,
                parent: _,
            } => import_decl
                .specifiers
                .iter()
                .map(|specifier| SchemyNode::ImportSpecifier {
                    node: Rc::new(specifier),
                    parent: Some(Box::from(self.clone())),
                })
                .collect(),
            SchemyNode::Pat { node, parent: _ } => match node.deref() {
                Pat::Ident(ident) if ident.type_ann.is_some() => {
                    vec![SchemyNode::TsTypeAnnotation {
                        node: Rc::new(ident.type_ann.as_ref().unwrap()),
                        parent: Some(Box::from(self.clone())),
                    }]
                }
                _ => vec![],
            },
            SchemyNode::BlockStmt { node, parent: _ } => node
                .stmts
                .iter()
                .map(|statement| match statement {
                    Stmt::Block(block_stmt) => SchemyNode::BlockStmt {
                        node: Rc::new(block_stmt),
                        parent: Some(Box::from(self.clone())),
                    },
                    Stmt::Empty(empty_stmt) => SchemyNode::EmptyStmt {
                        node: Rc::new(empty_stmt),
                        parent: Some(Box::from(self.clone())),
                    },
                    Stmt::Debugger(debugger_stmt) => SchemyNode::DebuggerStmt {
                        node: Rc::new(debugger_stmt),
                        parent: Some(Box::from(self.clone())),
                    },
                    Stmt::With(with_stmt) => SchemyNode::WithStmt {
                        node: Rc::new(with_stmt),
                        parent: Some(Box::from(self.clone())),
                    },
                    Stmt::Return(return_stmt) => SchemyNode::ReturnStmt {
                        node: Rc::new(return_stmt),
                        parent: Some(Box::from(self.clone())),
                    },
                    Stmt::Labeled(labeled_stmt) => SchemyNode::LabeledStmt {
                        node: Rc::new(labeled_stmt),
                        parent: Some(Box::from(self.clone())),
                    },
                    Stmt::Break(break_stmt) => SchemyNode::BreakStmt {
                        node: Rc::new(break_stmt),
                        parent: Some(Box::from(self.clone())),
                    },
                    Stmt::Continue(continue_stmt) => SchemyNode::ContinueStmt {
                        node: Rc::new(continue_stmt),
                        parent: Some(Box::from(self.clone())),
                    },
                    Stmt::If(if_stmt) => SchemyNode::IfStmt {
                        node: Rc::new(if_stmt),
                        parent: Some(Box::from(self.clone())),
                    },
                    Stmt::Switch(switch_stmt) => SchemyNode::SwitchStmt {
                        node: Rc::new(switch_stmt),
                        parent: Some(Box::from(self.clone())),
                    },
                    Stmt::Throw(throw_stmt) => SchemyNode::ThrowStmt {
                        node: Rc::new(throw_stmt),
                        parent: Some(Box::from(self.clone())),
                    },
                    Stmt::Try(try_stmt) => SchemyNode::TryStmt {
                        node: Rc::new(try_stmt),
                        parent: Some(Box::from(self.clone())),
                    },
                    Stmt::While(while_stmt) => SchemyNode::WhileStmt {
                        node: Rc::new(while_stmt),
                        parent: Some(Box::from(self.clone())),
                    },
                    Stmt::DoWhile(do_while_stmt) => SchemyNode::DoWhileStmt {
                        node: Rc::new(do_while_stmt),
                        parent: Some(Box::from(self.clone())),
                    },
                    Stmt::For(for_stmt) => SchemyNode::ForStmt {
                        node: Rc::new(for_stmt),
                        parent: Some(Box::from(self.clone())),
                    },
                    Stmt::ForIn(for_in_stmt) => SchemyNode::ForInStmt {
                        node: Rc::new(for_in_stmt),
                        parent: Some(Box::from(self.clone())),
                    },
                    Stmt::ForOf(for_of_stmt) => SchemyNode::ForOfStmt {
                        node: Rc::new(for_of_stmt),
                        parent: Some(Box::from(self.clone())),
                    },
                    Stmt::Decl(decl_stmt) => SchemyNode::Decl {
                        node: Rc::new(decl_stmt),
                        parent: Some(Box::from(self.clone())),
                    },
                    Stmt::Expr(expr_stmt) => SchemyNode::ExprStmt {
                        node: Rc::new(expr_stmt),
                        parent: Some(Box::from(self.clone())),
                    },
                })
                .collect(),
            SchemyNode::TsTypeAnnotation { node, parent: _ } => match &*node.type_ann {
                TsType::TsKeywordType(ts_keyword_type) => vec![SchemyNode::TsKeywordType {
                    node: Rc::new(ts_keyword_type),
                    parent: Some(Box::from(self.clone())),
                }],
                TsType::TsThisType(ts_this_type) => vec![SchemyNode::TsThisType {
                    node: Rc::new(ts_this_type),
                    parent: Some(Box::from(self.clone())),
                }],
                TsType::TsFnOrConstructorType(ts_fn_or_constructor_type) => vec![SchemyNode::TsFnOrConstructorType {
                    node: Rc::new(ts_fn_or_constructor_type),
                    parent: Some(Box::from(self.clone())),
                }],
                TsType::TsTypeRef(ts_type_ref) => vec![SchemyNode::TsTypeRef {
                    node: Rc::new(ts_type_ref),
                    parent: Some(Box::from(self.clone())),
                }],
                TsType::TsTypeQuery(ts_type_query) => vec![SchemyNode::TsTypeQuery {
                    node: Rc::new(ts_type_query),
                    parent: Some(Box::from(self.clone())),
                }],
                TsType::TsTypeLit(ts_type_lit) => vec![SchemyNode::TsTypeLit {
                    node: Rc::new(ts_type_lit),
                    parent: Some(Box::from(self.clone())),
                }],
                TsType::TsArrayType(ts_array_type) => vec![SchemyNode::TsArrayType {
                    node: Rc::new(ts_array_type),
                    parent: Some(Box::from(self.clone())),
                }],
                TsType::TsTupleType(ts_tuple_type) => vec![SchemyNode::TsTupleType {
                    node: Rc::new(ts_tuple_type),
                    parent: Some(Box::from(self.clone())),
                }],
                TsType::TsOptionalType(ts_optional_type) => vec![SchemyNode::TsOptionalType {
                    node: Rc::new(ts_optional_type),
                    parent: Some(Box::from(self.clone())),
                }],
                TsType::TsRestType(ts_rest_type) => vec![SchemyNode::TsRestType {
                    node: Rc::new(ts_rest_type),
                    parent: Some(Box::from(self.clone())),
                }],
                TsType::TsUnionOrIntersectionType(ts_union_or_intersection_type) => {
                    vec![SchemyNode::TsUnionOrIntersectionType {
                        node: Rc::new(ts_union_or_intersection_type),
                        parent: Some(Box::from(self.clone())),
                    }]
                }
                TsType::TsConditionalType(ts_conditional_type) => vec![SchemyNode::TsConditionalType {
                    node: Rc::new(ts_conditional_type),
                    parent: Some(Box::from(self.clone())),
                }],
                TsType::TsInferType(ts_infer_type) => vec![SchemyNode::TsInferType {
                    node: Rc::new(ts_infer_type),
                    parent: Some(Box::from(self.clone())),
                }],
                TsType::TsParenthesizedType(ts_parenthesized_type) => vec![SchemyNode::TsParenthesizedType {
                    node: Rc::new(ts_parenthesized_type),
                    parent: Some(Box::from(self.clone())),
                }],
                TsType::TsTypeOperator(ts_type_operator) => vec![SchemyNode::TsTypeOperator {
                    node: Rc::new(ts_type_operator),
                    parent: Some(Box::from(self.clone())),
                }],
                TsType::TsIndexedAccessType(ts_indexed_access_type) => vec![SchemyNode::TsIndexedAccessType {
                    node: Rc::new(ts_indexed_access_type),
                    parent: Some(Box::from(self.clone())),
                }],
                TsType::TsMappedType(ts_mapped_type) => vec![SchemyNode::TsMappedType {
                    node: Rc::new(ts_mapped_type),
                    parent: Some(Box::from(self.clone())),
                }],
                TsType::TsLitType(ts_lit_type) => vec![SchemyNode::TsLitType {
                    node: Rc::new(ts_lit_type),
                    parent: Some(Box::from(self.clone())),
                }],
                TsType::TsTypePredicate(ts_type_predicate) => vec![SchemyNode::TsTypePredicate {
                    node: Rc::new(ts_type_predicate),
                    parent: Some(Box::from(self.clone())),
                }],
                TsType::TsImportType(ts_import_type) => vec![SchemyNode::TsImportType {
                    node: Rc::new(ts_import_type),
                    parent: Some(Box::from(self.clone())),
                }],
            },
            _ => vec![],
        }
    }

    pub(crate) fn callee(&self) -> Option<SchemyNode> {
        match self {
            SchemyNode::CallExpr {
                node: call_expr,
                parent: _,
            } => Some(SchemyNode::Callee {
                node: Rc::new(&call_expr.callee),
                parent: Some(Box::from(self.clone())),
            }),
            _ => None,
        }
    }

    pub(crate) fn args(&self) -> Vec<SchemyNode> {
        match self {
            SchemyNode::CallExpr {
                node: call_expr,
                parent: _,
            } => call_expr
                .args
                .iter()
                .map(|arg| SchemyNode::ExprOrSpread {
                    node: Rc::new(arg),
                    parent: Some(Box::from(self.clone())),
                })
                .collect(),
            _ => vec![],
        }
    }

    pub(crate) fn params(&self) -> Vec<SchemyNode> {
        match self {
            SchemyNode::ArrowExpr { node, parent: _ } => node
                .params
                .iter()
                .map(|param| SchemyNode::Pat {
                    node: Rc::new(param),
                    parent: Some(Box::from(self.clone())),
                })
                .collect(),
            _ => vec![],
        }
    }

    pub(crate) fn body(&self) -> Option<SchemyNode> {
        match self {
            SchemyNode::ArrowExpr { node, parent: _ } => Some(SchemyNode::BlockStmtOrExpr {
                node: Rc::new(&*node.body),
                parent: Some(Box::from(self.clone())),
            }),
            _ => None,
        }
    }

    pub(crate) fn parent(&self) -> Option<SchemyNode> {
        match self {
            SchemyNode::BlockStmt { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::TsPropertySignature { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::TsKeywordType { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::TsThisType { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::TsFnOrConstructorType { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::TsTypeRef { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::TsTypeQuery { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::TsTypeLit { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::TsArrayType { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::TsTupleType { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::TsOptionalType { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::TsRestType { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::TsUnionOrIntersectionType { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::TsConditionalType { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::TsInferType { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::TsParenthesizedType { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::TsTypeOperator { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::TsIndexedAccessType { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::TsMappedType { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::TsLitType { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::TsTypePredicate { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::TsImportType { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::TsType { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::TsTypeAnnotation { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::ObjectLit { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::BlockStmtOrExpr { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::Pat { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::ArrowExpr { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::ExprOrSpread { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::Callee { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::CallExpr { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::Ident { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::Expr { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::VarDecl { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::NamedExport { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::ImportNamedSpecifier { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::ClassDecl { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::ClassExpr { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::ExportDecl { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::ExportDefaultDecl { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::ExportDefaultExpr { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::ImportDecl { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::ImportDefaultSpecifier { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::Module { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::ModuleItem { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::TsEnumDecl { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::TsInterfaceDecl { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::TsTypeAliasDecl { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::ImportSpecifier { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::EmptyStmt { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::DebuggerStmt { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::WithStmt { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::ReturnStmt { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::LabeledStmt { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::BreakStmt { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::ContinueStmt { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::IfStmt { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::SwitchStmt { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::ThrowStmt { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::TryStmt { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::WhileStmt { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::DoWhileStmt { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::ForStmt { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::ForInStmt { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::ForOfStmt { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::ExprStmt { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::Decl { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
        }
    }
}

impl <'m> Clone for SchemyNode<'m> {
    fn clone(&self) -> Self {
        match self {
            Self::BlockStmt { node, parent } => Self::BlockStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::EmptyStmt { node, parent } => Self::EmptyStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::Decl { node, parent } => Self::Decl {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::DebuggerStmt { node, parent } => Self::DebuggerStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::WithStmt { node, parent } => Self::WithStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ReturnStmt { node, parent } => Self::ReturnStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::LabeledStmt { node, parent } => Self::LabeledStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::BreakStmt { node, parent } => Self::BreakStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ContinueStmt { node, parent } => Self::ContinueStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::IfStmt { node, parent } => Self::IfStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::SwitchStmt { node, parent } => Self::SwitchStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ThrowStmt { node, parent } => Self::ThrowStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TryStmt { node, parent } => Self::TryStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::WhileStmt { node, parent } => Self::WhileStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::DoWhileStmt { node, parent } => Self::DoWhileStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ForStmt { node, parent } => Self::ForStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ForInStmt { node, parent } => Self::ForInStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ForOfStmt { node, parent } => Self::ForOfStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ExprStmt { node, parent } => Self::ExprStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsPropertySignature { node, parent } => Self::TsPropertySignature {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsKeywordType { node, parent } => Self::TsKeywordType {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsThisType { node, parent } => Self::TsThisType {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsFnOrConstructorType { node, parent } => Self::TsFnOrConstructorType {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsTypeRef { node, parent } => Self::TsTypeRef {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsTypeQuery { node, parent } => Self::TsTypeQuery {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsTypeLit { node, parent } => Self::TsTypeLit {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsArrayType { node, parent } => Self::TsArrayType {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsTupleType { node, parent } => Self::TsTupleType {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsOptionalType { node, parent } => Self::TsOptionalType {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsRestType { node, parent } => Self::TsRestType {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsUnionOrIntersectionType { node, parent } => Self::TsUnionOrIntersectionType {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsConditionalType { node, parent } => Self::TsConditionalType {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsInferType { node, parent } => Self::TsInferType {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsParenthesizedType { node, parent } => Self::TsParenthesizedType {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsTypeOperator { node, parent } => Self::TsTypeOperator {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsIndexedAccessType { node, parent } => Self::TsIndexedAccessType {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsMappedType { node, parent } => Self::TsMappedType {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsLitType { node, parent } => Self::TsLitType {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsTypePredicate { node, parent } => Self::TsTypePredicate {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsImportType { node, parent } => Self::TsImportType {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsType { node, parent } => Self::TsType {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsTypeAnnotation { node, parent } => Self::TsTypeAnnotation {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ObjectLit { node, parent } => Self::ObjectLit {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::BlockStmtOrExpr { node, parent } => Self::BlockStmtOrExpr {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::Pat { node, parent } => Self::Pat {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ArrowExpr { node, parent } => Self::ArrowExpr {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ExprOrSpread { node, parent } => Self::ExprOrSpread {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::Callee { node, parent } => Self::Callee {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::CallExpr { node, parent } => Self::CallExpr {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::Ident { node, parent } => Self::Ident {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::Expr { node, parent } => Self::Expr {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::VarDecl { node, parent } => Self::VarDecl {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::NamedExport { node, parent } => Self::NamedExport {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ImportNamedSpecifier { node, parent } => Self::ImportNamedSpecifier {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ClassDecl { node, parent } => Self::ClassDecl {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ClassExpr { node, parent } => Self::ClassExpr {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ExportDecl { node, parent } => Self::ExportDecl {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ExportDefaultDecl { node, parent } => Self::ExportDefaultDecl {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ExportDefaultExpr { node, parent } => Self::ExportDefaultExpr {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ImportDecl { node, parent } => Self::ImportDecl {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ImportDefaultSpecifier { node, parent } => Self::ImportDefaultSpecifier {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::Module { node, parent } => Self::Module {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ModuleItem { node, parent } => Self::ModuleItem {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsEnumDecl { node, parent } => Self::TsEnumDecl {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsInterfaceDecl { node, parent } => Self::TsInterfaceDecl {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::TsTypeAliasDecl { node, parent } => Self::TsTypeAliasDecl {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            Self::ImportSpecifier { node, parent } => Self::ImportSpecifier {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
        }
    }
}
