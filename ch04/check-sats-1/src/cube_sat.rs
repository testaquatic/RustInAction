use crate::mailbox::{MailBox, Message};

#[derive(Debug)]
pub struct CubeSat {
    id: u64,
}

impl CubeSat {
    pub fn new(id: u64) -> CubeSat {
        CubeSat { id }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn recv(&self, mailbox: &mut MailBox) -> Option<Message> {
        mailbox.deliver(self)
    }
}

pub fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}
