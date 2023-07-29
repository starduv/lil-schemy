use std::{rc::Rc, vec};

use deno_ast::swc::ast::*;
use lazy_static::__Deref;

#[derive(Debug)]
pub enum SchemyNode<'m> {
    TsNamespaceExport {
        node: Rc<&'m TsNamespaceExportDecl>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    TsExportAssignment {
        node: Rc<&'m TsExportAssignment>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    TsImportEquals {
        node: Rc<&'m TsImportEqualsDecl>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
    ExportAll {
        node: Rc<&'m ExportAll>,
        parent: Option<Box<SchemyNode<'m>>>,
    },
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

impl<'m> SchemyNode<'m> {
    pub fn children(&self) -> Vec<SchemyNode> {
        match self {
            SchemyNode::ModuleItem { node, parent:_ } => match node.deref() {
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

impl<'m> Clone for SchemyNode<'m> {
    fn clone(&self) -> Self {
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
