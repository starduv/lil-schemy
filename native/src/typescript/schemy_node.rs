use std::{rc::Rc, vec};

use deno_ast::swc::ast::*;
use lazy_static::__Deref;

#[derive(Debug)]
pub enum SchemyNode<'n> {
    TsNamespaceExport {
        node: Rc<&'n TsNamespaceExportDecl>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    TsExportAssignment {
        node: Rc<&'n TsExportAssignment>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    TsImportEquals {
        node: Rc<&'n TsImportEqualsDecl>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    ExportAll {
        node: Rc<&'n ExportAll>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    BlockStmt {
        node: Rc<&'n BlockStmt>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    EmptyStmt {
        node: Rc<&'n EmptyStmt>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    Decl {
        node: Rc<&'n Decl>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    DebuggerStmt {
        node: Rc<&'n DebuggerStmt>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    WithStmt {
        node: Rc<&'n WithStmt>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    ReturnStmt {
        node: Rc<&'n ReturnStmt>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    LabeledStmt {
        node: Rc<&'n LabeledStmt>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    BreakStmt {
        node: Rc<&'n BreakStmt>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    ContinueStmt {
        node: Rc<&'n ContinueStmt>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    IfStmt {
        node: Rc<&'n IfStmt>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    SwitchStmt {
        node: Rc<&'n SwitchStmt>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    ThrowStmt {
        node: Rc<&'n ThrowStmt>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    TryStmt {
        node: Rc<&'n TryStmt>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    WhileStmt {
        node: Rc<&'n WhileStmt>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    DoWhileStmt {
        node: Rc<&'n DoWhileStmt>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    ForStmt {
        node: Rc<&'n ForStmt>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    ForInStmt {
        node: Rc<&'n ForInStmt>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    ForOfStmt {
        node: Rc<&'n ForOfStmt>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    ExprStmt {
        node: Rc<&'n ExprStmt>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    TsPropertySignature {
        node: Rc<&'n TsPropertySignature>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    TsKeywordType {
        node: Rc<&'n TsKeywordType>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    TsThisType {
        node: Rc<&'n TsThisType>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    TsFnOrConstructorType {
        node: Rc<&'n TsFnOrConstructorType>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    TsTypeRef {
        node: Rc<&'n TsTypeRef>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    TsTypeQuery {
        node: Rc<&'n TsTypeQuery>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    TsTypeLit {
        node: Rc<&'n TsTypeLit>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    TsArrayType {
        node: Rc<&'n TsArrayType>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    TsTupleType {
        node: Rc<&'n TsTupleType>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    TsOptionalType {
        node: Rc<&'n TsOptionalType>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    TsRestType {
        node: Rc<&'n TsRestType>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    TsUnionOrIntersectionType {
        node: Rc<&'n TsUnionOrIntersectionType>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    TsConditionalType {
        node: Rc<&'n TsConditionalType>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    TsInferType {
        node: Rc<&'n TsInferType>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    TsParenthesizedType {
        node: Rc<&'n TsParenthesizedType>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    TsTypeOperator {
        node: Rc<&'n TsTypeOperator>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    TsIndexedAccessType {
        node: Rc<&'n TsIndexedAccessType>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    TsMappedType {
        node: Rc<&'n TsMappedType>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    TsLitType {
        node: Rc<&'n TsLitType>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    TsTypePredicate {
        node: Rc<&'n TsTypePredicate>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    TsImportType {
        node: Rc<&'n TsImportType>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    TsType {
        node: Rc<&'n TsType>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    TsTypeAnnotation {
        node: Rc<&'n TsTypeAnn>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    ObjectLit {
        node: Rc<&'n ObjectLit>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    BlockStmtOrExpr {
        node: Rc<&'n BlockStmtOrExpr>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    Pat {
        node: Rc<&'n Pat>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    ArrowExpr {
        node: Rc<&'n ArrowExpr>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    ExprOrSpread {
        node: Rc<&'n ExprOrSpread>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    Callee {
        node: Rc<&'n Callee>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    CallExpr {
        node: Rc<&'n CallExpr>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    Ident {
        node: Rc<&'n Ident>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    Expr {
        node: Rc<&'n Expr>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    VarDecl {
        node: Rc<&'n VarDecl>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    NamedExport {
        node: Rc<&'n NamedExport>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    ImportNamedSpecifier {
        node: Rc<&'n ImportNamedSpecifier>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    ClassDecl {
        node: Rc<&'n ClassDecl>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    ClassExpr {
        node: Rc<&'n ClassExpr>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    ExportDecl {
        node: Rc<&'n ExportDecl>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    ExportDefaultDecl {
        node: Rc<&'n ExportDefaultDecl>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    ExportDefaultExpr {
        node: Rc<&'n ExportDefaultExpr>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    ImportDecl {
        node: Rc<&'n ImportDecl>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    ImportDefaultSpecifier {
        node: Rc<&'n ImportDefaultSpecifier>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    Module {
        node: Rc<&'n Module>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    ModuleItem {
        node: Rc<&'n ModuleItem>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    TsEnumDecl {
        node: Rc<&'n TsEnumDecl>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    TsInterfaceDecl {
        node: Rc<&'n TsInterfaceDecl>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    TsTypeAliasDecl {
        node: Rc<&'n TsTypeAliasDecl>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
    ImportSpecifier {
        node: Rc<&'n ImportSpecifier>,
        parent: Option<Box<SchemyNode<'n>>>,
    },
}

