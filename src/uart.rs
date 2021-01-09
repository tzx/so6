use core::mem::replace;

// https://www.lammertbies.nl/comm/info/serial-uart

const BASE_ADDR: usize = 0x1000_0000;

pub struct PWrapper {
    uart: Option<Uart>,
}

impl PWrapper {
    pub const fn new() -> Self {
        PWrapper { uart: None }
    }

    pub fn take_uart(&mut self) -> Uart {
        let uart = replace(&mut self.uart, None);
        uart.unwrap()
    }

    pub fn put_uart(&mut self) {
        self.uart = Some(Uart::new());
    }
}

pub struct Uart {
    registers: &'static mut Registers,
}

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

fn get_uart() -> &'static mut Registers {
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
            registers: get_uart(),
        }
    }

    pub fn init(&mut self) {
        unsafe {
            self.registers.init();
        }
    }

    pub fn put(&mut self, c: u8) {
        unsafe {
            self.registers.write_to_transmit(c);
        }
    }
}
