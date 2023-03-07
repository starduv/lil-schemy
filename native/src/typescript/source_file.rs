use serde::Deserialize;

use super::{
    ARROW_FUNCTION, ASYNC_KEYWORD, BLOCK, CALL_EXPRESSION, CLASS_DECLARATION, EXPRESSION_STATEMENT, IMPORT_CLAUSE,
    INTERFACE_DECLARATION, NAMED_IMPORTS, SOURCE_FILE, TYPE_ALIAS_DECLARATION, TYPE_LITERAL, VARIABLE_DECLARATION,
    VARIABLE_DECLARATION_LIST, VARIABLE_STATEMENT,
};

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AstNode {
    pub arguments: Option<Vec<AstNode>>,
    pub body: Option<Box<AstNode>>,
    pub declaration_list: Option<Box<AstNode>>,
    pub declarations: Option<Vec<AstNode>>,
    pub elements: Option<Vec<AstNode>>,
    pub escaped_text: Option<String>,
    pub export_clause: Option<Box<AstNode>>,
    pub expression: Option<Box<AstNode>>,
    pub file_name: Option<String>,
    pub import_clause: Option<Box<AstNode>>,
    pub initializer: Option<Box<AstNode>>,
    pub literal: Option<Box<AstNode>>,
    pub kind: u16,
    pub members: Option<Vec<AstNode>>,
    pub module_specifier: Option<Box<AstNode>>,
    pub modifiers: Option<Vec<AstNode>>,
    pub name: Option<Box<AstNode>>,
    pub named_bindings: Option<Box<AstNode>>,
    pub parameters: Option<Vec<AstNode>>,
    pub statements: Option<Vec<AstNode>>,
    pub properties: Option<Vec<AstNode>>,
    pub property_name: Option<Box<AstNode>>,
    pub text: Option<String>,
    #[serde(rename = "type")]
    pub _type: Option<Box<AstNode>>,
    pub type_name: Option<Box<AstNode>>,
    pub type_arguments: Option<Vec<AstNode>>,
}

impl AstNode {
    pub(crate) fn for_each_child(&self, mut func: impl FnMut(&AstNode)) -> () {
        match self.kind {
            ARROW_FUNCTION => {
                for ref node in self.parameters.as_ref().unwrap() {
                    func(node)
                }

                func(self.body.as_ref().unwrap());
            }
            BLOCK => {
                for ref node in self.statements.as_ref().unwrap() {
                    func(node)
                }
            }
            CALL_EXPRESSION => {
                if let Some(ref args) = self.arguments {
                    for arg in args {
                        func(arg)
                    }
                }
            }
            CLASS_DECLARATION => {
                if let Some(ref members) = self.members {
                    for member in members {
                        func(member)
                    }
                }
            }
            INTERFACE_DECLARATION => {
                if let Some(ref members) = self.members {
                    for member in members {
                        func(member)
                    }
                }
            }
            EXPRESSION_STATEMENT => func(self.expression.as_ref().unwrap()),
            IMPORT_CLAUSE => {
                if let Some(ref bindings) = self.named_bindings {
                    func(bindings)
                }
            }
            NAMED_IMPORTS => {
                if let Some(ref elements) = self.elements {
                    for element in elements {
                        func(element)
                    }
                }
            }
            SOURCE_FILE => {
                for ref node in self.statements.as_ref().unwrap() {
                    func(node)
                }
            }
            TYPE_ALIAS_DECLARATION => {
                if let Some(ref members) = self.members {
                    for member in members {
                        func(member)
                    }
                }
            }
            TYPE_LITERAL => {
                if let Some(ref members) = self.members {
                    for member in members {
                        func(member)
                    }
                }
            }
            VARIABLE_DECLARATION => {
                if let Some(ref initializer) = self.initializer {
                    func(initializer)
                }
            }
            VARIABLE_DECLARATION_LIST => {
                if let Some(ref declarations) = self.declarations {
                    for declaration in declarations {
                        func(declaration)
                    }
                }
            }
            VARIABLE_STATEMENT => {
                let list = self.declaration_list.as_ref().unwrap();
                list.for_each_child(func);
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
pub enum Declaration {
    Alias { from: String, to: String },
    Type { node: AstNode },
    Export { name: String, file: String },
    Import { name: String, file: String },
}
