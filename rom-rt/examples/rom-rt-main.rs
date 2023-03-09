// Build this example with:
// cargo build --example rom-rt-main --features "allwinner-d1" --target riscv64imac-unknown-none-elf

#![no_std]
#![no_main]

#[rom_rt::entry]
fn main(params: rom_rt::Parameters) -> rom_rt::Handover {
    rom_rt::Handover::from(params)
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
