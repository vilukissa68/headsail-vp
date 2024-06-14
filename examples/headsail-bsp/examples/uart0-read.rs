#![no_std]
#![no_main]

extern crate alloc;
use headsail_bsp::{init_alloc, rt::entry, sprint, sprintln, uart::uart_read_to_heap};

#[entry]
fn main() -> ! {
    sprintln!("Connect to uart with: screen /tmp/uart0");
    init_alloc();
    loop {
        let res = uart_read_to_heap(8);
        for x in res {
            if x != 0 {
                sprint!("{:x} ", x)
            }
        }
    }
}
