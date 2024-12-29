use core::fmt;

use rand::RngCore;
use smoltcp::wire;

#[derive(Debug)]
pub struct MacAddress([u8; 6]);

impl fmt::Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }
}

impl MacAddress {
    pub fn new() -> MacAddress {
        let mut octets = [0_u8; 6];
        rand::thread_rng().fill_bytes(&mut octets);
        octets[0] |= 0b_0000_0010;
        octets[0] &= 0b_1111_1110;

        MacAddress(octets)
    }
}

impl From<MacAddress> for wire::EthernetAddress {
    fn from(value: MacAddress) -> Self {
        wire::EthernetAddress(value.0)
    }
}

impl Default for MacAddress {
    fn default() -> Self {
        Self::new()
    }
}
