//! Blinks a LED
//!
//! Tested working on ASIC: 2024-08-23
#![no_std]
#![no_main]

use core::ptr;
use headsail_bsp::rt::entry;

// Below addresses are in SysCtrl memory space
const GPIO: usize = 0x1a10_1000;
const GPIO_DIR: usize = GPIO;
const GPIO_OUT: usize = GPIO + 0xc;
const SOC_CONTROL: usize = 0x1a10_4000;
const PADMUX0: usize = SOC_CONTROL + 0x10;

// Number of nops SysCtrl is capable of executing at 30 MHz reference clocks
const NOPS_PER_SEC: usize = match () {
    #[cfg(debug_assertions)]
    // This is an experimentally found value
    () => 2_000_000 / 9,
    #[cfg(not(debug_assertions))]
    // This is just a guess for now (10x debug)
    () => 200_000 / 9,
};

#[entry]
fn main() -> ! {
    unsafe {
        ptr::write_volatile(PADMUX0 as *mut _, 0);
        ptr::write_volatile(GPIO_DIR as *mut _, 0);

        // Padmux enable GPIO9
        ptr::write_volatile(PADMUX0 as *mut _, 0x40000);

        // Set GPIO 9 as output
        ptr::write_volatile(GPIO_DIR as *mut _, 1 << 9);
    }

    loop {
        unsafe {
            // Toggle GPIO
            let mut r = ptr::read_volatile(GPIO_OUT as *mut u32);
            r ^= 1 << 9;
            ptr::write_volatile(GPIO_OUT as *mut u32, r);

            // 1 second period
            for _ in 0..NOPS_PER_SEC {
                core::arch::asm!("nop");
            }
        }
    }
}
