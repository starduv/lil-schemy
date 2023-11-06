use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use crate::open_api::schema::ApiPathOperation;

use super::Store;

#[derive(Debug, Default)]
pub struct DeferredSchemas {
    external_types: BTreeMap<String, BTreeMap<String, ExternalType>>,
    local_types: BTreeMap<String, Vec<LocalType>>,
    modules: Vec<String>,
    operation_types: BTreeMap<String, BTreeMap<String, OperationType>>,
}

impl Store {
    pub(crate) fn next_module(&mut self) -> Option<String> {
        self.deferred_schemas.modules.pop()
    }

    pub(crate) fn defer_local_type(
        &mut self,
        file_path: &str,
        schema_name: &str,
        type_name: &str,
        node_index: usize,
    ) -> () {
        self.deferred_schemas
            .local_types
            .entry(file_path.to_string())
            .or_insert(Vec::new())
            .push(LocalType {
                schema_name: schema_name.into(),
                type_name: type_name.into(),
                index: node_index,
            });
    }

    pub(crate) fn defer_external_type(&mut self, source_file_name: &str, schema_name: &str, type_name: &str) -> () {
        let schema_name = schema_name.to_string();
        let source_file_name = source_file_name.to_string();
        let type_name = type_name.to_string();

        if !self.deferred_schemas.modules.contains(&source_file_name) {
            self.deferred_schemas.modules.push(source_file_name.clone());
        }

        let types = self
            .deferred_schemas
            .external_types
            .entry(source_file_name)
            .or_insert(BTreeMap::new());

        types.insert(type_name, ExternalType { schema_name });
    }

    pub(crate) fn defer_operation_type(
        &mut self,
        source_file_name: &str,
        operation: &Rc<RefCell<ApiPathOperation>>,
        type_name: &str,
    ) -> () {
        if !self.deferred_schemas.modules.contains(&source_file_name.to_string()) {
            self.deferred_schemas.modules.push(source_file_name.to_string());
        }

        let types = self
            .deferred_schemas
            .operation_types
            .entry(source_file_name.to_string())
            .or_insert(BTreeMap::new());

        types.insert(
            type_name.to_string(),
            OperationType {
                operation: operation.clone(),
                type_name: type_name.to_string(),
            },
        );
    }

    pub(crate) fn recognize_external_type(&mut self, name: &str, source_file_name: &str) -> Option<ExternalType> {
        match self.deferred_schemas.external_types.get_mut(source_file_name) {
            Some(types) => match types.get_mut(name) {
                Some(deferred_type) => {
                    let clone = deferred_type.clone();
                    Some(clone)
                }
                None => None,
            },
            None => None,
        }
    }

    pub fn recognize_operation_type(&mut self, type_name: &str, source_file_name: &str) -> Option<OperationType> {
        match self.deferred_schemas.operation_types.get_mut(source_file_name) {
            Some(types) => types.remove(type_name),
            None => None,
        }
    }

    pub fn recognize_local_types(&mut self, file_path: &str) -> Vec<LocalType> {
        if let Some(local_types) = self.deferred_schemas.local_types.get_mut(file_path) {
            local_types.drain(..).collect()
        } else {
            Vec::new()
        }
    }

    pub fn has_unrecognized_local_types(&self, file_path: &str) -> bool {
        match self.deferred_schemas.local_types.get(file_path) {
            Some(local_types) => local_types.len() > 0,
            None => false,
        }
    }

    pub fn debug(&self) -> () {
        println!("{:?}", self.deferred_schemas.modules);
    }
}

#[derive(Debug, Clone)]
pub struct ExternalType {
    pub schema_name: String,
}

#[derive(Debug, Clone)]
pub struct OperationType {
    pub operation: Rc<RefCell<ApiPathOperation>>,
    pub type_name: String,
}

#[derive(Debug, Clone)]
pub struct LocalType {
    pub schema_name: String,
    pub type_name: String,
    pub index: usize,
}
