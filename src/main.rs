#![no_std]
#![no_main]

unsafe extern "C" {
    fn nek_serial_print(ptr: *const u8, len: usize);

    fn sleep(seconds: u32);
    fn exit();
}

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    let program_name = "[9-0] ";
    let newline = "\n";

    for i in 0..10 {
        unsafe {
            nek_serial_print(program_name.as_ptr(), program_name.len());
            nek_serial_print(&('9' as u8 - i), 1);
            nek_serial_print(newline.as_ptr(), newline.len());
            sleep(2);
        }
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text")]
pub extern "C" fn _start() -> ! {
    main();
    unsafe {
        exit();
    }

    let exit_confirmation = "-- should exit --";
    loop {
        unsafe {
            nek_serial_print(exit_confirmation.as_ptr(), exit_confirmation.len());
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
