use ahash::{HashSet, HashSetExt};
use neon::prelude::FunctionContext;

use crate::typescript::*;

use super::open_api::{ApiPathOperation, ApiSchema, OpenApi, PathArgs, ResponseOptions};

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
        BOOLEAN_KEYWORD => {
            schema.primitive("boolean");
        }
        NUMBER_KEYWORD => {
            schema.primitive("number");
        }
        BIG_INT_KEYWORD => {
            schema.primitive("number");
        }
        _ => {}
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

pub fn get_declaration<'n, F>(
    reference: &str,
    module_ref: &str,
    src_file_name: &String,
    cx: &mut FunctionContext,
    get_ast: &mut F,
) -> Option<&'n AstNode>
where
    F: FnMut(&str, &str, &mut FunctionContext) -> AstNode,
{
    unsafe {
        if !DECLARATIONS.contains_key(module_ref) {
            let source_file = get_ast(module_ref, src_file_name, cx);
            cache_declarations(&source_file, module_ref);
        }

        let mut key = reference;
        let mut previous: &str = "";
        let declarations = DECLARATIONS.get(module_ref).unwrap();
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
            Some(Declaration::Export { name, module_ref }) => {
                get_declaration(name, module_ref, src_file_name, cx, get_ast)
            }
            Some(Declaration::Import { name, module_ref }) => {
                get_declaration(name, module_ref, src_file_name, cx, get_ast)
            }
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

pub struct OpenApiGenerator<F: FnMut(&str, &str, &mut FunctionContext) -> AstNode> {
    get_ast: F,
    open_api: OpenApi,
    references: HashSet<TypeReference>,
}
impl<F: FnMut(&str, &str, &mut FunctionContext) -> AstNode> OpenApiGenerator<F> {
    pub fn new(get_ast: F) -> Self {
        OpenApiGenerator {
            get_ast,
            open_api: OpenApi::new(),
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

    fn find_api_paths(&mut self, node: &AstNode, file_name: &String, cx: &mut FunctionContext) -> () {
        if self.is_api_path(node) {
            let arguments = node.arguments.as_ref().unwrap();
            let route_handler = arguments.get(0).unwrap();

            let path_options = get_path_args(arguments.get(1).unwrap());
            let route = path_options.path.unwrap();
            let method = path_options.method.unwrap();
            self.open_api.path(&route).method(&method).tags(path_options.tags);

            let request_param = get_request_parameter(route_handler);
            self.add_operation_params(&route, &method, request_param, file_name);

            let route_handler_body = route_handler.body.as_ref().unwrap();
            self.add_operation_response(&route, &method, &*route_handler_body, file_name);
        } else {
            node.for_each_child(|node| self.find_api_paths(node, file_name, cx))
        }
    }

    pub(crate) fn api_paths_from(&mut self, path: String, cx: &mut FunctionContext) -> () {
        let source_file = (self.get_ast)(&path, &path, cx);

        cache_declarations(&source_file, &path);

        source_file.for_each_child(|node| self.find_api_paths(node, &path, cx));

        let mut references = self.references.iter().collect::<Vec<&TypeReference>>();
        while references.is_empty() == false {
            let reference = references.pop().unwrap();
            if let Some(node) = get_declaration(&reference.name, &path, &path, cx, &mut self.get_ast) {
                add_schema(&mut self.open_api, node, &reference);
            }
        }
    }
}
