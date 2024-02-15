#![feature(naked_functions)]
#![no_std]
#![no_main]
use core::arch::asm;

mod fdt;
mod opaque;

extern "C" fn main(hart_id: usize, opaque: usize) {
    let signature = opaque::read_failsafe_signature(opaque);
    if opaque::is_fdt_blob(signature) {
        let _ = opaque::parse_fdt(opaque);
    }

    let _ = hart_id; // TODO

    // TODO
}

#[naked]
#[export_name = "_start"]
unsafe extern "C" fn start() {
    asm!("", options(noreturn))
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
