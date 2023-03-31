//! Allwinner GPIO controller.
use base_address::{BaseAddress, Dynamic, Static};
use core::marker::PhantomData;
use volatile_register::RW;

use super::GPIO;

/// Generic Purpose Input/Output registers.
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u32; 12],
    pub port: [Port; 6],
    _reserved1: [u32; 52],
    pub eint: [Eint; 6],
    _reserved2: [u32; 24],
    pub pio_pow: PioPow,
}

/// Gpio port register group.
#[repr(C)]
pub struct Port {
    pub cfg: [RW<u32>; 4],
    pub dat: RW<u32>,
    pub drv: [RW<u32>; 4],
    pub pull: [RW<u32>; 2],
    _reserved0: [u32; 1],
}

/// External interrupt register group.
#[repr(C)]
pub struct Eint {
    pub cfg: [RW<u32>; 4],
    pub ctl: RW<u32>,
    pub status: RW<u32>,
    pub deb: RW<u32>,
    _reserved0: [u32; 1],
}

/// Input/Output Power register group.
#[repr(C)]
pub struct PioPow {
    pub mod_sel: RW<u32>,
    pub ms_ctl: RW<u32>,
    pub val: RW<u32>,
    _reserved0: [u32; 1],
    pub vol_sel_ctl: RW<u32>,
}

impl<const B: usize> GPIO<Static<B>> {
    /// Create a peripheral instance from statically known address.
    ///
    /// This function is unsafe for it forces to seize ownership from possible
    /// wrapped peripheral group types. Users should normally retrieve ownership
    /// from wrapped types.
    #[inline]
    pub const unsafe fn steal_static() -> GPIO<Static<B>> {
        GPIO { base: Static::<B> }
    }
}

impl GPIO<Dynamic> {
    /// Create a peripheral instance from dynamically known address.
    ///
    /// This function is unsafe for it forces to seize ownership from possible
    /// wrapped peripheral group types. Users should normally retrieve ownership
    /// from wrapped types.
    #[inline]
    pub unsafe fn steal_dynamic(base: *const ()) -> GPIO<Dynamic> {
        GPIO {
            base: Dynamic::new(base as usize),
        }
    }
}

impl<A: BaseAddress> GPIO<A> {
    /// Set GPIO pin mode.
    #[inline]
    pub fn set_mode<const P: char, const N: u8, OldM, NewM>(
        &self,
        pin: Pin<A, P, N, OldM>,
    ) -> Pin<A, P, N, NewM>
    where
        OldM: PinMode,
        NewM: PinMode,
    {
        // take ownership of Pin
        let Pin { base, .. } = pin;
        // calculate mask and value
        let (port_idx, cfg_reg_idx, cfg_field_idx) = port_cfg_index(P, N);
        let mask = !(0xF << cfg_field_idx);
        let value = (NewM::VALUE as u32) << cfg_field_idx;
        // apply configuration
        let cfg_reg = &self.port[port_idx].cfg[cfg_reg_idx];
        unsafe { cfg_reg.modify(|cfg| (cfg & mask) | value) };
        // return ownership of Pin
        Pin {
            base,
            _mode: PhantomData,
        }
    }
}

const fn port_cfg_index(p: char, n: u8) -> (usize, usize, u8) {
    assert!(p as usize >= b'B' as usize && p as usize <= b'G' as usize);
    assert!(n <= 31);
    let port_idx = p as usize - b'B' as usize;
    let cfg_reg_idx = (n >> 3) as usize;
    let cfg_field_idx = (n & 0b111) << 2;
    (port_idx, cfg_reg_idx, cfg_field_idx)
}

/// Individual GPIO pin.
pub struct Pin<A: BaseAddress, const P: char, const N: u8, M> {
    base: A,
    _mode: PhantomData<M>,
}

