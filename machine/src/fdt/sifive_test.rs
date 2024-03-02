use rustsbi::{
    spec::srst::{
        RESET_REASON_NO_REASON, RESET_REASON_SYSTEM_FAILURE, RESET_TYPE_COLD_REBOOT,
        RESET_TYPE_SHUTDOWN, RESET_TYPE_WARM_REBOOT,
    },
    Reset, SbiRet,
};
use sifive_test_device::SifiveTestDevice;

pub struct SifiveTestHandle<'a> {
    pub sifive_test: Option<&'a SifiveTestDevice>,
}

impl<'a> Reset for SifiveTestHandle<'a> {
    #[inline]
    fn system_reset(&self, reset_type: u32, reset_reason: u32) -> SbiRet {
        if let Some(test) = self.sifive_test {
            match reset_type {
                RESET_TYPE_SHUTDOWN => match reset_reason {
                    RESET_REASON_NO_REASON => test.pass(),
                    RESET_REASON_SYSTEM_FAILURE => test.fail(-1 as _),
                    value => test.fail(value as _),
                },
                RESET_TYPE_COLD_REBOOT | RESET_TYPE_WARM_REBOOT => {
                    test.reset();
                }
                _ => SbiRet::invalid_param(),
            }
        } else {
            SbiRet::not_supported()
        }
    }
}
