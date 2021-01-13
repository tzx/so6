#[macro_export]
macro_rules! print {
    ($($args:tt)*) => {{
        use core::fmt::Write;
        use crate::uart;

        let mut uart = uart::UART.lock();
        write!(uart, $($args)*).unwrap();
    }};
}

#[macro_export]
macro_rules! println {
    () => {{
        print!("\n");
    }};

    ($fmt:expr) => {{
        print!(concat!($fmt, "\n"));
    }};

    ($fmt:expr, $($args:tt)*) => {{
        print!(concat!($fmt, "\n"), $($args)*);
    }};
}
