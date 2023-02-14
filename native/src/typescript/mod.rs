mod syntax_kind;

use std::vec;

use neon::{object::PropertyKey, prelude::*, result::Throw};

use crate::utils::dbg_js;

use self::syntax_kind::{
    CLASS_DECLARATION, IMPORT_DECLARATION, INTERFACE_DECLARATION, NUMBER_KEYWORD, STRING_KEYWORD, TRUE_KEYWORD,
    TYPE_ALIAS_DECLARATION, VARIABLE_DECLARATION,
};

pub struct TsNode<'h> {
    ast: Handle<'h, JsObject>,
}

impl<'h> TsNode<'h> {
    pub(crate) fn new(ast: Handle<'h, JsObject>) -> Self {
        TsNode { ast }
    }

    pub fn is_api_path(&self, cx: &mut FunctionContext) -> Result<bool, Throw> {
        Ok(match self.ast.get_opt::<JsObject, _, _>(cx, "expression")? {
            Some(expression) => match expression.get_opt::<JsString, _, _>(cx, "escapedText")? {
                Some(name) => name.value(cx) == "Path",
                None => false,
            },
            None => false,
        })
    }

    pub(crate) fn is_api_response(&self, cx: &mut FunctionContext) -> Result<bool, Throw> {
        Ok(match self.ast.get_opt::<JsObject, _, _>(cx, "expression")? {
            Some(expression) => match expression.get_opt::<JsString, _, _>(cx, "escapedText")? {
                Some(name) => name.value(cx) == "Response",
                None => false,
            },
            None => false,
        })
    }

