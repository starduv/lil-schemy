use std::path::PathBuf;

use es_resolve::{EsResolver, TargetEnv};

use crate::typescript::{Declaration, DeclarationTables, SchemyNode};

pub fn store_declaration_maybe<'m>(
    node: &SchemyNode<'m>,
    file_path: &str,
    symbol_tables: &mut DeclarationTables<'m>,
) -> () {
    match node {
        SchemyNode::ClassDecl {
            node: class_declaration,
            parent: _,
        } => {
            let name = class_declaration.ident.sym.to_string();
            symbol_tables.insert(file_path, name.to_string(), Declaration::Type { node: node.clone() })
        }
        SchemyNode::ExportDecl {
            node: export_declaration,
            parent: _,
        } => match &export_declaration.decl {
            deno_ast::swc::ast::Decl::Class(declaration) => {
                let name = declaration.ident.sym.to_string();
                symbol_tables.insert(
                    file_path,
                    name,
                    Declaration::Type {
                        node: SchemyNode::ClassDecl {
                            node: &declaration,
                            parent: None,
                        },
                    },
                )
            }
            deno_ast::swc::ast::Decl::TsInterface(interface) => {
                let name = interface.id.sym.to_string();
                symbol_tables.insert(
                    file_path,
                    name,
                    Declaration::Type {
                        node: SchemyNode::TsInterfaceDecl {
                            node: &interface,
                            parent: None,
                        },
                    },
                )
            }
            deno_ast::swc::ast::Decl::TsTypeAlias(type_alias) => {
                let name = type_alias.id.sym.to_string();
                symbol_tables.insert(
                    file_path,
                    name,
                    Declaration::Type {
                        node: SchemyNode::TsTypeAliasDecl {
                            node: &type_alias,
                            parent: None,
                        },
                    },
                )
            }
            deno_ast::swc::ast::Decl::TsEnum(enum_declaration) => {
                let name = enum_declaration.id.sym.to_string();
                symbol_tables.insert(
                    file_path,
                    name,
                    Declaration::Type {
                        node: SchemyNode::TsEnumDecl {
                            node: &enum_declaration,
                            parent: None,
                        },
                    },
                )
            }
            _ => {}
        },
        SchemyNode::ExportDefaultExpr {
            node: default_expression,
            parent: _,
        } => match &*default_expression.expr {
            deno_ast::swc::ast::Expr::Ident(identifier) => {
                let target_name = identifier.sym.to_string();
                symbol_tables.insert(
                    file_path,
                    "default".into(),
                    Declaration::Alias {
                        from: "default".into(),
                        to: target_name,
                    },
                )
            }
            _ => {}
        },
        SchemyNode::ExportDefaultDecl {
            node: default_declaration,
            parent: _,
        } => match &default_declaration.decl {
            deno_ast::swc::ast::DefaultDecl::Class(class_expression) => symbol_tables.insert(
                file_path,
                "default".into(),
                Declaration::Type {
                    node: SchemyNode::ClassExpr {
                        node: &class_expression,
                        parent: None,
                    },
                },
            ),
            deno_ast::swc::ast::DefaultDecl::TsInterfaceDecl(interface_declaration) => symbol_tables.insert(
                file_path,
                "default".into(),
                Declaration::Type {
                    node: SchemyNode::TsInterfaceDecl {
                        node: &interface_declaration,
                        parent: None,
                    },
                },
            ),
            _ => {}
        },
        node @ SchemyNode::ImportDecl {
            node: import_declaration,
            parent: _,
        } => {
            for child in node.children() {
                match child {
                    SchemyNode::ImportDefaultSpecifier {
                        node: specifier,
                        parent: _,
                    } => {
                        let src = import_declaration.src.value.to_string();
                        match EsResolver::new(&src, &PathBuf::from(file_path), TargetEnv::Node).resolve() {
                            Ok(module_path) => {
                                let name = specifier.local.sym.to_string();
                                symbol_tables.insert(
                                    file_path,
                                    name,
                                    Declaration::Import {
                                        name: String::from("default"),
                                        source_file_name: module_path,
                                    },
                                )
                            }
                            Err(err) => println!("'{}', module resolution error: {:?}", file_path, err),
                        }
                    }
                    SchemyNode::ImportNamedSpecifier {
                        node: specifier,
                        parent: _,
                    } => {
                        let src = import_declaration.src.value.to_string();
                        match EsResolver::new(&src, &PathBuf::from(file_path), TargetEnv::Node).resolve() {
                            Ok(module_path) => {
                                let name = specifier.local.sym.to_string();
                                symbol_tables.insert(
                                    file_path,
                                    name.to_string(),
                                    Declaration::Import {
                                        name,
                                        source_file_name: module_path,
                                    },
                                )
                            }
                            Err(err) => println!("'{}', module resolution error: {:?}", file_path, err),
                        }
                    }
                    _ => {}
                }
            }
        }
        SchemyNode::NamedExport {
            node: named_export,
            parent: _,
        } => {
            let src = &named_export.src.as_ref().unwrap().value;
            match EsResolver::new(&src, &PathBuf::from(file_path), TargetEnv::Node).resolve() {
                Ok(module_file_name) => {
                    for specifier in &named_export.specifiers {
                        match specifier {
                            deno_ast::swc::ast::ExportSpecifier::Named(named_specifier) => {
                                let type_name = match &named_specifier.orig {
                                    deno_ast::swc::ast::ModuleExportName::Ident(identifier) => &identifier.sym,
                                    deno_ast::swc::ast::ModuleExportName::Str(identifier) => &identifier.value,
                                };

                                if let Some(exported_name) = &named_specifier.exported {
                                    let exported_name = match exported_name {
                                        deno_ast::swc::ast::ModuleExportName::Ident(id) => &id.sym,
                                        deno_ast::swc::ast::ModuleExportName::Str(id) => &id.value,
                                    };

                                    symbol_tables.insert(
                                        file_path,
                                        exported_name.to_string(),
                                        Declaration::Import {
                                            name: type_name.to_string(),
                                            source_file_name: module_file_name.to_string(),
                                        },
                                    )
                                } else {
                                    symbol_tables.insert(
                                        file_path,
                                        type_name.to_string(),
                                        Declaration::Import {
                                            name: type_name.to_string(),
                                            source_file_name: module_file_name.to_string(),
                                        },
                                    )
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Err(err) => println!("'{}', module resolution error: {:?}", file_path, err),
            }
        }
        SchemyNode::TsInterfaceDecl {
            node: ts_interface_declaration,
            parent: _,
        } => {
            let name = &ts_interface_declaration.id.sym;
            symbol_tables.insert(
                file_path,
                name.to_string(),
                Declaration::Type {
                    node: SchemyNode::TsInterfaceDecl {
                        node: &ts_interface_declaration,
                        parent: None,
                    },
                },
            )
        }
        SchemyNode::TsTypeAliasDecl {
            node: ts_type_alias_declaration,
            parent: _,
        } => {
            let name = &ts_type_alias_declaration.id.sym;
            symbol_tables.insert(
                file_path,
                name.to_string(),
                Declaration::Type {
                    node: SchemyNode::TsTypeAliasDecl {
                        node: &ts_type_alias_declaration,
                        parent: None,
                    },
                },
            )
        }
        SchemyNode::VarDecl {
            node: variable_declaration,
            parent: _,
        } => {
            for declaration in &variable_declaration.decls {
                match &declaration.name {
                    deno_ast::swc::ast::Pat::Ident(identifier) => {
                        let name = identifier.id.sym.to_string();
                        match &identifier.type_ann {
                            Some(type_annotation) => match &*type_annotation.type_ann {
                                deno_ast::swc::ast::TsType::TsTypeRef(type_ref) => match &type_ref.type_name {
                                    deno_ast::swc::ast::TsEntityName::Ident(identifier) => {
                                        let type_name = identifier.sym.to_string();
                                        symbol_tables.insert(
                                            file_path,
                                            name.to_string(),
                                            Declaration::Alias {
                                                from: name,
                                                to: type_name,
                                            },
                                        )
                                    }
                                    _ => {}
                                },
                                _ => {}
                            },
                            None => match &declaration.init {
                                Some(initializer) => {
                                    store_variable(
                                        &name,
                                        SchemyNode::Expr {
                                            node: &initializer,
                                            parent: None,
                                        },
                                        file_path,
                                        symbol_tables,
                                    );
                                }
                                None => {}
                            },
                        }
                    }
                    _ => {}
                };
            }
        }
        _ => {}
    }
}

fn store_variable(name: &str, node: SchemyNode, file_path: &str, symbol_tables: &mut DeclarationTables) -> () {
    for child in node.children() {
        match child {
            SchemyNode::Ident {
                node: identifier,
                parent: _,
            } => {
                let type_name = identifier.sym.to_string();
                symbol_tables.insert(
                    file_path,
                    name.to_string(),
                    Declaration::Alias {
                        from: name.to_string(),
                        to: type_name,
                    },
                )
            }
            SchemyNode::TsTypeRef {
                node: type_ref,
                parent: _,
            } => match &type_ref.type_name {
                deno_ast::swc::ast::TsEntityName::Ident(identifier) => {
                    let type_name = identifier.sym.to_string();
                    symbol_tables.insert(
                        file_path,
                        name.to_string(),
                        Declaration::Alias {
                            from: name.to_string(),
                            to: type_name,
                        },
                    )
                }
                _ => {}
            },
            _ => store_variable(name, child, file_path, symbol_tables),
        }
    }
}
