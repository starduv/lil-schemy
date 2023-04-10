use std::collections::BTreeMap;

use dprint_swc_ext::view::Node;

struct Scope<'a> {
    symbols: BTreeMap<String, Declaration<'a>>,
}

pub struct DeclarationTable<'a> {
    scopes: Vec<Scope<'a>>,
}
impl<'a> DeclarationTable<'a> {
    pub(crate) fn new() -> DeclarationTable<'a> {
        DeclarationTable {
            scopes: vec![Scope {
                symbols: BTreeMap::new(),
            }],
        }
    }

    pub(crate) fn insert(&mut self, name: String, value: Declaration<'a>) -> () {
        if let Some(scope) = self.scopes.last_mut() {
            scope.symbols.insert(name, value);
        }
    }

    pub(crate) fn push_scope(&mut self) -> &mut DeclarationTable<'a> {
        self.scopes.push(Scope {
            symbols: BTreeMap::new(),
        });
        self
    }

    pub(crate) fn pop_scope(&mut self) -> &mut DeclarationTable<'a> {
        self.scopes.pop();
        self
    }

    pub(crate) fn get_root_declaration(&self, reference: String) -> String {
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
