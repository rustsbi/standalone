#![no_std]

pub mod com;
pub mod phy;

use core::{marker::PhantomData, ops};

/// Common control peripheral
pub struct COM<const A: usize> {
    _marker: PhantomData<*const ()>,
}

unsafe impl<const A: usize> Send for COM<A> {}

impl<const A: usize> ops::Deref for COM<A> {
    type Target = com::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(A as *const _) }
    }
}

/// Physical layer peripheral
pub struct PHY<const A: usize> {
    _marker: PhantomData<*const ()>,
}

unsafe impl<const A: usize> Send for PHY<A> {}

impl<const A: usize> ops::Deref for PHY<A> {
    type Target = phy::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(A as *const _) }
    }
}
