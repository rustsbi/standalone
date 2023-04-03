//! Serial Peripheral Interface bus

use super::SPI;
use crate::{
    ccu::{self, ClockConfig, ClockGate, Clocks, FactorN, SpiClockSource},
    time::Hz,
    CCU,
};
use base_address::{BaseAddress, Dynamic, Static};
use core::cell::UnsafeCell;
use embedded_hal::spi::Mode;
use volatile_register::RW;

/// Serial Peripheral Interface registers.
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: u32,
    pub gcr: GCR,
    pub tcr: TCR,
    _reserved1: u32,
    pub ier: RW<u32>,
    pub isr: RW<u32>,
    pub fcr: RW<u32>,
    pub fsr: FSR,
    pub wcr: RW<u32>,
    _reserved2: u32,
    pub samp_dl: RW<u32>,
    _reserved3: u32,
    pub mbc: MBC,
    pub mtc: MTC,
    pub bcc: BCC,
    _reserved4: u32,
    pub batcr: RW<u32>,
    pub ba_ccr: RW<u32>,
    pub tbr: RW<u32>,
    pub rbr: RW<u32>,
    _reserved5: [u32; 14],
    pub ndma_mode_ctl: RW<u32>,
    _reserved6: [u32; 93],
    pub txd: TXD,
    _reserved7: [u32; 63],
    pub rxd: RXD,
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

/// Global control register.
#[repr(transparent)]
pub struct GCR(UnsafeCell<u32>);

impl GCR {
    /// Read global control.
    #[inline]
    pub fn read(&self) -> GlobalControl {
        GlobalControl(unsafe { self.0.get().read_volatile() })
    }
    /// Write global control.
    #[inline]
    pub fn write(&self, val: GlobalControl) {
        unsafe { self.0.get().write_volatile(val.0) }
    }
}

/// Global control for current peripheral.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct GlobalControl(u32);

impl GlobalControl {
    const SRST: u32 = 1 << 31;
    const TP_EN: u32 = 1 << 7;
    // const MODE_SELEC: u32 = 1 << 2;
    const MODE: u32 = 1 << 1;
    const EN: u32 = 1 << 0;
    /// Perform software reset to the SPI controller.
    #[inline]
    pub const fn software_reset(self) -> Self {
        Self(self.0 | Self::SRST)
    }
    /// Check if software reset request has finished.
    #[inline]
    pub const fn is_software_reset_finished(self) -> bool {
        self.0 & Self::SRST != 0
    }
    /// Set transmit pause enable flag.
    ///
    /// In master mode, if this flag is enabled, transmitting data will be
    /// stopped when receive FIFO is full.
    #[inline]
    pub const fn set_transmit_pause_enable(self, val: bool) -> Self {
        Self((self.0 & !Self::TP_EN) | if val { Self::TP_EN } else { 0 })
    }
    /// Check if transmit pause has enabled.
    #[inline]
    pub const fn transmit_pause_enabled(self) -> bool {
        self.0 & Self::TP_EN != 0
    }
    /// Set this peripheral to operate on master mode.
    #[inline]
    pub const fn set_master_mode(self) -> Self {
        Self(self.0 | Self::MODE)
    }
    /// Set this peripheral to operate on slave mode.
    #[inline]
    pub const fn set_slave_mode(self) -> Self {
        Self(self.0 & !Self::MODE)
    }
    /// Check if this peripheral operates on master mode.
    #[inline]
    pub const fn is_master_mode(self) -> bool {
        self.0 & Self::MODE != 0
    }
    /// Check if this peripheral operates on slave mode.
    #[inline]
    pub const fn is_slave_mode(self) -> bool {
        self.0 & Self::MODE == 0
    }
    /// Enable or disable this peripheral.
    #[inline]
    pub const fn set_enabled(self, val: bool) -> Self {
        Self((self.0 & !Self::EN) | if val { Self::EN } else { 0 })
    }
    /// Check if this peripheral is enabled.
    #[inline]
    pub const fn is_enabled(self) -> bool {
        self.0 & Self::EN != 0
    }
}

/// Transfer control register.
#[repr(transparent)]
pub struct TCR(UnsafeCell<u32>);

impl TCR {
    /// Read transfer control.
    #[inline]
    pub fn read(&self) -> TransferControl {
        TransferControl(unsafe { self.0.get().read_volatile() })
    }
    /// Write transfer control.
    #[inline]
    pub fn write(&self, val: TransferControl) {
        unsafe { self.0.get().write_volatile(val.0) }
    }
}

/// Transfer control for current peripheral.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct TransferControl(u32);

