/*
usage:

#[rom_rt::entry]
fn main(params: rom_rt::Parameters) -> rom_rt::Handover {
    /* code */
}

or

#[rom_rt::entry]
fn main(params: rom_rt::Parameters) {
    /* code */
    rom_rt::exit(handover)
}
*/
#![feature(naked_functions, asm_const)]
#![no_std]

mod soc;

#[cfg(any(feature = "allwinner-d1"))]
pub use soc::Parameters;

#[cfg(not(any(feature = "allwinner-d1")))]
pub struct Parameters {}

pub use rom_rt_macros::entry;
