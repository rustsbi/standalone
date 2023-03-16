//! Static and dynamic base address for peripheral buses.
#![no_std]

/// Types that would represent a base address of peripheral.
pub trait BaseAddress {
    /// Return pointer representation of this base address.
    fn ptr(&self) -> *const ();
}

/// Address known on compile time.
///
/// This is a zero-sized type; structures with `Static<A>` as
/// parameter would not take additional memory space.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Static<const A: usize>;

impl<const A: usize> BaseAddress for Static<A> {
    #[inline(always)]
    fn ptr(&self) -> *const () {
        A as *const ()
    }
}

/// Address only known on runtime but not compile time.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dynamic {
    base: usize,
}

impl Dynamic {
    /// Create a dynamically known base address.
    #[inline]
    pub const fn new(base: usize) -> Self {
        Dynamic { base }
    }
}

impl BaseAddress for Dynamic {
    #[inline(always)]
    fn ptr(&self) -> *const () {
        self.base as *const ()
    }
}