impl<A: BaseAddress, const P: char, const N: u8, M: PinMode> Pin<A, P, N, M> {
    /// Disables the pin.
    #[inline]
    pub fn into_disabled(self) -> Pin<A, P, N, Disabled> {
        unsafe { &*self.gpio() }.set_mode(self)
    }
    /// Configures the pin to operate as an input pin.
    #[inline]
    pub fn into_input(self) -> Pin<A, P, N, Input> {
        unsafe { &*self.gpio() }.set_mode(self)
    }
    /// Configures the pin to operate as an output pin.
    #[inline]
    pub fn into_output(self) -> Pin<A, P, N, Output> {
        unsafe { &*self.gpio() }.set_mode(self)
    }
    /// Configures the pin to operate as an external interrupt.
    #[inline]
    pub fn into_eint(self) -> Pin<A, P, N, EintMode> {
        unsafe { &*self.gpio() }.set_mode(self)
    }
    /// Configures the pin to operate as an alternate function.
    #[inline]
    pub fn into_function<const F: u8>(self) -> Pin<A, P, N, Function<F>> {
        unsafe { &*self.gpio() }.set_mode(self)
    }

    #[inline(always)]
    fn gpio(&self) -> *const GPIO<A> {
        self.base.ptr() as *const GPIO<A>
    }
}

/// External interrupt event.
pub enum Event {
    PositiveEdge,
    NegativeEdge,
    HighLevel,
    LowLevel,
    BothEdges,
}

/// Pin that can receive external interrupt.
pub trait EintPin {
    fn listen(&mut self, event: Event);

    fn enable_interrupt(&mut self);

    fn disable_interrupt(&mut self);

    fn clear_interrupt_pending_bit(&mut self);

    fn check_interrupt(&mut self) -> bool;
}

impl<A: BaseAddress, const P: char, const N: u8> EintPin for Pin<A, P, N, EintMode> {
    #[inline]
    fn listen(&mut self, event: Event) {
        let event_id = match event {
            Event::PositiveEdge => 0,
            Event::NegativeEdge => 1,
            Event::HighLevel => 2,
            Event::LowLevel => 3,
            Event::BothEdges => 4,
        };
        let (port_idx, cfg_reg_idx, cfg_field_idx) = port_cfg_index(P, N);
        let mask = !(0xF << cfg_field_idx);
        let value = event_id << cfg_field_idx;
        let cfg_reg = &unsafe { &*self.gpio() }.eint[port_idx].cfg[cfg_reg_idx];
        unsafe { cfg_reg.modify(|cfg| (cfg & mask) | value) };
    }
    #[inline]
    fn enable_interrupt(&mut self) {
        unsafe {
            (&*self.gpio()).eint[port_index(P)]
                .ctl
                .modify(|value| value | (1 << N))
        }
    }
    #[inline]
    fn disable_interrupt(&mut self) {
        unsafe {
            (&*self.gpio()).eint[port_index(P)]
                .ctl
                .modify(|value| value & !(1 << N))
        }
    }
    #[inline]
    fn clear_interrupt_pending_bit(&mut self) {
        unsafe { (&*self.gpio()).eint[port_index(P)].status.write(1 << N) }
    }
    #[inline]
    fn check_interrupt(&mut self) -> bool {
        unsafe { &*self.gpio() }.eint[port_index(P)].status.read() & (1 << N) != 0
    }
}

#[allow(unused)]
macro_rules! impl_gpio_pins {
    ($($px: ident:($P: expr, $N: expr, $M: ty);)+) => {
/// GPIO pins in current platform.
pub struct Pins<A: base_address::BaseAddress> {
    $(
    pub $px: $crate::gpio::Pin<A, $P, $N, $M>,
    )+
}
    };
}

impl<A: BaseAddress, const P: char, const N: u8> embedded_hal::digital::ErrorType
    for Pin<A, P, N, Input>
{
    type Error = core::convert::Infallible;
}

