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
pub struct Parameters {}

#[cfg(not(any(feature = "allwinner-d1")))]
pub struct Handover {}

#[cfg(not(any(feature = "allwinner-d1")))]
impl From<Parameters> for Handover {
    #[inline]
    fn from(_src: Parameters) -> Self {
        Handover {}
    }
}

pub use rom_rt_macros::entry;

#[cfg(not(any(feature = "allwinner-d1")))]
#[macro_export(local_inner_macros)]
macro_rules! println {
    ($($arg:tt)*) => {
        // empty.
    };
}
#[cfg(not(any(feature = "allwinner-d1")))]
#[macro_export(local_inner_macros)]
macro_rules! print {
    ($($arg:tt)*) => {
        // empty.
    };
}
