mod source_file;
mod syntax_kind;

pub use self::source_file::*;
pub use self::syntax_kind::*;

// pub struct AstCursorIter<'v> {
//     values: std::slice::Iter<'v, Value>,
// }

// impl<'v> AstCursorIter<'v> {
//     fn new(values: &'v [Value]) -> Self {
//         Self { values: values.iter() }
//     }

//     fn empty() -> Self {
//         Self::new(&[])
//     }
// }

// impl<'v> Iterator for AstCursorIter<'v> {
//     type Item = AstCursor<'v>;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.values.next().map(AstCursor::new)
//     }
// }

// #[derive(Clone)]
// pub struct AstCursor<'v> {
//     root: &'v Value,
//     current_name: Option<String>,
//     pub current: &'v Value,
// }

// impl<'v> AstCursor<'v> {
// pub fn new(root: &Value) -> AstCursor {
//     AstCursor {
//         root,
//         current: root,
//         current_name: None,
//     }
// }

// pub(crate) fn get_object<'p>(value: &'p Map<String, Value>, name: &str) -> &'p Map<String, Value> {
//     value
//         .get(name)
//         .expect(&format!("Could not get property {}", name))
//         .as_object()
//         .expect(&format!("Could not cast property {} to object", name))
// }

// pub(crate) fn move_to(&mut self, arg: &str) -> &mut AstCursor<'v> {
//     self.current = self
//         .current
//         .get(arg)
//         .expect(&format!("Could not move to property {}", arg));

//     if self.current.is_null() {
//         panic!("Current node does not have child {}", arg)
//     }
//     self.current_name.insert(arg.to_string());
//     self
// }

//     pub(crate) fn iter(&self) -> AstCursorIter<'v> {
//         let values = match self.current {
//             Value::Array(arr) => arr,
//             obj => std::slice::from_ref(obj),
//         };
//         AstCursorIter::new(values)
//     }

//     pub(crate) fn is_api_path(node: &AstNode) -> bool {
//         match node.expression {
//             Some(ref expression) => match expression.escaped_text {
//                 Some(ref text) => text.eq("Path"),
//                 _ => false,
//             },
//             _ => false,
//         }
//     }

//     fn to_iterable(&self) -> impl Iterator<Item = AstCursor> {
//         vec![self.clone()].into_iter()
//     }

//     pub fn get_kind(value: &serde_json::Value) -> u64 {
//         value
//             .get("kind")
//             .expect("Could not get kind")
//             .as_u64()
//             .expect("Could not cast kind to integer")
//     }

//     pub(crate) fn has_property(value: &serde_json::Value, index: &str) -> bool {
//         match value {
//             &Value::Object(ref root) => root.contains_key(index),
//             _ => false,
//         }
//     }

//     pub(crate) fn get_str<'p>(value: &'p Map<String, Value>, key: &str) -> &'p str {
//         value
//             .get(key)
//             .expect(&format!("Could not get {}", key))
//             .as_str()
//             .expect(&format!("Could not cast {} to &str", key))
//     }

//     pub(crate) fn has_index(&self, index: usize) -> bool {
//         match self.current {
//             &Value::Array(ref root) => root.len() > index,
//             _ => false,
//         }
//     }

//     pub(crate) fn get_vec<F, T>(&self, key: &str, map: F) -> Vec<T>
//     where
//         F: Fn(AstCursor) -> T,
//     {
//         self.current
//             .get(key)
//             .expect(&format!("Could not get property '{}'", key))
//             .as_array()
//             .expect(&format!("Could not cast '{}' to array", key))
//             .iter()
//             .map(|e| {
//                 map(AstCursor {
//                     current: e,
//                     root: e,
//                     current_name: Some(String::from("root")),
//                 })
//             })
//             .collect()
//     }

//     pub(crate) fn children(parameter: &AstNode) -> &Option<Vec<AstNode>> {
//         match parameter.kind {
//             CLASS_DECLARATION => &parameter.members,
//             INTERFACE_DECLARATION => &parameter.members,
//             SOURCE_FILE => &parameter.statements,
//             TYPE_ALIAS_DECLARATION => &parameter.members,
//             _ => &None,
//         }
//     }

//     pub(crate) fn is_api_response(node: &AstNode) -> bool {
//         match node.expression {
//             Some(ref expression) => match expression.escaped_text {
//                 Some(ref text) => text.eq("Response"),
//                 None => false,
//             },
//             None => false,
//         }
//     }
// }
