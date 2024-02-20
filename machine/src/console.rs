use spin::Mutex;
use uart16550::Uart16550;

struct Uart16550Wrap {
    uart16550: *const Uart16550<u8>,
}

unsafe impl Send for Uart16550Wrap {}
unsafe impl Sync for Uart16550Wrap {}

static CONSOLE: Mutex<Uart16550Wrap> = Mutex::new(Uart16550Wrap {
    uart16550: core::ptr::null(), //0x10000000 as *const _,
});

struct RCoreConsole;

impl rcore_console::Console for RCoreConsole {
    #[inline]
    fn put_char(&self, c: u8) {
        let console = CONSOLE.lock();
        while unsafe { &*console.uart16550 }.write(&[c]) == 0 {
            core::hint::spin_loop();
        }
    }

    #[inline]
    fn put_str(&self, s: &str) {
        let console = CONSOLE.lock();
        let mut bytes = s.as_bytes();
        while !bytes.is_empty() {
            let count = unsafe { &*console.uart16550 }.write(bytes);
            bytes = &bytes[count..];
        }
    }
}

pub fn load_console(uart16550: &Uart16550<u8>) {
    let mut console = CONSOLE.lock();
    *console = Uart16550Wrap { uart16550 };
    drop(console);
    rcore_console::init_console(&RCoreConsole);
    rcore_console::set_log_level(option_env!("LOG"));
}
