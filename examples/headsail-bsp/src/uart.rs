use crate::{mmap::UART0_ADDR, mmap::UART_DATA_READY_OFFSET, read_u8, write_u8};

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[inline]
pub fn uart_write(s: &str) {
    for b in s.as_bytes() {
        putc(*b);
    }
}

#[inline]
pub fn putc(c: u8) {
    // Safety: we don't know if u8 writes work for all target architectures
    unsafe { write_u8(UART0_ADDR, c) };
}

#[inline]
pub fn getc() -> u8 {
    // Wait for data to become ready
    while unsafe { read_u8(UART0_ADDR + UART_DATA_READY_OFFSET) } & 1 == 0 {}

    // SAFETY: UART0_ADDR is 4-byte aligned
    unsafe { read_u8(UART0_ADDR) }
}

#[cfg(feature = "alloc")]
pub fn uart_read_to_heap(bytes: usize) -> Vec<u8> {
    let mut result = Vec::with_capacity(bytes);
    for _ in 0..bytes {
        result.push(getc())
    }
    result
}
