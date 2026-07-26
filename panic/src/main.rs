#![no_std]
#![no_main]

use core::panic;

use nek_sys::{exit, macros::main, print};

#[main]
fn main() {
    print("Kablooey\n");
    panic!("PANIC")
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    exit();
    loop {}
}
