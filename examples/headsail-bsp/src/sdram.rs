use crate::{mask_u32, mmap::SDRAM_CONFIG_ADDR, read_u32};

pub const SDRAM_CFG_DDR_INIT_DONE_ADDR: usize = SDRAM_CONFIG_ADDR + 0x04;
pub const SDRAM_CFG_AXI_DDR_MODE_ADDR: usize = SDRAM_CONFIG_ADDR + 0x08;
pub const SDRAM_CFG_AXI_ENABLE_ADDR: usize = SDRAM_CONFIG_ADDR + 0x0C;

/// Masks `conf_val` bits in SDRAM_CFG_AXI_DDR_MODE_ADDR. Returns the previous value in register.
pub fn sdram_cfg_axi_ddr_mode_mask(conf_val: u32) -> u32 {
    let pvalue = read_u32(SDRAM_CFG_AXI_DDR_MODE_ADDR);
    mask_u32(SDRAM_CFG_AXI_DDR_MODE_ADDR, conf_val);
    pvalue
}

/// Masks `conf_val` bits in SDRAM_CFG_AXI_ENABLE_ADDR. Returns the previous value in register.
pub fn sdram_cfg_axi_enable_mask(conf_val: u32) -> u32 {
    let pvalue = read_u32(SDRAM_CFG_AXI_ENABLE_ADDR);
    mask_u32(SDRAM_CFG_AXI_ENABLE_ADDR, conf_val);
    pvalue
}
