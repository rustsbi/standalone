//! Physical layer peripheral of DDR SDRAM.

use base_address::{BaseAddress, Dynamic, Static};
use volatile_register::{RO, RW};

use super::PHY;

/// Physical layer peripheral.
// Ref: https://github.com/Moxa-Linux/BIOS-UC-8200_source_code/blob/master/arch/arm/include/asm/arch-sunxi/dram_sun8i_h3.h
#[repr(C)]
pub struct RegisterBlock {
    /// PHY initialization register
    pub pir: RW<u32>, // 0x00
    pub pwrctl: RW<u32>,  // 0x04
    pub mrctrl0: RW<u32>, // 0x08
    pub clken: RW<u32>,   // 0x0c
    /// PHY general status registers
    pub pgsr: [RW<u32>; 2], // 0x10 ..= 0x14
    pub statr: RW<u32>,   // 0x18
    _reserved0: [u32; 4],
    pub lp3mr11: RW<u32>, // 0x2c
    /// mode registers
    pub mr: [RW<u32>; 4], // 0x30 ..= 0x3c
    _reserved1: [u32; 1],
    /// PHY timing registers
    pub ptr: [RW<u32>; 5], // 0x44 ..= 0x54
    /// DRAM timing registers
    pub dramtmg: [RW<u32>; 9], // 0x58 ..= 0x78
    pub odtcfg: RW<u32>, // 0x7c
    /// PHY interface timing registers
    pub pitmg: [RW<u32>; 2], // 0x80 ..= 0x84
    pub lptpr: RW<u32>,  // 0x88
    pub rfshctl0: RW<u32>, // 0x8c
    /// refresh timing
    pub rfshtmg: RW<u32>, // 0x90
    pub rfshctl1: RW<u32>, // 0x94
    pub pwrtmg: RW<u32>, // 0x98
    pub asrc: RW<u32>,   // 0x9c
    pub asrtc: RW<u32>,  // 0xa0
    _reserved3: [u32; 5],
    pub vtfcr: RW<u32>,  // 0xb8
    pub dqsgmr: RW<u32>, // 0xbc
    pub dtcr: RW<u32>,   // 0xc0
    pub dtar0: RW<u32>,  // 0xc4
    _reserved4: [u32; 14],
    /// PHY general configuration registers
    pub pgcr: [RW<u32>; 4], // 0x100 ..= 0x10c
    pub iovcr0: RW<u32>, // 0x110
    pub iovcr1: RW<u32>, // 0x114
    _reserved5: [u32; 1],
    pub dxccr: RW<u32>,      // 0x11c
    pub odtmap: RW<u32>,     // 0x120
    pub zqctl: [RW<u32>; 2], // 0x124 ..= 0x128
    _reserved6: [u32; 5],
    /// ZQ control register
    pub zqcr: RW<u32>, // 0x140
    /// ZQ status register
    pub zqsr: RW<u32>, // 0x144
    /// ZQ data registers
    pub zqdr: [RW<u32>; 3], // 0x148 ..= 0x150
    _reserved7: [u32; 27],
    pub sched: RW<u32>,        // 0x1c0
    pub perfhpr: [RW<u32>; 2], // 0x1c4 ..= 0x1c8
    pub perflpr: [RW<u32>; 2], // 0x1cc ..= 0x1d0
    pub perfwr: [RW<u32>; 2],  // 0x1d4 ..= 0x1d8
    _reserved8: [u32; 9],
    pub acmdlr: RW<u32>,  // 0x200
    pub acldlr: RW<u32>,  // 0x204
    pub aciocr0: RW<u32>, // 0x208
    _reserved9: [u32; 61],
    pub datx: [Datx8; 4], // 0x300 ..= 0x4fc
    _reserved10: [u32; 226],
    pub upd2: RW<u32>, // 0x888
}

/// DATX8 register group.
#[repr(C)]
pub struct Datx8 {
    pub mdlr: RW<u32>,       // 0x00
    pub lcdlr: [RW<u32>; 3], // 0x04 ..= 0x0c
    /// IO configuration register
    pub iocr: [RW<u32>; 11], // 0x10 ..= 0x38
    pub sdlr6: RW<u32>,      // 0x3c
    pub gtr: RW<u32>,        // 0x40
    pub gcr: RW<u32>,        // 0x44
    pub gsr0: RO<u32>,       // 0x48
    pub gsr1: RW<u32>,       // 0x4c
    pub gsr2: RW<u32>,       // 0x50
    _reserved0: [u32; 11],
}

/// Data pin width.
pub enum DqWidth {
    /// Half DQ width.
    X8,
    /// Full DQ width.
    X16,
}

impl<const B: usize> PHY<Static<B>> {
    /// Create a peripheral instance from statically known address.
    ///
    /// This function is unsafe for it forces to seize ownership from possible
    /// wrapped peripheral group types. Users should normally retrieve ownership
    /// from wrapped types.
    #[inline]
    pub const unsafe fn steal_static() -> PHY<Static<B>> {
        PHY { base: Static::<B> }
    }
}

impl PHY<Dynamic> {
    /// Create a peripheral instance from dynamically known address.
    ///
    /// This function is unsafe for it forces to seize ownership from possible
    /// wrapped peripheral group types. Users should normally retrieve ownership
    /// from wrapped types.
    #[inline]
    pub unsafe fn steal_dynamic(base: *const ()) -> PHY<Dynamic> {
        PHY {
            base: Dynamic::new(base as usize),
        }
    }
}

impl<A: BaseAddress> PHY<A> {
    /// ?
    #[inline]
    pub fn dqs_gate_detect(&self) {
        let pgsr0 = self.pgsr[0].read();
        let qsgerr = pgsr0 & (1 << 22) != 0;
        if !qsgerr {
            // dual rank, full DQ
            return;
        }
        let dx0_gsr0 = self.datx[0].gsr0.read();
        let dx1_gsr0 = self.datx[1].gsr0.read();
        // qsgerr; each bit for one rank
        let dx0_qsgerr = (dx0_gsr0 >> 26) & 0xf;
        let dx1_qsgerr = (dx1_gsr0 >> 26) & 0xf;
        if dx0_qsgerr & 0x2 != 0 {
            if dx1_qsgerr & 0x2 != 0 {
                // single rank, full DQ
                return;
            } else {
                // single rank, half DQ
                return;
            }
        } else {
            if dx1_qsgerr != 0 {
                // todo
            } else {
                // dual rank, half DQ
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Datx8, RegisterBlock};
    use memoffset::offset_of;
    #[test]
    fn offset_phy() {
        assert_eq!(offset_of!(RegisterBlock, lp3mr11), 0x2c);
        assert_eq!(offset_of!(RegisterBlock, ptr), 0x44);
        assert_eq!(offset_of!(RegisterBlock, pgcr), 0x100);
        assert_eq!(offset_of!(RegisterBlock, zqcr), 0x140);
        assert_eq!(offset_of!(RegisterBlock, sched), 0x1c0);
        assert_eq!(offset_of!(RegisterBlock, acmdlr), 0x200);
        assert_eq!(offset_of!(RegisterBlock, datx), 0x300);
        assert_eq!(offset_of!(RegisterBlock, upd2), 0x888);
    }
    #[test]
    fn offset_datx8() {
        assert_eq!(offset_of!(Datx8, mdlr), 0x00);
        assert_eq!(offset_of!(Datx8, lcdlr), 0x04);
        assert_eq!(offset_of!(Datx8, iocr), 0x10);
        assert_eq!(offset_of!(Datx8, sdlr6), 0x3c);
        assert_eq!(offset_of!(Datx8, gsr0), 0x48);
    }
}
