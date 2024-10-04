use crate::{mask_u32, mmap::SDRAM_CONFIG_ADDR};

pub const SDRAM_CFG_DDR_INIT_DONE_ADDR: usize = SDRAM_CONFIG_ADDR + 0x04;
pub const SDRAM_CFG_AXI_DDR_MODE_ADDR: usize = SDRAM_CONFIG_ADDR + 0x08;
pub const SDRAM_CFG_AXI_ENABLE_ADDR: usize = SDRAM_CONFIG_ADDR + 0x0C;

pub fn sdram_cfg_axi_ddr_mode_mask(conf_val: u32) {
    mask_u32(SDRAM_CFG_AXI_DDR_MODE_ADDR, conf_val);
}

pub fn sdram_cfg_axi_enable_mask(conf_val: u32) {
    mask_u32(SDRAM_CFG_AXI_ENABLE_ADDR, conf_val);
}