impl TransferControl {
    const XCH: u32 = 1 << 31;
    const CPOL: u32 = 1 << 1;
    const CPHA: u32 = 1 << 0;
    /// Check if burst exchange has finished.
    #[inline]
    pub const fn burst_finished(self) -> bool {
        self.0 & Self::XCH == 0
    }
    /// Initiates burst exchange.
    #[inline]
    pub const fn start_burst_exchange(self) -> Self {
        Self(self.0 | Self::XCH)
    }
    /// Sets SPI work mode.
    #[inline]
    pub const fn set_work_mode(self, mode: Mode) -> Self {
        use embedded_hal::spi::{Phase, Polarity};
        let mut bits = self.0;
        match mode.polarity {
            Polarity::IdleLow => bits &= !Self::CPOL,
            Polarity::IdleHigh => bits |= Self::CPOL,
        }
        match mode.phase {
            Phase::CaptureOnFirstTransition => bits &= !Self::CPHA,
            Phase::CaptureOnSecondTransition => bits |= Self::CPHA,
        }
        Self(bits)
    }
}

/// FIFO status register.
#[repr(transparent)]
pub struct FSR(UnsafeCell<u32>);

impl FSR {
    /// Read FIFO status.
    #[inline]
    pub fn read(&self) -> FifoStatus {
        FifoStatus(unsafe { self.0.get().read_volatile() })
    }
}

/// Status of FIFO for current peripheral.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct FifoStatus(u32);

impl FifoStatus {
    const TB_WR: u32 = 0x1 << 31;
    const TB_CNT: u32 = 0x7 << 28;
    const TF_CNT: u32 = 0xff << 16;
    const RB_WR: u32 = 0x1 << 15;
    const RB_CNT: u32 = 0x7 << 12;
    const RF_CNT: u32 = 0xff << 0;

    #[inline]
    pub const fn transmit_buffer_write_enable(self) -> bool {
        self.0 & Self::TB_WR != 0
    }

    #[inline]
    pub const fn transmit_buffer_counter(self) -> u8 {
        ((self.0 & Self::TB_CNT) >> 28) as u8
    }

    #[inline]
    pub const fn transmit_fifo_counter(self) -> u8 {
        ((self.0 & Self::TF_CNT) >> 16) as u8
    }

    #[inline]
    pub const fn receive_buffer_write_enable(self) -> bool {
        self.0 & Self::RB_WR != 0
    }

    #[inline]
    pub const fn receive_buffer_counter(self) -> u8 {
        ((self.0 & Self::RB_CNT) >> 12) as u8
    }

    #[inline]
    pub const fn receive_fifo_counter(self) -> u8 {
        (self.0 & Self::RF_CNT) as u8
    }
}

/// Master burst counter register.
///
/// In master mode, this field specifies the total burst number.
/// The totcal transfer data include transmit, receive parts and
/// dummy burst.
#[repr(transparent)]
pub struct MBC(UnsafeCell<u32>);

impl MBC {
    /// Read master burst counter.
    #[inline]
    pub fn read(&self) -> u32 {
        unsafe { self.0.get().read_volatile() }
    }
    /// Write master burst counter.
    #[inline]
    pub fn write(&self, val: u32) {
        unsafe { self.0.get().write_volatile(val) }
    }
}

/// Master transmit counter register.
#[repr(transparent)]
pub struct MTC(UnsafeCell<u32>);

impl MTC {
    /// Read master transmit counter.
    #[inline]
    pub fn read(&self) -> u32 {
        unsafe { self.0.get().read_volatile() }
    }
    /// Write master transmit counter.
    #[inline]
    pub fn write(&self, val: u32) {
        unsafe { self.0.get().write_volatile(val) }
    }
}

/// Burst control counter register.
#[repr(transparent)]
pub struct BCC(UnsafeCell<u32>);

impl BCC {
    /// Read burst control counter.
    #[inline]
    pub fn read(&self) -> BurstControl {
        BurstControl(unsafe { self.0.get().read_volatile() })
    }
    /// Write burst control counter.
    #[inline]
    pub fn write(&self, val: BurstControl) {
        unsafe { self.0.get().write_volatile(val.0) }
    }
}

/// Burst control counter for current peripheral.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct BurstControl(u32);

impl BurstControl {
    const QUAD_EN: u32 = 0x1 << 29;
    // const DRM: u32 = 0x1 << 28;
    const DBC: u32 = 0xf << 24;
    const STC: u32 = 0xfff << 0;
    /// Enable quad mode.
    #[inline]
    pub const fn quad_mode_enable(self) -> Self {
        Self(self.0 | Self::QUAD_EN)
    }
    /// Disable quad mode.
    #[inline]
    pub const fn quad_mode_disable(self) -> Self {
        Self(self.0 & !Self::QUAD_EN)
    }
    /// Check if quad mode is enabled.
    #[inline]
    pub const fn is_quad_mode_enabled(self) -> bool {
        self.0 & Self::QUAD_EN != 0
    }

