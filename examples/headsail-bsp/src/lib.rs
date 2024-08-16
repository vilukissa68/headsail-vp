//! A light-weight memory map based board support package for Headsail.
#![no_std]

pub mod sprintln;
#[cfg(feature = "sysctrl")]
pub mod sysctrl;
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
#[cfg(feature = "panic-uart")]
mod ufmt_panic;

/// # Safety
///
/// Unaligned reads may fail to produce expected results on RISC-V.
#[inline(always)]
pub unsafe fn read_u8(addr: usize) -> u8 {
    core::ptr::read_volatile(addr as *const _)
}

/// # Safety
///
/// Unaligned writes may fail to produce expected results on RISC-V.
#[inline(always)]
pub unsafe fn write_u8(addr: usize, val: u8) {
    core::ptr::write_volatile(addr as *mut _, val)
}

#[inline(always)]
pub fn read_u32(addr: usize) -> u32 {
    unsafe { core::ptr::read_volatile(addr as *const _) }
}

#[inline(always)]
pub fn write_u32(addr: usize, val: u32) {
    unsafe { core::ptr::write_volatile(addr as *mut _, val) }
}

#[inline(always)]
pub fn mask_u32(addr: usize, mask: u32) {
    let r = unsafe { core::ptr::read_volatile(addr as *const u32) };
    unsafe { core::ptr::write_volatile(addr as *mut _, r | mask) }
}

#[inline(always)]
pub fn unmask_u32(addr: usize, unmask: u32) {
    let r = unsafe { core::ptr::read_volatile(addr as *const u32) };
    unsafe { core::ptr::write_volatile(addr as *mut _, r & !unmask) }
}

#[inline(always)]
pub fn toggle_u32(addr: usize, toggle_bits: u32) {
    let mut r = read_u32(addr);
    r ^= toggle_bits;
    write_u32(addr, r);
}

#[cfg(feature = "alloc")]
pub fn init_alloc() {
    unsafe { alloc::init_heap() };
}
