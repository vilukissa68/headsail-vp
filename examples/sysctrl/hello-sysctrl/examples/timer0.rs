//! Print using PAC-based register manipulation only.
//!
//! Tested working on ASIC: 2024-08-23
#![no_std]
#![no_main]

use core::arch::asm;

use headsail_bsp::{self as bsp, pac::Sysctrl, rt::entry, sysctrl::soc_ctrl, ufmt};

struct UdmaUart;

impl ufmt::uWrite for UdmaUart {
    type Error = core::convert::Infallible;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        print(s.as_bytes());
        Ok(())
    }
}

#[macro_export]
macro_rules! sprint {
    ($s:expr) => {{
        ufmt::uwrite!(UdmaUart, $s).unwrap()
    }};
    ($($tt:tt)*) => {{
        ufmt::uwrite!(UdmaUart, $($tt)*).unwrap()
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

fn print(buf: &[u8]) {
    let sysctrl = Sysctrl::ptr();
    let udma = unsafe { (*sysctrl).udma() };

    udma.uart_tx_saddr()
        .write(|w| unsafe { w.bits(buf.as_ptr() as u32) });
    udma.uart_tx_size()
        .write(|w| unsafe { w.bits(buf.len() as u32) });

    // (3) Dispatch transmission
    udma.uart_tx_cfg().write(
        |w| w.en().set_bit(), // If we want "continuous mode". In continuous mode, uDMA reloads the address and transmits it again
                              //.continous().set_bit()
    );

    // (4) Poll until finished
    while udma.uart_tx_saddr().read().bits() != 0 {}
}

#[entry]
fn main() -> ! {
    // Enable interconnect and TLP
    let icn_bit = 1 << 5;
    let tlp_bit = 1 << 8;
    soc_ctrl::ss_enable(icn_bit | tlp_bit);

    // Configure ICN clocks
    let conf_val = 0b1001 << 8;
    soc_ctrl::clk2_set(conf_val);

    // Configure TLP clocks
    let conf_val = 0b1001;
    soc_ctrl::clk3_set(conf_val);

    let (soc_freq, baud) = (30_000_000, 9600_u32);

    soc_ctrl::periph_clk_div_set(0);

    let sysctrl = Sysctrl::ptr();
    let udma = unsafe { (*sysctrl).udma() };
    // Enable UART clock pass-through at uDMA
    udma.ctrl_cfg_cg().modify(|_r, w| w.cg_uart().set_bit());

    // Reset configuration register prior to setting it up, this must be
    // done to allow new configurations to take effect.
    udma.uart_setup().write(|w| unsafe { w.bits(0) });

    // Set the bit length, enable TX, set clk_div
    let clk_div: u16 = (soc_freq / baud) as u16;
    udma.uart_setup().write(|w| unsafe {
        w
            // Use this if using parity bit
            .parity_ena()
            .bit(false)
            .bit_length()
            .bits(0b11)
            // Stop bit?
            .stop_bits()
            .bit(false)
            .tx_ena()
            .bit(true)
            .rx_ena()
            .bit(true)
            .clkdiv()
            .bits(clk_div)
    });

    sprintln!("Hello, timer");

    type T0 = bsp::timer::ApbTimer<0xFFF50000>;

    sprint!("get_count... ");
    let start = T0::get_count();
    sprintln!("done ({})", start);

    sprint!("enable... ");
    T0::enable();
    sprintln!("done");

    sprint!("get_count... ");
    let start = T0::get_count();
    sprintln!("done");

    for _ in 0..10_000 {
        unsafe { asm!("nop") }
    }

    let end = T0::get_count();
    let diff = end - start;
    sprintln!("Start {}, end {}, diff {}", start, end, diff);

    loop {
        continue;
    }
}
