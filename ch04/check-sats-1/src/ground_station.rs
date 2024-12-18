use crate::{
    cube_sat::CubeSat,
    mailbox::{MailBox, Message},
};

pub struct GroundStation;

impl GroundStation {
    pub fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat::new(sat_id)
    }

    pub fn send(&self, mailbox: &mut MailBox, msg: Message) {
        mailbox.post(msg);
    }
}
