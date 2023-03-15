use volatile_register::RW;

use super::COM;

/// Common control peripheral
pub struct RegisterBlock {
    pub work_mode_0: RW<u32>, // 0x00
    pub work_mode_1: RW<u32>, // 0x04
    pub dbgcr: RW<u32>,       // 0x08
    pub tmr: RW<u32>,         // 0x0c
    _reserved0: [u32; 1],
    pub cccr: RW<u32>, // 0x14
    _reserved1: [u32; 2],
    pub maer0: RW<u32>, // 0x20
    pub maer1: RW<u32>, // 0x24
    pub maer2: RW<u32>, // 0x28
    _reserved2: [u32; 309],
    pub remap0: RW<u32>, // 0x500
    pub remap1: RW<u32>, // 0x504
    pub remap2: RW<u32>, // 0x508
    pub remap3: RW<u32>, // 0x50c
}

/// Dram type
pub enum Type {
    /// DDR2
    Ddr2,
    /// DDR3
    Ddr3,
    /// LPDDR2
    LpDdr2,
    /// LPDDR3
    LpDdr3,
}

/// Dram configuration
pub struct Config {
    dram_type: Type,
    unknown_tpr13_bit5: bool,
    // rank: u8,
}

impl<const A: usize> COM<A> {
    /// Configure dram settings
    #[inline]
    pub fn configure(&self, config: Config) {
        let mut bits = 0;
        let mut mask = 0;
        let dram_type_bits = match config.dram_type {
            Type::Ddr2 => 2,
            Type::Ddr3 => 3,
            Type::LpDdr2 => 6,
            Type::LpDdr3 => 7,
        };
        bits |= dram_type_bits << 16;
        mask |= 0xff << 16;
        let dq_width = 1;
        bits |= dq_width << 12;
        mask |= 0x1 << 16;
        let unknown1 = 1;
        bits |= unknown1 << 22;
        mask |= 0x1 << 22;
        let unknown2 = match config.dram_type {
            Type::LpDdr2 | Type::LpDdr3 => 1,
            _ if config.unknown_tpr13_bit5 => 1,
            _ => 0,
        };
        bits |= unknown2 << 19;
        mask |= 0x1 << 19;
        unsafe { self.work_mode_0.modify(|v| (v & !mask) | bits) };
    }

    /// Get DRAM size in bytes
    #[inline]
    pub fn dram_size(&self) -> usize {
        let bits_0 = self.work_mode_0.read();
        let bits_1 = self.work_mode_1.read();
        fn rank_size_log2(bits: u32) -> u32 {
            let page_size_bits = (bits >> 8) & 0xf;
            let row_width_bits = (bits >> 4) & 0xf;
            let bank_count_bits = (bits >> 2) & 0x3;
            let page_size = page_size_bits + 3;
            let row_width = row_width_bits + 1;
            let bank_count = bank_count_bits + 2;
            page_size + row_width + bank_count
        }
        let ans_rank_0 = rank_size_log2(bits_0);
        let single_rank = (bits_0 & 0x3) == 0;
        if single_rank {
            return 1 << ans_rank_0;
        }
        let two_identical_ranks = (bits_1 & 0x3) == 0;
        if two_identical_ranks {
            return 1 << (ans_rank_0 + 1);
        }
        let ans_rank_1 = rank_size_log2(bits_1);
        1 << (ans_rank_0 + ans_rank_1)
    }
}

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use memoffset::offset_of;
    #[test]
    fn offset_com() {
        assert_eq!(offset_of!(RegisterBlock, tmr), 0x0c);
        assert_eq!(offset_of!(RegisterBlock, cccr), 0x14);
        assert_eq!(offset_of!(RegisterBlock, maer0), 0x20);
        assert_eq!(offset_of!(RegisterBlock, remap0), 0x500);
    }
}
