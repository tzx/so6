#![no_std]
#![no_main]
#![feature(global_asm)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

global_asm!(include_str!("boot.S"));

mod uart;

#[no_mangle]
fn kernel_init() -> ! {
    let mut uart = uart::Uart::new(0x1000_0000);
    unsafe {
        uart.init();
        let cute = "Hello World!";
        for c in cute.bytes() {
            uart.put(c);
        }
    }
    loop {}
}
