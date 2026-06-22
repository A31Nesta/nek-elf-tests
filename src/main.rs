#![no_std]
#![no_main]

use core::ffi::{c_char, c_int};

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text")]
fn main(_argc: c_int, _argv: *const *const c_char) -> c_int {
    67
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
