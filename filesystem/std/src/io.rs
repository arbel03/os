use core::fmt;
use alloc::String;

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

pub fn read_string() -> String {
    use syscalls::getc;
    use core::fmt::Write;

    let mut input_string = String::new();
    loop {
        let character = getc();
        if character == '7' {
            continue;
        }
        print!("{}", character);
        if character == '\n' {
            return input_string;
        } else {
            input_string.write_char(character);
        }
    }
    return input_string;
}