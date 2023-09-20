use std::thread::JoinHandle;

use crossbeam::channel::{Sender, Receiver};

use crate::typescript::Node;

pub mod open_api;

pub trait Mapper<TOptions, TResult> {
    fn run(options: Option<TOptions>, request_module: Sender<&str>, on_node: Receiver<(&str, Node)>) -> Option<JoinHandle<TResult>>;
}