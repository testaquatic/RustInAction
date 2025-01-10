#![cfg(not(windows))]

use libc::sighandler_t;

static mut SHUT_DOWN: bool = false;

fn register_signal_handler() {
    unsafe {
        libc::signal(libc::SIGTERM, handle_sigterm as sighandler_t);
        libc::signal(libc::SIGUSR1, handle_sigusr1 as sighandler_t);
    }
}

fn handle_sigterm(_signal: i32) {
    register_signal_handler();

    println!("SIGTERM");

    unsafe {
        SHUT_DOWN = true;
    }
}

fn handle_sigusr1(_signal: i32) {
    register_signal_handler();

    println!("SIGUSR1");
}

fn main() {
    register_signal_handler();

    println!("SIGUSR1");
}
