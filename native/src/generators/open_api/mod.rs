mod open_api_v3;

use std::{fs::File, io::Write, path::PathBuf};

use ahash::{HashMap, HashMapExt};
use neon::{prelude::*, result::Throw};
use serde_json::json;

use crate::typescript::*;

use self::open_api_v3::{ApiPathOperation, OpenApiV3};

struct Resolver<'d> {
    cache: HashMap<String, TsDeclaration<'d>>,
}
impl<'d> Resolver<'d> {
    pub fn new() -> Self {
        Resolver { cache: HashMap::new() }
    }

    pub fn cache_declaration(&mut self, key: &str, declaration: TsDeclaration<'d>) -> () {
        self.cache.insert(String::from(key), declaration);
    }

    fn get_declartion(&self, key: &str) -> Option<&TsDeclaration> {
        self.cache.get(key)
    }
}

fn merge_schemas(
    open_api: &OpenApiV3,
    open_api_handle: Handle<JsObject>,
    cx: &mut FunctionContext,
) -> Result<String, Throw> {
    let base_handle: Handle<JsString> = open_api_handle.get(cx, "base")?;
    let base = serde_json::from_str(&base_handle.value(cx)).expect("Could not deserialize base schema");
    let mut generated = json!(open_api);
    merge(&mut generated, &base);
    Ok(generated.to_string())
}

fn merge(target: &mut serde_json::Value, overlay: &serde_json::Value) {
    if target.is_object() && overlay.is_object() {
        let target = target.as_object_mut().unwrap();
        let overlay = overlay.as_object().unwrap();
        for (key, value) in overlay {
            if target.contains_key(key) {
                let gen_value = target.get_mut(key).unwrap();
                merge(gen_value, value);
            } else {
                target.insert(key.clone(), value.clone());
            }
        }
    } else {
        target.clone_from(overlay);
    }
}

fn find_type_name(name: &str, resolver: &Resolver) -> String {
    if let Some(declaration) = resolver.get_declartion(name) {
        match declaration.declaration_type {
            DeclarationType::Alias => {
                return find_type_name(&declaration.name, resolver);
            }
            DeclarationType::Variable => {
                return find_type_name(&declaration.name, resolver);
            }
            _ => {}
        }
    }

    name.to_string()
}

fn get_identifier(cursor: &AstCursor, resolver: &Resolver) -> Option<String> {
    if cursor.has_property("escpaedText") {
        return Some(find_type_name(cursor.get_str("escapedText"), resolver));
    }

    if cursor.has_property("name") {
        return Some(find_type_name(cursor.move_to("name").get_str("escapedText"), resolver));
    }

    if cursor.has_property("expression") {
        return Some(find_type_name(
            cursor.move_to("expression").get_str("escapedText"),
            resolver,
        ));
    }

    None
}

fn cache_declarations(cursor: &mut AstCursor, resolver: &mut Resolver) -> () {
    let kind = cursor.get_kind();
    if kind == IMPORT_DECLARATION {
        cursor.move_to("declarationList").move_to("declarations");
        for d in cursor.iter() {
            d.move_to("importClause");
            if d.has_property("name") {
                let default_name = d.get_cursor("name").get_str("escaptedText");
                resolver.cache_declaration(
                    default_name,
                    TsDeclaration {
                        declaration_type: DeclarationType::DefaultImport,
                        name: default_name.to_string(),
                        node: d.clone(),
                    },
                )
            }

            for element in d.move_to("namedBindings").move_to("elements").iter() {
                let name = element.get_cursor("name").get_str("escapedText");
                if element.has_property("propertyName") {
                    let real_name = element.move_to("propertyName").get_str("escapedText");
                    resolver.cache_declaration(
                        name,
                        TsDeclaration {
                            declaration_type: DeclarationType::Alias,
                            name: real_name.to_string(),
                            node: element.clone(),
                        },
                    )
                } else {
                    resolver.cache_declaration(
                        name,
                        TsDeclaration {
                            declaration_type: DeclarationType::NamedImport,
                            name: name.to_string(),
                            node: element.clone(),
                        },
                    )
                }
            }
        }
    } else if kind == VARIABLE_STATEMENT {
        for declaration in cursor.move_to("declarationList").move_to("declarations").iter() {
            let name = declaration.get_cursor("name").get_str("escapedText");
            declaration.move_to("initializer");
            resolver.cache_declaration(
                name,
                TsDeclaration {
                    declaration_type: DeclarationType::Variable,
                    name: name.to_string(),
                    node: declaration.clone(),
                },
            )
        }
    } else if kind == INTERFACE_DECLARATION || kind == CLASS_DECLARATION || kind == TYPE_ALIAS_DECLARATION {
        let name = cursor.get_cursor("name").get_str("escaptedText");
        let initializer = cursor.get_cursor("initializer");
        resolver.cache_declaration(
            name,
            TsDeclaration {
                declaration_type: DeclarationType::Structual,
                name: name.to_string(),
                node: initializer.clone(),
            },
        )
    }
}

