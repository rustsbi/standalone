//! `rom-rt` supported chips.
//! 
//! This module should include `rom-rt` parameter and handoff structure.
//! It would be checked, formatted and tested under any architecture,
//! i.e. should not include architecture specific code.

#[cfg_attr(not(feature = "allwinner-d1"), allow(unused))]
mod allwinner_d1;

#[cfg(feature = "allwinner-d1")]
pub use allwinner_d1::Parameters;
