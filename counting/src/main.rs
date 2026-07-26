#![no_std]
#![no_main]
extern crate alloc;

use alloc::format;
use nek_sys::{exit, macros::main, print, sleep};

#[main]
fn main() {
    for i in 0..10 {
        print(&format!("[0-9 NEK-SYS] {i} \n"));
        sleep(1);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    exit();
    loop {}
}
