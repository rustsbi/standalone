mod clint;
mod uart16550;

use core::ops::Range;
use dtb_walker::WalkOperation::{StepInto, StepOut, StepOver};
use dtb_walker::{Dtb, DtbObj, HeaderError, Property};
use rustsbi::RustSBI;

#[derive(RustSBI)]
pub struct FdtBoard<'a> {
    #[rustsbi(dbcn)]
    serial: uart16550::Uart16550Handle<'a>,
    #[rustsbi(time, ipi)]
    clint: clint::ClintHandle<'a>,
}

impl<'a> FdtBoard<'a> {
    pub fn new() -> Self {
        Self {
            serial: uart16550::Uart16550Handle {
                uart16550: None,
                range: 0x80200000..0x90000000usize, // TODO correct physical memory range
            },
            clint: clint::ClintHandle {
                clint: None,
                max_hart_id: 0,
            },
        }
    }

    fn set_uart16550_serial(&mut self, range: Range<usize>) {
        trace!("set_uart16550_serial range = {:x?}", range);
        // TODO check address range
        self.serial = uart16550::Uart16550Handle {
            uart16550: Some(unsafe { &*(range.start as *const _) }),
            range: 0x80200000..0x90000000usize, // TODO correct physical memory range
        }
    }

    fn set_clint(&mut self, range: Range<usize>) {
        trace!("set_clint range = {:x?}", range);
        // TODO check address range
        self.clint.clint = Some(unsafe { &*(range.start as *const _) })
    }

    pub fn load_main_console(&self) {
        if let Some(uart16550) = self.serial.uart16550 {
            crate::console::load_console_uart16550(uart16550)
        }
    }
}

// TODO unbounded lifetime
pub fn try_read_fdt<'a>(fdt_paddr: usize) -> Result<Dtb<'a>, HeaderError> {
    let fdt_ptr = fdt_paddr as *const u8;
    trace!("try_read_fdt, fdt_paddr = {:08x}", fdt_paddr);
    // TODO check permission of fdt_ptr, handle memory access error
    unsafe {
        Dtb::from_raw_parts_filtered(fdt_ptr, |e| {
            matches!(
                e,
                HeaderError::Misaligned(4) | HeaderError::LastCompVersion(_)
            )
        })
    }
}

pub fn parse_fdt(fdt: Dtb, board: &mut FdtBoard) {
    trace!("parse_fdt begin");
    fdt.walk(|ctx, obj| match obj {
        DtbObj::SubNode { name } => {
            trace!("visit SubNode {:?}", name.as_str());
            let current = ctx.name();
            if ctx.level() == 0 {
                if name == "soc".into() {
                    StepInto
                } else {
                    StepOver
                }
            } else if current == "soc".into() {
                if name.starts_with("uart")
                    || name.starts_with("serial")
                    || name.starts_with("clint")
                {
                    StepInto
                } else {
                    StepOver
                }
            } else {
                StepOver
            }
        }
        // DtbObj::Property(Property::Model(model)) if ctx.is_root() => {
        //     // ans.model.0 = model.as_bytes().len();
        //     // ans.model.1[..ans.model.0].copy_from_slice(model.as_bytes());
        //     StepOver
        // }
        DtbObj::Property(Property::Reg(mut reg)) => {
            trace!("visit DtbObj::Property Property::Reg {:x?}", reg);
            let node = ctx.name();
            if node.starts_with("uart") || node.starts_with("serial") {
                if let Some(range) = reg.next() {
                    board.set_uart16550_serial(range);
                }
                StepOut
            } else if node.starts_with("clint") {
                if let Some(range) = reg.next() {
                    board.set_clint(range);
                }
                StepOut
            } else {
                StepOver
            }
        }
        DtbObj::Property(_) => StepOver,
    });
}
