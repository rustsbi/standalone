/*
usage:

#[rom_rt::entry]
fn main(params: rom_rt::Parameters) -> rom_rt::Handover {
    /* code */
}
*/
#![feature(naked_functions, asm_const)]
#![no_std]

mod soc;

#[cfg(any(feature = "allwinner-d1"))]
pub use soc::{Handover, Parameters};

#[cfg(all(feature = "allwinner-d1", feature = "log"))]
#[doc(hidden)]
pub use soc::log;

#[cfg(not(any(feature = "allwinner-d1")))]
pub struct Parameters {
    #[cfg(not(feature = "log"))]
    pub uart: d1_hal::uart::Serial<d1_pac::UART0, (PB8<Function<6>>, PB9<Function<6>>)>,
    pub memory_meta: &'static mut Meta,
}

#[cfg(not(any(feature = "allwinner-d1")))]
pub struct Handover {
    #[cfg(not(feature = "log"))]
    pub uart: d1_hal::uart::Serial<d1_pac::UART0, (PB8<Function<6>>, PB9<Function<6>>)>,
}

#[cfg(not(any(feature = "allwinner-d1")))]
impl From<Parameters> for Handover {
    #[inline]
    fn from(src: Parameters) -> Self {
        match () {
            #[cfg(not(feature = "log"))]
            () => Handover { uart: src.uart },
            #[cfg(feature = "log")]
            () => {
                let _ = src;
                Handover {}
            }
        }
    }
}

pub use rom_rt_macros::entry;

#[cfg(not(any(feature = "allwinner-d1")))]
#[macro_export(local_inner_macros)]
macro_rules! println {
    () => ($crate::print!("\r\n"));
    ($fmt: literal $(, $($arg: tt)+)?) => ({
        extern crate ufmt;
        let mut logger = $crate::log::LOGGER.wait().inner.lock();
        let ans = ufmt::uwrite!(logger, $fmt $(, $($arg)+)?);
        drop(logger);
        let _ = $crate::print!("\r\n");
        ans
    });
}

#[cfg(not(any(feature = "allwinner-d1")))]
#[macro_export(local_inner_macros)]
macro_rules! print {
    ($($arg:tt)*) => ({
        extern crate ufmt;
        let mut logger = $crate::log::LOGGER.wait().inner.lock();
        let ans = ufmt::uwrite!(logger, $($arg)*);
        drop(logger);
        ans
    });
}
