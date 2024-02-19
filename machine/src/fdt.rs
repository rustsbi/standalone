mod uart16550;

use crate::board::Board;
use dtb_walker::WalkOperation::{StepInto, StepOut, StepOver};
use dtb_walker::{Dtb, DtbObj, HeaderError, Property};

// TODO unbounded lifetime
pub fn try_read_fdt<'a>(fdt_paddr: usize) -> Result<Dtb<'a>, HeaderError> {
    let fdt_ptr = fdt_paddr as *const u8;
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

pub fn parse_fdt(fdt: Dtb, board: &mut Board) {
    fdt.walk(|ctx, obj| match obj {
        DtbObj::SubNode { name } => {
            let current = ctx.last();
            if ctx.level() == 0 {
                if name == b"soc" {
                    StepInto
                } else {
                    StepOver
                }
            } else if current == b"soc" {
                if name.starts_with(b"uart") || name.starts_with(b"serial") {
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
            let node = ctx.last();
            if node.starts_with(b"uart") || node.starts_with(b"serial") {
                board.set_uart16550_serial(reg.next().unwrap());
                StepOut
            } else {
                StepOver
            }
        }
        DtbObj::Property(_) => StepOver,
    });
}
