//! Serial Peripheral Interface bus

use volatile_register::RW;

#[repr(C)]
pub struct RegisterBlock {
    _reserved0: u32,
    pub gcr: RW<u32>,
    pub tcr: RW<u32>,
    _reserved1: u32,
    pub ier: RW<u32>,
    pub isr: RW<u32>,
    pub fcr: RW<u32>,
    pub fsr: RW<u32>,
    pub wcr: RW<u32>,
    _reserved2: u32,
    pub samp_dl: RW<u32>,
    _reserved3: u32,
    pub mbc: RW<u32>,
    pub mtc: RW<u32>,
    pub bcc: RW<u32>,
    _reserved4: u32,
    pub batcr: RW<u32>,
    pub ba_ccr: RW<u32>,
    pub tbr: RW<u32>,
    pub rbr: RW<u32>,
    _reserved5: [u32; 14],
    pub ndma_mode_ctl: RW<u32>,
    _reserved6: [u32; 93],
    pub txd: RW<u32>,
    _reserved7: [u32; 63],
    pub rxd: RW<u32>,
}

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use memoffset::offset_of;
    #[test]
    fn offset_spi0() {
        assert_eq!(offset_of!(RegisterBlock, ier), 0x10);
        assert_eq!(offset_of!(RegisterBlock, samp_dl), 0x28);
        assert_eq!(offset_of!(RegisterBlock, mbc), 0x30);
        assert_eq!(offset_of!(RegisterBlock, ndma_mode_ctl), 0x88);
        assert_eq!(offset_of!(RegisterBlock, txd), 0x200);
        assert_eq!(offset_of!(RegisterBlock, rxd), 0x300);
    }
}
