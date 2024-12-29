use core::fmt;

use rand::RngCore;

#[derive(Debug)]
struct MacAddress([u8; 6]);

impl fmt::Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }
}

impl MacAddress {
    fn new() -> MacAddress {
        let mut octets = [0; 6];
        rand::thread_rng().fill_bytes(&mut octets);
        octets[0] |= 0b0000_0011;

        MacAddress(octets)
    }

    fn is_local(&self) -> bool {
        (self.0[0] & 0b0000_0010) == 0b0000_0010
    }

    fn is_unicast(&self) -> bool {
        (self.0[0] & 0b0000_0001) == 0b0000_0001
    }
}

fn main() {
    let mac = MacAddress::new();
    assert!(mac.is_local());
    assert!(mac.is_unicast());
    println!("mac: {}", mac);
}
