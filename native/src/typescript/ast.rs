// use super::syntax_kind::{
//     CLASS_DECLARATION, EXPRESSION_STATEMENT, INTERFACE_DECLARATION, SOURCE_FILE, TYPE_ALIAS_DECLARATION,
//     VARIABLE_STATEMENT,
// };

// impl AstCursor<'_> {
//     pub fn new(ast: &serde_json::Value) -> AstCursor {
//         AstCursor { ast }
//     }

//     pub(crate) fn is_api_path(&self) -> bool {
//         match self.ast.get("expression") {
//             Some(&Value::Object(ref expression)) => match expression.get("escapedText") {
//                 Some(&Value::String(ref text)) => text.eq("Path"),
//                 _ => false,
//             },
//             _ => false,
//         }
//     }

//     pub(crate) fn is_api_response(&self) -> bool {
//         match self.ast.get("expression") {
//             Some(ref expression) => match expression.get("escapedText") {
//                 Some(&Value::String(ref name)) => name == "Response",
//                 _ => false,
//             },
//             None => false,
//         }
//     }

//     pub(crate) fn get_arguments(&self) -> Vec<AstCursor> {
//         let arguments = self.ast.get("arguments").expect("Could not get arguments");
//         arguments
//             .as_array()
//             .expect("Could not cast arguments to array")
//             .iter()
//             .map(|a| AstCursor::new(a))
//             .collect()
//     }

//     pub(crate) fn get_properties(&self) -> Vec<AstCursor> {
//         let properties = self.ast.get("properties").expect("Could not properties");
//         properties
//             .as_array()
//             .expect("Could not cast properties to array")
//             .iter()
//             .map(|p| AstCursor::new(p))
//             .collect()
//     }

//     pub(crate) fn get_identifier(&self) -> Option<&str> {
//         if let Some(text) = self.ast.get("escapedText") {
//             return text.as_str();
//         }

//         if let Some(name) = self.ast.get("name") {
//             if let Some(text) = name.get("escapedText") {
//                 return text.as_str();
//             }
//         }

//         if let Some(name) = self.ast.get("expression") {
//             if let Some(text) = name.get("escapedText") {
//                 return text.as_str();
//             }
//         }

//         return None;
//     }

//     pub(crate) fn get_initialized_string(&self) -> &str {
//         let initializer = self.ast.get("initializer").expect("Could not get initializer");
//         let text = initializer.get("text").expect("Could not get text from initializer");
//         text.as_str().expect("Could not cast text to &str")
//     }

//     pub(crate) fn get_initialized_string_array(&self) -> Vec<String> {
//         let initializer = self.ast.get("initializer").expect("Could not get initializer");
//         let elements = initializer
//             .get("elements")
//             .expect("Could not get elements from initializer");

//         elements
//             .as_array()
//             .expect("Could not cast elements to array")
//             .iter()
//             .map(|e| e.to_string())
//             .collect()
//     }

//     pub(crate) fn get_parameters(&self) -> Vec<AstCursor> {
//         let parameters = self.ast.get("parameters").expect("Could not get parameters");
//         parameters
//             .as_array()
//             .expect("Could not case parameters to Array")
//             .iter()
//             .map(|p| AstCursor::new(p))
//             .collect()
//     }

//     pub(crate) fn get_body(&self) -> AstCursor {
//         let body = self.ast.get("body").expect("Could not get body");
//         AstCursor::new(body)
//     }

//     pub(crate) fn get_children(&self) -> Vec<AstCursor> {
//         let kind = self.get_kind();
//         match kind {
//             CLASS_DECLARATION => self.get_members().expect("Could not get members from class"),
//             EXPRESSION_STATEMENT => vec![AstCursor::new(
//                 self.ast
//                     .get("expression")
//                     .expect("Could not get expression from expressoin statement"),
//             )],
//             INTERFACE_DECLARATION => self.get_members().expect("Could not get members from interface"),
//             SOURCE_FILE => self.get_statements(),
//             TYPE_ALIAS_DECLARATION => self.get_members().expect("Could not get members from type alias"),
//             VARIABLE_STATEMENT => self.get_declarations(),
//             _ => Default::default(),
//         }
//     }

//     pub(crate) fn get_type(&self) -> AstCursor {
//         let type_obj = self.ast.get("type").expect("Could not get type property");
//         AstCursor::new(type_obj)
//     }

