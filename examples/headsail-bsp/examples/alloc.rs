#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use headsail_bsp::{init_alloc, rt::entry, sprintln};

#[entry]
fn main() -> ! {
    sprintln!("Hello sprintln!");
    init_alloc();
    let mut v = Vec::new();
    for x in 0..200 {
        sprintln!("Pushing {:?}", x);
        v.push(x)
    }
    for x in &v {
        sprintln!("Reading {:?}", x);
    }
    sprintln!("len: {}", v.len());
    loop {}
}
