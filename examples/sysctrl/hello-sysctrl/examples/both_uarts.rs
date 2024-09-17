#![no_std]
#![no_main]

use core::arch::asm;

use headsail_bsp::{mask_u32, mmap, read_u8, rt::entry, sysctrl, unmask_u32, write_u32, write_u8};
use headsail_bsp::{
    pac,
    sysctrl::{soc_ctrl, udma::Udma},
};

const PAD_CONF_UART0_TX: usize = 0xfff0_7064;

#[entry]
fn main() -> ! {
    use sysctrl::mmap::*;

    // Set peripheral clock divider to 1
    let valid_bit = 0x400;
    write_u32(PERIPH_CLK_DIV, valid_bit);

    // Enable interconnect and TLP
    let icn_bit = 1 << 5;
    let tlp_bit = 1 << 8;
    let hpc_bit = 1 << 2;
    write_u32(SS_RESET_EN, icn_bit | tlp_bit | hpc_bit);

    // Configure HPC clocks (enable core 0 at 5th bit)
    let conf_val = 0b1_1001 << 16;
    write_u32(SS_CLK_CTRL1, conf_val);

    // Configure ICN clocks
    let conf_val = 0b1001 << 8;
    write_u32(SS_CLK_CTRL2, conf_val);

    // Configure TLP clocks
    let conf_val = 0b1001;
    write_u32(SS_CLK_CTRL3, conf_val);

    write_u32(PAD_CONF_UART0_TX, 4);

    let (soc_freq, baud) = (30_000_000, 9600);

    let sysctrl = unsafe { pac::Sysctrl::steal() };
    let udma = Udma(sysctrl.udma());

    soc_ctrl::periph_clk_div_set(0);

    // Set the bit length, enable TX, set clk_div
    let clk_div: u16 = (soc_freq / baud) as u16;
    let mut uart = udma.split().uart.enable(|w| {
        unsafe {
            w
                // Use this if using parity bit
                .parity_ena()
                .bit(false)
                .bit_length()
                .bits(0b11)
                // Stop bit?
                .stop_bits()
                .bit(false)
                .tx_ena()
                .bit(true)
                .rx_ena()
                .bit(true)
                .clkdiv()
                .bits(clk_div)
        }
    });

    uart_init(soc_freq, baud);

    loop {
        uart.write(b"SysCtrl uDMA UART\r\n");
        tlp_uart_write(b"TLP\r\n");

        for _ in 0..40_000 {
            unsafe { asm!("nop") };
        }
    }
}

fn uart_init(soc_freq: u32, baud: u32) {
    use mmap::*;

    const PERIPH_CLK_DIV: u32 = 1;
    let divisor: u32 = soc_freq / PERIPH_CLK_DIV / (baud << 4);

    const LCR_FIFO_EN_BIT: u8 = 0b1;
    const LCR_NR_STOP_BITS_BIT: u8 = 0b1 << 1;
    /// Divisor latch access bit
    const LCR_DLAB_BIT: u8 = 0b1 << 7;

    const FCR_FIFO_EN_BIT: u8 = 0b1;
    const FCR_FIFO_RX_RESET_BIT: u8 = 0b1 << 1;
    const FCR_FIFO_TX_RESET_BIT: u8 = 0b1 << 2;
    const FCR_TRIG_RX_LSB_BIT: u8 = 0b1 << 6;
    const FCR_TRIG_RX_MSB_BIT: u8 = 0b1 << 7;

    // Enable DLAB (to set baud rate divisor)
    mask_u32(UART0_ADDR + UART_LCR_OFS, LCR_DLAB_BIT as u32);
    // Set low byte of divisor
    write_u32(UART0_ADDR + UART_RBR_THR_DLL_OFS, divisor);
    // Set high byte of divisor
    write_u32(UART0_ADDR + UART_IER_DLM_OFS, divisor >> 8);
    // 8 bits, no parity, one stop bit
    write_u32(
        UART0_ADDR + UART_LCR_OFS,
        (LCR_FIFO_EN_BIT | LCR_NR_STOP_BITS_BIT) as u32,
    );
    // Disable DLAB
    unmask_u32(UART0_ADDR + UART_LCR_OFS, LCR_DLAB_BIT as u32);

    // Enable FIFO, clear them, with 14-byte threshold
    write_u32(
        UART0_ADDR + UART_IIR_FCR_OFS,
        (FCR_FIFO_EN_BIT
            | FCR_FIFO_RX_RESET_BIT
            | FCR_FIFO_TX_RESET_BIT
            | FCR_TRIG_RX_LSB_BIT
            | FCR_TRIG_RX_MSB_BIT) as u32,
    );
}

fn is_transmit_empty() -> bool {
    use mmap::*;
    // Safety: UART_LSR is 4-byte aligned
    const LSR_FIFO_IS_EMPTY_BIT: u8 = 0b1 << 5;
    (unsafe { read_u8(UART0_ADDR + UART_LSR_OFS) } & LSR_FIFO_IS_EMPTY_BIT) != 0
}

fn putc(c: u8) {
    while !is_transmit_empty() {}

    // Wait some extra for extra traceability in this test case
    for _ in 0..1_000 {
        unsafe { asm!("nop") };
    }

    // Safety: UART_RBR_THR_DLL is 4-byte aligned
    unsafe { write_u8(mmap::UART0_ADDR + mmap::UART_RBR_THR_DLL_OFS, c) };
}

fn tlp_uart_write(buf: &[u8]) {
    for b in buf {
        putc(*b);
    }
}
