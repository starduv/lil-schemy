mod options;

use std::{
    option,
    thread::{self, JoinHandle},
};

pub use options::OpenApiOptions;
use serde::Serialize;

use super::Mapper;

pub struct OpenApiMapper {}

impl Mapper<OpenApiOptions, OpenApiResult> for OpenApiMapper {
    fn run(
        options: Option<OpenApiOptions>,
        request_module: crossbeam::channel::Sender<String>,
        on_node: crossbeam::channel::Receiver<(String, crate::typescript::Node)>,
    ) -> Option<JoinHandle<OpenApiResult>> {
        match options {
            Some(options) => Some(thread::spawn(|| pow(options, request_module, on_node))),
            None => None,
        }
    }
}

fn pow(
    options: OpenApiOptions,
    request_module: crossbeam::channel::Sender<String>,
    on_node: crossbeam::channel::Receiver<(String, crate::typescript::Node)>,
) -> OpenApiResult {
    println!("{:?}", options);
    OpenApiResult {
        schema: OpenApi {},
        filepath: options.output,
    }
}

#[derive(Serialize, Debug)]
pub struct OpenApi {}

#[derive(Serialize, Debug)]
pub struct OpenApiResult {
    pub schema: OpenApi,
    pub filepath: Option<String>,
}
