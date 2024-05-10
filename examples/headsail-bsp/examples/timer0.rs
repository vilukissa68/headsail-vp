#![no_std]
#![no_main]

use headsail_bsp::{rt::entry, sprintln, timer_unit::*};

#[entry]
fn main() -> ! {
    sprintln!("Timer0 example");
    let cnt_start = timer0_get_count();
    sprintln!("Timer0 counter value at start: {}", cnt_start);
    sprintln!("Starting timer");
    timer0_enable();
    sprintln!("Wasting time...");
    for _i in 1..1_000_000 {
        continue;
    }
    sprintln!("Stopping timer");
    timer0_disable();
    let cnt_stop = timer0_get_count();
    sprintln!("Timer0 counter value at stop: {}", cnt_stop);
    loop {}
}
