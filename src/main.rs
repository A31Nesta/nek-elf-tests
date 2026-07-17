#![no_std]
#![no_main]

unsafe extern "C" {
    fn nek_serial_print(ptr: *const u8, len: usize);
    fn nek_add(a: i32, b: i32) -> i32;

    fn nek_program_kill(pid: u32);
}

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    let rust_string = "Hello, World! - From ELF - Rust\n";
    let c_string = c"Hello, World! - From ELF - C\n";

    unsafe {
        nek_serial_print(rust_string.as_ptr(), rust_string.len());
        nek_serial_print(c_string.as_ptr(), c_string.count_bytes());

        let stupid_sum = nek_add(10, 9);
        nek_add(stupid_sum, 2); // to correct the addition :)
        // 21
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text")]
pub extern "C" fn _start() -> ! {
    let str = "Starting Program\n";
    unsafe {
        nek_serial_print(str.as_ptr(), str.len());
    }

    main();
    unsafe {
        nek_program_kill(0);
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
