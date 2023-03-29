use d1_rom_rt::{entry, println, Handover, Parameters};

#[entry]
fn main(params: Parameters) -> Handover {
    println!("RustSBI bootstrap dram init.").ok();
    d1_rom_rt::dram_init();
    let dram_size = params.com.dram_size();
    println!("DRAM INIT finished; dram size: {} bytes!", dram_size).ok();
    Handover::from(params)
}
