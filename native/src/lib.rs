#![allow(dead_code)]
mod open_api;
mod typescript;
mod utils;

use neon::{prelude::*, result::Throw};
use open_api::generate_openapi;

fn generate_schemas(mut cx: FunctionContext) -> Result<Handle<JsObject>, Throw> {
    let schemas_result: Handle<JsObject> = cx.empty_object();
    let options_handle: Handle<JsObject> = cx.argument(0)?;
    let get_ast = options_handle.get::<JsFunction, _, _>(&mut cx, "getAst")?;

    generate_openapi(schemas_result, options_handle, get_ast, &mut cx)?;

    Ok(schemas_result)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("generateSchemas", generate_schemas)?;
    Ok(())
}
