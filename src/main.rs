#![no_std]
#![no_main]
#![feature(global_asm)]

use core::panic::PanicInfo;

// TODO: create lib
#[macro_use]
mod macros;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
global_asm!(include_str!("boot.S"));

mod uart;
use uart::PWrapper;

static mut UART: PWrapper = PWrapper::new();

#[no_mangle]
fn kernel_init() -> ! {
    {
        let mut uart = unsafe {
            UART.put_uart();
            UART.take_uart()
        };

        uart.init();
    }
    let cute = "Hello World!";
    println!("{}", cute);
    loop {}
}
