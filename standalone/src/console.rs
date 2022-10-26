use core::fmt;

struct Stdout;

impl fmt::Write for Stdout {
    #[inline]
    fn write_str(&mut self, _s: &str) -> fmt::Result {
        // todo
        Ok(())
    }
}

#[inline]
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use fmt::Write;
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::legacy_stdio::_print(core::format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\r\n"));
    ($($arg:tt)*) => {
        $crate::legacy_stdio::_print(core::format_args!($($arg)*));
        $crate::print!("\r\n");
    }
}