    #[inline]
    pub const fn master_dummy_burst_counter(self) -> u8 {
        ((self.0 & Self::DBC) >> 24) as u8
    }

    #[inline]
    pub const fn set_master_dummy_burst_counter(self, val: u8) -> Self {
        Self((self.0 & !Self::DBC) | ((val as u32 & 0xf) << 24))
    }

    #[inline]
    pub const fn master_single_mode_transmit_counter(self) -> u32 {
        self.0 & Self::STC
    }

    #[inline]
    pub const fn set_master_single_mode_transmit_counter(self, val: u32) -> Self {
        Self((self.0 & !Self::STC) | (val & 0xfff))
    }
}

/// Transmit data register.
#[repr(transparent)]
pub struct TXD(UnsafeCell<u32>);

impl TXD {
    /// Write 8-bit data.
    #[inline]
    pub fn write_u8(&self, val: u8) {
        unsafe { (self.0.get() as *mut u8).write_volatile(val) }
    }
    /// Write 16-bit data.
    #[inline]
    pub fn write_u16(&self, val: u16) {
        unsafe { (self.0.get() as *mut u16).write_volatile(val) }
    }
    /// Write 32-bit data.
    #[inline]
    pub fn write_u32(&self, val: u32) {
        unsafe { self.0.get().write_volatile(val) }
    }
}

/// Receive data register.
#[repr(transparent)]
pub struct RXD(UnsafeCell<u32>);

impl RXD {
    /// Read 8-bit data.
    #[inline]
    pub fn read_u8(&self) -> u8 {
        unsafe { (self.0.get() as *const u8).read_volatile() }
    }
    /// Read 16-bit data.
    #[inline]
    pub fn read_u16(&self) -> u16 {
        unsafe { (self.0.get() as *const u16).read_volatile() }
    }
    /// Read 32-bit data.
    #[inline]
    pub fn read_u32(&self) -> u32 {
        unsafe { self.0.get().read_volatile() }
    }
}

/// Managed SPI structure with peripheral and pins.
pub struct Spi<A: BaseAddress, const I: usize, PINS: Pins<I>> {
    spi: SPI<A>,
    pins: PINS,
}

// Ref: rustsbi-d1 project
impl<A: BaseAddress, const I: usize, PINS: Pins<I>> Spi<A, I, PINS> {
    /// Create an SPI instance.
    pub fn new<A1: BaseAddress>(
        spi: SPI<A>,
        pins: PINS,
        mode: impl Into<Mode>,
        freq: Hz,
        clocks: &Clocks,
        ccu: &CCU<A1>,
    ) -> Self {
        // 1. unwrap parameters
        let (Hz(freq), Hz(psi)) = (freq, clocks.psi);
        let (factor_n, factor_m) = {
            let mut err = psi;
            let (mut best_n, mut best_m) = (0, 0);
            for m in 1u8..=16 {
                for n in [1, 2, 4, 8] {
                    let actual = psi / n / m as u32;
                    if actual.abs_diff(freq) < err {
                        err = actual.abs_diff(freq);
                        (best_n, best_m) = (n, m);
                    }
                }
            }
            let factor_n = match best_n {
                1 => FactorN::N1,
                2 => FactorN::N2,
                4 => FactorN::N4,
                8 => FactorN::N8,
                _ => unreachable!(),
            };
            let factor_m = best_m - 1;
            (factor_n, factor_m)
        };
        // 2. init peripheral clocks
        // clock and divider
        unsafe { PINS::Clock::config(SpiClockSource::PllPeri1x, factor_m, factor_n, ccu) };
        // de-assert reset
        unsafe { PINS::Clock::reset(ccu) };
        // 3. global configuration and soft reset
        spi.gcr.write(
            GlobalControl::default()
                .set_enabled(true)
                .set_master_mode()
                .set_transmit_pause_enable(true)
                .software_reset(),
        );
        while spi.gcr.read().is_software_reset_finished() {
            core::hint::spin_loop();
        }
        // 4. configure work mode
        spi.tcr
            .write(TransferControl::default().set_work_mode(mode.into()));
        // Finally, return ownership of this structure.
        Spi { spi, pins }
    }
    /// Close SPI and release peripheral.
    #[inline]
    pub fn free<A1: BaseAddress>(self, ccu: &CCU<A1>) -> (SPI<A>, PINS) {
        // clock is closed for self.clock_gate is dropped
        unsafe { PINS::Clock::free(&ccu) };
        (self.spi, self.pins)
    }
}

/// Valid SPI pins.
pub trait Pins<const I: usize> {
    type Clock: ccu::ClockGate + ccu::ClockConfig<Source = SpiClockSource>;
}

