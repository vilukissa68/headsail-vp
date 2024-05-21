//! Abstractions that only exist on HPC
mod hart_id;
mod interrupt;
pub use hart_id::*;
pub use interrupt::*;