impl<'n> SchemyNode<'n> {
    pub fn children(&'n self) -> Vec<SchemyNode<'n>> {
        match self {
            SchemyNode::ModuleItem { node, parent: _ } => match node.deref() {
                ModuleItem::ModuleDecl(declaration) => match declaration {
                    ModuleDecl::Import(declaration) => vec![SchemyNode::ImportDecl {
                        node: Rc::new(declaration),
                        parent: Some(Box::from(self.clone())),
                    }],
                    ModuleDecl::ExportDecl(declaration) => vec![SchemyNode::ExportDecl {
                        node: Rc::new(declaration),
                        parent: Some(Box::from(self.clone())),
                    }],
                    ModuleDecl::ExportNamed(declaration) => vec![SchemyNode::NamedExport {
                        node: Rc::new(declaration),
                        parent: Some(Box::from(self.clone())),
                    }],
                    ModuleDecl::ExportDefaultDecl(declaration) => vec![SchemyNode::ExportDefaultDecl {
                        node: Rc::new(declaration),
                        parent: Some(Box::from(self.clone())),
                    }],
                    ModuleDecl::ExportDefaultExpr(declaration) => vec![SchemyNode::ExportDefaultExpr {
                        node: Rc::new(declaration),
                        parent: Some(Box::from(self.clone())),
                    }],
                    ModuleDecl::ExportAll(declaration) => vec![SchemyNode::ExportAll {
                        node: Rc::new(declaration),
                        parent: Some(Box::from(self.clone())),
                    }],
                    ModuleDecl::TsImportEquals(declaration) => vec![SchemyNode::TsImportEquals {
                        node: Rc::new(declaration),
                        parent: Some(Box::from(self.clone())),
                    }],
                    ModuleDecl::TsExportAssignment(declaration) => vec![SchemyNode::TsExportAssignment {
                        node: Rc::new(declaration),
                        parent: Some(Box::from(self.clone())),
                    }],
                    ModuleDecl::TsNamespaceExport(declaration) => vec![SchemyNode::TsNamespaceExport {
                        node: Rc::new(declaration),
                        parent: Some(Box::from(self.clone())),
                    }],
                },
                _ => vec![],
            },
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
            SchemyNode::TsNamespaceExport { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::TsExportAssignment { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::TsImportEquals { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
            SchemyNode::ExportAll { node: _, parent } => parent.as_ref().map(|p| *p.clone()),
        }
    }
}

impl<'n> Clone for SchemyNode<'n> {
    fn clone(&self) -> SchemyNode<'n> {
        match self {
            SchemyNode::TsNamespaceExport { node, parent } => SchemyNode::TsNamespaceExport {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::TsExportAssignment { node, parent } => SchemyNode::TsExportAssignment {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::TsImportEquals { node, parent } => SchemyNode::TsImportEquals {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::ExportAll { node, parent } => SchemyNode::ExportAll {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::BlockStmt { node, parent } => SchemyNode::BlockStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::EmptyStmt { node, parent } => SchemyNode::EmptyStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::Decl { node, parent } => SchemyNode::Decl {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::DebuggerStmt { node, parent } => SchemyNode::DebuggerStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::WithStmt { node, parent } => SchemyNode::WithStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::ReturnStmt { node, parent } => SchemyNode::ReturnStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::LabeledStmt { node, parent } => SchemyNode::LabeledStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::BreakStmt { node, parent } => SchemyNode::BreakStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::ContinueStmt { node, parent } => SchemyNode::ContinueStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::IfStmt { node, parent } => SchemyNode::IfStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::SwitchStmt { node, parent } => SchemyNode::SwitchStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::ThrowStmt { node, parent } => SchemyNode::ThrowStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::TryStmt { node, parent } => SchemyNode::TryStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::WhileStmt { node, parent } => SchemyNode::WhileStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::DoWhileStmt { node, parent } => SchemyNode::DoWhileStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::ForStmt { node, parent } => SchemyNode::ForStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::ForInStmt { node, parent } => SchemyNode::ForInStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::ForOfStmt { node, parent } => SchemyNode::ForOfStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::ExprStmt { node, parent } => SchemyNode::ExprStmt {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::TsPropertySignature { node, parent } => SchemyNode::TsPropertySignature {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::TsKeywordType { node, parent } => SchemyNode::TsKeywordType {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::TsThisType { node, parent } => SchemyNode::TsThisType {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::TsFnOrConstructorType { node, parent } => SchemyNode::TsFnOrConstructorType {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::TsTypeRef { node, parent } => SchemyNode::TsTypeRef {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::TsTypeQuery { node, parent } => SchemyNode::TsTypeQuery {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::TsTypeLit { node, parent } => SchemyNode::TsTypeLit {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::TsArrayType { node, parent } => SchemyNode::TsArrayType {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::TsTupleType { node, parent } => SchemyNode::TsTupleType {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::TsOptionalType { node, parent } => SchemyNode::TsOptionalType {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::TsRestType { node, parent } => SchemyNode::TsRestType {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::TsUnionOrIntersectionType { node, parent } => SchemyNode::TsUnionOrIntersectionType {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::TsConditionalType { node, parent } => SchemyNode::TsConditionalType {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::TsInferType { node, parent } => SchemyNode::TsInferType {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::TsParenthesizedType { node, parent } => SchemyNode::TsParenthesizedType {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::TsTypeOperator { node, parent } => SchemyNode::TsTypeOperator {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::TsIndexedAccessType { node, parent } => SchemyNode::TsIndexedAccessType {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::TsMappedType { node, parent } => SchemyNode::TsMappedType {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::TsLitType { node, parent } => SchemyNode::TsLitType {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::TsTypePredicate { node, parent } => SchemyNode::TsTypePredicate {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::TsImportType { node, parent } => SchemyNode::TsImportType {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::TsType { node, parent } => SchemyNode::TsType {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::TsTypeAnnotation { node, parent } => SchemyNode::TsTypeAnnotation {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::ObjectLit { node, parent } => SchemyNode::ObjectLit {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::BlockStmtOrExpr { node, parent } => SchemyNode::BlockStmtOrExpr {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::Pat { node, parent } => SchemyNode::Pat {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::ArrowExpr { node, parent } => SchemyNode::ArrowExpr {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::ExprOrSpread { node, parent } => SchemyNode::ExprOrSpread {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::Callee { node, parent } => SchemyNode::Callee {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::CallExpr { node, parent } => SchemyNode::CallExpr {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::Ident { node, parent } => SchemyNode::Ident {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::Expr { node, parent } => SchemyNode::Expr {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::VarDecl { node, parent } => SchemyNode::VarDecl {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::NamedExport { node, parent } => SchemyNode::NamedExport {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::ImportNamedSpecifier { node, parent } => SchemyNode::ImportNamedSpecifier {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::ClassDecl { node, parent } => SchemyNode::ClassDecl {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::ClassExpr { node, parent } => SchemyNode::ClassExpr {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::ExportDecl { node, parent } => SchemyNode::ExportDecl {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::ExportDefaultDecl { node, parent } => SchemyNode::ExportDefaultDecl {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::ExportDefaultExpr { node, parent } => SchemyNode::ExportDefaultExpr {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::ImportDecl { node, parent } => SchemyNode::ImportDecl {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::ImportDefaultSpecifier { node, parent } => SchemyNode::ImportDefaultSpecifier {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::Module { node, parent } => SchemyNode::Module {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::ModuleItem { node, parent } => SchemyNode::ModuleItem {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::TsEnumDecl { node, parent } => SchemyNode::TsEnumDecl {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::TsInterfaceDecl { node, parent } => SchemyNode::TsInterfaceDecl {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::TsTypeAliasDecl { node, parent } => SchemyNode::TsTypeAliasDecl {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
            SchemyNode::ImportSpecifier { node, parent } => SchemyNode::ImportSpecifier {
                node: node.clone(),
                parent: parent.as_ref().map(|p| p.clone()),
            },
        }
    }
}