/// Valid clk pin for SPI peripheral.
pub trait Clk<const I: usize> {}

/// Valid receive pin for SPI peripheral.
pub trait Mosi<const I: usize> {}

pub trait Miso<const I: usize> {}

impl<const I: usize, CLK, MOSI, MISO> Pins<I> for (CLK, MOSI, MISO)
where
    CLK: Clk<I>,
    MOSI: Mosi<I>,
    MISO: Miso<I>,
{
    type Clock = ccu::SPI<I>;
}

impl<A: BaseAddress, const I: usize, PINS: Pins<I>> embedded_hal::spi::SpiBus for Spi<A, I, PINS> {
    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        assert!(read.len() + write.len() <= u32::MAX as usize);
        self.spi.mbc.write((read.len() + write.len()) as u32);
        self.spi.mtc.write(write.len() as u32);
        let bcc = self
            .spi
            .bcc
            .read()
            .set_master_dummy_burst_counter(0)
            .set_master_single_mode_transmit_counter(write.len() as u32);
        self.spi.bcc.write(bcc);
        self.spi
            .tcr
            .write(self.spi.tcr.read().start_burst_exchange());
        for &word in write {
            while self.spi.fsr.read().transmit_fifo_counter() > 63 {
                core::hint::spin_loop();
            }
            self.spi.txd.write_u8(word)
        }
        for word in read {
            while self.spi.fsr.read().receive_fifo_counter() == 0 {
                core::hint::spin_loop();
            }
            *word = self.spi.rxd.read_u8()
        }
        Ok(())
    }

    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        assert!(words.len() * 2 <= u32::MAX as usize);
        self.spi.mbc.write((words.len() * 2) as u32);
        self.spi.mtc.write(words.len() as u32);
        let bcc = self
            .spi
            .bcc
            .read()
            .set_master_dummy_burst_counter(0)
            .set_master_single_mode_transmit_counter(words.len() as u32);
        self.spi.bcc.write(bcc);
        self.spi
            .tcr
            .write(self.spi.tcr.read().start_burst_exchange());
        for &word in words.iter() {
            while self.spi.fsr.read().transmit_fifo_counter() > 63 {
                core::hint::spin_loop();
            }
            self.spi.txd.write_u8(word)
        }
        for word in words {
            while self.spi.fsr.read().receive_fifo_counter() == 0 {
                core::hint::spin_loop();
            }
            *word = self.spi.rxd.read_u8()
        }
        Ok(())
    }
}

impl<A: BaseAddress, const I: usize, PINS: Pins<I>> embedded_hal::spi::SpiBusRead
    for Spi<A, I, PINS>
{
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        assert!(words.len() <= u32::MAX as usize);
        self.spi.mbc.write(words.len() as u32);
        self.spi.mtc.write(0);
        let bcc = self
            .spi
            .bcc
            .read()
            .set_master_dummy_burst_counter(0)
            .set_master_single_mode_transmit_counter(0);
        self.spi.bcc.write(bcc);
        self.spi
            .tcr
            .write(self.spi.tcr.read().start_burst_exchange());
        for word in words {
            while self.spi.fsr.read().receive_fifo_counter() == 0 {
                core::hint::spin_loop();
            }
            *word = self.spi.rxd.read_u8()
        }
        Ok(())
    }
}

impl<A: BaseAddress, const I: usize, PINS: Pins<I>> embedded_hal::spi::SpiBusWrite<u8>
    for Spi<A, I, PINS>
{
    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        assert!(words.len() <= u32::MAX as usize);
        self.spi.mbc.write(words.len() as u32);
        self.spi.mtc.write(words.len() as u32);
        let bcc = self
            .spi
            .bcc
            .read()
            .set_master_dummy_burst_counter(0)
            .set_master_single_mode_transmit_counter(words.len() as u32);
        self.spi.bcc.write(bcc);
        self.spi
            .tcr
            .write(self.spi.tcr.read().start_burst_exchange());
        for &word in words {
            while self.spi.fsr.read().transmit_fifo_counter() > 63 {
                core::hint::spin_loop();
            }
            self.spi.txd.write_u8(word)
        }
        Ok(())
    }
}

impl<A: BaseAddress, const I: usize, PINS: Pins<I>> embedded_hal::spi::SpiBusFlush
    for Spi<A, I, PINS>
{
    fn flush(&mut self) -> Result<(), Self::Error> {
        while !self.spi.tcr.read().burst_finished() {
            core::hint::spin_loop();
        }
        Ok(())
    }
}

impl<A: BaseAddress, const I: usize, PINS: Pins<I>> embedded_hal::spi::ErrorType
    for Spi<A, I, PINS>
{
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
