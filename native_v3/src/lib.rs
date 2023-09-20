#![allow(dead_code)]

mod mappers;
mod typescript;
mod writer;

use mappers::{open_api::OpenApiResult, Mapper};
use neon::{prelude::*, result::Throw};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct GenerateSchemaOptions {
    pub open_api: Option<mappers::open_api::OpenApiOptions>,
}

#[derive(Default, Serialize)]
struct GenerateSchemaResult {
    pub open_api: Option<OpenApiResult>,
}

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

fn generate_schemas(mut cx: FunctionContext) -> Result<Handle<JsString>, Throw> {
    let mut result = GenerateSchemaResult::default();
    let string_options = cx.argument::<JsString>(0)?.value(&mut cx);
    let options = serde_json::from_str::<GenerateSchemaOptions>(&string_options).unwrap();

    let (request_module, on_request_module) = crossbeam::channel::unbounded();
    let (send_node, on_send_node) = crossbeam::channel::unbounded();

    let open_api_handle =
        mappers::open_api::OpenApiMapper::run(options.open_api, request_module.clone(), on_send_node.clone());

    if let Some(handle) = open_api_handle {
        result.open_api = Some(handle.join().unwrap());
    }

    Ok(cx.string(serde_json::to_string(&result).unwrap()))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("generateSchemas", generate_schemas)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{generate_schemas_debug, mappers::open_api::OpenApiOptions};

    #[test]
    fn sends_open_api_options_to_open_api_mapper() {
        generate_schemas_debug(OpenApiOptions {
            output: None,
            base: String::from("{}"),
            paths: vec!["./test/fixtures/".to_string()],
        });
    }
}
