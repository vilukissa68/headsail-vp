#![no_std]
#![no_main]

use core::ptr;
use panic_halt as _;
use riscv_rt::entry;

const UART_ADDR: usize = 0xFFF00000;

#[entry]
fn main() -> ! {
    unsafe { ptr::write_volatile(UART_ADDR as *mut _, 'h' as u8) };

    loop {}
}
