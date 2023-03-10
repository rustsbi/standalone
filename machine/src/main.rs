#![no_std]
#![no_main]

use rom_rt::entry;
use rom_rt::{Handover, Parameters};

#[entry]
fn main(params: Parameters) -> Handover {
    #[cfg(feature = "allwinner-d1")]
    {} // todo: use serial
    Handover::from(params)
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
