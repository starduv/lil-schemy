mod caching;
mod declaration_table;
mod deferred;

use self::{declaration_table::DeclarationTables, deferred::DeferredSchemas};

pub use self::declaration_table::Declaration;

pub struct Store {
    deferred_schemas: DeferredSchemas,
    symbol_tables: DeclarationTables,
}

impl Store {
    pub fn get_root_declaration(&mut self, file_path: &str, reference: &str) -> Option<Declaration> {
        self.symbol_tables.get_root_declaration(file_path, reference)
    }

    pub fn get_root_declaration_name(&mut self, file_path: &str, reference: &str) -> String {
        self.symbol_tables.get_root_declaration_name(file_path, reference)
    }

    pub(crate) fn new() -> Self {
        Self {
            deferred_schemas: DeferredSchemas::default(),
            symbol_tables: DeclarationTables::default(),
        }
    }
}
