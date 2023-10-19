mod options;
mod path_options;

use std::{
    collections::HashSet,
    ops::Deref,
    sync::Arc,
    thread::{self, JoinHandle},
};

pub use options::OpenApiOptions;
use serde::Serialize;
use swc_ecma_ast::{Expr, Lit, Prop, PropName};

use crate::{typescript::{Node, NodeKind}, schemy::{SchemyPath, self}};

use self::path_options::PathOptions;

use super::{Mapper, MessageBus};

pub struct OpenApiMapper {}

impl Mapper<OpenApiOptions, OpenApiResult> for OpenApiMapper {
    fn run(options: Option<OpenApiOptions>, bus: MessageBus) -> Option<JoinHandle<OpenApiResult>> {
        let mut module_ids = HashSet::<usize>::new();

        match options {
            Some(options) => Some(thread::spawn(move || {
                let mut open_api = OpenApi::default();
                for ref filepath in options.filepaths {
                    module_ids.insert(bus.request_module(filepath.clone()));
                }

                bus.wait_serialize();

                bus.on_schemy_created(|schemy| {
                    if module_ids.contains(&schemy.module_id) {
                        add_path(&mut open_api, schemy, &bus);
                    }
                });

                OpenApiResult {
                    schema: open_api,
                    filepath: options.output,
                }
            })),
            None => None,
        }
    }
}

fn add_path(open_api: &mut OpenApi, node: Arc<SchemyPath>, bus: &MessageBus) -> () {
    println!("add_path: {:?}", node);
}

#[derive(Serialize, Debug, Default)]
pub struct OpenApi {}
impl OpenApi {}

#[derive(Serialize, Debug)]
pub struct OpenApiResult {
    pub schema: OpenApi,
    pub filepath: Option<String>,
}
