#![no_std]
#![no_main]

extern crate alloc;
use headsail_bsp::{apb_uart::uart_read_to_heap, init_alloc, rt::entry, sprint, sprintln};

#[entry]
fn main() -> ! {
    sprintln!("Connect to APB UART 0 with: screen /tmp/uart0");
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
