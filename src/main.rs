#![no_std]
#![no_main]

use core::ffi::{c_char, c_int};

unsafe extern "C" {
    // fn nek_serial_print(ptr: *const u8, len: usize);
    fn nek_add(a: i32, b: i32) -> i32;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text")]
fn main(_argc: c_int, _argv: *const *const c_char) -> c_int {
    // let rust_string = "Hello, World! - From ELF - Rust\n";
    // let c_string = c"Hello, World! - From ELF - C\n";

    unsafe {
        // nek_serial_print(rust_string.as_ptr(), rust_string.len());
        // nek_serial_print(c_string.as_ptr(), c_string.count_bytes());

        let stupid_sum = nek_add(10, 9);
        nek_add(stupid_sum, 2) // to correct the addition :)
        // 21
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
