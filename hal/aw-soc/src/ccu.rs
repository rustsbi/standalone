//! Clock control unit.

use super::CCU;
use crate::time::Hz;
use base_address::{BaseAddress, Dynamic, Static};
use core::cell::UnsafeCell;

/// Clock configuration on current SoC.
#[derive(Debug)]
pub struct Clocks {
    /// PSI clock frequency.
    pub psi: Hz,
    /// Advanced Peripheral Bus 1 clock frequency.
    pub apb1: Hz,
}

/// Clock Control Unit registers.
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u32; 579],
    /// 0x90c - UART Bus Gating Reset register.
    pub uart_bgr: UART_BGR,
}

/// UART Bus Gating Reset register.
#[repr(C)]
pub struct UART_BGR(UnsafeCell<u32>);

mod uart_bgr {
    use super::UART_BGR;

    impl UART_BGR {
        /// Write settings to UART bus gating register.
        #[inline]
        pub fn write(&self, val: UartBgr) {
            unsafe { self.0.get().write_volatile(val.0) }
        }

        /// Read settings from UART bus gating register.
        #[inline]
        pub fn read(&self) -> UartBgr {
            UartBgr(unsafe { self.0.get().read_volatile() })
        }
    }

    /// Structure representation of UART bus gating register.
    pub struct UartBgr(u32);

    impl UartBgr {
        /// Disable clock gate for UART `I`.
        #[inline]
        pub fn gate_mask<const I: usize>(self) -> Self {
            Self(self.0 & !(1 << I))
        }
        /// Enable clock gate for UART `I`.
        #[inline]
        pub fn gate_pass<const I: usize>(self) -> Self {
            Self(self.0 | (1 << I))
        }
        /// Assert reset signal for UART `I`.
        #[inline]
        pub fn assert_reset<const I: usize>(self) -> Self {
            Self(self.0 & !(1 << (I + 16)))
        }
        /// Deassert reset signal for UART `I`.
        #[inline]
        pub fn deassert_reset<const I: usize>(self) -> Self {
            Self(self.0 | (1 << (I + 16)))
        }
    }
}

impl<const B: usize> CCU<Static<B>> {
    /// Create a peripheral instance from statically known address.
    ///
    /// This function is unsafe for it forces to seize ownership from possible
    /// wrapped peripheral group types. Users should normally retrieve ownership
    /// from wrapped types.
    #[inline]
    pub const unsafe fn steal_static() -> CCU<Static<B>> {
        CCU { base: Static::<B> }
    }
}

impl CCU<Dynamic> {
    /// Create a peripheral instance from dynamically known address.
    ///
    /// This function is unsafe for it forces to seize ownership from possible
    /// wrapped peripheral group types. Users should normally retrieve ownership
    /// from wrapped types.
    #[inline]
    pub unsafe fn steal_dynamic(base: *const ()) -> CCU<Dynamic> {
        CCU {
            base: Dynamic::new(base as usize),
        }
    }
}

/// Static peripheral that can be clock gated by CCU.
pub trait ClockGate {
    /// Reset this peripheral by provided `ccu`.
    unsafe fn reset<A: BaseAddress>(ccu: &CCU<A>);
    /// Free this peripheral by provided `ccu`.
    unsafe fn free<A: BaseAddress>(ccu: &CCU<A>);
}

/// Universal Asynchronous Receiver-Transmitter clock gate.
///
/// UART peripheral should be indexed by type parameter `IDX`.
pub struct UART<const IDX: usize>;

impl<const I: usize> ClockGate for UART<I> {
    #[inline]
    unsafe fn reset<A: BaseAddress>(ccu: &CCU<A>) {
        let uart_bgr = ccu.uart_bgr.read();
        ccu.uart_bgr
            .write(uart_bgr.gate_mask::<I>().assert_reset::<I>());
        let uart_bgr = ccu.uart_bgr.read();
        ccu.uart_bgr
            .write(uart_bgr.gate_pass::<I>().deassert_reset::<I>());
    }

    #[inline]
    unsafe fn free<A: BaseAddress>(ccu: &CCU<A>) {
        let uart_bgr = ccu.uart_bgr.read();
        ccu.uart_bgr
            .write(uart_bgr.gate_mask::<I>().assert_reset::<I>());
    }
}

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use memoffset::offset_of;
    #[test]
    fn offset_ccu() {
        assert_eq!(offset_of!(RegisterBlock, uart_bgr), 0x90c);
    }
}
