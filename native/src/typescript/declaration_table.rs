use ahash::{HashMap, HashMapExt};
use dprint_swc_ext::view::Node;

struct Scope<'a> {
    symbols: HashMap<String, Declaration<'a>>,
}

#[derive(Default)]
pub struct DeclarationTables<'n> {
    tables: HashMap<String, DeclarationTable<'n>>,
}
impl<'n> DeclarationTables<'n> {
    pub fn insert(&mut self, file_path: &str, name: String, value: Declaration<'n>) -> () {
        let table = self.tables.entry(file_path.to_owned()).or_insert_with(Default::default);
        table.insert(name, value);
    }

    pub fn push_scope(&mut self, file_path: &str) -> () {
        self.tables
            .entry(file_path.to_owned())
            .or_insert_with(Default::default)
            .push_scope();
    }

    pub fn pop_scope(&mut self, file_path: &str) -> () {
        self.tables
            .entry(file_path.to_owned())
            .or_insert_with(Default::default)
            .pop_scope();
    }

    pub fn get_root_declaration(&mut self, file_path: &str, reference: String) -> String {
        self.tables
            .entry(file_path.to_owned())
            .or_insert_with(Default::default)
            .get_root_declaration(reference)
    }

    pub fn get_table(&mut self, file_path: &str) -> &mut DeclarationTable<'n> {
        self.tables.entry(file_path.to_owned()).or_insert_with(Default::default)
    }
}

#[derive(Default)]
pub struct DeclarationTable<'a> {
    scopes: Vec<Scope<'a>>,
}
impl<'a> DeclarationTable<'a> {
    pub(crate) fn new() -> DeclarationTable<'a> {
        DeclarationTable {
            scopes: vec![Scope {
                symbols: HashMap::new(),
            }],
        }
    }

    fn insert(&mut self, name: String, value: Declaration<'a>) -> () {
        if let Some(scope) = self.scopes.last_mut() {
            scope.symbols.insert(name, value);
        }
    }

    fn push_scope(&mut self) -> &mut DeclarationTable<'a> {
        self.scopes.push(Scope {
            symbols: HashMap::new(),
        });
        self
    }

    fn pop_scope(&mut self) -> &mut DeclarationTable<'a> {
        self.scopes.pop();
        self
    }

    fn get_root_declaration(&self, reference: String) -> String {
        let mut current = &reference;
        let mut previous = &reference;
        for scope in self.scopes.iter().rev() {
            match scope.symbols.get(current) {
                Some(Declaration::Alias { from: _, to }) => current = to,
                Some(Declaration::Export {
                    name,
                    source_file_name: _,
                }) => current = name,
                Some(Declaration::Import {
                    name,
                    source_file_name: _,
                }) => current = name,
                _ => {}
            };

            if previous.eq(current) {
                break;
            }

            previous = current;
        }

        current.to_owned()
    }
}

pub enum Declaration<'a> {
    Alias { from: String, to: String },
    Type { node: Node<'a> },
    Export { name: String, source_file_name: String },
    Import { name: String, source_file_name: String },
}
