mod syntax_kind;

use neon::{prelude::*, result::Throw};

use self::syntax_kind::{NUMBER_KEYWORD, STRING_KEYWORD, TRUE_KEYWORD};

use super::open_api_v3::{Param, ValueType};

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

    pub(crate) fn is_api_response(&self, cx: &mut FunctionContext) -> Result<bool, Throw> {
        Ok(
            match self.ast.get_opt::<JsObject, FunctionContext, &str>(cx, "expression")? {
                Some(expression) => match expression.get_opt::<JsString, FunctionContext, &str>(cx, "escapedText")? {
                    Some(name) => name.value(cx) == "Response",
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
        if let Some(e) = self.ast.get_opt::<JsString, FunctionContext, &str>(cx, "escapedText")? {
            return Ok(Some(e.value(cx)));
        }

        if let Some(n) = self.ast.get_opt::<JsObject, FunctionContext, &str>(cx, "name")? {
            return Ok(Some(
                n.get::<JsString, FunctionContext, &str>(cx, "escapedText")?.value(cx),
            ));
        }

        if let Some(e) = self.ast.get_opt::<JsObject, FunctionContext, &str>(cx, "expression")? {
            if let Some(e) = e.get_opt::<JsString, FunctionContext, &str>(cx, "escapedText")? {
                return Ok(Some(e.value(cx)));
            }
        }

        Ok(None)
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

    pub(crate) fn get_arguments<'cx>(&self, cx: &mut FunctionContext<'cx>) -> Result<Vec<TsNode<'cx>>, Throw> {
        Ok(self
            .ast
            .get::<JsArray, FunctionContext, &str>(cx, "arguments")?
            .to_vec(cx)?
            .iter()
            .map(|p| TsNode::new(p.downcast_or_throw(cx).expect("Could not cast argument to JsObject")))
            .collect())
    }

    pub(crate) fn get_boolean_keyword(&self, cx: &mut FunctionContext) -> Result<Option<bool>, Throw> {
        if let Some(literal) = self.ast.get_opt::<JsObject, FunctionContext, &str>(cx, "literal")? {
            let kind = literal.get::<JsNumber, FunctionContext, &str>(cx, "kind")?.value(cx);
            return Ok(Some(kind == TRUE_KEYWORD));
        }

        return Ok(Some(false));
    }

    pub(crate) fn get_body<'cx>(&self, cx: &mut FunctionContext<'cx>) -> Result<TsNode<'cx>, Throw> {
        Ok(TsNode::new(
            self.ast.get::<JsObject, FunctionContext, &str>(cx, "body")?,
        ))
    }

    pub(crate) fn get_param<'cx>(&self, cx: &mut FunctionContext<'cx>) -> Result<Param<'cx>, Throw> {
        Ok(match self.ast.get_opt::<JsObject, FunctionContext, &str>(cx, "type")? {
            Some(type_obj) => match type_obj.get_opt::<JsObject, FunctionContext, &str>(cx, "typeName")? {
                Some(type_name) => match type_name.get_opt::<JsString, FunctionContext, &str>(cx, "escapedText")? {
                    Some(type_text) => {
                        let name = self.ast.get::<JsObject, FunctionContext, &str>(cx, "name")?;
                        let name_text = name
                            .get::<JsString, FunctionContext, &str>(cx, "escapedText")?
                            .value(cx);

                        match type_text.value(cx).as_str() {
                            "QueryParam" => Param::Query(name_text, TsNode::new(type_obj)),
                            "RouteParam" => Param::Route(name_text, TsNode::new(type_obj)),
                            "Header" => Param::Header(name_text, TsNode::new(type_obj)),
                            _ => Param::None,
                        }
                    }
                    None => Param::None,
                },
                None => Param::None,
            },
            None => Param::None,
        })
    }

    pub(crate) fn get_type_arguments<'cx>(
        &self,
        cx: &mut FunctionContext<'cx>,
    ) -> Result<Option<Vec<TsNode<'cx>>>, Throw> {
        Ok(
            match self
                .ast
                .get_opt::<JsArray, FunctionContext, &str>(cx, "typeArguments")?
            {
                Some(args) => Some(
                    args.to_vec(cx)?
                        .iter()
                        .map(|a| {
                            TsNode::new(
                                a.downcast_or_throw::<JsObject, FunctionContext>(cx)
                                    .expect("Could not cast Type Argument to JsObject"),
                            )
                        })
                        .collect(),
                ),
                None => None,
            },
        )
    }

    pub(crate) fn get_value_type(&self, cx: &mut FunctionContext) -> Result<ValueType, Throw> {
        let kind = self.ast.get::<JsNumber, FunctionContext, &str>(cx, "kind")?.value(cx);
        if kind == NUMBER_KEYWORD {
            return Ok(ValueType::Primitive(String::from("number"), None));
        }

        if kind == STRING_KEYWORD {
            return Ok(ValueType::Primitive(String::from("string"), None));
        }

        Ok(ValueType::Literal(Box::new(ValueType::Reference(String::from("ref")))))
    }
}
