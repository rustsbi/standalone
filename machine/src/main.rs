#![no_std]
#![no_main]

use rom_rt::{entry, println, Handover, Parameters};

#[entry]
fn main(params: Parameters) -> Handover {
    println!("Hello world!").ok();
    Handover::from(params)
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
