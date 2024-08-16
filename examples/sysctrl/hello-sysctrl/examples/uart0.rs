#![no_std]
#![no_main]

#[cfg(feature = "asic")]
use headsail_bsp::uart::init_uart;
use headsail_bsp::{rt::entry, sysctrl::soc_ctrl, uart::uart_write};
use panic_halt as _;

#[entry]
fn main() -> ! {
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

    #[cfg(feature = "asic")]
    init_uart(30_000_000, 9600);
    uart_write("Hello world!");

    loop {}
}
