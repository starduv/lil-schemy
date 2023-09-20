#![allow(dead_code)]

mod mappers;
mod typescript;
mod writer;

use mappers::{open_api::OpenApiResult, Mapper};
use neon::{prelude::*, result::Throw};
use serde_json::json;

pub fn generate_schemas_debug(open_api_options: mappers::open_api::OpenApiOptions) -> Option<OpenApiResult> {
    let (request_module, on_module_requested) = crossbeam::channel::unbounded();
    let (send_node, on_node_sent) = crossbeam::channel::unbounded();
    let handle = mappers::open_api::OpenApiMapper::run(Some(open_api_options), request_module, on_node_sent);

    if let Some(handle) = handle {
        let open_api = handle.join().unwrap();
        return Some(open_api);
    }

    None
}

fn generate_schemas(mut cx: FunctionContext) -> Result<Handle<JsObject>, Throw> {
    let schemas_result: Handle<JsObject> = cx.empty_object();
    let options_handle: Handle<JsObject> = cx.argument(0)?;
    let (request_module, on_module_requested) = crossbeam::channel::unbounded();
    let (send_node, on_node_sent) = crossbeam::channel::unbounded();

    let open_api_handle: Option<Handle<JsObject>> = options_handle.get_opt(&mut cx, "open_api")?;
    let options = mappers::open_api::OpenApiOptions::from_js_object(&mut cx, open_api_handle);
    let handle = mappers::open_api::OpenApiMapper::run(options, request_module, on_node_sent);

    if let Some(handle) = handle {
        let open_api = handle.join().unwrap();
        let open_api_result = cx.empty_object();
        if let Some(filepath) = open_api.filepath {
            let filepath = cx.string(filepath);
            open_api_result.set(&mut cx, "filepath", filepath)?;
        }

        let schema = cx.string(json!(open_api.schema).to_string());
        open_api_result.set(&mut cx, "schema", schema)?;
        schemas_result.set(&mut cx, "openApi", open_api_result)?;
    }

    Ok(schemas_result)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("generateSchemas", generate_schemas)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        generate_schemas_debug,
        mappers::open_api::OpenApiOptions,
    };

    #[test]
    fn sends_open_api_options_to_open_api_mapper() {
        generate_schemas_debug(OpenApiOptions {
            output: None,
            base: String::from("{}"),
            paths: vec!["./test/fixtures/".to_string()],
        });
    }
}
