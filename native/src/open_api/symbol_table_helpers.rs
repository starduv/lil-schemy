use dprint_swc_ext::view::{Node, NodeTrait, Pat};

use crate::typescript::{Declaration, DeclarationTable};

pub fn store_symbol_maybe<'n>(node: &Node<'n>, symbol_table: &mut DeclarationTable<'n>) -> () {
    match node {
        Node::ClassDecl(class_declaration) => {
            println!("{:?}", class_declaration.inner);
        }
        Node::ExportDecl(export_declaration) => {
            println!("{:?}", export_declaration.inner);
        }
        Node::ExportDefaultDecl(export_default_declaration) => {
            println!("{:?}", export_default_declaration.inner);
        }
        Node::FnDecl(function_declaration) => {
            println!("{:?}", function_declaration.inner);
        }
        Node::ImportDecl(import_declaration) => {
            for child in import_declaration.children() {
                match child {
                    Node::ImportNamedSpecifier(specifier) => {
                        // TODO this will need module resolution
                        let src = import_declaration.src.value().to_string();
                        let name = specifier.local.sym().to_string();
                        symbol_table.insert(
                            name.to_string(),
                            Declaration::Import {
                                name,
                                source_file_name: src,
                            },
                        )
                    }
                    _ => {}
                }
            }
        }
        Node::TsEnumDecl(ts_enum_declaration) => {
            println!("{:?}", ts_enum_declaration.inner);
        }
        Node::TsInterfaceDecl(ts_interface_declaration) => {
            println!("{:?}", ts_interface_declaration.inner);
        }
        Node::TsTypeAliasDecl(ts_type_alias_declaration) => {
            println!("{:?}", ts_type_alias_declaration.inner);
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
                                        symbol_table.insert(
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
                                    store_variable(&name, initializer.as_node(), symbol_table);
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

fn store_variable(name: &str, node: Node, symbol_table: &mut DeclarationTable) -> () {
    for child in node.children() {
        match child {
            Node::Ident(identifier) => {
                let type_name = identifier.sym().to_string();
                symbol_table.insert(
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
                    symbol_table.insert(
                        name.to_string(),
                        Declaration::Alias {
                            from: name.to_string(),
                            to: type_name,
                        },
                    )
                }
                _ => {}
            },
            _ => store_variable(name, child, symbol_table),
        }
    }
}
