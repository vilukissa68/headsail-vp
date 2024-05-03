#![no_std]
#![no_main]

use headsail_bsp::{riscv, rt::entry, sprintln, CLINT};

#[entry]
fn main() -> ! {
    sprintln!("Hello main!");

    sprintln!("mtime: {:?}", CLINT::mtime().read());

    // Pend timer interrupt after 20000 mtime ticks
    CLINT::mtimecmp0().write(20000);

    // Pend software interrupt
    CLINT::msip0().pend();

    // Enable relevant interrupts
    unsafe {
        riscv::register::mie::set_msoft();
        riscv::register::mie::set_mtimer();
        riscv::interrupt::enable();
    };

    // Print timer value
    loop {
        for _ in 0..500_000 {
            unsafe { core::arch::asm!("nop") }
        }
        sprintln!("mtime: {:?}", CLINT::mtime().read());
    }
}

#[export_name = "MachineSoft"]
fn machine_soft() {
    CLINT::msip0().unpend();

    // do something here
    sprintln!("Hello MachineSoft!");
}

#[export_name = "MachineTimer"]
fn machine_timer() {
    unsafe { riscv::register::mie::clear_mtimer() };

    // do something here
    sprintln!("Hello MachineTimer!");
}
