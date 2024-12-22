fn main() {
    let a = 42_i64;
    let a_ptr = &a as *const i64;
    let a_addr = a_ptr as usize;
    println!("a: {a} ({a_ptr:p}...{:#x})", a_addr + 7);
}
