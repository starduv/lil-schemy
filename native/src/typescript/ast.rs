use ahash::{HashMap, HashMapExt};
use serde_json::{from_str, Value};

pub struct AstCache<'n> {
    asts: serde_json::Value,
    cache: HashMap<&'n str, AstNode<'n>>,
}

impl AstCache<'_> {
    pub fn new(asts: String) -> Self {
        AstCache {
            asts: from_str(&asts).expect("Could not parse asts"),
            cache: HashMap::new(),
        }
    }

    pub(crate) fn get(&self, path: &str) -> &AstNode {
        self.cache.entry(path).or_insert_with(|| {
            let msg = &format!("Could not find ast for {}", path);
            let ast = self.asts.get(path).expect(msg);
            AstNode::new(ast)
        })
    }
}

pub struct AstNode<'v> {
    ast: &'v Value,
}

impl AstNode<'_> {
    pub fn new(ast: &serde_json::Value) -> AstNode {
        AstNode { ast }
    }

    pub(crate) fn get_statements(&self) -> Option<Vec<&AstNode>> {
        match self.ast.get("statements") {
            Some(statements) => match statements {
                &Value::Array(statements) => Some(statements.iter().map(|s| &AstNode::new(s)).collect()),
                _ => None,
            },
            None => None,
        }
    }

    pub(crate) fn is_api_path(&self) -> bool {
        match self.ast.get("expression") {
            Some(expression) => match expression {
                &Value::Object(expression) => match expression.get("escapedText") {
                    Some(text) => match text {
                        &Value::String(text) => text.eq("Path"),
                    },
                    None => false,
                },
                _ => false,
            },
            None => false,
        }
    }

    pub(crate) fn get_arguments(&self) -> Option<Vec<AstNode>> {
        match self.ast.get("arguments") {
            Some(arguments) => match arguments {
                &Value::Array(arguments) => Some(arguments.iter().map(|s| AstNode::new(s)).collect()),
                _ => None,
            },
            None => None,
        }
    }

    pub(crate) fn get_properties(&self) -> Option<Vec<AstNode>> {
        match self.ast.get("properties") {
            Some(properties) => match properties {
                &Value::Array(properties) => Some(properties.iter().map(|s| AstNode::new(s)).collect()),
                _ => None,
            },
            None => None,
        }
    }

    pub(crate) fn get_identifier(&self) -> Option<&str> {
        if let Some(text) = self.ast.get("escapedText") {
            return text.as_str();
        }

        if let Some(name) = self.ast.get("name") {
            if let Some(text) = name.get("escapedText") {
                return text.as_str();
            }
        }

        if let Some(name) = self.ast.get("expression") {
            if let Some(text) = name.get("escapedText") {
                return text.as_str();
            }
        }

        return None;
    }

    pub(crate) fn get_initialized_string(&self) -> Option<String> {
        match self.ast.get("initializer") {
            Some(initializer) => match initializer {
                &Value::Object(initializer) => match initializer.get("text") {
                    Some(text) => match text {
                        &Value::String(text) => Some(text.to_string()),
                        _ => None,
                    },
                    None => None,
                },
                _ => None,
            },
            None => None,
        }
    }

    pub(crate) fn get_initialized_array(&self) -> Option<Vec<String>> {
        match self.ast.get("initializer") {
            Some(initializer) => match initializer {
                &Value::Object(initializer) => match initializer.get("elements") {
                    Some(elements) => match elements {
                        &Value::Array(elements) => Some(elements.iter().map(|e| e.to_string()).collect()),
                        _ => None,
                    },
                    None => None,
                },
                _ => None,
            },
            None => None,
        }
    }
}
