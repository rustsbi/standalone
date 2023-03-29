use d1_rom_rt::{entry, println, Handover, Parameters};

#[entry]
fn main(params: Parameters) -> Handover {
    println!("This is SPI flash sample!").ok();
    // todo: SPI from rom configuration
    // let flash = flashes::SpiNand::new(spi);
    Handover::from(params)
}
