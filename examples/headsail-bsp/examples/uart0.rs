#![no_std]
#![no_main]

use headsail_bsp::{apb_uart::ApbUart0, rt::entry};

#[entry]
fn main() -> ! {
    let (soc_freq, baud) = (30_000_000, 115_200);
    let mut uart = ApbUart0::init(soc_freq, baud);
    uart.write_str("Hello world!");
    loop {}
}
