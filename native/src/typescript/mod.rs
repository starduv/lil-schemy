mod ast;
mod syntax_kind;

pub use self::syntax_kind::*;
pub use ast::{DeclarationType, TsDeclaration};
use serde_json::Value;

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

#[derive(Clone)]
pub struct AstCursor<'v> {
    root: &'v Value,
    current_name: Option<String>,
    pub current: &'v Value,
}
impl<'v> AstCursor<'v> {
    pub fn new(root: &Value) -> AstCursor {
        AstCursor {
            root,
            current: root,
            current_name: None,
        }
    }

    pub(crate) fn get_cursor(&self, name: &str) -> &mut AstCursor {
        let child = self
            .current
            .get(name)
            .expect(&format!("Could not get property {}", name));

        &mut AstCursor {
            root: &child,
            current: &child,
            current_name: Some(String::from("root")),
        }
    }

    pub(crate) fn move_to(&mut self, arg: &str) -> &mut AstCursor<'v> {
        self.current = self
            .current
            .get(arg)
            .expect(&format!("Could not move to property {}", arg));

        if self.current.is_null() {
            panic!("Current node does not have child {}", arg)
        }
        self.current_name.insert(arg.to_string());
        self
    }

    pub(crate) fn for_each<F>(&mut self, iteratee: F) -> ()
    where
        F: Fn(&mut AstCursor) -> (),
    {
        let list = self.current.as_array().expect(&format!(
            "Cannot iterate {:?}, it's not an array {:?}",
            self.current_name, self.current
        ));

        for node in list {
            iteratee(&mut AstCursor {
                current: node,
                current_name: Some(String::from("root")),
                root: node,
            })
        }
    }

    pub(crate) fn is_api_path(&self) -> bool {
        match self.current.get("expression") {
            Some(&Value::Object(ref expression)) => match expression.get("escapedText") {
                Some(&Value::String(ref text)) => text.eq("Path"),
                _ => false,
            },
            _ => false,
        }
    }

    pub(crate) fn for_each_child<F>(&mut self, iteratee: F) -> ()
    where
        F: Fn(&mut AstCursor) -> (),
    {
        let kind = self.get_kind();
        match kind {
            CLASS_DECLARATION => self.move_to("members").for_each(iteratee),
            EXPRESSION_STATEMENT => iteratee(self.move_to("expression")),
            INTERFACE_DECLARATION => self.move_to("members").for_each(iteratee),
            SOURCE_FILE => self.move_to("statements").for_each(iteratee),
            TYPE_ALIAS_DECLARATION => self.move_to("members").for_each(iteratee),
            VARIABLE_STATEMENT => self
                .move_to("declarationList")
                .move_to("declarations")
                .for_each(iteratee),
            _ => Default::default(),
        }
    }

    pub fn get_kind(&self) -> u64 {
        self.current
            .get("kind")
            .expect("Could not get kind")
            .as_u64()
            .expect("Could not cast kind to integer")
    }

    pub(crate) fn has_property(&self, index: &str) -> bool {
        match self.current {
            &Value::Object(ref root) => root.contains_key(index),
            _ => false,
        }
    }

    pub(crate) fn get_str(&self, key: &str) -> &str {
        self.current
            .get(key)
            .expect(&format!("Could not get {}", key))
            .as_str()
            .expect(&format!("Could not cast {} to &str", key))
    }

    pub(crate) fn has_index(&self, index: usize) -> bool {
        match self.current {
            &Value::Array(ref root) => root.len() > index,
            _ => false,
        }
    }

    pub(crate) fn get_vec<F, T>(&self, key: &str, map: F) -> Vec<T>
    where
        F: Fn(AstCursor) -> T,
    {
        self.current
            .get(key)
            .expect(&format!("Could not get property '{}'", key))
            .as_array()
            .expect(&format!("Could not cast '{}' to array", key))
            .iter()
            .map(|e| {
                map(AstCursor {
                    current: e,
                    root: e,
                    current_name: Some(String::from("root")),
                })
            })
            .collect()
    }
}
