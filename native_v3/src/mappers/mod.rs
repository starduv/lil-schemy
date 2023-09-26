use std::thread::JoinHandle;

use crate::messaging::MessageBus;

pub mod open_api;

pub trait Mapper<TOptions, TResult> {
    fn run(options: Option<TOptions>, mapper_bus: MessageBus) -> Option<JoinHandle<TResult>>;
}