fn get_response_options(cursor: &mut AstCursor, resolver: &Resolver) -> ResponseOptions {
    let mut response_args = ResponseOptions::new();
    for prop in cursor.move_to("properties").iter() {
        if let Some(prop_name) = get_identifier(&prop.clone(), resolver) {
            if prop_name == "description" {
                response_args.description = Some(prop.move_to("initializer").get_str("text").to_string());
            } else if prop_name == "example" {
                response_args.example = Some(prop.move_to("initializer").get_str("text").to_string());
            } else if prop_name == "namespace" {
                response_args.namespace = Some(prop.move_to("initializer").get_str("text").to_string());
            } else if prop_name == "statusCode" {
                response_args.status_code = Some(prop.move_to("initializer").get_str("text").to_string());
            }
        }
    }

    response_args
}

fn get_path_args(cursor: &AstCursor) -> PathArgs {
    let mut path_args = PathArgs::new();
    for prop in cursor.move_to("properties").iter() {
        let name = prop.get_cursor("name").get_str("escapedText");
        let initializer = prop.get_cursor("initializer");
        if name == "method" {
            path_args.method = Some(initializer.get_str("text").to_string());
        } else if name == "path" {
            path_args.path = Some(initializer.get_str("text").to_string());
        } else if name == "tags" {
            path_args.tags = Some(initializer.get_vec("elements", |e| e.get_str("text").to_string()));
        }
    }

    path_args
}

fn add_operation_response(
    cursor: &mut AstCursor,
    operation: &mut ApiPathOperation,
    resolver: &mut Resolver,
) -> Result<(), Throw> {
    if cursor.has_property("expression") && cursor.get_cursor("expression").get_str("escaptedText").eq("Response") {
        cursor.move_to("arguments");
        if cursor.has_property("0") && cursor.has_property("1") {
            let response_type_arg = cursor.get_cursor("0");
            let mut response_options_arg = cursor.get_cursor("1");
            let response_options = get_response_options(&mut response_options_arg, resolver);
            let response = operation.response(response_options);
            let schema = response.content().schema();
            if let Some(ref_name) = get_identifier(&response_type_arg, resolver) {
                schema.reference(ref_name);
            }
            // TODO register need for references
        }
    } else {
        for child in cursor.iter() {
            cache_declarations(&mut child, resolver);
            add_operation_response(&mut child, operation, resolver);
        }
    }
    Ok(())
}

fn add_operation_parameter(
    name: &str,
    location: &str,
    operation: &mut ApiPathOperation,
    cursor: &mut AstCursor,
) -> Result<(), Throw> {
    let param = operation.param(name, location);
    cursor.move_to("typeArguments");
    if cursor.has_index(0) {
        let type_arg = cursor.get_cursor("0");
        let type_name = type_arg.get_cursor("typeName").get_str("escpadText");
        match type_arg.get_cursor("0").get_kind() {
            NUMBER_KEYWORD => param.content().schema().primitive(type_name).format(None),
            STRING_KEYWORD => param.content().schema().primitive(type_name).format(None),
            _ => param.content().schema().reference(String::from("")),
        };
    }

    if cursor.has_index(1) {
        let mut required = cursor.get_cursor("1");
        let kind = required.move_to("literal").get_kind();
        param.required(kind == TRUE_KEYWORD);
    }

    Ok(())
}

