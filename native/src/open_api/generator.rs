use std::collections::BTreeMap;

use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};

use crate::typescript::*;

use super::open_api::{ApiPathOperation, ApiSchema, OpenApi, PathArgs, ResponseOptions};

static mut DECLARATIONS: BTreeMap<String, HashMap<String, Declaration>> = BTreeMap::new();

fn add_body_parameter(
    operation: &mut ApiPathOperation,
    node: &AstNode,
    type_refs: &mut HashSet<TypeReference>,
    file_name: &String,
) -> () {
    let param = operation.body();
    let arguments = node.type_arguments.as_ref().unwrap();
    if let Some(type_arg) = arguments.get(0) {
        match type_arg.kind {
            NUMBER_KEYWORD => {
                param.content().schema().primitive("number");
            }
            STRING_KEYWORD => {
                param.content().schema().primitive("string");
            }
            BOOLEAN_KEYWORD => {
                param.content().schema().primitive("boolean");
            }
            _ => {
                let type_ref = get_identifier(type_arg, file_name);
                param.content().schema().reference(type_ref.clone(), false);
                if let Some(type_ref) = type_ref {
                    type_refs.insert(TypeReference {
                        name: type_ref,
                        namespace: match arguments.get(2) {
                            Some(namespace) => match namespace.literal {
                                Some(ref literal) => literal.text.clone(),
                                None => None,
                            },
                            None => None,
                        },
                    });
                }
            }
        };
    }

    if let Some(required) = arguments.get(1) {
        let literal = required.literal.as_ref().unwrap();
        param.required(literal.kind == TRUE_KEYWORD);
    }

    if let Some(namespace) = arguments.get(2) {
        if let Some(ref literal) = namespace.literal {
            let text = literal.text.clone();
            param.content().schema().namespace(text);
        }
    }

    if let Some(format) = arguments.get(3) {
        if let Some(ref literal) = format.literal {
            let text = literal.text.clone();
            param.content().schema().format(text);
        }
    }
}

fn add_operation_parameter(
    name: &str,
    location: &str,
    operation: &mut ApiPathOperation,
    node: &AstNode,
    type_refs: &mut HashSet<TypeReference>,
    file_name: &String,
) -> () {
    let param = operation.param(name, location);
    let arguments = node.type_arguments.as_ref().unwrap();
    if let Some(type_arg) = arguments.get(0) {
        match type_arg.kind {
            NUMBER_KEYWORD => {
                param.content().schema().primitive("number");
            }
            STRING_KEYWORD => {
                param.content().schema().primitive("string");
            }
            BOOLEAN_KEYWORD => {
                param.content().schema().primitive("boolean");
            }
            _ => {
                let type_ref = get_identifier(type_arg, file_name);
                param.content().schema().reference(type_ref.clone(), false);
                if let Some(type_ref) = type_ref {
                    type_refs.insert(TypeReference {
                        name: type_ref,
                        namespace: match arguments.get(2) {
                            Some(namespace) => match namespace.literal {
                                Some(ref literal) => literal.text.clone(),
                                None => None,
                            },
                            None => None,
                        },
                    });
                }
            }
        };
    }

    if let Some(required) = arguments.get(1) {
        let literal = required.literal.as_ref().unwrap();
        param.required(literal.kind == TRUE_KEYWORD);
    }

    if let Some(namespace) = arguments.get(2) {
        if let Some(ref literal) = namespace.literal {
            let text = literal.text.clone();
            param.content().schema().namespace(text);
        }
    }

    if let Some(format) = arguments.get(3) {
        if let Some(ref literal) = format.literal {
            let text = literal.text.clone();
            param.content().schema().format(text);
        }
    }
}

fn add_schema(open_api: &mut OpenApi, node: &AstNode, reference: &TypeReference) -> () {
    let schema = match reference.namespace {
        Some(ref namespace) => {
            let schema = open_api.components.schema(&namespace).object();
            schema.property(&reference.name).object()
        }
        None => open_api.components.schema(&reference.name).object(),
    };

    update_schema(schema, node);
}

