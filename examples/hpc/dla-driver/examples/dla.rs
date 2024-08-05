#![no_std]
#![no_main]

use dla_driver::*;
use headsail_bsp::{rt::entry, sprintln};
use panic_halt as _;

#[entry]
fn main() -> ! {
    sprintln!("Hello world!");
    let _dla = Dla::new();
    sprintln!("Dla initalized");
    loop {}
}
