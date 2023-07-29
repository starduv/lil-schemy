mod deferred_schemas;
mod declaration_helpers;
mod open_api;

use std::{fs::File, io::Write, path::PathBuf};

use neon::{prelude::*, result::Throw};
use serde_json::json;

use self::open_api::{OpenApi, OpenApiFactory};

fn merge_schemas(
    open_api: &OpenApi,
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

fn generate_schema(open_api_handle: Handle<JsObject>, cx: &mut FunctionContext) -> Result<String, Throw> {
    let mut factory = OpenApiFactory::new();
    let paths = open_api_handle.get::<JsArray, FunctionContext, &str>(cx, "entry")?;

    let mut result = OpenApi::new();
    for path in paths.to_vec(cx)? {
        let path = path.downcast_or_throw::<JsString, _>(cx)?.value(cx);
        let temp = factory.from_source_file(&path);
        result.merge(temp);
    }

    merge_schemas(&mut result, open_api_handle, cx)
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

            let filepath = cx.string(filepath.to_str().unwrap());
            schema_result.set(cx, "filepath", filepath)?;
        } else {
            let schema = cx.string(schema);
            schema_result.set(cx, "schema", schema)?;
        }
    }

    schemas_result.set(cx, "openApi", schema_result)?;

    Ok(())
}
