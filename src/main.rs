#![no_std]
#![no_main]

use nek_sys::{exit, print, sleep};

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    for i in 0..10 {
        print("[0-9 NEK-SYS] ");
        print(str::from_utf8(&['0' as u8 + i]).unwrap());
        print("\n");
        sleep(1);
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text")]
pub extern "C" fn _start() -> ! {
    main();
    exit();

    loop {
        print("-- should exit --");
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
