#![no_std]
#![no_main]

use headsail_bsp::{rt::entry, uart::uart_write};
use hello_sysctrl::print_example_name;
use panic_halt as _;

#[entry]
fn main() -> ! {
    print_example_name!();
    uart_write("Hello world!");
    loop {}
}
