use spin::Mutex;
use uart16550::Uart16550;

enum MachineConsole {
    Uart16550(*const Uart16550<u8>),
}

impl MachineConsole {
    #[inline]
    fn put_char(&self, ch: u8) {
        match self {
            Self::Uart16550(uart16550) => {
                while unsafe { &**uart16550 }.write(&[ch]) == 0 {
                    core::hint::spin_loop();
                }
            }
        }
    }

    #[inline]
    fn put_str(&self, string: &str) {
        let mut bytes = string.as_bytes();
        match self {
            Self::Uart16550(uart16550) => {
                while !bytes.is_empty() {
                    let count = unsafe { &**uart16550 }.write(bytes);
                    bytes = &bytes[count..];
                }
            }
        }
    }
}

unsafe impl Send for MachineConsole {}
unsafe impl Sync for MachineConsole {}

static CONSOLE: Mutex<MachineConsole> =
    Mutex::new(MachineConsole::Uart16550(0x10000000 as *const _));

pub(crate) struct RCoreConsole;

impl rcore_console::Console for RCoreConsole {
    #[inline]
    fn put_char(&self, c: u8) {
        let console = CONSOLE.lock();
        console.put_char(c)
    }

    #[inline]
    fn put_str(&self, s: &str) {
        let console = CONSOLE.lock();
        console.put_str(s)
    }
}

pub fn load_console_uart16550(uart16550: &Uart16550<u8>) {
    let mut console = CONSOLE.lock();
    *console = MachineConsole::Uart16550(uart16550);
    drop(console);
}
