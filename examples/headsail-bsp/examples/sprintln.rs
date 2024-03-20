#![no_std]
#![no_main]

use headsail_bsp::{rt::entry, sprintln};

#[entry]
fn main() -> ! {
    sprintln!("Hello sprintln!");
    loop {}
}
