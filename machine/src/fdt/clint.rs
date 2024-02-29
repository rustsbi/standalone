use rustsbi::SbiRet;

pub struct ClintHandle<'a> {
    pub clint: Option<&'a aclint::SifiveClint>,
    pub max_hart_id: usize,
}

impl<'a> rustsbi::Timer for ClintHandle<'a> {
    #[inline]
    fn set_timer(&self, stime_value: u64) {
        if let Some(clint) = self.clint {
            let current_hart_id = riscv::register::mhartid::read();
            clint.write_mtimecmp(current_hart_id, stime_value);
        } else {
            debug!("SBI TIME set_timer when no CLINT peripheral in handle")
        }
    }
}

impl<'a> rustsbi::Ipi for ClintHandle<'a> {
    #[inline]
    fn send_ipi(&self, hart_mask: rustsbi::HartMask) -> SbiRet {
        if let Some(clint) = self.clint {
            for hart_id in 0..=self.max_hart_id {
                if hart_mask.has_bit(hart_id) {
                    clint.set_msip(hart_id);
                }
            }
            SbiRet::success(0)
        } else {
            debug!("SBI IPI send_ipi when no CLINT peripheral in handle");
            SbiRet::not_supported()
        }
    }
}
