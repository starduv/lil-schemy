use dprint_swc_ext::view::{Node, NodeTrait, Pat};

use crate::typescript::{Declaration, DeclarationTable};

// pub enum Declaration {
//     Alias { from: String, to: String },
//     Type { node: AstNode },
//     Export { name: String, module_ref: String },
//     Import { name: String, module_ref: String },
// }

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
            // let file_name = import_declaration.inner.src.value;
            // for specifier in import_declaration.inner.specifiers {
            //     match specifier {
            //         deno_ast::swc::ast::ImportSpecifier::Named(named_specifier) => {
            //             let name = named_specifier.local.sym.to_string();
            //             symbol_table.insert(
            //                 name,
            //                 Symbol {
            //                     file_name: Some(file_name.to_string()),
            //                     node: named_specifier,
            //                 },
            //             )
            //         }
            //         deno_ast::swc::ast::ImportSpecifier::Default(default_specifier) => todo!(),
            //         _ => {}
            //     }
            // }
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
                println!("{:?}", declaration.inner);
                let name = match declaration.name {
                    Pat::Ident(identifier) => identifier.id.sym().to_string(),
                    _ => "other".into(),
                };

                match declaration.init {
                    Some(dprint_swc_ext::view::Expr::Assign(assignment)) => {
                        println!("{:?}", assignment.inner);
                    }
                    Some(dprint_swc_ext::view::Expr::New(new_expression)) => {
                        println!("{:?}", new_expression.inner);
                    }
                    Some(dprint_swc_ext::view::Expr::Ident(identifier)) => {
                        println!("{:?}", identifier.inner);
                    }
                    Some(dprint_swc_ext::view::Expr::Class(class_declaration)) => {
                        println!("{:?}", class_declaration.inner);
                    }
                    Some(dprint_swc_ext::view::Expr::TsAs(as_expression)) => {
                        println!("{:?}", as_expression.inner);
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}
