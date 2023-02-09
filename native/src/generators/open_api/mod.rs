mod open_api_v3;
mod typescript;

use std::{fs::File, io::Write, path::PathBuf};

use neon::{prelude::*, result::Throw};
use serde_json::json;

use self::{
    open_api_v3::{ApiPathOperation, OpenApiV3, Param, PathArgs, ResponseOptions, ValueType},
    typescript::TsNode,
};

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

fn get_response_type(type_arg: Option<&TsNode>, cx: &mut FunctionContext) -> Result<Option<String>, Throw> {
    match type_arg {
        Some(type_arg) => type_arg.get_identifier(cx),
        None => Ok(None),
    }
}

fn get_response_options(options: Option<&TsNode>, cx: &mut FunctionContext) -> Result<ResponseOptions, Throw> {
    let options = options.expect("Response requires options object");
    let mut response_args = ResponseOptions::new();
    for prop in options.get_properties(cx)? {
        if let Some(prop_name) = prop.get_identifier(cx)? {
            if prop_name == "description" {
                response_args.description = prop.get_initialized_string(cx)?;
            } else if prop_name == "example" {
                response_args.example = prop.get_initialized_string(cx)?;
            } else if prop_name == "namespace" {
                response_args.namespace = prop.get_initialized_string(cx)?;
            } else if prop_name == "statusCode" {
                response_args.status_code = prop.get_initialized_string(cx)?;
            }
        }
    }
    Ok(response_args)
}

fn get_path_args(root: &mut TsNode, cx: &mut FunctionContext) -> Result<PathArgs, Throw> {
    let mut path_args = PathArgs::new();
    for prop in root.get_properties(cx)? {
        if let Some(arg_name) = prop.get_identifier(cx)? {
            if arg_name == "method" {
                path_args.method = prop.get_initialized_string(cx)?;
            } else if arg_name == "path" {
                path_args.path = prop.get_initialized_string(cx)?;
            } else if arg_name == "tags" {
                path_args.tags = prop.get_initialized_array(cx)?;
            }
        }
    }

    Ok(path_args)
}

fn add_operation_response(
    root: &mut TsNode,
    operation: &mut ApiPathOperation,
    cx: &mut FunctionContext,
) -> Result<(), Throw> {
    if root.is_api_response(cx)? {
        let response_args = root.get_arguments(cx)?;
        let response_type_arg = response_args.first();
        let response_options_arg = response_args.last();
        if response_type_arg.is_some() && response_options_arg.is_some() {
            let response_options = get_response_options(response_options_arg, cx)?;
            let response = operation.response(response_options);

            if let Some(response_type) = get_response_type(response_type_arg, cx)? {
                let schema = response.content().schema().reference(response_type);
                // TODO register schema so that component schema is created for ref type
            }
        }
    } else {
        for mut child in root.get_children(cx)? {
            add_operation_response(&mut child, operation, cx)?;
        }
    }
    Ok(())
}

fn add_operation_parameter(
    name: String,
    location: &str,
    operation: &mut ApiPathOperation,
    node: TsNode,
    cx: &mut FunctionContext,
) -> Result<(), Throw> {
    let header = operation.param(name, location);
    if let Some(mut args) = node.get_type_arguments(cx)? {
        if let Some(type_arg) = args.get_mut(0) {
            match type_arg.get_value_type(cx)? {
                ValueType::Primitive(type_name, format) => {
                    header.content().schema().primitive(type_name).format(format);
                }
                ValueType::Reference(ref_name) => {
                    header.content().schema().reference(ref_name);
                }
                _ => {}
            }
        }

        if let Some(required) = args.get_mut(1) {
            header.required(required.get_boolean_keyword(cx)?);
        }
    }
    Ok(())
}

fn add_operation_params(
    root: &mut TsNode,
    operation: &mut ApiPathOperation,
    cx: &mut FunctionContext,
) -> Result<(), Throw> {
    match root.get_param(cx)? {
        Param::Header(name, type_obj) => add_operation_parameter(name, "header", operation, type_obj, cx)?,
        Param::Query(name, type_obj) => add_operation_parameter(name, "query", operation, type_obj, cx)?,
        Param::Route(name, type_obj) => add_operation_parameter(name, "path", operation, type_obj, cx)?,
        _ => {}
    }
    Ok(())
}

fn add_api_paths<'cx>(open_api: &mut OpenApiV3, root: &mut TsNode, cx: &mut FunctionContext<'cx>) -> Result<(), Throw> {
    if root.is_api_path(cx)? {
        let path_args = get_path_args(root, cx)?;
        let route = path_args.path.expect("Property 'path' of PathOptions is required");
        let mut operation = open_api.path(route).method(path_args.method);
        operation.tags(path_args.tags);

        for mut property in root.get_arguments(cx)? {
            add_operation_params(&mut property, &mut operation, cx)?;
        }

        add_operation_response(&mut root.get_body(cx)?, &mut operation, cx)?;
    } else {
        for mut child in root.get_children(cx)? {
            add_api_paths(open_api, &mut child, cx)?;
        }
    }

    Ok(())
}

fn generate_schema(open_api_handle: Handle<JsObject>, cx: &mut FunctionContext) -> Result<String, Throw> {
    let get_ast = open_api_handle.get::<JsFunction, FunctionContext, &str>(cx, "get_ast")?;
    let paths = open_api_handle.get::<JsArray, FunctionContext, &str>(cx, "paths")?;
    let mut open_api = OpenApiV3::new();

    for path in paths.to_vec(cx)? {
        let path = path.downcast_or_throw::<JsString, FunctionContext>(cx)?;
        let ast = get_ast.call_with(cx).arg(path).apply::<JsObject, FunctionContext>(cx)?;
        add_api_paths(&mut open_api, &mut TsNode::new(ast), cx)?;
    }

    merge_schemas(&open_api, open_api_handle, cx)
}

pub fn generate_openapi(
    schemas_result: Handle<JsObject>,
    options_handle: Handle<JsObject>,
    cx: &mut FunctionContext,
) -> Result<(), Throw> {
    let schema_result: Handle<JsObject> = cx.empty_object();
    if let Some(open_api_handle) = options_handle.get_opt(cx, "openApi")? as Option<Handle<JsObject>> {
        let schema: String = generate_schema(open_api_handle, cx)?;

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
