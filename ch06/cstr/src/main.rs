use std::ffi::{c_char, CStr};

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn main() {
    let a = 42;

    let b = String::from_utf8_lossy(&B);

    let c = unsafe { <CStr>::from_ptr(&C as *const u8 as *const c_char).to_string_lossy() };

    println!("a: {a}, b: {b}, c: {c}");
}