    pub fn get_children<'cx>(&self, cx: &mut FunctionContext<'cx>) -> Result<Vec<TsNode<'cx>>, Throw> {
        let get_children = self.ast.get::<JsFunction, _, _>(cx, "getChildren")?;
        let children = get_children.call_with(cx).this(self.ast).apply::<JsArray, _>(cx)?;

        return Ok(children
            .to_vec(cx)?
            .iter()
            .map(|child| {
                TsNode::new(
                    child
                        .downcast_or_throw(cx)
                        .expect("Could not cast child node to JsObject"),
                )
            })
            .collect());
    }

    pub(crate) fn get_properties<'cx>(&self, cx: &mut FunctionContext<'cx>) -> Result<Vec<TsNode<'cx>>, Throw> {
        Ok(self
            .ast
            .get_opt::<JsArray, _, _>(cx, "properties")?
            .expect("!!! NO PROPERTIES HERE !!!")
            .to_vec(cx)?
            .iter()
            .map(|p| TsNode::new(p.downcast_or_throw(cx).expect("Could not cast property to JsObject")))
            .collect())
    }

    pub(crate) fn get_identifier(&self, cx: &mut CallContext<JsObject>) -> Result<Option<String>, Throw> {
        if let Some(e) = self.ast.get_opt::<JsString, _, _>(cx, "escapedText")? {
            return Ok(Some(e.value(cx)));
        }

        if let Some(n) = self.ast.get_opt::<JsObject, _, _>(cx, "name")? {
            return Ok(Some(
                n.get::<JsString, _, _>(cx, "escapedText")
                    .expect("I expected a string here")
                    .value(cx),
            ));
        }

        if let Some(e) = self.ast.get_opt::<JsObject, _, _>(cx, "expression")? {
            if let Some(e) = e.get_opt::<JsString, _, _>(cx, "escapedText")? {
                return Ok(Some(e.value(cx)));
            }
        }

        Ok(None)
    }

    pub(crate) fn get_initialized_string(&self, cx: &mut CallContext<JsObject>) -> Result<Option<String>, Throw> {
        Ok(match self.ast.get_opt::<JsObject, _, _>(cx, "initializer")? {
            Some(initializer) => match initializer.get_opt::<JsString, _, _>(cx, "text")? {
                Some(text) => Some(text.value(cx)),
                None => None,
            },
            None => None,
        })
    }

    pub(crate) fn get_initialized_array(&self, cx: &mut CallContext<JsObject>) -> Result<Option<Vec<String>>, Throw> {
        Ok(match self.ast.get_opt::<JsObject, _, _>(cx, "initializer")? {
            Some(initializer) => match initializer.get_opt::<JsArray, _, _>(cx, "elements")? {
                Some(elements) => Some(
                    elements
                        .to_vec(cx)?
                        .iter()
                        .map(|el| {
                            let el = el
                                .downcast_or_throw::<JsObject, FunctionContext>(cx)
                                .expect("Could not cast array element to JsObject");

                            el.get::<JsString, _, _>(cx, "text")
                                .expect("Expected array element to have string value")
                                .value(cx)
                        })
                        .collect(),
                ),
                None => None,
            },
            None => None,
        })
    }

    pub(crate) fn get_arguments<'cx>(&self, cx: &mut FunctionContext<'cx>) -> Result<Vec<TsNode<'cx>>, Throw> {
        Ok(self
            .ast
            .get_opt::<JsArray, _, _>(cx, "arguments")?
            .expect("!!! NO ARGUMENTS FOUND HERE !!!")
            .to_vec(cx)?
            .iter()
            .map(|p| TsNode::new(p.downcast_or_throw(cx).expect("Could not cast argument to JsObject")))
            .collect())
    }

    pub(crate) fn get_boolean_keyword(&self, cx: &mut FunctionContext) -> Result<Option<bool>, Throw> {
        if let Some(literal) = self.ast.get_opt::<JsObject, _, _>(cx, "literal")? {
            let kind = literal.get::<JsNumber, _, _>(cx, "kind")?.value(cx);
            return Ok(Some(kind == TRUE_KEYWORD));
        }

        return Ok(Some(false));
    }

    pub(crate) fn get_body<'cx>(&self, cx: &mut FunctionContext<'cx>) -> Result<TsNode<'cx>, Throw> {
        Ok(TsNode::new(self.ast.get::<JsObject, _, _>(cx, "body")?))
    }

    pub(crate) fn get_api_param<'cx>(&self, cx: &mut FunctionContext<'cx>) -> Result<Param<'cx>, Throw> {
        Ok(match self.ast.get_opt::<JsObject, _, _>(cx, "type")? {
            Some(type_obj) => match type_obj.get_opt::<JsObject, _, _>(cx, "typeName")? {
                Some(type_name) => match type_name.get_opt::<JsString, _, _>(cx, "escapedText")? {
                    Some(type_text) => {
                        let name = self.ast.get::<JsObject, _, _>(cx, "name")?;
                        let name_text = name
                            .get::<JsString, _, _>(cx, "escapedText")
                            .expect("I expected a string here")
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
        Ok(match self.ast.get_opt::<JsArray, _, _>(cx, "typeArguments")? {
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
        })
    }

    pub(crate) fn get_value_type(&self, cx: &mut FunctionContext) -> Result<ValueType, Throw> {
        let kind = self.ast.get::<JsNumber, _, _>(cx, "kind")?.value(cx);
        if kind == NUMBER_KEYWORD {
            return Ok(ValueType::Primitive(String::from("number"), None));
        }

        if kind == STRING_KEYWORD {
            return Ok(ValueType::Primitive(String::from("string"), None));
        }

        Ok(ValueType::Reference(String::from("ref")))
    }

    pub(crate) fn get_type_literal<'cx>(&self, cx: &mut FunctionContext<'cx>) -> Result<Option<TsNode<'cx>>, Throw> {
        Ok(match self.ast.get_opt::<JsObject, _, _>(cx, "type")? {
            Some(type_literal) => Some(TsNode::new(type_literal)),
            None => None,
        })
    }

    pub(crate) fn get_members<'cx>(&self, cx: &mut FunctionContext<'cx>) -> Result<Vec<TsNode<'cx>>, Throw> {
        let members = self
            .ast
            .get_opt::<JsArray, _, _>(cx, "members")?
            .expect("!!! OH YOU WANT MEMBERS?! I DON'T THINK SO!");

        Ok(members
            .to_vec(cx)?
            .iter()
            .map(|m| {
                let error_message = "Could not convert member to JsObject";
                TsNode::new(m.downcast_or_throw(cx).expect(error_message))
            })
            .collect())
    }

    pub(crate) fn get_parameters<'cx>(&self, cx: &mut FunctionContext<'cx>) -> Result<Option<Vec<TsNode<'cx>>>, Throw> {
        Ok(match self.ast.get_opt::<JsArray, _, _>(cx, "parameters")? {
            Some(parameters) => Some(
                parameters
                    .to_vec(cx)?
                    .iter()
                    .map(|p| {
                        let error_message = "Could not convert parameter to JsObject";
                        TsNode::new(p.downcast_or_throw(cx).expect(error_message))
                    })
                    .collect(),
            ),
            None => None,
        })
    }

    pub(crate) fn dbg_js(&self, cx: &mut FunctionContext) -> Result<(), Throw> {
        dbg_js(cx, self.ast)
    }

    pub(crate) fn is_import_declaration(&self, cx: &mut FunctionContext) -> Result<bool, Throw> {
        Ok(self.ast.get::<JsNumber, _, _>(cx, "kind")?.value(cx) == IMPORT_DECLARATION)
    }

    pub(crate) fn is_variable_declaration(&self, cx: &mut CallContext<JsObject>) -> Result<bool, Throw> {
        Ok(self.ast.get::<JsNumber, _, _>(cx, "kind")?.value(cx) == VARIABLE_DECLARATION)
    }

    pub(crate) fn is_class_interface_or_type_declaration(&self, cx: &mut FunctionContext) -> Result<bool, Throw> {
        let kind = self.ast.get::<JsNumber, _, _>(cx, "kind")?.value(cx);
        Ok(kind == CLASS_DECLARATION || kind == INTERFACE_DECLARATION || kind == TYPE_ALIAS_DECLARATION)
    }

    pub(crate) fn get_import_declarations(
        &self,
        cx: &mut FunctionContext<'h>,
    ) -> Result<Option<Vec<TsDeclaration<'h>>>, Throw> {
        if let Some(import_clause) = self.ast.get_opt::<JsObject, _, _>(cx, "importClause")? {
            let mut vec = Vec::new();
            if let Some(default_import) = import_clause.get_opt::<JsObject, _, _>(cx, "name")? {
                let text = default_import.get::<JsString, _, _>(cx, "escapedText")?.value(cx);
                vec.push(TsDeclaration {
                    declaration_type: DeclarationType::DefaultImport,
                    name: text,
                    node: TsNode::new(self.ast),
                });
            }

            if let Some(named_imports) = import_clause.get_opt::<JsObject, _, _>(cx, "namedBindings")? {
                for element in named_imports
                    .get_opt::<JsArray, _, _>(cx, "elements")?
                    .expect("!!! I WISH I COULD HELP YOU OUT WITH SOME ELEMENTS !!!")
                    .to_vec(cx)?
                {
                    let element = element.downcast_or_throw::<JsObject, _>(cx)?;
                    let name = element.get::<JsObject, _, _>(cx, "name")?;
                    let text = name.get::<JsString, _, _>(cx, "escapedText")?.value(cx);
                    vec.push(TsDeclaration {
                        declaration_type: DeclarationType::NamedImport,
                        name: text.clone(),
                        node: TsNode::new(self.ast),
                    });

                    if let Some(alias) = element.get_opt::<JsObject, _, _>(cx, "propertyName")? {
                        let alias_text = alias.get::<JsString, _, _>(cx, "escapedText")?.value(cx);
                        vec.push(TsDeclaration {
                            declaration_type: DeclarationType::Alias,
                            name: alias_text,
                            node: TsNode::new(self.ast),
                        })
                    }
                }
            }

            return Ok(Some(vec));
        }

        Ok(None)
    }

    pub(crate) fn get_variable_declarations<'cx>(
        &self,
        cx: &mut FunctionContext<'cx>,
    ) -> Result<Option<Vec<TsDeclaration<'cx>>>, Throw> {
        if let Some(declaration_list) = self.ast.get_opt::<JsArray, _, _>(cx, "declarationList")? {
            let mut vec = Vec::new();
            for declaration in declaration_list
                .get_opt::<JsArray, _, _>(cx, "declarations")?
                .expect("!!! NO DECLARATIONS HERE !!")
                .to_vec(cx)?
            {
                let declaration = declaration.downcast_or_throw::<JsObject, _>(cx)?;
                let name = declaration.get::<JsObject, _, _>(cx, "name")?;
                let text = name.get::<JsString, _, _>(cx, "escapedText")?.value(cx);
                let initializer = declaration.get::<JsObject, _, _>(cx, "initializer")?;
                vec.push(TsDeclaration {
                    declaration_type: DeclarationType::Variable,
                    name: text,
                    node: TsNode::new(initializer),
                });
            }

            return Ok(Some(vec));
        }

        Ok(None)
    }

    pub(crate) fn get_type_declaration(
        &self,
        cx: &mut FunctionContext<'h>,
    ) -> Result<Option<TsDeclaration<'h>>, Throw> {
        let kind = self.ast.get::<JsNumber, _, _>(cx, "kind")?.value(cx);
        if kind == CLASS_DECLARATION || kind == INTERFACE_DECLARATION || kind == TYPE_ALIAS_DECLARATION {
            let name = self.ast.get::<JsObject, _, _>(cx, "name")?;
            let text = name.get::<JsString, _, _>(cx, "escapedText")?.value(cx);
            return Ok(Some(TsDeclaration {
                declaration_type: DeclarationType::Structual,
                name: text,
                node: TsNode::new(self.ast),
            }));
        }

        Ok(None)
    }

    pub(crate) fn get_property_name(&self, cx: &mut CallContext<JsObject>) -> Result<Option<String>, Throw> {
        if let Some(property_name) = self.ast.get_opt::<JsObject, _, _>(cx, "propertyName")? {
            let text = property_name.get::<JsString, _, _>(cx, "escapedText")?.value(cx);
            return Ok(Some(text));
        }
        Ok(None)
    }

    pub(crate) fn get_initialized_expression<'cx>(
        &self,
        cx: &mut FunctionContext<'cx>,
    ) -> Result<Option<TsNode<'cx>>, Throw> {
        Ok(match self.ast.get_opt::<JsObject, _, _>(cx, "initializer")? {
            Some(initializer) => Some(TsNode::new(initializer)),
            None => None,
        })
    }
}

pub enum Param<'cx> {
    Header(String, TsNode<'cx>),
    Query(String, TsNode<'cx>),
    Route(String, TsNode<'cx>),
    None,
}

pub enum ValueType {
    Primitive(String, Option<String>),
    Literal(Vec<ValueType>),
    Reference(String),
}

pub struct PathArgs {
    pub method: Option<String>,
    pub path: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl PathArgs {
    pub(crate) fn new() -> Self {
        PathArgs {
            method: None,
            path: None,
            tags: None,
        }
    }
}

pub struct ResponseOptions {
    pub description: Option<String>,
    pub example: Option<String>,
    pub namespace: Option<String>,
    pub status_code: Option<String>,
}
impl ResponseOptions {
    pub(crate) fn new() -> Self {
        ResponseOptions {
            description: None,
            example: None,
            namespace: None,
            status_code: None,
        }
    }
}

pub enum DeclarationType {
    DefaultImport,
    NamedImport,
    Alias,
    Variable,
    Structual,
}

pub struct TsDeclaration<'n> {
    pub declaration_type: DeclarationType,
    pub name: String,
    pub node: TsNode<'n>,
}
