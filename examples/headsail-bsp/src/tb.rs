//! Common testbench definitions to retain a consistent test setup

use crate::apb_uart::uart_write;

pub const TAG_FAIL: &str = "[FAIL]";
pub const TAG_PASS: &str = "[PASS]";

/// Tag signifies partial success in test, i.e. part of the test succeeded, but doesn't implicate that whole test has been either succesful/[PASS] or unsuccesful/[FAIL].
pub const TAG_OK: &str = "[OK]";

pub fn report_pass() {
    uart_write(TAG_PASS);
}

pub fn report_fail() {
    uart_write(TAG_FAIL);
}

pub fn report_ok() {
    uart_write(TAG_OK);
}
