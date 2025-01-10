static mut SHUT_DOWN: bool = false;

fn main() {
    loop {
        unsafe {
            SHUT_DOWN = rand::random();
        }
        println!(".");

        if unsafe { SHUT_DOWN } {
            break;
        }
    }
    println!();
}
