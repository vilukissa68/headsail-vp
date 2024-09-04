//! Print over TLP UART (PULP APB UART) #0 using HAL
//!
//! | Date              | Status     | Changes   |
//! | :-                | :-:        | :-        |
//! | 2024-09-04        | *Untested* |           |
#![no_std]
#![no_main]

use core::arch::asm;

use headsail_bsp::{apb_uart::ApbUart, rt::entry, sysctrl::soc_ctrl, unmask_u32};
use hello_sysctrl::{print_example_name, NOPS_PER_SEC};

#[entry]
fn main() -> ! {
    // Set peripheral clock divider to 1
    soc_ctrl::periph_clk_div_set(0);

    // Enable interconnect and TLP
    let icn_bit = 1 << 5;
    let tlp_bit = 1 << 8;
    soc_ctrl::ss_enable(icn_bit | tlp_bit);

    // Configure ICN clocks
    let conf_val = 0b1001 << 8;
    soc_ctrl::clk2_set(conf_val);

    // Configure TLP clocks
    let conf_val = 0b1001;
    soc_ctrl::clk3_set(conf_val);

    // Disable GPIO behavior for UART pins
    const PAD_CONF_UART0_TX: usize = 0xfff0_7064;
    unmask_u32(PAD_CONF_UART0_TX, (0b1 << 5) | (0b1 << 10));

    let (soc_freq, baud) = (30_000_000, 9600);
    let mut uart = ApbUart::<0xFFF00000>::init(soc_freq, baud);

    print_example_name!();
    loop {
        uart.write(b"Hello TLP UART #0!\r\n");

        for _ in 0..NOPS_PER_SEC {
            unsafe { asm!("nop") };
        }
    }
}
