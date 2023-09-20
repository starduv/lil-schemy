mod options;

use std::thread::{self, JoinHandle};

pub use options::OpenApiOptions;
use serde::Serialize;

use super::Mapper;

pub struct OpenApiMapper {}

impl Mapper<OpenApiOptions, OpenApiResult> for OpenApiMapper {
    fn run(
        options: Option<OpenApiOptions>,
        request_module: crossbeam::channel::Sender<&str>,
        on_node: crossbeam::channel::Receiver<(&str, crate::typescript::Node)>,
    ) -> Option<JoinHandle<OpenApiResult>> {
        match options {
            Some(options) => Some(thread::spawn(|| {
                println!("{:?}", options);
                OpenApiResult {
                    schema: OpenApi {},
                    filepath: options.output,
                }
            })),
            None => None,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct OpenApi {}

pub struct OpenApiResult {
    pub schema: OpenApi,
    pub filepath: Option<String>,
}
