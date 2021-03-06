use core::fmt::{Error, Write};
use spin::Mutex;
use lazy_static::lazy_static;

/// SAFETY: In QEMU, the UART registers should be initialized always at BASE_ADDR.
/// We will write to these registers, and accessing the Uart would require a MutexGuard.

const BASE_ADDR: usize = 0x1000_0000;

pub struct Uart {
    registers: &'static mut Registers,
}

// https://www.lammertbies.nl/comm/info/serial-uart
#[repr(C)]
struct Registers {
    data: u8,
    int_enable: u8,
    fifo_ctrl: u8,
    lcr: u8,
    mcr: u8,
    lsr: u8,
    msr: u8,
    scr: u8,
}

fn get_registers() -> &'static mut Registers {
    unsafe { &mut *(BASE_ADDR as *mut Registers) }
}

unsafe fn write_to_reg(addr: &mut u8, value: u8) {
    core::ptr::write_volatile(addr, value);
}

impl Registers {
    unsafe fn init(&mut self) {
        // No interrupts
        write_to_reg(&mut self.int_enable, 0x00);

        // DLAB = 1
        write_to_reg(&mut self.lcr, 1 << 7);

        // Highest communication speed: 115200 bps
        write_to_reg(&mut self.data, 0x01);
        write_to_reg(&mut self.int_enable, 0x00);

        // DLAB = 0, Word length = 8 bits, No parity bit
        write_to_reg(&mut self.lcr, 0x03);

        // FIFO
        write_to_reg(&mut self.fifo_ctrl, 0x01);
    }

    unsafe fn write_to_transmit(&mut self, c: u8) {
        write_to_reg(&mut self.data, c);
    }
}

impl Uart {
    fn new() -> Self {
        Uart {
            registers: get_registers(),
        }
    }

    fn init(&mut self) {
        unsafe {
            self.registers.init();
        }
    }

    fn put(&mut self, c: u8) {
        unsafe {
            self.registers.write_to_transmit(c);
        }
    }

    pub fn write(&mut self, buf: &[u8]) {
        for &c in buf {
            match c {
                // Backspace or Del: https://unix.stackexchange.com/questions/414159/behaviour-of-the-backspace-on-terminal
                8 | 127 => {
                    self.put(8);
                    self.put(b' ');
                    self.put(8);
                }
                // Newline or Carriage Return: https://stackoverflow.com/questions/1761051/difference-between-n-and-r
                10 | 13 => {
                    self.put(b'\r');
                    self.put(b'\n');
                }
                _ => {
                    self.put(c);
                }
            }
        }
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        self.write(s.as_bytes());
        Ok(())
    }
}

lazy_static! {
    pub static ref UART: Mutex<Uart> = Mutex::new(Uart::new());
}

pub fn init() {
    let mut uart = UART.lock();
    uart.init();
}
