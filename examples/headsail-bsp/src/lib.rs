//! A light-weight memory map based board support package for Headsail.
#![no_std]

pub mod sprintln;
pub mod uart;
pub mod timer {
    /*!
     * Timer module for Headsail. When running on the Renode
     * Virtual Prototype, the "vp" feature should be enabled.
     * All operations are Read-Modify-Write.
     *
     * HOW TO USE THIS DRIVER:
     * In order to use any of the four timers that come with
     * Headsail HPC SubSystem, use the respective Timer{0..3}
     * type alias provided and the functions associated with it.
     */
    #[cfg(not(feature = "vp"))]
    pub use crate::apb_timer::*;
    #[cfg(feature = "vp")]
    pub use crate::timer_unit::*;
}
pub mod tb;

#[cfg(not(feature = "vp"))]
mod apb_timer;
#[cfg(feature = "vp")]
mod timer_unit;

#[cfg(feature = "hpc")]
pub use hpc::*;
pub use riscv;
#[cfg(feature = "rt")]
pub use riscv_rt as rt;
pub use ufmt;

#[cfg(feature = "alloc")]
pub mod alloc;
#[cfg(feature = "hpc")]
mod hpc;
mod mmap;
#[cfg(feature = "sysctrl")]
mod sysctrl;
#[cfg(feature = "panic-uart")]
mod ufmt_panic;

#[cfg(feature = "hpc")]
const EXTERNAL_BIT: usize = 0x1_0000_0000;

/// # Safety
///
/// Unaligned reads may fail to produce expected results on RISC-V.
#[inline(always)]
pub unsafe fn read_u8(addr: usize) -> u8 {
    #[cfg(feature = "hpc")]
    return core::ptr::read_volatile((addr | EXTERNAL_BIT) as *const _);
    #[cfg(feature = "sysctrl")]
    core::ptr::read_volatile(addr as *const _)
}

/// # Safety
///
/// Unaligned writes may fail to produce expected results on RISC-V.
#[inline(always)]
pub unsafe fn write_u8(addr: usize, val: u8) {
    #[cfg(feature = "hpc")]
    core::ptr::write_volatile((addr | EXTERNAL_BIT) as *mut _, val);
    #[cfg(feature = "sysctrl")]
    core::ptr::write_volatile(addr as *mut _, val)
}

#[inline(always)]
pub fn read_u32(addr: usize) -> u32 {
    #[cfg(feature = "hpc")]
    return unsafe { core::ptr::read_volatile((addr | EXTERNAL_BIT) as *const _) };
    #[cfg(feature = "sysctrl")]
    unsafe {
        core::ptr::read_volatile(addr as *const _)
    }
}

#[inline(always)]
pub fn write_u32(addr: usize, val: u32) {
    #[cfg(feature = "hpc")]
    unsafe {
        core::ptr::write_volatile((addr | EXTERNAL_BIT) as *mut _, val)
    };
    #[cfg(feature = "sysctrl")]
    unsafe {
        core::ptr::write_volatile(addr as *mut _, val)
    }
}

#[cfg(feature = "alloc")]
pub fn init_alloc(heap_start: usize, heap_size: usize) {
    unsafe { alloc::init_heap(heap_start, heap_size) };
}
