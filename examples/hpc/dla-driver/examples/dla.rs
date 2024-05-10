#![no_std]
#![no_main]

use dla_driver::*;
use headsail_bsp::{rt::entry, sprintln};
use panic_halt as _;

#[entry]
fn main() -> ! {
    sprintln!("Hello world!");
    let mut dla = Dla::new();
    dla.init_layer();
    sprintln!("Dla initalized");
    loop {}
}
