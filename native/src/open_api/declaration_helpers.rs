use std::path::PathBuf;

use dprint_swc_ext::view::{Node, NodeTrait, Pat};
use es_resolve::{EsResolver, TargetEnv};

use crate::typescript::{Declaration, DeclarationTables};

pub fn store_declaration_maybe<'n>(node: &Node<'n>, file_path: &str, symbol_tables: &mut DeclarationTables<'n>) -> () {
    match node {
        Node::ClassDecl(class_declaration) => {
            let name = class_declaration.ident.sym();
            symbol_tables.insert(
                file_path,
                name.to_string(),
                Declaration::Type {
                    node: class_declaration.as_node(),
                },
            )
        }
        Node::ExportDecl(export_declaration) => match export_declaration.decl {
            dprint_swc_ext::view::Decl::Class(declaration) => {
                let name = declaration.ident.sym().to_string();
                symbol_tables.insert(
                    file_path,
                    name,
                    Declaration::Type {
                        node: export_declaration.decl.as_node(),
                    },
                )
            }
            dprint_swc_ext::view::Decl::TsInterface(declaration) => {
                let name = declaration.id.sym().to_string();
                symbol_tables.insert(
                    file_path,
                    name,
                    Declaration::Type {
                        node: export_declaration.decl.as_node(),
                    },
                )
            }
            dprint_swc_ext::view::Decl::TsTypeAlias(declaration) => {
                let name = declaration.id.sym().to_string();
                symbol_tables.insert(
                    file_path,
                    name,
                    Declaration::Type {
                        node: export_declaration.decl.as_node(),
                    },
                )
            }
            dprint_swc_ext::view::Decl::TsEnum(declaration) => {
                let name = declaration.id.sym().to_string();
                symbol_tables.insert(
                    file_path,
                    name,
                    Declaration::Type {
                        node: export_declaration.decl.as_node(),
                    },
                )
            }
            _ => {}
        },
        Node::ExportDefaultExpr(default_expression) => match default_expression.expr {
            dprint_swc_ext::view::Expr::Ident(identifier) => {
                let target_name = identifier.sym().to_string();
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
        Node::ExportDefaultDecl(default_declaration) => match default_declaration.decl {
            dprint_swc_ext::view::DefaultDecl::Class(class_declaration) => symbol_tables.insert(
                file_path,
                "default".into(),
                Declaration::Type {
                    node: class_declaration.as_node(),
                },
            ),
            dprint_swc_ext::view::DefaultDecl::TsInterfaceDecl(interface_declaration) => symbol_tables.insert(
                file_path,
                "default".into(),
                Declaration::Type {
                    node: interface_declaration.as_node(),
                },
            ),
            _ => {}
        },
        Node::ImportDecl(import_declaration) => {
            for child in import_declaration.children() {
                match child {
                    Node::ImportDefaultSpecifier(specifier) => {
                        let src = import_declaration.src.value().to_string();
                        match EsResolver::new(&src, &PathBuf::from(file_path), TargetEnv::Node).resolve() {
                            Ok(module_path) => {
                                let name = specifier.local.sym().to_string();
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
                    Node::ImportNamedSpecifier(specifier) => {
                        let src = import_declaration.src.value().to_string();
                        match EsResolver::new(&src, &PathBuf::from(file_path), TargetEnv::Node).resolve() {
                            Ok(module_path) => {
                                let name = specifier.local.sym().to_string();
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
        Node::NamedExport(named_export) => {
            let src = named_export.src.unwrap().value();
            match EsResolver::new(&src, &PathBuf::from(file_path), TargetEnv::Node).resolve() {
                Ok(module_file_name) => {
                    for specifier in &named_export.specifiers {
                        match specifier {
                            dprint_swc_ext::view::ExportSpecifier::Default(_) => todo!(),
                            dprint_swc_ext::view::ExportSpecifier::Named(named_specifier) => {
                                let type_name = match named_specifier.orig {
                                    dprint_swc_ext::view::ModuleExportName::Ident(identifier) => identifier.sym(),
                                    dprint_swc_ext::view::ModuleExportName::Str(identifier) => identifier.value(),
                                };

                                if let Some(exported_name) = named_specifier.exported {
                                    let exported_name = match exported_name {
                                        dprint_swc_ext::view::ModuleExportName::Ident(id) => id.sym(),
                                        dprint_swc_ext::view::ModuleExportName::Str(id) => id.value(),
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
        Node::TsEnumDecl(ts_enum_declaration) => {
            println!("Should I store this enum?");
        }
        Node::TsInterfaceDecl(ts_interface_declaration) => {
            let name = ts_interface_declaration.id.sym();
            symbol_tables.insert(
                file_path,
                name.to_string(),
                Declaration::Type {
                    node: ts_interface_declaration.as_node(),
                },
            )
        }
        Node::TsTypeAliasDecl(ts_type_alias_declaration) => {
            let name = ts_type_alias_declaration.id.sym();
            symbol_tables.insert(
                file_path,
                name.to_string(),
                Declaration::Type {
                    node: ts_type_alias_declaration.type_ann.as_node(),
                },
            )
        }
        Node::VarDecl(variable_declaration) => {
            for declaration in &variable_declaration.decls {
                match declaration.name {
                    Pat::Ident(identifier) => {
                        let name = identifier.id.sym().to_string();
                        match identifier.type_ann {
                            Some(type_annotation) => match type_annotation.type_ann {
                                dprint_swc_ext::view::TsType::TsTypeRef(type_ref) => match type_ref.type_name {
                                    dprint_swc_ext::view::TsEntityName::Ident(identifier) => {
                                        let type_name = identifier.sym().to_string();
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
                            None => match declaration.init {
                                Some(initializer) => {
                                    store_variable(&name, initializer.as_node(), file_path, symbol_tables);
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

fn store_variable(name: &str, node: Node, file_path: &str, symbol_tables: &mut DeclarationTables) -> () {
    for child in node.children() {
        match child {
            Node::Ident(identifier) => {
                let type_name = identifier.sym().to_string();
                symbol_tables.insert(
                    file_path,
                    name.to_string(),
                    Declaration::Alias {
                        from: name.to_string(),
                        to: type_name,
                    },
                )
            }
            Node::TsTypeRef(type_ref) => match type_ref.type_name {
                dprint_swc_ext::view::TsEntityName::Ident(identifier) => {
                    let type_name = identifier.sym().to_string();
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
