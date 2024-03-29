use std::{
    cell::RefCell,
    collections::{BTreeMap, VecDeque},
    fmt,
    rc::Rc,
};

use crate::typescript::SchemyNode;

#[derive(Debug, Default)]
struct Scope {
    symbols: BTreeMap<String, Declaration>,
    children: Option<Vec<Rc<RefCell<Scope>>>>,
    parent: Option<Rc<RefCell<Scope>>>,
}

#[derive(Debug, Default)]
pub struct DeclarationTables {
    tables: BTreeMap<String, DeclarationTable>,
}
impl<'n> DeclarationTables {
    pub fn insert(&mut self, file_path: &str, name: String, value: Declaration) -> () {
        let table = self.tables.entry(file_path.to_owned()).or_insert_with(Default::default);
        table.insert(name, value);
    }

    pub fn add_child_scope(&mut self, file_path: &str) -> () {
        self.tables
            .entry(file_path.to_owned())
            .or_insert_with(Default::default)
            .add_child_scope();
    }

    pub fn parent_scope(&mut self, file_path: &str) -> () {
        self.tables
            .entry(file_path.to_owned())
            .or_insert_with(Default::default)
            .parent_scope();
    }

    pub fn get_root_declaration_name(&mut self, file_path: &str, reference: &str) -> String {
        self.tables
            .entry(file_path.to_owned())
            .or_insert_with(Default::default)
            .get_root_declaration_name(reference)
    }

    pub(crate) fn get_root_declaration(&mut self, file_path: &str, reference: &str) -> Option<Declaration> {
        self.tables
            .entry(file_path.to_owned())
            .or_insert_with(Default::default)
            .get_root_declaration(reference)
    }

    #[allow(dead_code)]
    pub fn debug(&self, file_path: &str, reference: &str) -> () {
        let table = self.tables.get(file_path);
        match table {
            Some(table) => {
                table.debug(reference);
            }
            None => println!("No table found for file path: {}", file_path),
        }
    }
}

#[derive(Debug, Default)]
pub struct DeclarationTable {
    current_scope: Rc<RefCell<Scope>>,
}
impl DeclarationTable {
    #[allow(dead_code)]
    fn debug(&self, reference: &str) -> () {
        let mut declaration: Option<Declaration> = None;
        let mut queue = VecDeque::from([Rc::clone(&self.current_scope)]);
        let mut references = vec![reference.to_string()];
        let mut last_reference = String::from("__none__");

        while declaration.is_none() && !queue.is_empty() {
            if let Some(scope) = queue.pop_front() {
                let scope = scope.borrow();
                while references.len() > 0 {
                    let temp = references.pop().unwrap();
                    match scope.symbols.get(&temp) {
                        Some(Declaration::Alias { to }) => {
                            println!("Alias: {} -> {}", temp, to);
                            references.push(to.clone());
                        }
                        _ => {}
                    }
                    last_reference = temp;
                }

                match scope.symbols.get(&last_reference) {
                    Some(decl) => declaration = Some(decl.clone()),
                    None => {
                        if let Some(parent) = &scope.parent {
                            references.push(last_reference.clone());
                            queue.push_back(Rc::clone(&parent));
                        }
                    }
                }
            }
        }

        declaration
            .iter()
            .for_each(|d| println!("{} has the declaration: {:?}", reference, d));
    }

    fn insert(&mut self, name: String, value: Declaration) -> () {
        self.current_scope.borrow_mut().symbols.insert(name, value);
    }

    fn add_child_scope(&mut self) -> &mut DeclarationTable {
        let child_scope = Rc::new(RefCell::new(Scope {
            symbols: BTreeMap::new(),
            children: None,
            parent: Some(Rc::clone(&self.current_scope)),
        }));

        self.current_scope
            .borrow_mut()
            .children
            .get_or_insert_with(Default::default)
            .push(Rc::clone(&child_scope));

        self.current_scope = child_scope;

        self
    }

    fn parent_scope(&mut self) -> &mut DeclarationTable {
        let parent = Rc::clone(
            self.current_scope
                .borrow()
                .parent
                .as_ref()
                .expect("Expected current scope to have parent."),
        );

        self.current_scope = Rc::clone(&parent);

        self
    }

    fn get_root_declaration(&self, reference: &str) -> Option<Declaration> {
        let mut declaration: Option<Declaration> = None;
        let mut queue = VecDeque::from([Rc::clone(&self.current_scope)]);
        let mut references = vec![reference.to_string()];
        let mut last_reference = String::from("__none__");

        while declaration.is_none() && !queue.is_empty() {
            if let Some(scope) = queue.pop_front() {
                let scope = scope.borrow();
                while references.len() > 0 {
                    let temp = references.pop().unwrap();
                    match scope.symbols.get(&temp) {
                        Some(Declaration::Alias { to }) => {
                            references.push(to.clone());
                        }
                        _ => {}
                    }
                    last_reference = temp;
                }

                match scope.symbols.get(&last_reference) {
                    Some(decl) => declaration = Some(decl.clone()),
                    None => {
                        if let Some(parent) = &scope.parent {
                            references.push(last_reference.clone());
                            queue.push_back(Rc::clone(&parent));
                        }
                    }
                }
            }
        }

        return declaration;
    }

    fn get_root_declaration_name(&self, reference: &str) -> String {
        let mut current = reference.to_string();
        let mut previous = reference.to_string();
        let mut queue = VecDeque::from([Rc::clone(&self.current_scope)]);
        while !queue.is_empty() {
            if let Some(scope) = queue.pop_front() {
                let scope = scope.borrow();
                match scope.symbols.get(&current) {
                    Some(Declaration::Alias { to }) => current = to.clone(),
                    Some(Declaration::Export {
                        name,
                        source_file_name: _,
                    }) => current = name.clone(),
                    Some(Declaration::Import {
                        name,
                        source_file_name: _,
                    }) => current = name.clone(),
                    _ => {}
                };

                if previous.eq(&current) {
                    break;
                }

                previous = current.clone();

                if let Some(parent) = &scope.parent {
                    queue.push_back(Rc::clone(&parent))
                }
            }
        }

        current.to_owned()
    }
}

pub enum Declaration {
    Alias { to: String },
    Type { node: Rc<SchemyNode<'static>> },
    Export { name: String, source_file_name: String },
    Import { name: String, source_file_name: String },
}

impl Clone for Declaration {
    fn clone(&self) -> Self {
        match self {
            Self::Alias { to } => Self::Alias { to: to.clone() },
            Self::Type { node } => Self::Type { node: node.clone() },
            Self::Export { name, source_file_name } => Self::Export {
                name: name.clone(),
                source_file_name: source_file_name.clone(),
            },
            Self::Import { name, source_file_name } => Self::Import {
                name: name.clone(),
                source_file_name: source_file_name.clone(),
            },
        }
    }
}

impl fmt::Debug for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Alias { to } => f.debug_struct("Alias").field("to", to).finish(),
            Self::Export { name, source_file_name } => f
                .debug_struct("Export")
                .field("name", name)
                .field("source_file_name", source_file_name)
                .finish(),
            Self::Import { name, source_file_name } => f
                .debug_struct("Import")
                .field("name", name)
                .field("source_file_name", source_file_name)
                .finish(),
            _ => fmt::Result::Ok(()),
        }
    }
}
