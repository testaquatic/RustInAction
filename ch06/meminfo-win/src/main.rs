#![cfg(windows)]
use winapi::um::winnt::MEMORY_BASIC_INFORMATION;

fn main() {
    const MEMEINFO_SIZE: usize = std::mem::size_of::<MEMORY_BASIC_INFORMATION>();

    let base_addr = unsafe { std::mem::zeroed::<PVOID>() };
}