fn update_schema(schema: &mut ApiSchema, node: &AstNode) -> () {
    match node.kind {
        ARRAY_TYPE => {
            let schema = schema.array().items();
            update_schema(schema, node.element_type.as_ref().unwrap());
        }
        CLASS_DECLARATION => node.for_each_child(|n| update_schema(schema, n)),
        INTERFACE_DECLARATION => node.for_each_child(|n| update_schema(schema, n)),
        PROPERTY_DECLARATION => {
            let name = node.name.as_ref().unwrap();
            let name_text = name.escaped_text.as_ref().unwrap();
            let schema = schema.property(&name_text);
            update_schema(schema, node._type.as_ref().unwrap());
        }
        PROPERTY_SIGNATURE => {
            let name = node.name.as_ref().unwrap();
            let name_text = name.escaped_text.as_ref().unwrap();
            let schema = schema.property(&name_text);
            update_schema(schema, node._type.as_ref().unwrap());
        }
        STRING_KEYWORD => {
            schema.primitive("string");
        }
        _ => {}
    }
}

fn cache_declarations(node: &AstNode, file_name: &str, module_map: &HashMap<String, String>) -> () {
    match node.kind {
        CLASS_DECLARATION => cache_object_type(node, file_name),
        IMPORT_DECLARATION => cache_import_declaration(node, file_name, module_map),
        EXPORT_DECLARATION => cache_export_declaration(node, file_name, module_map),
        INTERFACE_DECLARATION => cache_object_type(node, file_name),
        TYPE_ALIAS_DECLARATION => cache_object_type(node, file_name),
        VARIABLE_STATEMENT => cache_variables(node, file_name),
        _ => node.for_each_child(|n| cache_declarations(n, file_name, module_map)),
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

fn cache_import_declaration(node: &AstNode, file_name: &str, module_map: &HashMap<String, String>) -> () {
    unsafe {
        let declarations = DECLARATIONS.entry(file_name.to_owned()).or_insert(HashMap::new());
        let module_specifier = node.module_specifier.as_ref().unwrap();
        let module_reference = module_specifier.text.as_ref().unwrap();
        let absolute = module_map.get(module_reference).expect("Could not find module");

        let import_clause = node.import_clause.as_ref().unwrap();
        if let Some(ref name) = import_clause.name {
            let text = name.escaped_text.as_ref().unwrap();
            declarations.insert(
                text.to_string(),
                Declaration::Import {
                    name: String::from("default"),
                    file: absolute.to_string(),
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
                        file: absolute.to_string(),
                    },
                );
            })
        })
    }
}

fn cache_export_declaration(node: &AstNode, file_name: &str, module_map: &HashMap<String, String>) -> () {
    unsafe {
        let declarations = DECLARATIONS.entry(file_name.to_owned()).or_insert(HashMap::new());
        let module_specifier = node.module_specifier.as_ref().unwrap();
        let module_reference = module_specifier.text.as_ref().unwrap();
        let absolute = module_map.get(module_reference).expect("Could not find module");

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
                        file: absolute.to_string(),
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

fn find_type_name(name: &str, file_name: &String) -> String {
    unsafe {
        let mut key = name;
        let mut previous = "";
        let declarations = DECLARATIONS.get(file_name).unwrap();
        while previous != key && declarations.contains_key(key) {
            match declarations.get(key) {
                Some(Declaration::Alias { from: _, to }) => {
                    previous = key;
                    key = to;
                }
                _ => previous = key,
            }
        }

        key.to_string()
    }
}

fn get_declaration<'n>(
    reference: &str,
    file_name: &str,
    ast_map: &'n HashMap<String, AstNode>,
    module_map: &HashMap<String, String>,
) -> Option<&'n AstNode> {
    unsafe {
        if !DECLARATIONS.contains_key(file_name) {
            let source_file = ast_map.get(file_name).unwrap();
            cache_declarations(source_file, file_name, module_map);
        }

        let mut key = reference;
        let mut previous: &str = "";
        let declarations = DECLARATIONS.get(file_name).unwrap();
        while key != previous && declarations.contains_key(key) {
            match declarations.get(key) {
                Some(Declaration::Alias { from: _, to }) => {
                    previous = key;
                    key = to;
                }
                _ => previous = key,
            }
        }

        match declarations.get(key) {
            Some(Declaration::Export { name, file }) => get_declaration(name, file, ast_map, module_map),
            Some(Declaration::Import { name, file }) => get_declaration(name, file, ast_map, module_map),
            Some(Declaration::Type { node }) => Some(node),
            _ => None,
        }
    }
}

