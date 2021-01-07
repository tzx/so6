// https://www.lammertbies.nl/comm/info/serial-uart

pub struct Uart {
    base_address: *mut u8,
}

impl Uart {
    pub fn new(base_address: usize) -> Self {
        Uart {
            base_address: base_address as *mut u8,
        }
    }

    pub unsafe fn init(&mut self) {
        // No interrupts
        self.base_address.add(1).write_volatile(0x00);

        // DLAB = 1
        self.base_address.add(3).write_volatile(1 << 7);

        // Highest communication speed: 115200 bps
        self.base_address.add(0).write_volatile(0x01);
        self.base_address.add(1).write_volatile(0x00);

        // DLAB = 0, Word length = 8 bits, No parity bit
        self.base_address.add(3).write_volatile(0x03);

        // FIFO
        self.base_address.add(2).write_volatile(0x01);
    }

    pub unsafe fn put(&mut self, c: u8) {
        self.base_address.add(0).write_volatile(c);
    }
}
