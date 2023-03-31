//! Serial Peripheral Interface bus

use super::SPI;
use base_address::{BaseAddress, Dynamic, Static};
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

impl<const B: usize> SPI<Static<B>> {
    /// Create a peripheral instance from statically known address.
    ///
    /// This function is unsafe for it forces to seize ownership from possible
    /// wrapped peripheral group types. Users should normally retrieve ownership
    /// from wrapped types.
    #[inline]
    pub const unsafe fn steal_static() -> SPI<Static<B>> {
        SPI { base: Static::<B> }
    }
}

impl SPI<Dynamic> {
    /// Create a peripheral instance from dynamically known address.
    ///
    /// This function is unsafe for it forces to seize ownership from possible
    /// wrapped peripheral group types. Users should normally retrieve ownership
    /// from wrapped types.
    #[inline]
    pub unsafe fn steal_dynamic(base: *const ()) -> SPI<Dynamic> {
        SPI {
            base: Dynamic::new(base as usize),
        }
    }
}

pub struct Spi<A: BaseAddress, PINS> {
    spi: SPI<A>,
    pins: PINS,
}

impl<A: BaseAddress, PINS> embedded_hal::spi::SpiBusRead for Spi<A, PINS> {
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        for word in words {
            while self.spi.fsr.read() & 0xff == 0 {
                core::hint::spin_loop();
            }
            *word = self.spi.rxd.read() as u8
        }
        Ok(())
    }
}

impl<A: BaseAddress, PINS> embedded_hal::spi::SpiBusWrite<u8> for Spi<A, PINS> {
    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        for word in words {
            while (self.spi.fsr.read() >> 16) & 0xff >= 64 {
                core::hint::spin_loop();
            }
            unsafe { self.spi.txd.write(*word as u32) }
        }
        Ok(())
    }
}

impl<A: BaseAddress, PINS> embedded_hal::spi::SpiBusFlush for Spi<A, PINS> {
    fn flush(&mut self) -> Result<(), Self::Error> {
        while (self.spi.fsr.read() >> 16) & 0xff > 0 {
            core::hint::spin_loop();
        }
        Ok(())
    }
}

impl<A: BaseAddress, PINS> embedded_hal::spi::ErrorType for Spi<A, PINS> {
    type Error = embedded_hal::spi::ErrorKind;
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
