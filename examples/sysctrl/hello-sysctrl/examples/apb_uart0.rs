#![no_std]
#![no_main]

use headsail_bsp::{apb_uart::ApbUart, rt::entry, sysctrl::soc_ctrl};

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

    let (soc_freq, baud) = (30_000_000, 9600);
    let mut uart = ApbUart::<0xFFF00000>::init(soc_freq, baud);
    uart.write(b"Hello world!");

    loop {}
}
