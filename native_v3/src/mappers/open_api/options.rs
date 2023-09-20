use neon::{
    prelude::{FunctionContext, Handle, Object},
    types::{JsArray, JsObject, JsString},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OpenApiOptions {
    pub base: String,
    pub output: Option<String>,
    pub paths: Vec<String>,
}

impl OpenApiOptions {
    pub fn from_js_object(cx: &mut FunctionContext, open_api_handle: Option<Handle<JsObject>>) -> Option<Self> {
        match open_api_handle {
            Some(open_api_handle) => Some(Self {
                base: match open_api_handle.get_opt::<JsString, FunctionContext, &str>(cx, "base") {
                    Ok(Some(base_handle)) => base_handle.value(cx),
                    _ => String::from(""),
                },
                output: match open_api_handle.get_opt::<JsString, FunctionContext, &str>(cx, "output") {
                    Ok(Some(output_handle)) => Some(output_handle.value(cx)),
                    _ => None,
                },
                paths: match open_api_handle.get_opt::<JsArray, FunctionContext, &str>(cx, "entry") {
                    Ok(Some(entry_handle)) => entry_handle
                        .to_vec(cx)
                        .unwrap_or(Vec::new())
                        .iter()
                        .filter_map(|entry| match entry.downcast::<JsString, FunctionContext>(cx) {
                            Ok(entry_handle) => Some(entry_handle.value(cx)),
                            _ => None,
                        })
                        .collect(),
                    _ => Vec::new(),
                },
            }),
            None => None,
        }
    }
}
