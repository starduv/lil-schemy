use std::rc::Rc;

use deno_ast::swc::ast::*;

use super::{NodeKind, SchemyNode};

impl<'n> SchemyNode<'n> {
    pub fn children(&self) -> Vec<usize> {
        let mut children = vec![];
        match self.kind {
            NodeKind::ExportDecl(raw) => self.get_export_declartion_children(raw, &self.index, &mut children),
            NodeKind::ExportDefaultDecl(raw) => self.get_export_default_decl_children(raw, &self.index, &mut children),
            NodeKind::ExportDefaultExpr(raw) => self.get_export_default_expr_children(raw, &self.index, &mut children),
            NodeKind::ModuleItem(raw) => self.get_module_item_children(raw, &self.index, &mut children),
            NodeKind::Module(raw) => self.get_module_children(raw, &self.index, &mut children),
            NodeKind::ImportDecl(raw) => self.get_import_decl_children(raw, &self.index, &mut children),
            NodeKind::Pat(raw) => self.get_pat_children(raw, &self.index, &mut children),
            NodeKind::BlockStmt(raw) => self.get_statement_children(raw, &self.index, &mut children),
            NodeKind::TsTypeAnnotation(raw) => self.get_type_annotation_children(raw, &self.index, &mut children),
            NodeKind::VarDecl(raw) => self.get_var_decl_children(raw, &self.index, &mut children),
            _ => {}
        }
        children
    }

    fn get_source_children(&self, source: &'n Module, index: &usize, children: &mut Vec<usize>) {
        let mut borrow = self.context.borrow_mut();
        let child_index = borrow.nodes.len();
        let child_node = SchemyNode {
            index: child_index,
            parent_index: Some(index.clone()),
            kind: NodeKind::Module(source),
            context: self.context.clone(),
        };
        borrow.nodes.push(Rc::new(child_node));
        children.push(child_index);
    }

    fn get_export_declartion_children(&self, export_decl: &'n ExportDecl, index: &usize, children: &mut Vec<usize>) {
        match &export_decl.decl {
            Decl::Class(declaration) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    index: child_index,
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
                    kind: NodeKind::TsModuleDecl(declaration),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            _ => {}
        }
    }

    fn get_export_default_expr_children(
        &self,
        expression: &'n ExportDefaultExpr,
        index: &usize,
        children: &mut Vec<usize>,
    ) {
        match &*expression.expr {
            deno_ast::swc::ast::Expr::Ident(identifier) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::Ident(identifier),
                    index: child_index,
                    parent_index: Some(index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            _ => {}
        }
    }

    fn get_export_default_decl_children(
        &self,
        export_declaration: &'n ExportDefaultDecl,
        index: &usize,
        children: &mut Vec<usize>,
    ) {
        match &export_declaration.decl {
            deno_ast::swc::ast::DefaultDecl::Class(declaration) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::ClassExpr(&declaration),
                    index: child_index,
                    parent_index: Some(index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            deno_ast::swc::ast::DefaultDecl::TsInterfaceDecl(declaration) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::TsInterfaceDecl(&declaration),
                    index: child_index,
                    parent_index: Some(index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            _ => {}
        }
    }

    fn get_import_decl_children(&self, import_declaration: &'n ImportDecl, index: &usize, children: &mut Vec<usize>) {
        let mut borrow = self.context.borrow_mut();
        for specifier in &import_declaration.specifiers {
            let child_index = borrow.nodes.len();
            let child_node = SchemyNode {
                kind: NodeKind::ImportSpecifier(&specifier),
                index: child_index,
                parent_index: Some(index.clone()),
                context: self.context.clone(),
            };
            borrow.nodes.push(Rc::new(child_node));
            children.push(child_index);
        }
    }

    fn get_module_children(&self, module: &'n Module, index: &usize, children: &mut Vec<usize>) {
        let mut borrow = self.context.borrow_mut();
        module.body.iter().for_each(|item| {
            let child_index = borrow.nodes.len();
            let child_node = SchemyNode {
                kind: NodeKind::ModuleItem(item),
                index: child_index,
                parent_index: Some(index.clone()),
                context: self.context.clone(),
            };
            borrow.nodes.push(Rc::new(child_node));
            children.push(child_index);
        })
    }

    fn get_module_item_children(&self, module_item: &'n ModuleItem, index: &usize, children: &mut Vec<usize>) {
        match module_item {
            ModuleItem::ModuleDecl(declaration) => match declaration {
                ModuleDecl::Import(declaration) => {
                    let mut borrow = self.context.borrow_mut();
                    let child_index = borrow.nodes.len();
                    let child_node = SchemyNode {
                        kind: NodeKind::ImportDecl(&declaration),
                        index: child_index,
                        parent_index: Some(index.clone()),
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
                        parent_index: Some(index.clone()),
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
                        parent_index: Some(index.clone()),
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
                        parent_index: Some(index.clone()),
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
                        parent_index: Some(index.clone()),
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
                        parent_index: Some(index.clone()),
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
                        parent_index: Some(index.clone()),
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
                        parent_index: Some(index.clone()),
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
                        parent_index: Some(index.clone()),
                        context: self.context.clone(),
                    };
                    borrow.nodes.push(Rc::new(child_node));
                    children.push(child_index);
                }
            },
            _ => {}
        }
    }

    fn get_pat_children(&self, pat: &'n Pat, index: &usize, children: &mut Vec<usize>) {
        match pat {
            Pat::Ident(ident) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::TsTypeAnnotation(ident.type_ann.as_ref().unwrap()),
                    index: child_index,
                    parent_index: Some(index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
            _ => {}
        }
    }

    fn get_statement_children(&self, block_stmnt: &'n BlockStmt, index: &usize, children: &mut Vec<usize>) {
        block_stmnt.stmts.iter().for_each(|statement| match statement {
            Stmt::Block(block_stmt) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::BlockStmt(block_stmt),
                    index: child_index,
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
        })
    }

    fn get_type_annotation_children(&self, type_annotation: &'n TsTypeAnn, index: &usize, children: &mut Vec<usize>) {
        match &*type_annotation.type_ann {
            TsType::TsKeywordType(ts_keyword_type) => {
                let mut borrow = self.context.borrow_mut();
                let child_index = borrow.nodes.len();
                let child_node = SchemyNode {
                    kind: NodeKind::TsKeywordType(&ts_keyword_type),
                    index: child_index,
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
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
                    parent_index: Some(index.clone()),
                    context: self.context.clone(),
                };
                borrow.nodes.push(Rc::new(child_node));
                children.push(child_index);
            }
        }
    }

    fn get_var_decl_children(&self, variable_declaration: &'n VarDecl, index: &usize, children: &mut Vec<usize>) {
        let mut borrow = self.context.borrow_mut();
        variable_declaration.decls.iter().for_each(|decl| {
            let child_index = borrow.nodes.len();
            let child_node = SchemyNode {
                kind: NodeKind::VarDeclarator(decl),
                index: child_index,
                parent_index: Some(index.clone()),
                context: self.context.clone(),
            };
            borrow.nodes.push(Rc::new(child_node));
            children.push(child_index);
        })
    }
}
