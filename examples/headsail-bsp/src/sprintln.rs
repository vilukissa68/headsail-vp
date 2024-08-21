//! Macros to implement Rust-style print formatting using `print`/`println`
use crate::apb_uart::ApbUart0;
pub const UART: ApbUart0 = unsafe { ApbUart0::instance() };

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

impl ufmt::uWrite for ApbUart0 {
    type Error = core::convert::Infallible;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        self.write_str(s);
        Ok(())
    }
}
