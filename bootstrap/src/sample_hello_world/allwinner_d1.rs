use d1_rom_rt::{entry, println, Handover, Parameters};

#[entry]
fn main(params: Parameters) -> Handover {
    println!("Hello world!").ok();
    Handover::from(params)
}
