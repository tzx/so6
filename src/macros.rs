#[macro_export]
macro_rules! print {
    ($($args:tt)*) => {{
        // XXX: Is this really singleton???
        use core::fmt::Write;
        let mut uart = unsafe { UART.put_uart(); UART.take_uart() }; 
        let _ = write!(uart, $($args)*);
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
