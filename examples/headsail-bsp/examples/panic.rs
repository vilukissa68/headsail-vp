#![no_std]
#![no_main]

use headsail_bsp::rt::entry;

#[entry]
fn main() -> ! {
    panic!("panic");
}
