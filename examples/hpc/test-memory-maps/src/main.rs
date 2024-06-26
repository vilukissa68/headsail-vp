#![no_std]
#![no_main]

mod tests;

use bsp::{
    riscv::asm::wfi,
    rt::entry,
    sprint, sprintln,
    tb::{report_fail, report_pass},
    uart::{putc, uart_write},
    ufmt::{self, uDebug},
};
use panic_halt as _;

struct Error(tests::Error);

impl uDebug for Error {
    fn fmt<W>(&self, f: &mut bsp::ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: bsp::ufmt::uWrite + ?Sized,
    {
        match self.0 {
            tests::Error::ReadValueIsNotResetValue {
                read_val,
                reset_val,
                reg_uid,
                reg_addr,
            } => {
                bsp::ufmt::uwrite!(
                    f,
                    "read value is not reset value: {} != {}, register: {}@{}",
                    read_val,
                    reset_val,
                    reg_uid,
                    reg_addr
                )
            }
        }
    }
}

fn print_test_case_info(idx: usize, count: usize, longest_uid: usize, uid: &str, addr: usize) {
    let spacing = longest_uid - uid.chars().count() + 1;
    sprint!("[{}/{}] Testing {}", idx + 1, count, uid);
    for _ in 0..spacing {
        putc(b' ');
    }
    sprintln!("@0x{:x}", addr);
}

#[entry]
fn main() -> ! {
    sprintln!("[{}]", core::env!("CARGO_CRATE_NAME"));

    let cases = &tests::TEST_CASES;
    let longest_uid = cases.iter().map(|t| t.uid.chars().count()).max().unwrap();
    let count = cases.len();
    for (idx, t) in cases.iter().enumerate() {
        print_test_case_info(idx, count, longest_uid, t.uid, t.addr);
        if let Err(e) = (t.function)() {
            sprintln!("  {:?}", Error(e));
            // Report failure but run to completion
            report_fail();
        } else {
            sprintln!("  reset value ok")
        }
    }

    report_pass();

    loop {
        wfi();
    }
}
