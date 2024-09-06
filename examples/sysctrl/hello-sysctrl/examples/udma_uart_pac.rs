//! Print using PAC-based register manipulation only.
//!
//! Tested working on ASIC: 2024-08-23
#![no_std]
#![no_main]

use headsail_bsp::{pac::Sysctrl, rt::entry, sysctrl::soc_ctrl};

#[entry]
fn main() -> ! {
    let (soc_freq, baud) = (30_000_000, 9600_u32);

    soc_ctrl::periph_clk_div_set(0);

    let sysctrl = Sysctrl::ptr();

    let udma = unsafe { (*sysctrl).udma() };
    // Enable UART clock pass-through at uDMA
    udma.ctrl_cfg_cg().modify(|_r, w| w.cg_uart().set_bit());

    // Reset configuration register prior to setting it up, this must be
    // done to allow new configurations to take effect.
    udma.uart_setup().write(|w| unsafe { w.bits(0) });

    // Set the bit length, enable TX, set clk_div
    let clk_div: u16 = (soc_freq / baud) as u16;
    udma.uart_setup().write(|w| unsafe {
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
    });

    let s = "[ok]\r\n";
    udma.uart_tx_saddr()
        .write(|w| unsafe { w.bits(s.as_ptr() as u32) });
    udma.uart_tx_size()
        .write(|w| unsafe { w.bits(s.len() as u32) });

    // (3) Dispatch transmission
    udma.uart_tx_cfg().write(
        |w| w.en().set_bit(), // If we want "continuous mode". In continuous mode, uDMA reloads the address and transmits it again
                              //.continous().set_bit()
    );

    // (4) Poll until finished
    while udma.uart_tx_saddr().read().bits() != 0 {}

    loop {
        unsafe { core::arch::asm!("wfi") };
    }
}
