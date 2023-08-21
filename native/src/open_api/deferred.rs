use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use super::schema::ApiPathOperation;

#[derive(Debug, Default)]
pub struct DeferredSchemas {
    local_types: BTreeMap<String, Vec<LocalType>>,
    modules: Vec<String>,
    operation_types: BTreeMap<String, BTreeMap<String, OperationType>>,
    external_types: BTreeMap<String, BTreeMap<String, ExternalType>>,
}

impl DeferredSchemas {
    pub(crate) fn next_module(&mut self) -> Option<String> {
        self.modules.pop()
    }

    pub(crate) fn defer_local_type(&mut self, file_path: &str, root_name: &str, node_index: usize) -> () {
        self.local_types
            .entry(file_path.to_string())
            .or_insert(Vec::new())
            .push(LocalType {
                schema_name: root_name.into(),
                index: node_index,
            });
    }

    pub(crate) fn defer_external_type(&mut self, source_file_name: &str, schema_name: &str, type_name: &str) -> () {
        let schema_name = schema_name.to_string();
        let source_file_name = source_file_name.to_string();
        let type_name = type_name.to_string();

        if !self.modules.contains(&source_file_name) {
            self.modules.push(source_file_name.clone());
        }

        let types = self.external_types.entry(source_file_name).or_insert(BTreeMap::new());

        types.insert(
            type_name,
            ExternalType {
                schema_name,
            },
        );
    }

    pub(crate) fn defer_operation_type(
        &mut self,
        source_file_name: &str,
        operation: &Rc<RefCell<ApiPathOperation>>,
        type_name: &str,
    ) -> () {
        if !self.modules.contains(&source_file_name.to_string()) {
            self.modules.push(source_file_name.to_string());
        }

        let types = self
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
        match self.external_types.get_mut(source_file_name) {
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
        match self.operation_types.get_mut(source_file_name) {
            Some(types) => types.remove(type_name),
            None => None,
        }
    }

    pub fn recognize_local_types(&mut self, file_path: &str) -> Vec<LocalType> {
        if let Some(immediates) = self.local_types.get_mut(file_path) {
            immediates.drain(..).collect()
        } else {
            Vec::new()
        }
    }

    pub fn has_unrecognized_local_types(&self, file_path: &str) -> bool {
        match self.local_types.get(file_path) {
            Some(immediate_types) => immediate_types.len() > 0,
            None => false,
        }
    }

    pub fn debug(&self) -> () {
        println!("{:?}", self.modules);
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
    pub index: usize,
}
