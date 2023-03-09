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

#[cfg(not(any(feature = "allwinner-d1")))]
pub struct Parameters {}

#[cfg(not(any(feature = "allwinner-d1")))]
pub struct Handover {}

impl From<Parameters> for Handover {
    #[inline]
    fn from(_src: Parameters) -> Self {
        Handover {}
    }
}

pub use rom_rt_macros::entry;
