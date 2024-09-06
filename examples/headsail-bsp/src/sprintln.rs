//! Macros to implement Rust-style print formatting using `sprint`/`sprintln`
pub const UART: crate::apb_uart::ApbUart0 = unsafe { crate::apb_uart::ApbUart0::instance() };

#[macro_export]
macro_rules! sprint {
    ($s:expr) => {{
        use $crate::{sprintln, ufmt};
        ufmt::uwrite!(sprintln::UART, $s).unwrap()
    }};
    ($($tt:tt)*) => {{
        use $crate::{sprintln, ufmt};
        ufmt::uwrite!(sprintln::UART, $($tt)*).unwrap()
    }};
}

#[macro_export]
macro_rules! sprintln {
    () => {{
        use $crate::sprint;
        sprint!("\r\n");
    }};
    // IMPORTANT use `tt` fragments instead of `expr` fragments (i.e. `$($exprs:expr),*`)
    ($($tt:tt)*) => {{
        use $crate::sprint;
        sprint!($($tt)*);
        sprint!("\r\n");
    }};
}
