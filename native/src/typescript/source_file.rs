use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SourceFile {
    pub kind: u16,
    pub statements: Vec<AstNode>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AstNode {
    pub named_bindings: Option<Box<AstNode>>,
    pub arguments: Option<Vec<AstNode>>,
    pub body: Option<Box<AstNode>>,
    pub declaration_list: Option<Box<AstNode>>,
    pub declarations: Option<Vec<AstNode>>,
    pub elements: Option<Vec<AstNode>>,
    pub escaped_text: Option<String>,
    pub expression: Option<Box<AstNode>>,
    pub import_clause: Option<Box<AstNode>>,
    pub initializer: Option<Box<AstNode>>,
    pub literal: Option<Box<AstNode>>,
    pub kind: u16,
    pub members: Option<Vec<AstNode>>,
    pub name: Option<Box<AstNode>>,
    pub parameters: Option<Vec<AstNode>>,
    pub statements: Option<Vec<AstNode>>,
    pub properties: Option<Vec<AstNode>>,
    pub property_name: Option<Box<AstNode>>,
    pub text: Option<String>,
    pub type_description: Option<Box<AstNode>>,
    pub type_name: Option<Box<AstNode>>,
    pub type_arguments: Option<Vec<AstNode>>,
}
