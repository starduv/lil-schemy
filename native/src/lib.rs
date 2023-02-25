#![allow(dead_code)]
mod open_api;
mod typescript;
mod utils;

use neon::{prelude::*, result::Throw};
use open_api::generate_openapi;

fn generate_schemas(mut cx: FunctionContext) -> Result<Handle<JsObject>, Throw> {
    let schemas_result: Handle<JsObject> = cx.empty_object();
    let options_handle: Handle<JsObject> = cx.argument(0)?;
    let asts = options_handle.get::<JsString, _, _>(&mut cx, "asts")?.value(&mut cx);
    let ast_map = serde_json::from_str(&asts).expect("Could not parse asts");
    generate_openapi(schemas_result, options_handle, &ast_map, &mut cx)?;

    Ok(schemas_result)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("generateSchemas", generate_schemas)?;
    Ok(())
}
