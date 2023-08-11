use std::{collections::BTreeMap, rc::Rc, cell::RefCell};

use super::schema::ApiPathOperation;

#[derive(Debug, Default)]
pub struct DeferredSchemas {
    modules: Vec<String>,
    immediate_types: BTreeMap<String, Vec<DeferredImmediateType>>,
    operation_types: BTreeMap<String, BTreeMap<String, DeferredOperationType>>,
    types: BTreeMap<String, BTreeMap<String, DeferredType>>,
}

impl DeferredSchemas {
    pub(crate) fn next_module(&mut self) -> Option<String> {
        self.modules.pop()
    }

    pub(crate) fn add_deferred_type(&mut self, source_file_name: &str, schema_name: &str, type_name: &str) -> () {
        let schema_name = schema_name.to_string();
        let source_file_name = source_file_name.to_string();
        let type_name = type_name.to_string();

        if !self.modules.contains(&source_file_name) {
            self.modules.push(source_file_name.clone());
        }

        let types = self.types.entry(source_file_name).or_insert(BTreeMap::new());

        if !types.contains_key(&type_name) {
            types.insert(type_name, DeferredType { schema_name });
        }
    }

    pub(crate) fn take_deferred_type(&mut self, name: &str, source_file_name: &str) -> Option<DeferredType> {
        match self.types.get_mut(source_file_name) {
            Some(types) => types.remove(name),
            None => None,
        }
    }

    pub(crate) fn add_deferred_operation_type(
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

        if !types.contains_key(type_name){
            types.insert(
                type_name.to_string(),
                DeferredOperationType {
                    operation: operation.clone(),
                    type_name: type_name.to_string(),
                },
            );
        }
    }

    pub fn take_deferred_operation_type(
        &mut self,
        type_name: &str,
        source_file_name: &str,
    ) -> Option<DeferredOperationType> {
        match self.operation_types.get_mut(source_file_name) {
            Some(types) => types.remove(type_name),
            None => None,
        }
    }

    pub(crate) fn add_deferred_immediate(&mut self, file_path: &str, root_name: &str, node_index: usize) -> () {
        self.immediate_types
            .entry(file_path.to_string())
            .or_insert(Vec::new())
            .push(DeferredImmediateType {
                schema_name: root_name.into(),
                index: node_index,
            });
    }

    pub fn debug(&self) -> () {
        println!("{:?}", self.modules);
    }

    pub fn take_immediate_types(&mut self, file_path: &str) -> Vec<DeferredImmediateType> {
        if let Some(immediates) = self.immediate_types.get_mut(file_path) {
            immediates.drain(..).collect()
        } else {
            Vec::new()
        }
    }
}

#[derive(Debug)]
pub struct DeferredType {
    pub schema_name: String,
}

#[derive(Debug)]
pub struct DeferredOperationType {
    pub operation: Rc<RefCell<ApiPathOperation>>,
    pub type_name: String,
}

#[derive(Debug)]
pub struct DeferredImmediateType {
    pub schema_name: String,
    pub index: usize,
}
