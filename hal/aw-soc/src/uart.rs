//! Universal Asynchronous Receiver-Transmitter.

use core::cell::UnsafeCell;

use super::UART;
#[allow(unused)]
use crate::gpio::{Function, Pin};
use crate::{
    ccu::{self, ClockGate, Clocks},
    time::Bps,
    CCU,
};
use base_address::{BaseAddress, Dynamic, Static};
use uart16550::{CharLen, Register, Uart16550, PARITY};

/// Universal Asynchronous Receiver-Transmitter registers.
#[repr(C)]
pub struct RegisterBlock {
    uart16550: Uart16550<u32>,
    _reserved0: [u32; 24],
    usr: USR<u32>, // offset = 31(0x7c)
}

/// Serial configuration structure.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Config {
    /// Serial baudrate in `Bps`.
    pub baudrate: Bps,
    /// Word length, can be 5, 6, 7 or 8.
    pub wordlength: WordLength,
    /// Parity checks, can be `None`, `Odd` or `Even`.
    pub parity: Parity,
    /// Number of stop bits, can be `One` or `Two`.
    pub stopbits: StopBits,
}

/// Serial word length settings.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WordLength {
    Five,
    Six,
    Seven,
    Eight,
}

/// Serial parity bit settings.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Parity {
    /// No parity checks.
    None,
    /// Odd parity.
    Odd,
    /// Even parity.
    Even,
}

/// Stop bit settings.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StopBits {
    /// 1 stop bit
    One,
    /// 2 stop bits, or 1.5 bits when WordLength is Five
    Two,
}

impl<const B: usize> UART<Static<B>> {
    /// Create a peripheral instance from statically known address.
    ///
    /// This function is unsafe for it forces to seize ownership from possible
    /// wrapped peripheral group types. Users should normally retrieve ownership
    /// from wrapped types.
    #[inline]
    pub const unsafe fn steal_static() -> UART<Static<B>> {
        UART { base: Static::<B> }
    }
}

impl UART<Dynamic> {
    /// Create a peripheral instance from dynamically known address.
    ///
    /// This function is unsafe for it forces to seize ownership from possible
    /// wrapped peripheral group types. Users should normally retrieve ownership
    /// from wrapped types.
    #[inline]
    pub unsafe fn steal_dynamic(base: *const ()) -> UART<Dynamic> {
        UART {
            base: Dynamic::new(base as usize),
        }
    }
}

impl core::ops::Deref for RegisterBlock {
    type Target = Uart16550<u32>;

    fn deref(&self) -> &Self::Target {
        &self.uart16550
    }
}

/// Managed serial structure with peripheral and pins.
pub struct Serial<A: BaseAddress, const I: usize, PINS: Pins<I>> {
    uart: UART<A>,
    pins: PINS,
}

impl<A: BaseAddress, const I: usize, PINS: Pins<I>> Serial<A, I, PINS> {
    /// Create a serial instance.
    #[inline]
    pub fn new<A1: BaseAddress>(
        uart: UART<A>,
        pins: PINS,
        config: impl Into<Config>,
        clocks: &Clocks,
        ccu: &CCU<A1>,
    ) -> Self {
        // 1. unwrap parameters
        let Config {
            baudrate,
            wordlength,
            parity,
            stopbits,
        } = config.into();
        let bps = baudrate.0;
        // 2. init peripheral clocks
        // note(unsafe): async read and write using ccu registers
        unsafe { PINS::ClockGate::reset(&ccu) };
        // 3. set interrupt configuration
        // on BT0 stage we disable all uart interrupts
        let interrupt_types = uart.ier().read();
        uart.ier().write(
            interrupt_types
                .disable_ms()
                .disable_rda()
                .disable_rls()
                .disable_thre(),
        );
        // 4. calculate and set baudrate
        let uart_clk = (clocks.apb1.0 + 8 * bps) / (16 * bps);
        uart.write_divisor(uart_clk as u16);
        // 5. additional configurations
        let char_len = match wordlength {
            WordLength::Five => CharLen::FIVE,
            WordLength::Six => CharLen::SIX,
            WordLength::Seven => CharLen::SEVEN,
            WordLength::Eight => CharLen::EIGHT,
        };
        let one_stop_bit = matches!(stopbits, StopBits::One);
        let parity = match parity {
            Parity::None => PARITY::NONE,
            Parity::Odd => PARITY::ODD,
            Parity::Even => PARITY::EVEN,
        };
        let lcr = uart.lcr().read();
        uart.lcr().write(
            lcr.set_char_len(char_len)
                .set_one_stop_bit(one_stop_bit)
                .set_parity(parity),
        );
        // 6. return the instance
        Serial { uart, pins }
    }
    /// Close uart and release peripheral.
    #[inline]
    pub fn free<A1: BaseAddress>(self, ccu: &CCU<A1>) -> (UART<A>, PINS) {
        // clock is closed for self.clock_gate is dropped
        unsafe { PINS::ClockGate::free(&ccu) };
        (self.uart, self.pins)
    }
}

