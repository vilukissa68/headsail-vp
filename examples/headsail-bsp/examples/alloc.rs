#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec;
use headsail_bsp::{init_alloc, rt::entry, sprintln};

#[entry]
fn main() -> ! {
    sprintln!("Hello sprintln!");
    init_alloc();
    let v = vec![1, 2, 3];
    for x in v {
        sprintln!("{:?}", x);
    }
    loop {}
}
