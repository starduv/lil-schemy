use crate::typescript::{node::GetProperty, Node, NodeKind};

use super::{Symbol, SymbolTables};

use std::{
    path::PathBuf,
    rc::Rc,
    sync::{Arc, Weak},
};

use es_resolve::{EsResolver, TargetEnv};
use swc_ecma_ast::{Callee, ExportSpecifier, Expr, ImportSpecifier, ModuleExportName, Pat, TsEntityName, TsType};

impl SymbolTables {
    pub fn store_declaration_maybe(&mut self, root: Arc<Node>, file_path: &str) -> () {
        match root.kind {
            NodeKind::ModuleItem(_) => {
                for child in root.children(Arc::downgrade(&root)) {
                    self.store_declaration_maybe(child, file_path)
                }
            }
            NodeKind::ExportDecl(_) => {
                for child in root.children(Arc::downgrade(&root)) {
                    self.store_declaration_maybe(child, file_path)
                }
            }
            NodeKind::ExportDefaultExpr(_) => {
                for child in root.children(Arc::downgrade(&root)) {
                    self.store_default_declaration(child, file_path)
                }
            }
            NodeKind::Decl(_) => {
                for child in root.children(Arc::downgrade(&root)) {
                    self.store_declaration_maybe(child, file_path)
                }
            }
            NodeKind::ClassDecl(raw) => {
                let name = raw.ident.sym.to_string();
                self.insert(
                    file_path,
                    name.to_string(),
                    Symbol::Type {
                        node: Arc::clone(&root),
                    },
                )
            }
            NodeKind::TsInterfaceDecl(raw) => {
                let name = raw.id.sym.to_string();
                self.insert(
                    file_path,
                    name,
                    Symbol::Type {
                        node: Arc::clone(&root),
                    },
                )
            }
            NodeKind::TsTypeAliasDecl(raw) => {
                let name = raw.id.sym.to_string();
                self.insert(
                    file_path,
                    name,
                    Symbol::Type {
                        node: Arc::clone(&root),
                    },
                )
            }
            NodeKind::TsEnumDecl(raw) => {
                let name = raw.id.sym.to_string();
                self.insert(
                    file_path,
                    name,
                    Symbol::Type {
                        node: Arc::clone(&root),
                    },
                )
            }
            NodeKind::ExportDefaultDecl(raw_decl) => {
                if let Some(raw_default) = root.find_child(|child| match child.kind {
                    NodeKind::DefaultDecl(_) => true,
                    _ => false,
                }) {
                    if let Some(class_or_interface) = raw_default.find_child(|child| match child.kind {
                        NodeKind::Class(_) => true,
                        NodeKind::TsInterfaceDecl(_) => true,
                        _ => false,
                    }) {
                        self.insert(
                            file_path,
                            "default".into(),
                            Symbol::Type {
                                node: Arc::clone(&class_or_interface),
                            },
                        )
                    }
                }
            }
            NodeKind::ImportDecl(raw) => {
                for child in root.children(Arc::downgrade(&root)) {
                    match child.kind {
                        NodeKind::ImportSpecifier(ImportSpecifier::Default(raw_specifier)) => {
                            let src = raw.src.value.to_string();
                            match EsResolver::new(&src, &PathBuf::from(file_path), TargetEnv::Node).resolve() {
                                Ok(module_path) => {
                                    let name = raw_specifier.local.sym.to_string();
                                    self.insert(
                                        file_path,
                                        name,
                                        Symbol::Import {
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
                                    self.insert(
                                        file_path,
                                        name.to_string(),
                                        Symbol::Import {
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

                                                self.insert(
                                                    file_path,
                                                    exported_name.to_string(),
                                                    Symbol::Import {
                                                        name: type_name.to_string(),
                                                        source_file_name: module_path.replace(".js", ".d.ts"),
                                                    },
                                                )
                                            } else {
                                                self.insert(
                                                    file_path,
                                                    type_name.to_string(),
                                                    Symbol::Import {
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
                if let Some(ident) = root.find_child(|child| match child.kind {
                    NodeKind::Ident(_) => true,
                    _ => false,
                }) {
                    let name = ident
                        .get_property::<&str>(|ident| match ident.kind {
                            NodeKind::Ident(raw) => Some(&raw.sym),
                            _ => None,
                        })
                        .unwrap();

                    if let Some(annotation) = ident.find_child(|child| match child.kind {
                        NodeKind::TsTypeRef(type_ref) => true,
                        NodeKind::TsTypeLit(type_lit) => true,
                        _ => false,
                    }) {
                        if let Some(alias) = annotation.find_child(|child| match child.kind {
                            NodeKind::Ident(_) => true,
                            _ => false,
                        }) {
                            let alias = alias
                                .get_property(|alias| match alias.kind {
                                    NodeKind::Ident(raw) => Some(raw.sym),
                                    _ => None,
                                })
                                .unwrap();

                            self.insert(file_path, name.to_string(), Symbol::Alias { to: alias.to_string() })
                        } else {
                            self.insert(
                                file_path,
                                name.to_string(),
                                Symbol::Type {
                                    node: Arc::clone(&annotation),
                                },
                            )
                        }
                    } else if let Some(initializer) = root.find_child(|child| match child.kind {
                        NodeKind::Expr(_) => true,
                        _ => false,
                    }) {
                        self.store_variable(&name, initializer, file_path)
                    }
                }
            }
            _ => {}
        }
    }

    fn store_default_declaration(&mut self, root: Arc<Node>, file_path: &str) -> () {
        match root.kind {
            NodeKind::CallExpr(raw_call) => match &raw_call.callee {
                Callee::Expr(raw_callee) => match &**raw_callee {
                    Expr::Ident(raw_ident) => self.insert(
                        file_path,
                        "default".into(),
                        Symbol::Alias {
                            to: raw_ident.sym.to_string(),
                        },
                    ),
                    _ => {}
                },
                _ => {}
            },
            NodeKind::ArrayLit(_) => self.insert(
                file_path,
                "default".into(),
                Symbol::Type {
                    node: Arc::clone(&root),
                },
            ),
            NodeKind::ObjectLit(_) => self.insert(
                file_path,
                "default".into(),
                Symbol::Type {
                    node: Arc::clone(&root),
                },
            ),
            NodeKind::NewExpr(expr) => match &*expr.callee {
                Expr::Ident(raw_ident) => self.insert(
                    file_path,
                    "default".into(),
                    Symbol::Alias {
                        to: raw_ident.sym.to_string(),
                    },
                ),
                _ => {}
            },
            NodeKind::Ident(raw_ident) => self.insert(
                file_path,
                "default".into(),
                Symbol::Alias {
                    to: raw_ident.sym.to_string(),
                },
            ),
            NodeKind::ArrowExpr(_) => self.insert(
                file_path,
                "default".into(),
                Symbol::Type {
                    node: Arc::clone(&root),
                },
            ),
            NodeKind::ClassExpr(expr) => match &expr.ident {
                Some(raw_ident) => self.insert(
                    file_path,
                    "default".into(),
                    Symbol::Alias {
                        to: raw_ident.sym.to_string(),
                    },
                ),
                None => {}
            },
            NodeKind::TsAsExpr(raw_expr) => match &*raw_expr.type_ann {
                TsType::TsTypeRef(raw_ref) => match &raw_ref.type_name {
                    TsEntityName::Ident(raw_ident) => self.insert(
                        file_path,
                        "default".into(),
                        Symbol::Alias {
                            to: raw_ident.sym.to_string(),
                        },
                    ),
                    _ => {}
                },
                _ => {}
            },
            NodeKind::TsInstantiationExpr(raw_expr) => match &*raw_expr.expr {
                Expr::Ident(raw_ident) => self.insert(
                    file_path,
                    "default".into(),
                    Symbol::Alias {
                        to: raw_ident.sym.to_string(),
                    },
                ),
                _ => {}
            },
            _ => {}
        }
    }

    fn store_variable(&mut self, name: &str, root: Arc<Node>, file_path: &str) -> () {
        for child in root.children(Arc::downgrade(&root)) {
            match child.kind {
                NodeKind::Ident(raw) => {
                    let type_name = raw.sym.to_string();
                    self.insert(file_path, name.to_string(), Symbol::Alias { to: type_name })
                }
                NodeKind::TsTypeRef(raw) => match &raw.type_name {
                    TsEntityName::Ident(identifier) => {
                        let type_name = identifier.sym.to_string();
                        self.insert(
                            file_path,
                            name.to_string(),
                            Symbol::Alias {
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
