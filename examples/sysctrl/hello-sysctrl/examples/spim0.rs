//! Test SPIM0 using HAL
//!
//! | Date              | Status     | Changes   |
//! | :-                | :-:        | :-        |
//! | 2024-10-25        | Tested     |           |
#![no_std]
#![no_main]

use core::arch::asm;
use headsail_bsp::apb_uart::ApbUart0;
use headsail_bsp::{pac::Sysctrl, rt::entry, sysctrl::udma::Udma};

#[entry]
fn main() -> ! {
    let sysctrl = unsafe { Sysctrl::steal() };
    let udma = Udma(sysctrl.udma());

    // Split uDMA into sub-drivers for each peripheral
    let udma_periphs = udma.split();

    let (soc_freq, baud) = (30_000_000, 115_200);
    let mut uart = ApbUart0::init(soc_freq, baud);

    let mut spim = udma_periphs.spim.enable();
    uart.write_str("SPI enabled!\n\r");

    let tx_data: [u8; 8] = [0x01, 0x42, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    let rx_data: [u8; 8] = [0; 8];

    spim.write_sot();

    // Send 8 bytes
    spim.send_data(&tx_data);
    spim.write_eot_keep_cs();
    uart.write_str("Data sent!\n\r");

    for _ in 0..10_000 {
        unsafe { asm!("nop") }
    }

    // Receive 8 bytes
    spim.receive(&rx_data);
    spim.write_eot();
    uart.write_str("Data received!\n\r");

    loop {
        continue;
    }
}
