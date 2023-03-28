//! Universal Asynchronous Receiver-Transmitter

use core::cell::UnsafeCell;

use super::UART;
use crate::{
    ccu::{self, Clocks, Gate},
    gpio::{Function, Pin},
    time::Bps,
};
use base_address::{BaseAddress, Dynamic, Static};
use uart16550::{CharLen, Register, Uart16550, PARITY};

#[repr(C)]
pub struct RegisterBlock {
    uart16550: Uart16550<u32>,
    _reserved0: [u32; 24],
    usr: USR<u32>, // offset = 31(0x7c)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Config {
    pub baudrate: Bps,
    pub wordlength: WordLength,
    pub parity: Parity,
    pub stopbits: StopBits,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(unused)] // should be used as exported structure in HAL crate
pub enum WordLength {
    Five,
    Six,
    Seven,
    Eight,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(unused)] // should be used as exported structure in HAL crate
pub enum Parity {
    None,
    Odd,
    Even,
}

/// Stop Bit configuration parameter for serial.
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

pub struct Serial<A: BaseAddress, A1: BaseAddress, PINS: Pins<UART<A>>> {
    uart: UART<A>,
    pins: PINS,
    clock_gate: Gate<A1, PINS::ClockGate>,
}

impl<const B: usize, const B1: usize, PINS: Pins<UART<Static<B>>>>
    Serial<Static<B>, Static<B1>, PINS>
{
    /// Create instance of Uart.
    #[inline]
    pub fn new_static(
        uart: UART<Static<B>>,
        pins: PINS,
        config: impl Into<Config>,
        clock: &Clocks,
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
        let clock_gate: Gate<Static<B1>, PINS::ClockGate> = unsafe { Gate::steal_static() };
        // note(unsafe): async read and write using ccu registers
        unsafe { clock_gate.reset() };
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
        let uart_clk = (clock.apb1.0 + 8 * bps) / (16 * bps);
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
        Serial {
            uart,
            pins,
            clock_gate,
        }
    }
    /// Close uart and release peripheral.
    #[inline]
    pub fn free(self) -> (UART<Static<B>>, PINS) {
        // clock is closed for self.clock_gate is dropped
        let _ = unsafe { self.clock_gate.free() };
        (self.uart, self.pins)
    }
}

pub trait Pins<UART> {
    type ClockGate: ccu::ClockGate;
}

impl<A, A1, A2> Pins<UART<A>> for (Pin<A1, 'B', 8, Function<6>>, Pin<A2, 'B', 9, Function<6>>)
where
    A: BaseAddress,
    A1: BaseAddress,
    A2: BaseAddress,
{
    type ClockGate = ccu::UART<0>;
}

impl<A: BaseAddress, A1: BaseAddress, PINS: Pins<UART<A>>> embedded_hal::serial::ErrorType
    for Serial<A, A1, PINS>
{
    type Error = embedded_hal::serial::ErrorKind;
}

impl<A: BaseAddress, A1: BaseAddress, PINS: Pins<UART<A>>> embedded_hal::serial::Write
    for Serial<A, A1, PINS>
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

impl<A: BaseAddress, A1: BaseAddress, PINS: Pins<UART<A>>> embedded_hal_nb::serial::Write<u8>
    for Serial<A, A1, PINS>
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

/// 串口控制设置寄存器。
pub struct USR<R: Register>(UnsafeCell<R>);

/// 串口控制设置。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct UartStatus(u8);

impl<R: uart16550::Register> USR<R> {
    /// 写入线控制设置。
    #[inline]
    pub fn write(&self, val: UartStatus) {
        unsafe { self.0.get().write_volatile(R::from(val.0)) }
    }

    /// 读取线控制设置。
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

    /// 接收队列是否为满。
    #[inline]
    pub const fn receive_fifo_full(self) -> bool {
        self.0 & Self::RFF != 0
    }

    /// 接收队列是否非空。
    #[inline]
    pub const fn receive_fifo_not_empty(self) -> bool {
        self.0 & Self::RFNE != 0
    }

    /// 发送队列是否为空。
    #[inline]
    pub const fn transmit_fifo_empty(self) -> bool {
        self.0 & Self::TFE != 0
    }

    /// 发送队列是否未满。
    #[inline]
    pub const fn transmit_fifo_not_full(self) -> bool {
        self.0 & Self::TFNF != 0
    }

    /// 线路是否忙碌。
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
