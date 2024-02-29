//! Frequently used first boot stage dynamic information on RISC-V.

/// M-mode firmware dynamic information.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DynamicInfo {
    /// Dynamic information magic value.
    pub magic: usize,
    /// Version of dynamic information.
    pub version: usize,
    /// Address of the next boot-loading stage.
    pub next_addr: usize,
    /// RISC-V privilege mode of the next boot-loading stage.
    pub next_mode: usize,
    /// M-mode firmware options; its definition varies between SBI implementations.
    pub options: usize,
}

// TODO unconstrained lifetime
pub fn try_read_dynamic<'a>(paddr: usize) -> Result<&'a DynamicInfo, ()> {
    // TODO check pointer before dereference
    let ans = unsafe { &*(paddr as *const DynamicInfo) };
    Ok(ans)
}
