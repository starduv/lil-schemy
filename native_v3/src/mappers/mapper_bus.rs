use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use crossbeam::channel::{Receiver, Sender};

use crate::{
    messaging::{Message, Reply},
    typescript::Node,
};

static IDENTITY: Mutex<u8> = Mutex::new(0);

pub struct MessageBus {
    id: u8,
    message: Sender<Message>,
    reply: Receiver<Reply>,
}

impl MessageBus {
    pub fn new(send_message: Sender<Message>, on_send_response: Receiver<Reply>) -> Self {
        let mut id = IDENTITY.lock().unwrap();
        *id += 1;

        Self {
            id: *id,
            message: send_message,
            reply: on_send_response,
        }
    }

    pub(crate) fn request_children(&self, node_id: usize) -> Vec<Arc<Node<'_>>> {
        self.message.send(Message::RequestChildren(self.id, node_id));
        loop {
            match self.reply.recv_timeout(Duration::from_secs(20)) {
                Ok(Reply::SendChildren(ref id, children)) if self.id.eq(id) => return children,
                Err(err) => panic!("Error receiving reply: {}", err),
                _ => continue,
            }
        }
    }

    pub(crate) fn request_module(&self, filepath: String) -> Arc<Node<'_>> {
        self.message.send(Message::RequestModule(self.id, filepath)).unwrap();
        loop {
            match self.reply.recv_timeout(Duration::from_secs(20)) {
                Ok(Reply::SendModule(ref id, node)) if self.id.eq(id) => return node,
                Err(err) => panic!("Error receiving reply: {}", err),
                _ => continue,
            }
        }
    }
}
