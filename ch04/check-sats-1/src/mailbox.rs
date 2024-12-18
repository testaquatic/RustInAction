use crate::cube_sat::CubeSat;

#[derive(Debug)]
pub struct MailBox {
    messages: Vec<Message>,
}

#[derive(Debug)]
pub struct Message {
    to: u64,
    content: String,
}

impl Default for MailBox {
    fn default() -> Self {
        Self::new()
    }
}

impl MailBox {
    pub fn new() -> MailBox {
        MailBox {
            messages: Vec::new(),
        }
    }

    pub fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    pub fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        let find = self
            .messages
            .iter()
            .enumerate()
            .find(|(_, msg)| msg.to == recipient.id())
            .map(|(idx, _)| idx);

        find.map(|idx| self.messages.remove(idx))
    }
}

impl Message {
    pub fn new(to: u64, content: String) -> Message {
        Self { to, content }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}
