use core::fmt;

pub struct Terminal;

impl fmt::Write for Terminal {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        super::syscalls::printf(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        use io::Terminal;
        let mut writer = Terminal;
        writer.write_fmt(format_args!($($arg)*)).unwrap();
    });
}