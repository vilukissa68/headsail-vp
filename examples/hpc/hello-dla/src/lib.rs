#![no_std]

mod mmap;

use core::ptr;
use mmap::{UART0_ADDR, DLA0_ADDR};

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
    for i in 0..len {
        unsafe { buf[i] = ptr::read_volatile((DLA0_ADDR + offset + i) as *mut u8)} 
    }
}



