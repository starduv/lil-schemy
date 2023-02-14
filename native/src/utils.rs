use neon::{prelude::*, result::Throw};

#[allow(dead_code)]
pub fn dbg_js<V>(cx: &mut FunctionContext, v: Handle<V>) -> Result<(), Throw>
where
    V: Value,
{
    cx.global()
        .get::<JsObject, _, _>(cx, "console")?
        .get::<JsFunction, _, _>(cx, "debug")?
        .call_with(cx)
        .arg(v)
        .exec(cx)?;

    Ok(())
}
