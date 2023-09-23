use std::{
    sync::{
        atomic::{AtomicU8, Ordering},
        Arc,
    },
    time::Duration,
};

use crossbeam::channel::{Receiver, Sender};

use crate::{
    messaging::{Message, Reply},
    typescript::Node,
};

static IDENTITY: AtomicU8 = AtomicU8::new(0);

pub struct MessageBus {
    id: u8,
    message: Sender<Message>,
    reply: Receiver<Reply>,
}

impl Drop for MessageBus {
    fn drop(&mut self) {
        self.message.send(Message::UnregisterSender(self.id)).unwrap();
    }
}

impl MessageBus {
    pub fn new(send_message: Sender<Message>, on_send_response: Receiver<Reply>) -> Self {
        let id = IDENTITY.fetch_add(1, Ordering::AcqRel);

        send_message.send(Message::RegisterSender(id)).unwrap();

        Self {
            id,
            message: send_message,
            reply: on_send_response,
        }
    }

    pub(crate) fn request_module(&self, filepath: String) -> Arc<Node<'static>> {
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
