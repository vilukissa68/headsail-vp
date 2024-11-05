//! Test SPIM0 using HAL
//!
//! | Date              | Status     | Changes   |
//! | :-                | :-:        | :-        |
//! | 2024-10-25        | Tested     |           |
#![no_std]
#![no_main]

use core::arch::asm;
use headsail_bsp::{
    pac::Sysctrl,
    rt::entry,
    sysctrl::{soc_ctrl, udma::Udma},
    ufmt,
};
use hello_sysctrl::{print_example_name, sprintln};

#[entry]
fn main() -> ! {
    // These lines are necessary to initialize uDMA UART prints for sprint-macro
    soc_ctrl::periph_clk_div_set(0);
    hello_sysctrl::UdmaUart::init();
    print_example_name!();

    let sysctrl = unsafe { Sysctrl::steal() };
    let udma = Udma(sysctrl.udma());

    // Split uDMA into sub-drivers for each peripheral
    let udma_periphs = udma.split();

    let mut spim = udma_periphs.spim.enable();
    sprintln!("SPI enabled!");

    let tx_data: [u8; 8] = [0x01, 0x42, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    let rx_data: [u8; 8] = [0; 8];

    spim.write_sot();

    // Send 8 bytes
    spim.send_data(&tx_data);
    spim.write_eot_keep_cs();
    sprintln!("Data sent!\n\r");

    for _ in 0..10_000 {
        unsafe { asm!("nop") }
    }

    // Receive 8 bytes
    spim.receive_data(&rx_data);
    spim.write_eot();
    sprintln!("Data received!\n\r");

    loop {
        continue;
    }
}
