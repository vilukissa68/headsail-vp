#![no_std]
#![no_main]

use core::ptr;

use headsail_bsp::{
    pac,
    rt::entry,
    sdram,
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
    let sdram_bit = 1 << 3;
    soc_ctrl::ss_enable(hpc_bit | icn_bit | tlp_bit | sdram_bit);

    // Configure HPC and SDRAM clocks
    soc_ctrl::clk1_mask(0b1001 << 16 | 0b1001 << 24);

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

    // Enable SDRAM
    let ddr_mode = 0b1;
    let axi_enable = 0b1 << 1;
    sprint!("Enabling SDRAM ddr_mode={:#x}, axi_enable={:#x}...",
            ddr_mode,
            axi_enable,
    );
    sdram::sdram_cfg_axi_ddr_mode_mask(ddr_mode);
    sdram::sdram_cfg_axi_enable_mask(axi_enable);
    sprintln!(" done");

    for i in 0..5 {
        let addr = HPC_BOOTRAM_ADDR + i * 4;
        sprint!("Writing 0x6f into {:#x}...", addr);
        unsafe { ptr::write_volatile(addr as *mut _, 0x6f) };
        sprintln!(" done");
    }

    // Configure execute regions for SDRAM
    let execute_region_pattern_2 = 0x7000_0000;
    sprint!("Configuring execute regions for SDRAM using pattern: {:#x}...",
            execute_region_pattern_2,
    );

    let hpc = unsafe { pac::Hpc::steal()};
    hpc.cluster_config()
        .execute_region_length2()
        .write(|w| unsafe {w.bits(execute_region_pattern_2)});
    sprintln!(" done");

    // Configure execute regions for C2C
    let execute_region_pattern_3 = 0x2000_0000;
    sprint!("Configuring execute regions for C2C using pattern: {:#x}...",
            execute_region_pattern_3,
    );

    let hpc = unsafe { pac::Hpc::steal()};
    hpc.cluster_config()
        .execute_region_length3()
        .write(|w| unsafe {w.bits(execute_region_pattern_3)});
    sprintln!(" done");

    // Configure execute regions for SRAM
    let execute_region_pattern_4 = 0x10_0000;
    sprint!("Configuring execute regions for SRAM using pattern: {:#x}...",
            execute_region_pattern_4,
    );

    let hpc = unsafe { pac::Hpc::steal()};
    hpc.cluster_config()
        .execute_region_length4()
        .write(|w| unsafe {w.bits(execute_region_pattern_4)});
    sprintln!(" done");


    // Turn on HPC core #0
    let hpc_core_en = 0x1;
    sprint!(
        "Enabling core clock(s) for HPC using pattern: {:#x}...",
        hpc_core_en,
    );
    soc_ctrl::clk1_mask(0b1 << 20);
    sprintln!(" done");

    loop {
        continue;
    }
}
