use aw_soc::{
    spi::Spi,
    time::U32Ext,
    uart::{self, Parity, Serial, StopBits, WordLength},
};
use d1_rom_rt::{entry, Handover, Parameters};
use embedded_hal::{serial::Write, spi};

#[entry]
fn main(params: Parameters) -> Handover {
    let config = uart::Config {
        baudrate: 115200.bps(),
        wordlength: WordLength::Eight,
        parity: Parity::None,
        stopbits: StopBits::One,
    };
    let tx = params.gpio.pb8.into_function::<6>();
    let rx = params.gpio.pb9.into_function::<6>();
    let mut serial = Serial::new(params.uart0, (tx, rx), config, &params.clocks, &params.ccu);
    serial.write(b"This is SPI flash sample!").ok();
    let clk = params.gpio.pc2.into_function::<2>();
    let cs = params.gpio.pc3.into_function::<2>();
    let mosi = params.gpio.pc4.into_function::<2>();
    let miso = params.gpio.pc5.into_function::<2>();
    let spi = Spi::new(
        params.spi0,
        (clk, mosi, miso),
        spi::MODE_3,
        100_000_000.hz(),
        &params.clocks,
        &params.ccu,
    );
    // todo: SPI from rom configuration
    // let flash = flashes::SpiNand::new(spi);
    Handover::from(params)
}
