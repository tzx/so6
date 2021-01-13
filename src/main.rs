#![no_std]
#![no_main]
#![feature(global_asm)]

use core::panic::PanicInfo;

// TODO: create lib
#[macro_use]
mod macros;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("That's a panic dood: {}", _info);
    loop {}
}
global_asm!(include_str!("boot.S"));

mod uart;

#[no_mangle]
fn kernel_init() -> ! {
    uart::init();

    let cute = "Hello World!";
    println!("{}", cute);
    println!("Goodbye :(");
    panic!("Nothing else to do so I am panicking");
}
