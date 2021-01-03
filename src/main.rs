#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// _start is linked in the linker script as the entry
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
