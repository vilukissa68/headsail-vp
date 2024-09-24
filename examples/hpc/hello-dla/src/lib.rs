#![no_std]

mod mmap;

use core::ptr;
use mmap::{DLA0_ADDR, UART0_ADDR};

pub fn uart_write(s: &str) {
    for b in s.as_bytes() {
        unsafe { ptr::write_volatile(UART0_ADDR as *mut u8, *b) };
    }
}

pub fn dla_write(s: &str) {
    for b in s.as_bytes() {
        unsafe { ptr::write_volatile(DLA0_ADDR as *mut u8, *b) };
    }
}

pub fn dla_read(buf: &mut [u8], len: usize, offset: usize) {
    buf.iter_mut()
        .take(len)
        .enumerate()
        .for_each(|(i, byte)| unsafe {
            *byte = ptr::read_volatile((DLA0_ADDR + offset + i) as *const u8);
        });
}

// Number of nops HPC is capable of executing at 30 MHz reference clocks
pub const NOPS_PER_SEC: usize = match () {
    // These are experimentally found values
    #[cfg(debug_assertions)]
    () => 6_000,
    #[cfg(not(debug_assertions))]
    () => 120_000,
};
