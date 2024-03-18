#![no_std]
#![no_main]

use hello_hpc::{dla_read, dla_write, uart_write};
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    uart_write("Hello world!\r\n");
    dla_write("Hello DLA");
    let mut buf: [u8; 9] = [0; 9];
    dla_read(&mut buf, 9, 0);
    loop {}
}
