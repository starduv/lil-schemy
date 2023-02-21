#![allow(dead_code)]
mod generators;
mod typescript;
mod utils;

use generators::generate_openapi;
use neon::{prelude::*, result::Throw};
use serde_json::json;

fn generate_schemas(mut cx: FunctionContext) -> Result<Handle<JsObject>, Throw> {
    let schemas_result: Handle<JsObject> = cx.empty_object();
    let options_handle: Handle<JsObject> = cx.argument(0)?;
    let asts = options_handle.get::<JsString, _, _>(&mut cx, "asts")?.value(&mut cx);

    generate_openapi(schemas_result, options_handle, &json!(asts), &mut cx)?;

    Ok(schemas_result)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("generateSchemas", generate_schemas)?;
    Ok(())
}
