use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct DeferredSchemas {
    modules: Vec<String>,
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
}

#[derive(Debug)]
pub struct DeferredType {
    pub schema_name: String,
    pub namespace: Option<String>,
}
