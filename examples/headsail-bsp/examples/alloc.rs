#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;
use headsail_bsp::{init_alloc, rt::entry, sprintln};

const HEAP_START: usize = 0x1_3000_0000;
const HEAP_SIZE: usize = 0x1000_0000;

#[entry]
fn main() -> ! {
    sprintln!("Hello sprintln!");
    init_alloc(HEAP_START, HEAP_SIZE);
    let address: usize = 0x1000_0000;
    let s = unsafe { core::ptr::read_volatile(address as *const u32) };
    sprintln!("Illeageal {:?}", s);

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
