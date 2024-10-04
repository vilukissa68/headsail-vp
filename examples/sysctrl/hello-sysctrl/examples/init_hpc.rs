#![no_std]
#![no_main]

use core::ptr;

use headsail_bsp::{pac, rt::entry, sdram, sysctrl::soc_ctrl, ufmt};
use hello_sysctrl::{print_example_name, sprint, sprintln};

const HPC_BASE_ADDR: usize = 0xFFE00000;
const BOOTRAM_OFFSET: usize = 0x10000;
const HPC_BOOTRAM_ADDR: usize = HPC_BASE_ADDR + BOOTRAM_OFFSET;
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

    // This line is necessary to initialize uDMA UART prints for sprint-macro
    hello_sysctrl::UdmaUart::init();

    print_example_name!();

    // Enable SDRAM
    let ddr_mode = 0b1;
    let axi_enable = 0b1 << 1;
    sprint!(
        "Enabling SDRAM ddr_mode={:#x}, axi_enable={:#x}...",
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
    sprint!(
        "Configuring execute regions for SDRAM using pattern: {:#x}...",
        execute_region_pattern_2,
    );

    let hpc = unsafe { pac::Hpc::steal() };
    hpc.cluster_config()
        .execute_region_length2()
        .write(|w| unsafe { w.bits(execute_region_pattern_2) });
    sprintln!(" done");

    // Configure execute regions for C2C
    let execute_region_pattern_3 = 0x2000_0000;
    sprint!(
        "Configuring execute regions for C2C using pattern: {:#x}...",
        execute_region_pattern_3,
    );

    let hpc = unsafe { pac::Hpc::steal() };
    hpc.cluster_config()
        .execute_region_length3()
        .write(|w| unsafe { w.bits(execute_region_pattern_3) });
    sprintln!(" done");

    // Configure execute regions for SRAM
    let execute_region_pattern_4 = 0x10_0000;
    sprint!(
        "Configuring execute regions for SRAM using pattern: {:#x}...",
        execute_region_pattern_4,
    );

    let hpc = unsafe { pac::Hpc::steal() };
    hpc.cluster_config()
        .execute_region_length4()
        .write(|w| unsafe { w.bits(execute_region_pattern_4) });
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
