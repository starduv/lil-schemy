mod generators;

use generators::generate_openapi;
use neon::{prelude::*, result::Throw};

fn generate_schemas(mut cx: FunctionContext) -> Result<Handle<JsObject>, Throw> {
    let schemas_result: Handle<JsObject> = cx.empty_object();
    let options_handle: Handle<JsObject> = cx.argument(0)?;

    generate_openapi(schemas_result, options_handle, &mut cx)?;

    Ok(schemas_result)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("generateSchemas", generate_schemas)?;
    Ok(())
}
