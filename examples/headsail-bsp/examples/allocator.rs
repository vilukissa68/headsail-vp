#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec;
use headsail_bsp::{allocator, rt::entry, sprintln};

#[entry]
fn main() -> ! {
    sprintln!("Hello sprintln!");
    unsafe { allocator::init_heap() };
    let v = vec![1, 2, 3];
    for x in v {
        sprintln!("{:?}", x);
    }
    loop {}
}
