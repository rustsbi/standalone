// Build this example with:
// cargo build --example rom-rt-main --features "allwinner-d1" --target riscv64imac-unknown-none-elf --release
// Checkout target assembly code:
// rust-objdump -d target/riscv64imac-unknown-none-elf/release/examples/rom-rt-main > target/1.asm

#![no_std]
#![no_main]

use rom_rt::{Handover, Parameters};

#[rom_rt::entry]
fn main(params: Parameters) -> Handover {
    // on most platforms, params has a UART inside.
    Handover::from(params)
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
