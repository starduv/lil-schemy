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
                let mut open_api = OpenApi::default();
                for ref filepath in options.filepaths {
                    add_paths(
                        &mut open_api,
                        mapper_bus.request_module(filepath.clone()),
                        &filepath,
                        &mapper_bus,
                    );
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

fn add_paths(open_api: &mut OpenApi, node: Arc<Node<'static>>, filepath: &str, bus: &MessageBus) -> () {
    match node.is_path() {
        true => {
            add_path(open_api, node, filepath, bus);
        }
        false => {
            for child in node.children() {
                add_paths(open_api, child, filepath, bus);
            }
        }
    }
}

fn add_path(open_api: &mut OpenApi, node: Arc<Node>, filepath: &str, bus: &MessageBus) -> () {
    node.with_parent(|callee| callee.with_parent(|call_exp| {
        
    }))
}

#[derive(Serialize, Debug, Default)]
pub struct OpenApi {}
impl OpenApi {}

#[derive(Serialize, Debug)]
pub struct OpenApiResult {
    pub schema: OpenApi,
    pub filepath: Option<String>,
}
