#![no_std]
#![no_main]

use headsail_bsp::{apb_uart::uart_write, rt::entry};

#[entry]
fn main() -> ! {
    uart_write("Hello world!");
    loop {}
}
