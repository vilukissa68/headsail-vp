#![no_std]
#![no_main]

use headsail_bsp::{rt::entry, sprintln};
use hello_hpc::print_example_name;
use panic_halt as _;

#[entry]
fn main() -> ! {
    print_example_name!();
    sprintln!("Hello sprintln!");
    loop {}
}
