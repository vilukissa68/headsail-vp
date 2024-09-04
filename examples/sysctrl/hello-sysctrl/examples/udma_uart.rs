//! Prints over SysCtrl UART
#![no_std]
#![no_main]

use core::arch::asm;

use headsail_bsp::{
    pac,
    rt::entry,
    sysctrl::{soc_ctrl, udma::Udma},
};
use hello_sysctrl::NOPS_PER_SEC;

#[entry]
fn main() -> ! {
    let sysctrl = unsafe { pac::Sysctrl::steal() };
    let udma = Udma(sysctrl.udma());

    soc_ctrl::periph_clk_div_set(0);

    // Set the bit length, enable TX, set clk_div
    let (soc_freq, baud) = (30_000_000, 9600_u32);
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

    loop {
        uart.write(b"Hello uDMA UART HAL\r\n");

        for _ in 0..NOPS_PER_SEC {
            unsafe { asm!("nop") };
        }
    }
}