/// Valid serial pins.
pub trait Pins<const I: usize> {
    type ClockGate: ccu::ClockGate;
}

/// Valid transmit pin for UART peripheral.
pub trait Transmit<const I: usize> {}

/// Valid receive pin for UART peripheral.
pub trait Receive<const I: usize> {}

impl<const I: usize, T, R> Pins<I> for (T, R)
where
    T: Transmit<I>,
    R: Receive<I>,
{
    type ClockGate = ccu::UART<I>;
}

impl<A: BaseAddress, const I: usize, PINS: Pins<I>> embedded_hal::serial::ErrorType
    for Serial<A, I, PINS>
{
    type Error = embedded_hal::serial::ErrorKind;
}

impl<A: BaseAddress, const I: usize, PINS: Pins<I>> embedded_hal::serial::Write
    for Serial<A, I, PINS>
{
    #[inline]
    fn write(&mut self, buffer: &[u8]) -> Result<(), Self::Error> {
        for c in buffer {
            // FIXME: should be transmit_fifo_not_full
            while self.uart.usr.read().busy() {
                core::hint::spin_loop()
            }
            self.uart.rbr_thr().tx_data(*c);
        }
        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        while !self.uart.usr.read().transmit_fifo_empty() {
            core::hint::spin_loop()
        }
        Ok(())
    }
}

impl<A: BaseAddress, const I: usize, PINS: Pins<I>> embedded_hal_nb::serial::Write<u8>
    for Serial<A, I, PINS>
{
    #[inline]
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        // FIXME: should be transmit_fifo_not_full
        if self.uart.usr.read().busy() {
            return Err(nb::Error::WouldBlock);
        }
        self.uart.rbr_thr().tx_data(word);
        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        if !self.uart.usr.read().transmit_fifo_empty() {
            return Err(nb::Error::WouldBlock);
        }
        Ok(())
    }
}

/// UART Status Register.
#[repr(transparent)]
pub struct USR<R: Register>(UnsafeCell<R>);

/// Status settings for current peripheral.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct UartStatus(u8);

impl<R: uart16550::Register> USR<R> {
    /// Write UART status settings.
    #[inline]
    pub fn write(&self, val: UartStatus) {
        unsafe { self.0.get().write_volatile(R::from(val.0)) }
    }

    /// Read UART status settings.
    #[inline]
    pub fn read(&self) -> UartStatus {
        UartStatus(unsafe { self.0.get().read_volatile() }.val())
    }
}

impl UartStatus {
    const RFF: u8 = 1 << 4;
    const RFNE: u8 = 1 << 3;
    const TFE: u8 = 1 << 2;
    const TFNF: u8 = 1 << 1;
    const BUSY: u8 = 1 << 0;

    /// Returns if the receive FIFO is full.
    #[inline]
    pub const fn receive_fifo_full(self) -> bool {
        self.0 & Self::RFF != 0
    }

    /// Returns if the receive FIFO is non-empty.
    #[inline]
    pub const fn receive_fifo_not_empty(self) -> bool {
        self.0 & Self::RFNE != 0
    }

    /// Returns if the transmit FIFO is empty.
    #[inline]
    pub const fn transmit_fifo_empty(self) -> bool {
        self.0 & Self::TFE != 0
    }

    /// Returns if the transmit FIFO is not full.
    #[inline]
    pub const fn transmit_fifo_not_full(self) -> bool {
        self.0 & Self::TFNF != 0
    }

    /// Returns if the peripheral is busy.
    #[inline]
    pub const fn busy(self) -> bool {
        self.0 & Self::BUSY != 0
    }
}

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use memoffset::offset_of;
    #[test]
    fn offset_uart() {
        assert_eq!(offset_of!(RegisterBlock, usr), 0x7c);
    }
}