fn add_operation_params<'cx>(
    cursor: &mut AstCursor,
    operation: &mut ApiPathOperation,
    resolver: &mut Resolver,
) -> Result<(), Throw> {
    if cursor.has_property("members") {
        for member in cursor.move_to("members").iter() {
            let name = member.get_cursor("propertyName").get_str("escaptedText");
            let mut param_type = member.get_cursor("type");
            let type_name = param_type.get_cursor("typeName").get_str("escaptedText");
            if type_name == "QueryParam" {
                add_operation_parameter(name, "query", operation, &mut param_type);
            }

            if type_name == "RouteParam" {
                add_operation_parameter(name, "path", operation, &mut param_type);
            }

            if type_name == "Header" {
                add_operation_parameter(name, "header", operation, &mut param_type);
            }
        }
    } else {
        for child in cursor.iter() {
            cache_declarations(&mut child, resolver);
            add_operation_params(&mut child, operation, resolver);
        }
    }

    Ok(())
}

fn add_api_paths<'cx>(open_api: &mut OpenApiV3, root: &mut AstCursor, resolver: &mut Resolver) -> () {
    if root.is_api_path() {
        root.move_to("arguments");
        let mut route_handler = root.get_cursor("0");
        let path_args = get_path_args(&root.get_cursor("1"));
        let route = path_args.path.expect("Property 'path' of PathOptions is required");
        let mut operation = open_api.path(route).method(path_args.method);
        operation.tags(path_args.tags);

        let mut parameters = route_handler.get_cursor("parameters");
        add_operation_params(parameters.move_to("0"), &mut operation, resolver);
        add_operation_response(route_handler.move_to("body"), &mut operation, resolver);
    } else {
        for child in root.iter() {
            cache_declarations(&mut child, resolver);
            add_api_paths(open_api, &mut child, resolver);
        }
    }
}

fn generate_schema(
    open_api_handle: Handle<JsObject>,
    asts: &serde_json::Value,
    cx: &mut FunctionContext,
) -> Result<String, Throw> {
    let mut resolver = Resolver::new();
    let paths = open_api_handle.get::<JsArray, FunctionContext, &str>(cx, "paths")?;
    let mut open_api = OpenApiV3::new();

    for path in paths.to_vec(cx)? {
        let path = path.downcast_or_throw::<JsString, _>(cx)?.value(cx);
        let value = asts.get(&path).expect(&format!("Could not fined '{}'", path));
        let mut node = AstCursor::new(value);
        for statement in node.iter() {
            add_api_paths(&mut open_api, &mut statement, &mut resolver)
        }
    }

    merge_schemas(&open_api, open_api_handle, cx)
}

pub fn generate_openapi(
    schemas_result: Handle<JsObject>,
    options_handle: Handle<JsObject>,
    asts: &serde_json::Value,
    cx: &mut FunctionContext,
) -> Result<(), Throw> {
    let schema_result: Handle<JsObject> = cx.empty_object();
    if let Some(open_api_handle) = options_handle.get_opt(cx, "openApi")? as Option<Handle<JsObject>> {
        let schema: String = generate_schema(open_api_handle, asts, cx)?;

        if let Some(output_handle) = open_api_handle.get_opt::<JsString, FunctionContext, &str>(cx, "output")? {
            let filepath = match options_handle.get_opt::<JsString, FunctionContext, &str>(cx, "cwd")? {
                Some(cwd) => {
                    let mut path = PathBuf::from(cwd.value(cx));
                    path.push(output_handle.value(cx));
                    path
                }
                None => PathBuf::from(output_handle.value(cx)),
            };

            let mut file: File = File::create(filepath.clone()).expect("Could not create filepath: ");
            file.write_all(schema.as_bytes()).expect("Could not write to file");

            let is_file = cx.boolean(true);
            let filepath = cx.string(filepath.to_str().unwrap());
            schema_result.set(cx, "isFile", is_file)?;
            schema_result.set(cx, "filepath", filepath)?;
        } else {
            let is_file = cx.boolean(false);
            let schema = cx.string(schema);
            schema_result.set(cx, "isFile", is_file)?;
            schema_result.set(cx, "schema", schema)?;
        }
    }

    schemas_result.set(cx, "openApi", schema_result)?;

    Ok(())
}