//     pub(crate) fn get_members(&self) -> Option<Vec<AstCursor>> {
//         match self.ast.get("members") {
//             Some(&Value::Array(ref members)) => Some(members.iter().map(|m| AstCursor::new(m)).collect()),
//             _ => None,
//         }
//     }

//     pub(crate) fn name(&self) -> &str {
//         let name = self.ast.get("name").expect("Could not get name");
//         let text = name.get("escapedText").expect("Could not get escaped text from name");
//         text.as_str().expect("Could not cast escaped text to &str")
//     }

//     pub(crate) fn get_type_name(&self) -> &str {
//         let type_name = self.ast.get("typeName").expect("Could not get type name");
//         let text = type_name
//             .get("escapedText")
//             .expect("Could not get escapted text from type name");

//         text.as_str().expect("Could not case escaped text to &str")
//     }

//     pub(crate) fn get_type_arguments(&self) -> Vec<AstCursor> {
//         let type_args = self.ast.get("typeArguments").expect("Could not get type arguments");
//         let type_args = type_args.as_array().expect("Could not case type arguments to array");
//         type_args.iter().map(|t| AstCursor::new(t)).collect()
//     }

//     // pub(crate) fn get_value_type(&self) -> ValueType {
//     //     match self.ast.get("kind") {
//     //         Some(&Value::Number(kind)) => {
//     //             let kind = kind.as_f64().expect("Could not cast kind to f64");

//     //             if kind == NUMBER_KEYWORD {
//     //                 return ValueType::Number;
//     //             }

//     //             if kind == STRING_KEYWORD {
//     //                 return ValueType::String;
//     //             }

//     //             ValueType::Reference
//     //         }
//     //         None => ValueType::None,
//     //     }
//     // }

//     pub(crate) fn get_kind(&self) -> u64 {
//         let kind = self.ast.get("kind").expect("Could not get kind");
//         kind.as_u64().expect("Could not cast kind to u64")
//     }

//     fn get_statements(&self) -> Vec<AstCursor> {
//         let statements = self.ast.get("statements").expect("Could not get statements");
//         statements
//             .as_array()
//             .expect("Could not cast statements as array")
//             .iter()
//             .map(|s| AstCursor::new(s))
//             .collect()
//     }

//     fn get_declarations(&self) -> Vec<AstCursor> {
//         let decl_list = self.ast.get("declarationList").expect("Could not get declaration list");
//         let declarations = decl_list.get("declarations").expect("Could not get declarations");
//         declarations
//             .as_array()
//             .expect("Could not cast declarations to array")
//             .iter()
//             .map(|d| AstCursor::new(d))
//             .collect()
//     }

//     // pub(crate) fn get_boolean_keyword(&self) -> Option<bool> {
//     //     let literal = self.ast.get("literal").expect("Could not get literal");
//     //     let kind = literal.get("kind")
//     //     if let Some(literal) = self.ast.get_opt::<JsObject, _, _>(cx, "literal")? {
//     //         let kind = literal.get::<JsNumber, _, _>(cx, "kind")?.value(cx);
//     //         return Ok(Some(kind == TRUE_KEYWORD));
//     //     }
//     // }

//     pub(crate) fn get_type_literal(&self) -> AstCursor {
//         let literal = self.ast.get("literal").expect("Could not get literal");
//         AstCursor::new(literal)
//     }

//     pub(crate) fn get_import_declarations(&self) -> Option<Vec<TsDeclaration>> {
//         todo!()
//     }

//     pub(crate) fn get_variable_declarations(&self) -> Option<Vec<TsDeclaration>> {
//         todo!()
//     }

//     pub(crate) fn get_type_declaration(&self) -> Option<TsDeclaration> {
//         todo!()
//     }

//     pub(crate) fn get_initialized_expression(&self) -> AstCursor {
//         todo!()
//     }

//     pub(crate) fn get_property_name(&self) -> Option<&str> {
//         let property_name = self.ast.get("propertyName").expect("Could not get property name");
//         let escaped_text = property_name
//             .get("escapedText")
//             .expect("Could not get text from property name");

//         escaped_text.as_str()
//     }
// }

use super::AstCursor;

pub enum Param {
    Header,
    Query,
    Route,
    None,
}

pub enum ValueType {
    Number,
    String,
    Literal,
    Reference,
    None,
}

pub struct TsDeclaration<'n> {
    pub declaration_type: DeclarationType,
    pub name: String,
    pub node: AstCursor<'n>,
}

pub enum DeclarationType {
    DefaultImport,
    NamedImport,
    Alias,
    Variable,
    Structual,
}