impl<A: BaseAddress, const P: char, const N: u8> embedded_hal::digital::InputPin
    for Pin<A, P, N, Input>
{
    #[inline]
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(unsafe { &*self.gpio() }.port[port_index(P)].dat.read() & (1 << N) != 0)
    }
    #[inline]
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(unsafe { &*self.gpio() }.port[port_index(P)].dat.read() & (1 << N) == 0)
    }
}

impl<A: BaseAddress, const P: char, const N: u8> embedded_hal::digital::ErrorType
    for Pin<A, P, N, Output>
{
    type Error = core::convert::Infallible;
}

impl<A: BaseAddress, const P: char, const N: u8> embedded_hal::digital::OutputPin
    for Pin<A, P, N, Output>
{
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        unsafe {
            (&*self.gpio()).port[port_index(P)]
                .dat
                .modify(|value| value & !(1 << N))
        };
        Ok(())
    }
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        unsafe {
            (&*self.gpio()).port[port_index(P)]
                .dat
                .modify(|value| value | (1 << N))
        };
        Ok(())
    }
}

impl<A: BaseAddress, const P: char, const N: u8> embedded_hal::digital::StatefulOutputPin
    for Pin<A, P, N, Output>
{
    #[inline]
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(unsafe { &*self.gpio() }.port[port_index(P)].dat.read() & (1 << N) != 0)
    }
    #[inline]
    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(unsafe { &*self.gpio() }.port[port_index(P)].dat.read() & (1 << N) == 0)
    }
}

const fn port_index(p: char) -> usize {
    assert!(p as usize >= b'B' as usize && p as usize <= b'G' as usize);
    p as usize - b'B' as usize
}

/// Input mode (type state).
pub struct Input;
/// Output mode (type state).
pub struct Output;
/// Function modes (type state).
///
/// N should be in 2..=8.
pub struct Function<const N: u8>;
/// External interrupt mode (type state).
pub struct EintMode;
/// Disabled mode (type state).
pub struct Disabled;

/// Valid GPIO pin mode.
pub trait PinMode {
    /// GPIO mode value as is represented in `cfg_reg` register.
    const VALUE: u8;
}

impl PinMode for Input {
    const VALUE: u8 = 0;
}

impl PinMode for Output {
    const VALUE: u8 = 1;
}

impl<const N: u8> PinMode for Function<N> {
    const VALUE: u8 = N;
}

impl PinMode for EintMode {
    const VALUE: u8 = 14;
}

impl PinMode for Disabled {
    const VALUE: u8 = 15;
}

#[cfg(test)]
mod tests {
    use super::{Eint, PioPow, Port, RegisterBlock};
    use memoffset::offset_of;
    #[test]
    fn offset_gpio() {
        assert_eq!(offset_of!(RegisterBlock, port), 0x30);
        assert_eq!(offset_of!(RegisterBlock, eint), 0x220);
        assert_eq!(offset_of!(RegisterBlock, pio_pow), 0x340);
    }
    #[test]
    fn offset_port() {
        assert_eq!(offset_of!(Port, cfg), 0x00);
        assert_eq!(offset_of!(Port, dat), 0x10);
        assert_eq!(offset_of!(Port, drv), 0x14);
        assert_eq!(offset_of!(Port, pull), 0x24);
    }
    #[test]
    fn offset_eint() {
        assert_eq!(offset_of!(Eint, cfg), 0x00);
        assert_eq!(offset_of!(Eint, ctl), 0x10);
        assert_eq!(offset_of!(Eint, status), 0x14);
        assert_eq!(offset_of!(Eint, deb), 0x18);
    }
    #[test]
    fn offset_pio_pow() {
        assert_eq!(offset_of!(PioPow, mod_sel), 0x00);
        assert_eq!(offset_of!(PioPow, ms_ctl), 0x04);
        assert_eq!(offset_of!(PioPow, val), 0x08);
        assert_eq!(offset_of!(PioPow, vol_sel_ctl), 0x10);
    }
}
