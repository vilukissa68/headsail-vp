use crate::{
    mmap::{UART0_ADDR, UART0_THR, UART_DATA_READY_OFFSET},
    read_u8, write_u8,
};

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "asic")]
#[inline]
pub fn init_uart(freq: u32, baud: u32) {
    use crate::mmap::{
        UART0_DIV_LSB, UART0_DIV_MSB, UART0_FIFO_CONTROL, UART0_INTERRUPT_ENABLE,
        UART0_LINE_CONTROL, UART0_MODEM_CONTROL,
    };
    const PERIPH_CLK_DIV: u32 = 2;
    let divisor: u32 = freq / PERIPH_CLK_DIV / (baud << 4);

    // Safety: unknown; we don't know if 8-bit writes will succeed
    unsafe {
        // Disable all interrupts
        write_u8(UART0_INTERRUPT_ENABLE, 0x00);
        // Enable DLAB (set baud rate divisor)
        write_u8(UART0_LINE_CONTROL, 0x80);
        // Divisor (lo byte)
        write_u8(UART0_DIV_LSB, divisor as u8);
        // Divisor (hi byte)
        write_u8(UART0_DIV_MSB, (divisor >> 8) as u8);
        // 8 bits, no parity, one stop bit
        write_u8(UART0_LINE_CONTROL, 0x03);
        // Enable FIFO, clear them, with 14-byte threshold
        write_u8(UART0_FIFO_CONTROL, 0xC7);
        // Autoflow mode
        write_u8(UART0_MODEM_CONTROL, 0x20);
    }
}

/// Dummy definition for VP, as the UART on VP does not require configuration
#[cfg(feature = "vp")]
pub fn init_uart(_freq: u32, _baud: u32) {}

#[cfg(feature = "asic")]
#[inline]
fn is_transmit_empty() -> bool {
    // Safety: UART_LINE_STATUS is 4-byte aligned
    unsafe { (read_u8(crate::mmap::UART0_LINE_STATUS) & 0x20) != 0 }
}

#[inline]
pub fn uart_write(s: &str) {
    for b in s.as_bytes() {
        putc(*b);
    }
}

#[inline]
pub fn putc(c: u8) {
    // Wait for hardware to report completion
    #[cfg(feature = "asic")]
    while !is_transmit_empty() {}

    // Safety: UART_THR is 4-byte aligned
    unsafe { write_u8(UART0_THR, c) };
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
