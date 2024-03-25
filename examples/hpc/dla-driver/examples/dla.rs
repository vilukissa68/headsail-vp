#![no_std]
#![no_main]

use headsail_bsp::{rt::entry, sprintln};
use hello_dla::*;
use panic_halt as _;

#[entry]
fn main() -> ! {
    sprintln!("Hello world!");
    dla_write_str("Hello DLA");
    dla_init();
    sprintln!("Dla initalized");
    loop {}
}
