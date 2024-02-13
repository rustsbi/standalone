pub fn is_fdt_blob(signature: u32) -> bool {
    signature == 0xd00dfeed
}

pub fn read_failsafe_signature(opaque: usize) -> u32 {
    // TODO handle memory access error
    unsafe { *(opaque as *const u32) }
}

pub fn parse_fdt(opaque: usize) {
    let _ = opaque;
    todo!()
}
