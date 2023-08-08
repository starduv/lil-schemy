use std::{collections::BTreeMap, rc::Rc, cell::RefCell};

use super::schema::ApiPathOperation;

#[derive(Debug, Default)]
pub struct DeferredSchemas {
    modules: Vec<String>,
    operation_types: BTreeMap<String, BTreeMap<String, DeferredOperationType>>,
    types: BTreeMap<String, BTreeMap<String, DeferredType>>,
}

impl DeferredSchemas {
    pub(crate) fn next_module(&mut self) -> Option<String> {
        self.modules.pop()
    }

    pub(crate) fn add_deferred_type(
        &mut self,
        source_file_name: String,
        schema_name: String,
        type_name: String,
        namespace: Option<String>,
    ) -> () {
        if !self.modules.contains(&source_file_name) {
            self.modules.push(source_file_name.clone());
        }

        let types = self.types.entry(source_file_name).or_insert(BTreeMap::new());

        types.insert(type_name.clone(), DeferredType { schema_name, namespace });
    }

    pub(crate) fn get_deferred_type(&self, name: &str, source_file_name: &str) -> Option<&DeferredType> {
        match self.types.get(source_file_name) {
            Some(types) => types.get(name),
            None => None,
        }
    }

    pub(crate) fn add_deferred_operation_type(
        &mut self,
        source_file_name: &str,
        operation: *mut ApiPathOperation,
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
            DeferredOperationType {
                operation,
                type_name: type_name.to_string(),
            },
        );
    }

    pub fn get_deferred_operation_type(
        &self,
        type_name: &str,
        source_file_name: &str,
    ) -> Option<&DeferredOperationType> {
        match self.operation_types.get(source_file_name) {
            Some(types) => types.get(type_name),
            None => None,
        }
    }

    pub fn debug(&self) -> () {
        println!("{:?}", self.modules);
    }
}

#[derive(Debug)]
pub struct DeferredType {
    pub schema_name: String,
    pub namespace: Option<String>,
}

#[derive(Debug)]
pub struct DeferredOperationType {
    pub operation: *mut ApiPathOperation,
    pub type_name: String,
}
