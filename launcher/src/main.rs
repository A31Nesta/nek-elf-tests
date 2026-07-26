#![no_std]
#![no_main]
extern crate alloc;

use alloc::format;
use nek_sys::{exit, macros::main, print, program_run, sleep};

#[main]
fn main() {
    let program = include_bytes!("counting");
    for round in 0..5 {
        for i in 0..5 {
            print(&format!(
                "[launcher] ROUND {} | PROGRAM {} - Running...\n",
                round + 1,
                i + 1
            ));
            program_run(program, &format!("counting_{i}"));
            sleep(1);
        }
        sleep(5);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    exit();
    loop {}
}
