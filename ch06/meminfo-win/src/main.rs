#![cfg(windows)]
use winapi::um::{
    memoryapi,
    sysinfoapi::{self, SYSTEM_INFO},
    winnt::{MEMORY_BASIC_INFORMATION, PVOID},
};

fn main() {
    const MEMEINFO_SIZE: usize = std::mem::size_of::<MEMORY_BASIC_INFORMATION>();

    let mut base_addr = unsafe { std::mem::zeroed::<PVOID>() };
    let mut proc_info = unsafe { std::mem::zeroed::<SYSTEM_INFO>() };
    let mut mem_info = unsafe { std::mem::zeroed::<MEMORY_BASIC_INFORMATION>() };

    let this_pid = unsafe { winapi::um::processthreadsapi::GetCurrentProcessId() };
    let this_proc = unsafe { winapi::um::processthreadsapi::GetCurrentProcess() };
    unsafe {
        sysinfoapi::GetSystemInfo(&mut proc_info);
    }

    let min_app_addr = proc_info.lpMinimumApplicationAddress;
    let max_app_addr = proc_info.lpMaximumApplicationAddress;

    println!("{this_pid:?} @ {this_proc:p}");
    proc_info.debug();
    println!("min {min_app_addr:p}, max: {max_app_addr:p}");

    loop {
        let rc = unsafe {
            memoryapi::VirtualQueryEx(this_proc, base_addr, &mut mem_info, MEMEINFO_SIZE)
        };
        if rc == 0 {
            break;
        }

        mem_info.debug();
        base_addr = (base_addr as usize + mem_info.RegionSize) as PVOID;
    }
}

trait DebugStruct {
    fn debug(&self);
}

impl DebugStruct for MEMORY_BASIC_INFORMATION {
    fn debug(&self) {
        println!(
            "MEMORY_BASIC_INFORMATION {{
    BaseAddress: {:?},
    AllocationBase: {:?},
    AllocationProtect: {:?},
    RegionSize: {:?},
    State: {:?},
    Protect: {:?},
    Type: {:?},
}}",
            self.BaseAddress,
            self.AllocationBase,
            self.AllocationProtect,
            self.RegionSize,
            self.State,
            self.Protect,
            self.Type
        );
    }
}

impl DebugStruct for SYSTEM_INFO {
    fn debug(&self) {
        println!(
            "SYSTEM_INFO {{
    u: private,
    dwPageSize: {:?},
    lpMinimumApplicationAddress: {:?},
    lpMaximumApplicationAddress: {:?},
    dwActiveProcessorMask: {:?},
    dwNumberOfProcessors: {:?},
    dwProcessorType: {:?},
    dwAllocationGranularity: {:?},
    wProcessorLevel: {:?},
    wProcessorRevision: {:?},
}}",
            self.dwPageSize,
            self.lpMinimumApplicationAddress,
            self.lpMaximumApplicationAddress,
            self.dwActiveProcessorMask,
            self.dwNumberOfProcessors,
            self.dwProcessorType,
            self.dwAllocationGranularity,
            self.wProcessorLevel,
            self.wProcessorRevision
        );
    }
}
