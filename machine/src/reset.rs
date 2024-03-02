use sifive_test_device::SifiveTestDevice;
use spin::Mutex;

static SBI_RESET: Mutex<MachineReset> = Mutex::new(MachineReset::DeadLoop);

pub fn fail() -> ! {
    let lock = SBI_RESET.lock();
    match *lock {
        MachineReset::DeadLoop => {
            trace!("test fail, begin dead loop");
            loop {}
        }
        MachineReset::SifiveTest(test) => {
            trace!("SiFive Test test fail");
            unsafe { &*test }.fail(0)
        }
    }
}

enum MachineReset {
    DeadLoop,
    SifiveTest(*const SifiveTestDevice),
}

unsafe impl Send for MachineReset {}
unsafe impl Sync for MachineReset {}

#[cfg(feature = "fdt")] // TODO
pub fn load_reset_sifive_test(sifive_test: &SifiveTestDevice) {
    let mut lock = SBI_RESET.lock();
    *lock = MachineReset::SifiveTest(sifive_test);
    drop(lock);
}
