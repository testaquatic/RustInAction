#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![allow(internal_features)]
#![feature(lang_items)]

use core::{
    fmt::{self, Write},
    panic::PanicInfo,
};

use x86_64::instructions::hlt;

#[allow(unused)]
#[derive(Clone, Copy)]
#[repr(u8)]
enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    Gray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

struct Cursor {
    position: isize,
    foreground: Color,
    background: Color,
}

impl Cursor {
    fn color(&self) -> u8 {
        let fg = self.foreground as u8;
        let bg = (self.background as u8) << 4;
        fg | bg
    }

    fn print(&mut self, text: &[u8]) {
        let color = self.color();
        let framebuffer = 0xb8000 as *mut u8;

        text.iter().for_each(|character| {
            unsafe {
                framebuffer.offset(self.position).write_volatile(*character);
                framebuffer.offset(self.position + 1).write_volatile(color);
            }
            self.position += 2;
        });
    }
}

impl fmt::Write for Cursor {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print(s.as_bytes());
        Ok(())
    }
}

#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    let mut cursor = Cursor {
        position: 0,
        foreground: Color::White,
        background: Color::Red,
    };
    (0..(80 * 25)).for_each(|_| cursor.print(b" "));
    cursor.position = 0;
    write!(cursor, "{}", info).unwrap();

    loop {
        hlt();
    }
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    panic!("Help!");
}
