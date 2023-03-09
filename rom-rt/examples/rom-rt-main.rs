#![no_std]
#![no_main]

#[rom_rt::entry]
fn main(params: rom_rt::Parameters) {
    let _ = params;
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
