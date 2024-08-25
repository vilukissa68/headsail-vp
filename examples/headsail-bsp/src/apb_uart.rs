use crate::{
    mmap::{UART0_ADDR, UART1_ADDR},
    read_u8, write_u8,
};

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Relocatable driver for NS16550 UART IP
///
/// The generic represents the base address for the UART. This driver is
/// compatible with both ASIC and the VP. Use
///
/// * `-Fasic` for ASIC implementation
/// * and `-Fvp` for VP implementation.
pub struct ApbUart<const BASE_ADDRESS: usize>;

/// Type alias for APB UART 0
pub type ApbUart0 = ApbUart<UART0_ADDR>;

/// Type alias for APB UART 1
pub type ApbUart1 = ApbUart<UART1_ADDR>;

impl<const BASE_ADDR: usize> ApbUart<BASE_ADDR> {
    /// # Parameters
    ///
    /// * `soc_freq`-  used to calculate BAUD rate together with divisor
    /// * `baud` - target BAUD (sa. UART protocol)
    #[allow(unused_variables)]
    pub fn init(soc_freq: u32, baud: u32) -> Self {
        #[cfg(feature = "asic")]
        {
            use crate::mmap::{
                UART_DIV_LSB_OFFSET, UART_DIV_MSB_OFFSET, UART_FIFO_CONTROL_OFFSET,
                UART_INTERRUPT_ENABLE_OFFSET, UART_LINE_CONTROL_OFFSET, UART_MODEM_CONTROL_OFFSET,
            };

            const PERIPH_CLK_DIV: u32 = 2;
            let divisor: u32 = soc_freq / PERIPH_CLK_DIV / (baud << 4);

            // Safety: unknown; we don't know if 8-bit writes will succeed
            unsafe {
                // Disable all interrupts
                write_u8(BASE_ADDR + UART_INTERRUPT_ENABLE_OFFSET, 0x00);
                // Enable DLAB (set baud rate divisor)
                write_u8(BASE_ADDR + UART_LINE_CONTROL_OFFSET, 0x80);
                // Divisor (lo byte)
                write_u8(BASE_ADDR + UART_DIV_LSB_OFFSET, divisor as u8);
                // Divisor (hi byte)
                write_u8(BASE_ADDR + UART_DIV_MSB_OFFSET, (divisor >> 8) as u8);
                // 8 bits, no parity, one stop bit
                write_u8(BASE_ADDR + UART_LINE_CONTROL_OFFSET, 0x03);
                // Enable FIFO, clear them, with 14-byte threshold
                write_u8(BASE_ADDR + UART_FIFO_CONTROL_OFFSET, 0xC7);
                // Autoflow mode
                write_u8(BASE_ADDR + UART_MODEM_CONTROL_OFFSET, 0x20);
            }
        }
        Self {}
    }

    /// # Safety
    ///
    /// Returns a potentially uninitialized instance of APB UART. On ASIC, make
    /// sure to call [ApbUart::init] prior to this call, otherwise the UART
    /// won't behave properly.
    pub const unsafe fn instance() -> Self {
        Self {}
    }

    #[cfg(feature = "asic")]
    #[inline]
    fn is_transmit_empty(&self) -> bool {
        // Safety: UART_LINE_STATUS is 4-byte aligned
        unsafe { (read_u8(BASE_ADDR + crate::mmap::UART_LINE_STATUS_OFFSET) & 0x20) != 0 }
    }

    #[inline]
    pub fn write(&mut self, buf: &[u8]) {
        for b in buf {
            self.putc(*b);
        }
    }

    #[inline]
    pub fn write_str(&mut self, s: &str) {
        self.write(s.as_bytes());
    }

    /// Flush this output stream, blocking until all intermediately buffered contents reach their
    /// destination.
    #[inline]
    pub fn flush(&mut self) {
        // Wait for hardware to report completion
        #[cfg(feature = "asic")]
        while !self.is_transmit_empty() {}
    }

    #[inline]
    pub fn putc(&mut self, c: u8) {
        // Wait for hardware to report completion
        #[cfg(feature = "asic")]
        while !self.is_transmit_empty() {}

        // Safety: UART_THR is 4-byte aligned
        unsafe { write_u8(BASE_ADDR + crate::mmap::UART_THR_OFFSET, c) };
    }

    #[inline]
    pub fn getc(&mut self) -> u8 {
        // Wait for data to become ready
        while unsafe { read_u8(UART0_ADDR + crate::mmap::UART_DATA_READY_OFFSET) } & 1 == 0 {}

        // SAFETY: UART0_ADDR is 4-byte aligned
        unsafe { read_u8(UART0_ADDR) }
    }

    #[cfg(feature = "alloc")]
    pub fn read_to_heap(&mut self, bytes: usize) -> Vec<u8> {
        let mut result = Vec::with_capacity(bytes);
        for _ in 0..bytes {
            result.push(self.getc())
        }
        result
    }
}
