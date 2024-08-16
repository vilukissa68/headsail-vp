//! Blinks a LED
//!
//! | Date              | Status    | Changes   |
//! | :-                | :-:       | :-        |
//! | 2024-08-15        | *Works*   |           |
//! | 2024-08-15T1530   | Untested  | Use HAL   |
#![no_std]
#![no_main]

use headsail_bsp::{rt::entry, sysctrl::soc_ctrl};
use hello_sysctrl::NOPS_PER_SEC;
use panic_halt as _;

#[entry]
fn main() -> ! {
    // Set pad9 as GPIO
    let pads = unsafe { soc_ctrl::Pads::steal() };
    let mut gpio9 = pads.p9.into_gpio().into_output();

    loop {
        gpio9.toggle();

        // Wait for 1 second to produce 0.5 Hz period
        for _ in 0..NOPS_PER_SEC {
            unsafe { core::arch::asm!("nop") };
        }
    }
}
