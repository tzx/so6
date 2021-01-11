#[macro_export]
macro_rules! print {
    ($($args:tt)*) => {{
        use core::fmt::Write;
        use crate::uart;

        let mut wrap = uart::get_uart().lock();
        let uart = wrap.as_mut().unwrap();
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
