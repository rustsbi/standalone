use d1_rom_rt::{entry, println, Handover, Parameters};

#[entry]
fn main(params: Parameters) -> Handover {
    println!("This is SPI flash sample!").ok();
    Handover::from(params)
}
