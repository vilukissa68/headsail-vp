#![no_std]
#![no_main]

use headsail_bsp::{rt::entry, sprintln, timer::*};

#[entry]
fn main() -> ! {
    sprintln!("Timer0 example");
    let cnt_start = Timer0::get_count();
    sprintln!("Timer0 counter value at start: {}", cnt_start);
    sprintln!("Starting timer");
    Timer0::enable();
    sprintln!("Wasting time...");
    for _i in 1..1_000_000 {
        continue;
    }
    sprintln!("Stopping timer");
    Timer0::disable();
    let cnt_stop = Timer0::get_count();
    sprintln!("Timer0 counter value at stop: {}", cnt_stop);
    loop {}
}
