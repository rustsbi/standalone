#![no_std]

pub mod com;
pub mod gpio;
pub mod phy;
pub mod spi;

use base_address::BaseAddress;
use core::ops;

/// Common control peripheral of DDR SDRAM
pub struct COM<A: BaseAddress> {
    base: A,
}

unsafe impl<A: BaseAddress> Send for COM<A> {}

impl<A: BaseAddress> ops::Deref for COM<A> {
    type Target = com::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.base.ptr() as *const _) }
    }
}

/// Generic Purpose Input/Output peripheral
pub struct GPIO<A: BaseAddress> {
    base: A,
}

unsafe impl<A: BaseAddress> Send for GPIO<A> {}

impl<A: BaseAddress> ops::Deref for GPIO<A> {
    type Target = gpio::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.base.ptr() as *const _) }
    }
}

/// Physical layer peripheral of DDR SDRAM
pub struct PHY<A: BaseAddress> {
    base: A,
}

unsafe impl<A: BaseAddress> Send for PHY<A> {}

impl<A: BaseAddress> ops::Deref for PHY<A> {
    type Target = phy::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.base.ptr() as *const _) }
    }
}

/// Serial Peripheral Interface bus
pub struct SPI<A: BaseAddress> {
    base: A,
}

unsafe impl<A: BaseAddress> Send for SPI<A> {}

impl<A: BaseAddress> ops::Deref for SPI<A> {
    type Target = spi::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.base.ptr() as *const _) }
    }
}
