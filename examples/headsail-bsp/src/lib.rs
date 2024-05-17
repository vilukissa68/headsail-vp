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

mod apb_timer;
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
#[cfg(any(feature = "panic-uart"))]
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

#[cfg(feature = "sysctrl-rt")]
#[export_name = "_setup_interrupts"]
fn setup_interrupt_vector() {
    use riscv::register::mtvec;

    // Set the trap vector
    unsafe {
        extern "C" {
            fn _trap_vector();
        }

        // Set all the trap vectors for good measure
        let bits = _trap_vector as usize;
        mtvec::write(bits, mtvec::TrapMode::Vectored);
    }
}

#[cfg(feature = "alloc")]
pub fn init_alloc() {
    unsafe { alloc::init_heap() };
}

// The vector table
//
// Do the ESP trick and route all interrupts to the direct dispatcher.
//
// N.b. vectors length must be exactly 0x80
#[cfg(feature = "sysctrl-rt")]
core::arch::global_asm!(
    "
.section .vectors, \"ax\"
    .global _trap_vector
    // Trap vector base address must always be aligned on a 4-byte boundary
    .align 4
_trap_vector:
    j _start_trap
    .rept 31
    .word _start_trap // 1..31
    .endr
"
);
