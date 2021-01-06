#![no_std]
#![no_main]
#![feature(global_asm)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

global_asm!(include_str!("boot.S"));

#[no_mangle]
fn kernel_init() -> ! {
    loop {}
}
