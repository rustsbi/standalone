use spin::Mutex;
use uart16550::Uart16550;

struct Console {
    uart16550: *const Uart16550<u8>,
}

unsafe impl Send for Console {}
unsafe impl Sync for Console {}

static CONSOLE: Mutex<Option<Console>> = Mutex::new(None);

impl rcore_console::Console for Console {
    #[inline]
    fn put_char(&self, c: u8) {
        if let Some(console) = &*CONSOLE.lock() {
            while unsafe { &*console.uart16550 }.write(&[c]) == 0 {
                core::hint::spin_loop();
            }
        }
    }

    #[inline]
    fn put_str(&self, s: &str) {
        if let Some(console) = &*CONSOLE.lock() {
            let mut bytes = s.as_bytes();
            while !bytes.is_empty() {
                let count = unsafe { &*console.uart16550 }.write(bytes);
                bytes = &bytes[count..];
            }
        }
    }
}

pub fn load_console(uart16550: &Uart16550<u8>) {
    *CONSOLE.lock() = Some(Console { uart16550 })
}
