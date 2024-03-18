#![no_std]
#![no_main]

use hello_hpc::{uart_write, dla_write, dla_read};
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    uart_write("Hello world!");
    dla_write("Hello DLA");
    let mut buf: [u8; 9] = [0; 9];
    dla_read(&mut buf, 9, 0);
    loop {}
}
