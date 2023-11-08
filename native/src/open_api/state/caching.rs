use std::{path::PathBuf, rc::Rc};

use es_resolve::{EsResolver, TargetEnv};
use swc_ecma_ast::{Callee, ExportSpecifier, Expr, ImportSpecifier, ModuleExportName, Pat, TsEntityName, TsType};

use crate::typescript::{NodeKind, SchemyNode};

use super::{declaration_table::Declaration, Store};

impl Store {
    pub fn add_child_scope(&mut self, file_path: &str) -> () {
        self.symbol_tables.add_child_scope(file_path)
    }

    pub fn parent_scope(&mut self, file_path: &str) -> () {
        self.symbol_tables.parent_scope(file_path)
    }

    pub fn store_declaration_maybe(&mut self, root: Rc<SchemyNode<'static>>, file_path: &str) -> () {
        match root.kind {
            NodeKind::TsTypeRef(raw_ref) => match &raw_ref.type_name {
                TsEntityName::Ident(identifier) => {
                    let type_name = identifier.sym.to_string();
                    self.symbol_tables.insert(
                        file_path,
                        type_name.to_string(),
                        Declaration::Type { node: root.clone() },
                    );
                }
                _ => {}
            },
            NodeKind::ModuleItem(_) => {
                for child in root.children() {
                    self.store_declaration_maybe(child, file_path)
                }
            }
            NodeKind::ExportDecl(_) => {
                for child in root.children() {
                    self.store_declaration_maybe(child, file_path)
                }
            }
            NodeKind::ExportDefaultExpr(_) => {
                for child in root.children() {
                    self.store_default_declaration(child, file_path)
                }
            }
            NodeKind::Decl(_) => {
                for child in root.children() {
                    self.store_declaration_maybe(child, file_path)
                }
            }
            NodeKind::ClassDecl(raw) => {
                let name = raw.ident.sym.to_string();
                self.symbol_tables
                    .insert(file_path, name.to_string(), Declaration::Type { node: root.clone() })
            }
            NodeKind::TsInterfaceDecl(raw) => {
                let name = raw.id.sym.to_string();
                self.symbol_tables
                    .insert(file_path, name, Declaration::Type { node: root.clone() })
            }
            NodeKind::TsTypeAliasDecl(raw) => {
                let name = raw.id.sym.to_string();
                self.symbol_tables
                    .insert(file_path, name, Declaration::Type { node: root.clone() })
            }
            NodeKind::TsEnumDecl(raw) => {
                let name = raw.id.sym.to_string();
                self.symbol_tables
                    .insert(file_path, name, Declaration::Type { node: root.clone() })
            }
            NodeKind::ExportDefaultDecl(raw_decl) => {
                match &raw_decl.decl {
                    swc_ecma_ast::DefaultDecl::Class(raw_class) => {
                        let class = root.to_child(NodeKind::Class(&*raw_class.class));

                        self.symbol_tables.insert(
                            file_path,
                            "default".into(),
                            Declaration::Type { node: class.clone() },
                        )
                    }
                    swc_ecma_ast::DefaultDecl::TsInterfaceDecl(raw_int) => {
                        let interface = root.to_child(NodeKind::TsInterfaceDecl(&*raw_int));
                        self.symbol_tables.insert(
                            file_path,
                            "default".into(),
                            Declaration::Type {
                                node: interface.clone(),
                            },
                        )
                    }
                    _ => {}
                };
            }
            NodeKind::ImportDecl(raw) => {
                for child in root.children() {
                    match child.kind {
                        NodeKind::ImportSpecifier(ImportSpecifier::Default(raw_specifier)) => {
                            let src = raw.src.value.to_string();
                            match EsResolver::new(&src, &PathBuf::from(file_path), TargetEnv::Node).resolve() {
                                Ok(module_path) => {
                                    let name = raw_specifier.local.sym.to_string();
                                    self.symbol_tables.insert(
                                        file_path,
                                        name,
                                        Declaration::Import {
                                            name: String::from("default"),
                                            source_file_name: module_path.replace(".js", ".d.ts"),
                                        },
                                    )
                                }
                                Err(_) => {} // TODO improve debugging
                            }
                        }
                        NodeKind::ImportSpecifier(ImportSpecifier::Named(raw_specifier)) => {
                            let src = raw.src.value.to_string();
                            match EsResolver::new(&src, &PathBuf::from(file_path), TargetEnv::Node).resolve() {
                                Ok(module_path) => {
                                    let name = &raw_specifier.local.sym;
                                    self.symbol_tables.insert(
                                        file_path,
                                        name.to_string(),
                                        Declaration::Import {
                                            name: name.to_string(),
                                            source_file_name: module_path.replace(".js", ".d.ts"),
                                        },
                                    )
                                }
                                Err(_) => {} // TODO improve debugging
                            }
                        }
                        _ => {}
                    }
                }
            }
            NodeKind::NamedExport(raw) => {
                match &raw.src.as_ref() {
                    Some(src) => {
                        let src = &src.value;
                        match EsResolver::new(&src, &PathBuf::from(file_path), TargetEnv::Node).resolve() {
                            Ok(module_path) => {
                                for specifier in &raw.specifiers {
                                    match specifier {
                                        ExportSpecifier::Named(named_specifier) => {
                                            let type_name = match &named_specifier.orig {
                                                ModuleExportName::Ident(identifier) => &identifier.sym,
                                                ModuleExportName::Str(identifier) => &identifier.value,
                                            };

                                            if let Some(exported_name) = &named_specifier.exported {
                                                let exported_name = match exported_name {
                                                    ModuleExportName::Ident(id) => &id.sym,
                                                    ModuleExportName::Str(id) => &id.value,
                                                };

                                                self.symbol_tables.insert(
                                                    file_path,
                                                    exported_name.to_string(),
                                                    Declaration::Import {
                                                        name: type_name.to_string(),
                                                        source_file_name: module_path.replace(".js", ".d.ts"),
                                                    },
                                                )
                                            } else {
                                                self.symbol_tables.insert(
                                                    file_path,
                                                    type_name.to_string(),
                                                    Declaration::Import {
                                                        name: type_name.to_string(),
                                                        source_file_name: module_path.replace(".js", ".d.ts"),
                                                    },
                                                )
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            Err(_) => {} // TODO improve debugging
                        }
                    }
                    None => {}
                }
            }
            NodeKind::VarDeclarator(raw) => {
                match &raw.name {
                    Pat::Ident(identifier) => {
                        let name = identifier.id.sym.to_string();
                        match &identifier.type_ann {
                            Some(type_annotation) => match &*type_annotation.type_ann {
                                TsType::TsTypeRef(type_ref) => match &type_ref.type_name {
                                    TsEntityName::Ident(identifier) => self.symbol_tables.insert(
                                        file_path,
                                        name.to_string(),
                                        Declaration::Alias {
                                            to: identifier.sym.to_string(),
                                        },
                                    ),
                                    _ => {}
                                },
                                TsType::TsTypeLit(raw_type) => self.symbol_tables.insert(
                                    file_path,
                                    name.to_string(),
                                    Declaration::Type {
                                        node: root.to_child(NodeKind::TsTypeLit(raw_type)).clone(),
                                    },
                                ),
                                _ => {}
                            },
                            None => match &raw.init {
                                Some(initializer) => {
                                    let node = root.to_child(NodeKind::Expr(initializer));
                                    self.store_variable(&name, node, file_path);
                                }
                                None => {}
                            },
                        }
                    }
                    _ => {}
                };
            }
            _ => {}
        }
    }

    fn store_default_declaration(&mut self, root: Rc<SchemyNode<'static>>, file_path: &str) -> () {
        match root.kind {
            NodeKind::CallExpr(raw_call) => match &raw_call.callee {
                Callee::Expr(raw_callee) => match &**raw_callee {
                    Expr::Ident(raw_ident) => self.symbol_tables.insert(
                        file_path,
                        "default".into(),
                        Declaration::Alias {
                            to: raw_ident.sym.to_string(),
                        },
                    ),
                    _ => {}
                },
                _ => {}
            },
            NodeKind::ArrayLit(_) => {
                self.symbol_tables
                    .insert(file_path, "default".into(), Declaration::Type { node: root.clone() })
            }
            NodeKind::ObjectLit(_) => {
                self.symbol_tables
                    .insert(file_path, "default".into(), Declaration::Type { node: root.clone() })
            }
            NodeKind::NewExpr(expr) => match &*expr.callee {
                Expr::Ident(raw_ident) => self.symbol_tables.insert(
                    file_path,
                    "default".into(),
                    Declaration::Alias {
                        to: raw_ident.sym.to_string(),
                    },
                ),
                _ => {}
            },
            NodeKind::Ident(raw_ident) => self.symbol_tables.insert(
                file_path,
                "default".into(),
                Declaration::Alias {
                    to: raw_ident.sym.to_string(),
                },
            ),
            NodeKind::ArrowExpr(_) => {
                self.symbol_tables
                    .insert(file_path, "default".into(), Declaration::Type { node: root })
            }
            NodeKind::ClassExpr(expr) => match &expr.ident {
                Some(raw_ident) => self.symbol_tables.insert(
                    file_path,
                    "default".into(),
                    Declaration::Alias {
                        to: raw_ident.sym.to_string(),
                    },
                ),
                None => {}
            },
            NodeKind::TsAsExpr(raw_expr) => match &*raw_expr.type_ann {
                TsType::TsTypeRef(raw_ref) => match &raw_ref.type_name {
                    TsEntityName::Ident(raw_ident) => self.symbol_tables.insert(
                        file_path,
                        "default".into(),
                        Declaration::Alias {
                            to: raw_ident.sym.to_string(),
                        },
                    ),
                    _ => {}
                },
                _ => {}
            },
            NodeKind::TsInstantiationExpr(raw_expr) => match &*raw_expr.expr {
                Expr::Ident(raw_ident) => self.symbol_tables.insert(
                    file_path,
                    "default".into(),
                    Declaration::Alias {
                        to: raw_ident.sym.to_string(),
                    },
                ),
                _ => {}
            },
            _ => {}
        }
    }

    fn store_variable(&mut self, name: &str, root: Rc<SchemyNode>, file_path: &str) -> () {
        for child in root.children() {
            match child.kind {
                NodeKind::Ident(raw) => {
                    let type_name = raw.sym.to_string();
                    self.symbol_tables
                        .insert(file_path, name.to_string(), Declaration::Alias { to: type_name })
                }
                NodeKind::TsTypeRef(raw) => match &raw.type_name {
                    TsEntityName::Ident(identifier) => {
                        let type_name = identifier.sym.to_string();
                        self.symbol_tables.insert(
                            file_path,
                            name.to_string(),
                            Declaration::Alias {
                                to: type_name.to_string(),
                            },
                        );
                    }
                    _ => {}
                },
                _ => self.store_variable(name, child, file_path),
            }
        }
    }
}
