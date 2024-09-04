//! Print over TLP UART (PULP APB UART) #0 using MMIO only
//!
//! | Date              | Status     | Changes   |
//! | :-                | :-:        | :-        |
//! | 2024-09-04        | *Untested* |           |
#![no_std]
#![no_main]

use core::arch::asm;

use headsail_bsp::{
    mask_u8, mmap, read_u8, rt::entry, sysctrl, unmask_u32, unmask_u8, write_u32, write_u8,
};
use hello_sysctrl::{print_example_name, NOPS_PER_SEC};

#[entry]
fn main() -> ! {
    // Set peripheral clock divider to 1
    let valid_bit = 0x400;
    write_u32(sysctrl::mmap::PERIPH_CLK_DIV, valid_bit);

    // Enable interconnect and TLP
    let icn_bit = 1 << 5;
    let tlp_bit = 1 << 8;
    write_u32(sysctrl::mmap::SS_RESET_EN, icn_bit | tlp_bit);

    // Configure ICN clocks
    let conf_val = 0b1001 << 8;
    write_u32(sysctrl::mmap::SS_CLK_CTRL2, conf_val);

    // Configure TLP clocks
    let conf_val = 0b1001;
    write_u32(sysctrl::mmap::SS_CLK_CTRL3, conf_val);

    // Disable GPIO behavior for UART pins
    const PAD_CONF_UART0_TX: usize = 0xfff0_7064;
    unmask_u32(PAD_CONF_UART0_TX, (0b1 << 5) | (0b1 << 10));

    let (soc_freq, baud) = (30_000_000, 9600);
    uart_init(soc_freq, baud);

    print_example_name!();
    loop {
        uart_write(b"Hello TLP UART #0!\r\n");

        for _ in 0..NOPS_PER_SEC {
            unsafe { asm!("nop") };
        }
    }
}

fn uart_init(soc_freq: u32, baud: u32) {
    use mmap::*;

    const PERIPH_CLK_DIV: u32 = 1;
    let divisor: u32 = soc_freq / PERIPH_CLK_DIV / (baud << 4);

    // Safety: all PULP APB UART registers are 4-byte aligned so no bus can stop us
    unsafe {
        // Enable DLAB (to set baud rate divisor)
        mask_u8(UART0_ADDR + UART_LCR_OFS, UART_LCR_DLAB_BIT);
        // Set low byte of divisor
        write_u8(UART0_ADDR + UART_RBR_THR_DLL_OFS, divisor as u8);
        // Set high byte of divisor
        write_u8(UART0_ADDR + UART_IER_DLM_OFS, (divisor >> 8) as u8);
        // Data is 8 bits, one stop bit, no parity
        write_u8(UART0_ADDR + UART_LCR_OFS, 0b11);
        // Restore DLAB state
        unmask_u8(UART0_ADDR + UART_LCR_OFS, UART_LCR_DLAB_BIT);

        // Enable FIFO, clear RX & TX, use 14-byte threshold
        write_u8(
            UART0_ADDR + UART_IIR_FCR_OFS,
            UART_FCR_FIFO_EN_BIT
                | UART_FCR_FIFO_RX_RESET_BIT
                | UART_FCR_FIFO_TX_RESET_BIT
                | UART_FCR_TRIG_RX_LSB
                | UART_FCR_TRIG_RX_MSB,
        );
    }
}

fn uart_write(buf: &[u8]) {
    for b in buf {
        putc(*b);
    }
}

fn putc(c: u8) {
    while !is_transmit_empty() {}

    // Safety: UART_RBR_THR_DLL is 4-byte aligned
    unsafe { write_u8(mmap::UART0_ADDR + mmap::UART_RBR_THR_DLL_OFS, c) };
}

fn is_transmit_empty() -> bool {
    use mmap::*;

    // Safety: UART_LSR is 4-byte aligned
    const LSR_FIFO_IS_EMPTY_BIT: u8 = 0b1 << 5;
    (unsafe { read_u8(UART0_ADDR + UART_LSR_OFS) } & LSR_FIFO_IS_EMPTY_BIT) != 0
}