fn get_identifier(node: &AstNode, file_name: &String) -> Option<String> {
    if let Some(ref text) = node.escaped_text {
        return find_type_name(text, file_name).into();
    }

    if let Some(ref name) = node.name {
        let text = name.escaped_text.as_ref().unwrap();
        return find_type_name(text, file_name).into();
    }

    if let Some(ref expression) = node.expression {
        let text = expression.escaped_text.as_ref().unwrap();
        return find_type_name(text, file_name).into();
    }

    if let Some(ref type_name) = node.type_name {
        let text = type_name.escaped_text.as_ref().unwrap();
        return find_type_name(text, file_name).into();
    }

    None
}

fn get_path_args(node: &AstNode) -> PathArgs {
    let mut path_args = PathArgs::new();
    let properties = node.properties.as_ref().unwrap();
    for prop in properties {
        let name = prop.name.as_ref().unwrap().escaped_text.as_ref().unwrap();
        let initializer = prop.initializer.as_ref().unwrap();
        if name == "method" {
            path_args.method = match initializer.text {
                Some(ref text) => Some(text.clone()),
                None => None,
            };
        } else if name == "path" {
            path_args.path = match initializer.text {
                Some(ref text) => Some(text.clone()),
                None => None,
            };
        } else if name == "tags" {
            path_args.tags = match initializer.elements {
                Some(ref elements) => Some(elements.iter().map(|e| e.text.as_ref().unwrap().clone()).collect()),
                None => None,
            };
        }
    }
    path_args
}

fn get_request_parameter<'n>(node: &'n AstNode) -> &'n AstNode {
    let parameters = node.parameters.as_ref().unwrap();
    parameters.first().unwrap()
}

fn get_response_options(node: &AstNode, file_name: &String) -> ResponseOptions {
    let mut response_args = ResponseOptions::new();
    let properties = node.properties.as_ref().unwrap();
    for prop in properties {
        if let Some(ref_name) = get_identifier(prop, file_name) {
            match ref_name.as_str() {
                "description" => response_args.description = prop.initializer.as_ref().unwrap().text.clone(),
                "example" => response_args.example = prop.initializer.as_ref().unwrap().text.clone(),
                "namespace" => response_args.namespace = prop.initializer.as_ref().unwrap().text.clone(),
                "statusCode" => response_args.status_code = prop.initializer.as_ref().unwrap().text.clone(),
                _ => {}
            }
        }
    }

    response_args
}

pub struct OpenApiGenerator<'m> {
    ast_map: &'m HashMap<String, AstNode>,
    open_api: OpenApi,
    module_map: &'m HashMap<String, String>,
    references: HashSet<TypeReference>,
}
impl<'m> OpenApiGenerator<'m> {
    pub fn new(module_map: &'m HashMap<String, String>, ast_map: &'m HashMap<String, AstNode>) -> Self {
        OpenApiGenerator {
            ast_map,
            open_api: OpenApi::new(),
            module_map,
            references: HashSet::new(),
        }
    }

    pub(crate) fn result(&self) -> &super::open_api::OpenApi {
        &self.open_api
    }

