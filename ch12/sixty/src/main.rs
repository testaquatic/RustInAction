use core::time;
use std::{process, thread::sleep};

fn main() {
    let delay = time::Duration::from_secs(1);
    let pid = process::id();
    println!("{pid}");

    (1..=60).for_each(|i| {
        sleep(delay);
        println!(". {i}")
    });
}
