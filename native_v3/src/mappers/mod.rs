use std::thread::JoinHandle;

mod mapper_bus;

pub mod open_api;
pub use mapper_bus::MessageBus;

pub trait Mapper<TOptions, TResult> {
    fn run(options: Option<TOptions>, mapper_bus: MessageBus) -> Option<JoinHandle<TResult>>;
}
