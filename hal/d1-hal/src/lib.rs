//! The whole package is kept for the community doesn't have a `d1-hal` yet.
//! If anyone would like to maintain `d1-hal`, this package should be moved
//! out from RustSBI Prototyping System and merge to a new `d1-hal` project.
#![no_std]

pub mod ccu;
pub mod gpio;
pub mod jtag;
pub mod spi;
pub mod time;
pub mod uart;

pub use d1_pac as pac;
