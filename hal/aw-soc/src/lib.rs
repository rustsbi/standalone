//! SoC peripheral support for Allwinner chips.
//!
//! This package is built under the concept of componentized drivers. It is designed to
//! use in kernels, firmwares and embedded development with both dynamic and static base
//! address support.
//!
//! Most of `aw-soc` structures have `embedded-hal` traits implemented. Users may combine
//! this package with `embedded-hal` ecosystem drivers to provide abundant amount of features.
#![no_std]
#[deny(missing_docs)]
pub mod ccu;
pub mod com;
#[macro_use]
pub mod gpio;
pub mod phy;
pub mod spi;
#[macro_use]
pub mod uart;

use base_address::BaseAddress;
use core::ops;

/// Clock control unit.
pub struct CCU<A: BaseAddress> {
    base: A,
}

unsafe impl<A: BaseAddress> Send for CCU<A> {}

impl<A: BaseAddress> ops::Deref for CCU<A> {
    type Target = ccu::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.base.ptr() as *const _) }
    }
}

/// Common control peripheral of DDR SDRAM.
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

/// Generic Purpose Input/Output peripheral.
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

/// Physical layer peripheral of DDR SDRAM.
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

/// Serial Peripheral Interface bus.
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

/// Universal Asynchronous Receiver-Transmitter.
pub struct UART<A: BaseAddress> {
    base: A,
}

unsafe impl<A: BaseAddress> Send for UART<A> {}

impl<A: BaseAddress> ops::Deref for UART<A> {
    type Target = uart::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.base.ptr() as *const _) }
    }
}

/// Time constants and traits.
pub mod time {
    /// Bits per second.
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub struct Bps(pub u32);

    /// Hertz.
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub struct Hz(pub u32);

    /// Extension trait that adds convenience methods to the `u32` type.
    pub trait U32Ext {
        /// Wrap in `Bps`.
        fn bps(self) -> Bps;
        /// Wrap in `Hz`.
        fn hz(self) -> Hz;
    }

    impl U32Ext for u32 {
        #[inline(always)]
        fn bps(self) -> Bps {
            Bps(self)
        }
        #[inline(always)]
        fn hz(self) -> Hz {
            Hz(self)
        }
    }
}

mod wafer {
    #[cfg(feature = "d1")]
    mod d1;
    pub mod prelude {
        #[cfg(feature = "d1")]
        pub use super::d1::Pins;
    }
}

pub use wafer::prelude::*;
