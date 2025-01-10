#![allow(internal_features)]
#![allow(non_camel_case_types)]
#![feature(link_llvm_intrinsics)]
#![cfg(not(windows))]

use std::{
    ffi::{c_char, c_int},
    mem,
};

use libc::{SIGALRM, SIGHUP, SIGQUIT, SIGTERM, SIGUSR1};

const JMP_BUF_WIDTH: usize = mem::size_of::<usize>() * 8;
type jmp_buf = [c_char; JMP_BUF_WIDTH];

static mut SHUT_DOWN: bool = false;
static mut RETRURN_HERE: jmp_buf = [0; JMP_BUF_WIDTH];
const MOCK_SIGNAL_AT: usize = 3;

extern "C" {
    #[link_name = "llvm.eh.sjlj.setjmp"]
    pub fn setjmp(jmp_buf: *mut c_char) -> c_int;

    #[link_name = "llvm.eh.sjlj.longjmp"]
    pub fn longjmp(jmp_buf: *mut c_char);
}

#[inline]
fn ptr_to_jmp_buf() -> *mut c_char {
    &raw const RETRURN_HERE as *const c_char as *mut c_char
}

#[inline]
fn return_early() {
    let franken_pointer = ptr_to_jmp_buf();
    unsafe {
        longjmp(franken_pointer);
    }
}

fn register_signal_handler() {
    unsafe {
        libc::signal(SIGUSR1, handle_signals as usize);
    }
}

fn handle_signals(sig: c_int) {
    register_signal_handler();

    let should_shut_down = match sig {
        SIGHUP => false,
        SIGALRM => false,
        SIGTERM => true,
        SIGQUIT => true,
        SIGUSR1 => true,
        _ => false,
    };

    unsafe {
        SHUT_DOWN = should_shut_down;
    }

    return_early();
}

fn print_depth(depth: usize) {
    (0..depth).for_each(|_| print!("#"));
    println!();
}

fn dive(depth: usize, max_depth: usize) {
    unsafe {
        if SHUT_DOWN {
            println!("!");
            return;
        }
    }
    print_depth(depth);

    if depth >= max_depth {
        return;
    } else if depth == MOCK_SIGNAL_AT {
        unsafe {
            libc::raise(SIGUSR1);
        }
    } else {
        dive(depth + 1, max_depth);
    }
    print_depth(depth);
}

fn main() {
    const JUMP_SET: i32 = 0;

    register_signal_handler();

    let return_point = ptr_to_jmp_buf();
    let rc = unsafe { setjmp(return_point) };
    if rc == JUMP_SET {
        dive(0, 10);
    } else {
        println!("early return!");
    }

    println!("finishing!");
}
