mod options;

use std::{
    sync::Arc,
    thread::{self, JoinHandle},
};

pub use options::OpenApiOptions;
use serde::Serialize;

use crate::typescript::Node;

use super::{Mapper, MessageBus};

pub struct OpenApiMapper {}

impl Mapper<OpenApiOptions, OpenApiResult> for OpenApiMapper {
    fn run(options: Option<OpenApiOptions>, mapper_bus: MessageBus) -> Option<JoinHandle<OpenApiResult>> {
        match options {
            Some(options) => Some(thread::spawn(move || {
                let open_api = OpenApi::default();
                for filepath in options.filepaths {
                    let module = mapper_bus.request_module(filepath);
                    open_api.add_paths(module, &mapper_bus);
                }

                OpenApiResult {
                    schema: open_api,
                    filepath: options.output,
                }
            })),
            None => None,
        }
    }
}

#[derive(Serialize, Debug, Default)]
pub struct OpenApi {}
impl OpenApi {
    fn add_paths(&self, node: Arc<Node<'_>>, mapper_bus: &MessageBus) -> () {
        match node.is_path() {
            true => {
                println!("Found path: {:?}", node);
            }
            false => for child in mapper_bus.request_children(node.id()) {},
        }
    }
}

#[derive(Serialize, Debug)]
pub struct OpenApiResult {
    pub schema: OpenApi,
    pub filepath: Option<String>,
}
