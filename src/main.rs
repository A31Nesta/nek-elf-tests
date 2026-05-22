#![no_std]
#![no_main]

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text")]
fn main() -> i32 {
    67
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
