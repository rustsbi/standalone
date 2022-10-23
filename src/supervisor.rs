pub enum Operation {
    Stop,
    SystemReset,
}

pub struct SupervisorContext {
    machine_sp: usize,
    x: [usize; 31],
    mstatus: usize,
    mepc: usize,
}

// todo: save/restore context can have macros like offset_of!(mstatus, SupervisorContext) etc.
