#![no_std]
#![no_main]

extern crate alloc;
use headsail_bsp::{init_alloc, rt::entry, sprint, sprintln, uart::uart_read_heap};

const HEAP_START: usize = 0x1_3000_0000;
const HEAP_SIZE: usize = 0x1000_0000;

#[entry]
fn main() -> ! {
    sprintln!("Connect to uart with: screen /tmp/uart0");
    init_alloc(HEAP_START, HEAP_SIZE);
    loop {
        let res = uart_read_heap(8);
        for x in res {
            if x != 0 {
                sprint!("{} ", x)
            }
        }
    }
    loop {}
}
