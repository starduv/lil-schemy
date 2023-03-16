use std::collections::BTreeMap;

use ahash::{HashMap, HashMapExt};

use crate::typescript::*;

pub static mut DECLARATIONS: BTreeMap<String, HashMap<String, Declaration>> = BTreeMap::new();

pub fn cache_declarations(node: &AstNode, file_name: &str) -> () {
    match node.kind {
        CLASS_DECLARATION => cache_object_type(node, file_name),
        IMPORT_DECLARATION => cache_import_declaration(node, file_name),
        EXPORT_DECLARATION => cache_export_declaration(node, file_name),
        INTERFACE_DECLARATION => cache_object_type(node, file_name),
        TYPE_ALIAS_DECLARATION => cache_object_type(node, file_name),
        VARIABLE_STATEMENT => cache_variables(node, file_name),
        _ => node.for_each_child(|n| cache_declarations(n, file_name)),
        // _ => {}
    }
}

fn cache_object_type(node: &AstNode, file_name: &str) -> () {
    unsafe {
        let name = node.name.as_ref().unwrap();
        let text = match node.modifiers {
            Some(ref modifiers) => match modifiers.iter().find(|n| n.kind == DEFAULT_KEYWORD) {
                Some(_) => "default",
                None => name.escaped_text.as_ref().unwrap(),
            },
            None => name.escaped_text.as_ref().unwrap(),
        };

        let declarations = DECLARATIONS.entry(file_name.to_string()).or_insert(HashMap::new());
        declarations.insert(text.to_string(), Declaration::Type { node: node.clone() });
    }
}

fn cache_import_declaration(node: &AstNode, file_name: &str) -> () {
    unsafe {
        let declarations = DECLARATIONS.entry(file_name.to_owned()).or_insert(HashMap::new());
        let module_specifier = node.module_specifier.as_ref().unwrap();
        let module_reference = module_specifier.text.as_ref().unwrap();

        let import_clause = node.import_clause.as_ref().unwrap();
        if let Some(ref name) = import_clause.name {
            let text = name.escaped_text.as_ref().unwrap();
            declarations.insert(
                text.to_string(),
                Declaration::Import {
                    name: String::from("default"),
                    module_ref: module_reference.to_string(),
                },
            );
        }

        import_clause.for_each_child(|bindings| {
            bindings.for_each_child(|element| {
                let name = element.name.as_ref().unwrap();
                let name_text = name.escaped_text.as_ref().unwrap();
                let alias = match element.property_name {
                    Some(ref node) => node.escaped_text.as_ref().unwrap(),
                    None => name_text,
                };

                declarations.insert(
                    name_text.to_string(),
                    Declaration::Import {
                        name: alias.to_string(),
                        module_ref: module_reference.to_string(),
                    },
                );
            })
        })
    }
}

fn cache_export_declaration(node: &AstNode, file_name: &str) -> () {
    unsafe {
        let declarations = DECLARATIONS.entry(file_name.to_owned()).or_insert(HashMap::new());
        let module_specifier = node.module_specifier.as_ref().unwrap();
        let module_reference = module_specifier.text.as_ref().unwrap();

        let export_clause = node.export_clause.as_ref().unwrap();

        export_clause.for_each_child(|exports| {
            exports.for_each_child(|element| {
                let name = element.name.as_ref().unwrap();
                let name_text = name.escaped_text.as_ref().unwrap();
                let alias = match element.property_name {
                    Some(ref node) => node.escaped_text.as_ref().unwrap(),
                    None => name_text,
                };

                declarations.insert(
                    name_text.to_string(),
                    Declaration::Export {
                        name: alias.to_string(),
                        module_ref: module_reference.to_string(),
                    },
                );
            })
        })
    }
}

fn cache_variables(node: &AstNode, file_name: &str) -> () {
    unsafe {
        let declarations = DECLARATIONS.get_mut(file_name).unwrap();
        if let Some(ref list) = node.declaration_list {
            list.for_each_child(|declaration| {
                let initializer = declaration.initializer.as_ref().unwrap();
                let name = declaration.name.as_ref().unwrap();
                let text = name.escaped_text.as_ref().unwrap();
                match initializer.kind {
                    AS_EXPRESSION => {
                        let _type = initializer._type.as_ref().unwrap();
                        let type_name = _type.type_name.as_ref().unwrap();
                        let type_name_text = type_name.escaped_text.as_ref().unwrap();
                        declarations.insert(
                            text.to_string(),
                            Declaration::Alias {
                                from: text.to_string(),
                                to: type_name_text.to_string(),
                            },
                        );
                    }
                    TYPE_ASSERTION_EXPRESSION => {
                        let _type = initializer._type.as_ref().unwrap();
                        let type_name = _type.type_name.as_ref().unwrap();
                        let type_name_text = type_name.escaped_text.as_ref().unwrap();
                        declarations.insert(
                            text.to_string(),
                            Declaration::Alias {
                                from: text.to_string(),
                                to: type_name_text.to_string(),
                            },
                        );
                    }
                    CALL_EXPRESSION => {
                        let expression = initializer.expression.as_ref().unwrap();
                        let expression_text = expression.escaped_text.as_ref().unwrap();
                        declarations.insert(
                            text.to_string(),
                            Declaration::Alias {
                                from: text.to_string(),
                                to: expression_text.to_string(),
                            },
                        );
                    }
                    NEW_EXPRESSION => {
                        let expression = initializer.expression.as_ref().unwrap();
                        let expression_text = expression.escaped_text.as_ref().unwrap();
                        declarations.insert(
                            text.to_string(),
                            Declaration::Alias {
                                from: text.to_string(),
                                to: expression_text.to_string(),
                            },
                        );
                    }
                    _ => {
                        declarations.insert(text.to_string(), Declaration::Type { node: node.clone() });
                    }
                }
            })
        }
    }
}
