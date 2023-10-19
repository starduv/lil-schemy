use std::{
    collections::HashSet,
    fmt::Debug,
    sync::{
        atomic::{AtomicU8, Ordering},
        Arc,
    },
};

use crate::{schemy::SchemyPath, typescript::Node};

use std::time::Duration;

use crossbeam::channel::{Receiver, Sender};

static IDENTITY: AtomicU8 = AtomicU8::new(0);

pub struct MessageBus {
    id: u8,
    module_id: Sender<(u8, usize)>,
    module_parsed: Sender<(usize, Arc<Node<'static>>)>,
    on_module_id: Receiver<(u8, usize)>,
    on_module_parsed: Receiver<(usize, Arc<Node<'static>>)>,
    on_parse_module: Receiver<(usize, String)>,
    on_request_module: Receiver<(u8, String)>,
    on_schemy: Receiver<Arc<SchemyPath>>,
    on_serialize: Receiver<u8>,
    parse_module: Sender<(usize, String)>,
    request_module: Sender<(u8, String)>,
    schemy: Sender<Arc<SchemyPath>>,
    serialize: Sender<u8>,
}

impl Debug for MessageBus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MessageBus").field("id", &self.id).finish()
    }
}

impl Clone for MessageBus {
    fn clone(&self) -> Self {
        let id = IDENTITY.fetch_add(1, Ordering::AcqRel);

        Self {
            id,
            module_id: self.module_id.clone(),
            module_parsed: self.module_parsed.clone(),
            on_module_id: self.on_module_id.clone(),
            on_module_parsed: self.on_module_parsed.clone(),
            on_parse_module: self.on_parse_module.clone(),
            on_request_module: self.on_request_module.clone(),
            on_schemy: self.on_schemy.clone(),
            on_serialize: self.on_serialize.clone(),
            parse_module: self.parse_module.clone(),
            request_module: self.request_module.clone(),
            schemy: self.schemy.clone(),
            serialize: self.serialize.clone(),
        }
    }
}

impl MessageBus {
    pub fn new() -> Self {
        let id = IDENTITY.fetch_add(1, Ordering::AcqRel);

        let (mappers_go, on_mappers_go) = crossbeam::channel::bounded::<u8>(1);
        let (module_id, on_module_id) = crossbeam::channel::unbounded::<(u8, usize)>();
        let (module_parsed, on_module_parsed) = crossbeam::channel::unbounded::<(usize, Arc<Node<'static>>)>();
        let (parse_module, on_parse_module) = crossbeam::channel::unbounded::<(usize, String)>();
        let (request_module, on_request_module) = crossbeam::channel::unbounded::<(u8, String)>();
        let (schemy, on_schemy) = crossbeam::channel::unbounded::<Arc<SchemyPath>>();

        Self {
            id,
            module_id,
            module_parsed,
            on_module_id,
            on_module_parsed,
            on_parse_module,
            on_request_module,
            on_schemy,
            on_serialize: on_mappers_go,
            parse_module,
            request_module,
            schemy,
            serialize: mappers_go,
        }
    }

    pub(crate) fn gather_module_requests(&self, mut handler: impl FnMut(u8, String)) -> () {
        loop {
            match self.on_request_module.recv_timeout(Duration::from_millis(1)) {
                Ok(message) => handler(message.0, message.1),
                _ => break,
            }
        }
    }

    pub(crate) fn request_module(&self, module_path: String) -> usize {
        self.request_module.send((self.id, module_path)).unwrap();
        loop {
            match self.on_module_id.recv() {
                Ok((id, module_id)) if id.eq(&self.id) => {
                    if id == self.id {
                        return module_id;
                    }
                }
                _ => continue,
            }
        }
    }

    pub(crate) fn send_module_id(&self, id: u8, module_id: usize) -> () {
        self.module_id.send((id, module_id)).unwrap();
    }

    pub(crate) fn parse_module(&self, module_id: usize, module_path: String) -> () {
        self.parse_module.send((module_id, module_path)).unwrap();
    }

    pub(crate) fn on_parse_module(&self, mut handler: impl FnMut(&usize, &String)) -> () {
        loop {
            match self.on_parse_module.recv_timeout(Duration::from_millis(1)) {
                Ok((module_id, module_path)) => handler(&module_id, &module_path),
                _ => break,
            }
        }
    }

    pub(crate) fn module_parsed(&self, module_id: usize, node: Arc<Node<'static>>) -> () {
        self.module_parsed.send((module_id, node)).unwrap();
    }

    pub(crate) fn on_module_parsed(&self, mut handler: impl FnMut(&usize, Arc<Node<'static>>)) -> () {
        loop {
            match self.on_module_parsed.recv_timeout(Duration::from_millis(1)) {
                Ok((module_id, node)) => handler(&module_id, node),
                _ => break,
            }
        }
    }

    pub(crate) fn schemy_created(&self, schemy_path: SchemyPath) -> () {
        self.schemy.send(Arc::new(schemy_path)).unwrap();
    }

    pub(crate) fn on_schemy_created(&self, mut handler: impl FnMut(Arc<SchemyPath>)) -> () {
        loop {
            match self.on_schemy.recv_timeout(Duration::from_millis(1)) {
                Ok(schemy) => handler(schemy),
                _ => break,
            }
        }
    }

    pub(crate) fn wait_serialize(&self) -> () {
        self.on_serialize.recv().unwrap();
    }

    pub(crate) fn begin_serialize(&self) -> () {
        self.serialize.send(self.id).unwrap();
    }
}
