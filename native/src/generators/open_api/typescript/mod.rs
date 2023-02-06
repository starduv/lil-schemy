mod syntax_kind;

use neon::{prelude::*, result::Throw};

use self::syntax_kind::*;

use super::open_api_v3::OpenApiV3;

pub struct TsNode<'h> {
    ast: Handle<'h, JsObject>,
}

impl<'h> TsNode<'h> {
    pub(crate) fn new(ast: Handle<'h, JsObject>) -> Self {
        TsNode { ast }
    }

    pub fn is_api_path(&self, cx: &mut FunctionContext) -> Result<bool, Throw> {
        Ok(
            match self.ast.get_opt::<JsObject, FunctionContext, &str>(cx, "expression")? {
                Some(expression) => match expression.get_opt::<JsString, FunctionContext, &str>(cx, "escapedText")? {
                    Some(name) => name.value(cx) == "Path",
                    None => false,
                },
                None => false,
            },
        )
    }

    pub fn get_children<'cx>(&self, cx: &mut FunctionContext<'cx>) -> Result<Vec<TsNode<'cx>>, Throw> {
        let get_children = self.ast.get::<JsFunction, FunctionContext, &str>(cx, "getChildren")?;
        let children = get_children.call_with(cx).apply::<JsArray, FunctionContext>(cx)?;
        Ok(children
            .to_vec(cx)?
            .iter()
            .map(|child| {
                TsNode::new(
                    child
                        .downcast_or_throw(cx)
                        .expect("Could not cast child node to JsObject"),
                )
            })
            .collect())
    }

    pub(crate) fn get_properties<'cx>(&self, cx: &mut FunctionContext<'cx>) -> Result<Vec<TsNode<'cx>>, Throw> {
        Ok(self
            .ast
            .get::<JsArray, FunctionContext, &str>(cx, "properties")?
            .to_vec(cx)?
            .iter()
            .map(|p| TsNode::new(p.downcast_or_throw(cx).expect("Could not cast property to JsObject")))
            .collect())
    }

    pub(crate) fn get_identifier(&self, cx: &mut CallContext<JsObject>) -> Result<Option<String>, Throw> {
        Ok(match self.ast.get_opt::<JsObject, FunctionContext, &str>(cx, "name")? {
            Some(n) => match n.get_opt::<JsString, FunctionContext, &str>(cx, "escapedText")? {
                Some(name) => Some(name.value(cx)),
                None => None,
            },
            None => None,
        })
    }

    pub(crate) fn get_initialized_string(&self, cx: &mut CallContext<JsObject>) -> Result<Option<String>, Throw> {
        Ok(
            match self.ast.get_opt::<JsObject, FunctionContext, &str>(cx, "initializer")? {
                Some(initializer) => match initializer.get_opt::<JsString, FunctionContext, &str>(cx, "text")? {
                    Some(text) => Some(text.value(cx)),
                    None => None,
                },
                None => None,
            },
        )
    }

    pub(crate) fn get_initialized_array(&self, cx: &mut CallContext<JsObject>) -> Result<Option<Vec<String>>, Throw> {
        Ok(
            match self.ast.get_opt::<JsObject, FunctionContext, &str>(cx, "initializer")? {
                Some(initializer) => match initializer.get_opt::<JsArray, FunctionContext, &str>(cx, "elements")? {
                    Some(elements) => Some(
                        elements
                            .to_vec(cx)?
                            .iter()
                            .map(|el| {
                                let el = el
                                    .downcast_or_throw::<JsObject, FunctionContext>(cx)
                                    .expect("Could not cast array element to JsObject");

                                el.get::<JsString, FunctionContext, &str>(cx, "text")
                                    .expect("Expected array element to have string value")
                                    .value(cx)
                            })
                            .collect(),
                    ),
                    None => None,
                },
                None => None,
            },
        )
    }
}
