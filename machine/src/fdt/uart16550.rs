//! FDT UART 16550 driver module

use core::ops::Range;
use rustsbi::{Physical, SbiRet};
use uart16550::Uart16550;

pub struct SerialHandle<'a> {
    pub uart16550: Option<&'a Uart16550<u8>>,
    pub range: Range<usize>,
}

impl<'a> rustsbi::Console for SerialHandle<'a> {
    fn write(&self, bytes: Physical<&[u8]>) -> SbiRet {
        if let Some(uart16550) = self.uart16550 {
            let start = bytes.phys_addr_lo();
            let end = start + bytes.num_bytes();
            if self.range.contains(&start) && self.range.contains(&(end - 1)) {
                let buf =
                    unsafe { core::slice::from_raw_parts(start as *const u8, bytes.num_bytes()) };
                SbiRet::success(uart16550.write(buf))
            } else {
                SbiRet::invalid_param()
            }
        } else {
            SbiRet::failed()
        }
    }

    fn read(&self, _bytes: Physical<&mut [u8]>) -> SbiRet {
        todo!()
    }

    fn write_byte(&self, byte: u8) -> SbiRet {
        if let Some(uart16550) = self.uart16550 {
            uart16550.write(&[byte]);
            SbiRet::success(0)
        } else {
            SbiRet::failed()
        }
    }
}
