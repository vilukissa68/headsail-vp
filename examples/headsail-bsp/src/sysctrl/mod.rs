//! Abstractions that only exist on SysCtrl
pub mod gpio;
pub mod soc_ctrl;
#[cfg(feature = "pac")]
pub mod udma;

mod interrupt;
mod mmap;
