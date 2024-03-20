#![no_std]
#![no_main]

use headsail_bsp::{rt::entry, uart::uart_write};

#[entry]
fn main() -> ! {
    uart_write("Hello world!");
    loop {}
}
