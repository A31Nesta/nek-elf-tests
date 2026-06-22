#![no_std]
#![no_main]

use core::ffi::{c_char, c_int};

fn fn_ptr_test() -> i32 {
    42
}

static FP: fn() -> i32 = fn_ptr_test;

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text")]
fn main(_argc: c_int, _argv: *const *const c_char) -> c_int {
    FP()
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
