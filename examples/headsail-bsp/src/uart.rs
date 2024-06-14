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
pub fn getc() -> Option<u8> {
    let data_ready = unsafe { read_u8(UART0_ADDR + UART_DATA_READY_OFFSET) };
    if data_ready & 1 == 0 {
        None
    } else {
        let byte = unsafe { read_u8(UART0_ADDR) };
        Some(byte)
    }
}

#[cfg(feature = "alloc")]
pub fn uart_read_to_heap(bytes: usize) -> Vec<u8> {
    let mut result = Vec::with_capacity(bytes);
    while result.len() < bytes {
        match getc() {
            Some(c) => result.push(c),
            None => (),
        }
    }
    result
}
