use rustsbi::{Fence, Hsm, Ipi, Pmu, Reset, RustSBI, Timer};

pub struct DynRustSBI {
    inner: RustSBI<
        &'static dyn Timer,
        &'static dyn Ipi,
        &'static dyn Fence,
        &'static dyn Hsm,
        &'static dyn Reset,
        &'static dyn Pmu,
    >,
}

// pub struct DynPenglai

// pub struct DynRaven

// pub struct DynDramHypervisor