    fn add_operation_response(&mut self, route: &str, method: &str, node: &AstNode, file_name: &String) -> () {
        if let Some(ref expression) = node.expression {
            if let Some(ref text) = expression.escaped_text {
                if text.eq("Response") {
                    let arguments = node.arguments.as_ref().unwrap();
                    let response_type_arg = arguments.first();
                    let response_options = arguments.last();
                    if let Some(mut response_type) = response_type_arg {
                        if let Some(response_options) = response_options {
                            if response_type.kind == TYPE_ASSERTION_EXPRESSION {
                                response_type = response_type._type.as_ref().unwrap();
                            }

                            if response_type.kind == AS_EXPRESSION {
                                response_type = response_type._type.as_ref().unwrap();
                            }

                            let response_options = get_response_options(response_options, file_name);
                            let operation = self.open_api.path(route).method(method);
                            let ref_name = get_identifier(&response_type, file_name);
                            let namespace = response_options.namespace.clone();
                            operation.response(&ref_name, response_options);

                            if let Some(ref ref_name) = ref_name {
                                self.references.insert(TypeReference {
                                    name: ref_name.to_owned(),
                                    namespace: namespace,
                                });
                            }
                        }
                    }
                }
            }
        } else {
            node.for_each_child(|node| self.add_operation_response(route, method, node, file_name))
        }
    }

    fn add_operation_params<'cx>(&mut self, route: &str, method: &str, node: &AstNode, file_name: &String) -> () {
        if let Some(ref _type) = node._type {
            if let Some(ref type_name) = _type.type_name {
                if let Some(ref text) = type_name.escaped_text {
                    let operation = self.open_api.path(&route).method(&method);

                    if text.eq("QueryParam") {
                        let property_name = node.name.as_ref().unwrap();
                        let name = property_name.escaped_text.as_ref().unwrap();
                        add_operation_parameter(&name, "query", operation, _type, &mut self.references, file_name)
                    }

                    if text.eq("RouteParam") {
                        let property_name = node.name.as_ref().unwrap();
                        let name = property_name.escaped_text.as_ref().unwrap();
                        add_operation_parameter(&name, "path", operation, _type, &mut self.references, file_name)
                    }

                    if text.eq("Header") {
                        let property_name = node.name.as_ref().unwrap();
                        let name = property_name.escaped_text.as_ref().unwrap();
                        add_operation_parameter(&name, "header", operation, _type, &mut self.references, file_name)
                    }

                    if text.eq("BodyParam") {
                        add_body_parameter(operation, _type, &mut self.references, file_name)
                    }
                }
            } else {
                _type.for_each_child(|node| self.add_operation_params(route, method, node, file_name))
            }
        } else {
            node.for_each_child(|node| self.add_operation_params(route, method, node, file_name))
        }
    }

    fn is_api_path(&self, node: &AstNode) -> bool {
        match node.expression {
            Some(ref expression) => match expression.escaped_text {
                Some(ref text) => text.eq("Path"),
                None => false,
            },
            _ => false,
        }
    }

    fn find_api_paths(&mut self, node: &AstNode, file_name: &String) -> () {
        if self.is_api_path(node) {
            let arguments = node.arguments.as_ref().unwrap();
            let route_handler = arguments.get(0).unwrap();
            let route_handler_body = route_handler.body.as_ref().unwrap();
            let request_param = get_request_parameter(route_handler);
            let path_options = get_path_args(arguments.get(1).unwrap());

            let route = path_options.path.unwrap();
            let method = path_options.method.unwrap();
            self.open_api.path(&route).method(&method).tags(path_options.tags);

            self.add_operation_params(&route, &method, request_param, file_name);
            self.add_operation_response(&route, &method, &*route_handler_body, file_name);
        } else {
            node.for_each_child(|node| self.find_api_paths(node, file_name))
        }
    }

    pub(crate) fn api_paths_from(&mut self, path: String) -> () {
        let source_file = self
            .ast_map
            .get(&path)
            .expect(&format!("Could not find ast for path '{}'", path));

        cache_declarations(source_file, &path, self.module_map);

        source_file.for_each_child(|node| self.find_api_paths(node, &path));

        let mut references = self.references.iter().collect::<Vec<&TypeReference>>();
        while references.is_empty() == false {
            let reference = references.pop().unwrap();
            if let Some(node) = get_declaration(&reference.name, &path, self.ast_map, self.module_map) {
                add_schema(&mut self.open_api, node, &reference);
            }
        }
    }
}
