#![no_std]
#![no_main]

use core::ptr;

use headsail_bsp::{
    pac,
    rt::entry,
    sysctrl::{soc_ctrl, udma::Udma},
    ufmt,
};
use hello_sysctrl::{print_example_name, sysctrl_print};

const HPC_BASE_ADDR: usize = 0xFFE00000;
const BOOTRAM_OFFSET: usize = 0x10000;
const HPC_BOOTRAM_ADDR: usize = HPC_BASE_ADDR + BOOTRAM_OFFSET;

struct UdmaUart;

impl ufmt::uWrite for UdmaUart {
    type Error = core::convert::Infallible;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        sysctrl_print(s.as_bytes());
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

#[entry]
fn main() -> ! {
    // Enable interconnect, TLP and HPC
    let hpc_bit = 1 << 2;
    let icn_bit = 1 << 5;
    let tlp_bit = 1 << 8;
    soc_ctrl::ss_enable(hpc_bit | icn_bit | tlp_bit);

    // Configure HPC clocks
    soc_ctrl::clk1_mask(0b1001 << 16);

    // Configure ICN clocks
    let conf_val = 0b1001 << 8;
    soc_ctrl::clk2_mask(conf_val);

    // Configure TLP clocks
    let conf_val = 0b1001;
    soc_ctrl::clk3_mask(conf_val);

    soc_ctrl::periph_clk_div_set(0);

    let sysctrl = unsafe { pac::Sysctrl::steal() };
    let udma = Udma(sysctrl.udma());

    // Set the bit length, enable TX, set clk_div
    let (soc_freq, baud) = (30_000_000, 9600_u32);
    let clk_div: u16 = (soc_freq / baud) as u16;
    let _uart = udma.split().uart.enable(|w| {
        unsafe {
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
        }
    });

    print_example_name!();

    for i in 0..5 {
        let addr = HPC_BOOTRAM_ADDR + i * 4;
        sprint!("Writing 0x6f into {:#x}...", addr);
        unsafe { ptr::write_volatile(addr as *mut _, 0x6f) };
        sprintln!(" done");
    }

    let hpc_core_en = 0xf;
    sprint!(
        "Enabling core clock(s) for HPC using pattern: {:#x}...",
        hpc_core_en,
    );

    // Turn on all HPC cores
    soc_ctrl::clk1_mask(0x1111 << 20);

    sprintln!(" done");

    loop {
        continue;
    }
}
