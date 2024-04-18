#![no_std]
#![no_main]

use headsail_bsp::{rt::entry, sprintln};
use dla_driver::*;
use panic_halt as _;

#[entry]
fn main() -> ! {
    sprintln!("Hello world!");
    dla_write_str("Hello DLA");
    dla_init();
    sprintln!("Dla initalized");
    loop {}
}
