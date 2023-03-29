use super::CCU;
use crate::time::Hz;
use base_address::{BaseAddress, Dynamic, Static};
use core::{cell::UnsafeCell, marker::PhantomData};

#[derive(Debug)]
pub struct Clocks {
    pub psi: Hz,
    pub apb1: Hz,
}

#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u32; 579],
    pub uart_bgr: UART_BGR,
}

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

pub struct Gate<A, P> {
    base: A,
    _peripheral: PhantomData<P>,
}

impl<const B: usize, P: ClockGate> Gate<Static<B>, P> {
    #[inline]
    pub unsafe fn steal_static() -> Gate<Static<B>, P> {
        Self {
            base: Static::<B>,
            _peripheral: PhantomData,
        }
    }
}

impl<const B: usize, P: ClockGate> Gate<Static<B>, P> {
    #[inline]
    pub unsafe fn reset(&self) {
        let ccu: CCU<Static<B>> = CCU::steal_static();
        P::reset(&ccu);
    }
    #[inline]
    pub unsafe fn free(self) {
        let ccu: CCU<Static<B>> = CCU::steal_static();
        unsafe { P::free(&ccu) };
    }
}

impl<P: ClockGate> Gate<Dynamic, P> {
    #[inline]
    pub unsafe fn reset(&self) {
        let ccu: CCU<Dynamic> = CCU::steal_dynamic(self.base.ptr());
        P::reset(&ccu);
    }
    #[inline]
    pub unsafe fn free(self) {
        let ccu: CCU<Dynamic> = CCU::steal_dynamic(self.base.ptr());
        unsafe { P::free(&ccu) };
    }
}

pub trait ClockGate {
    unsafe fn reset<A: BaseAddress>(ccu: &CCU<A>);
    unsafe fn free<A: BaseAddress>(ccu: &CCU<A>);
}

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
