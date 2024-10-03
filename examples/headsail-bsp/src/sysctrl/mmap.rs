// Allowed for extra clarity in certain cases
#![allow(clippy::identity_op)]
/// SysCtrl-specific, SysCtrl internal memory maps

pub(crate) const SYSCTRL_ADDR: usize = 0x1a10_0000;
pub(crate) const SDRAM_BASE_ADDR: usize = 0xFFD00000;

pub(crate) const GPIO_ADDR: usize = SYSCTRL_ADDR + 0x1000;
pub(crate) const GPIO_DIR: usize = GPIO_ADDR + 0x0;
pub(crate) const GPIO_OUT: usize = GPIO_ADDR + 0xc;

pub(crate) const SOC_CONTROL_ADDR: usize = SYSCTRL_ADDR + 0x4000;
pub const PADMUX0: usize = SOC_CONTROL_ADDR + 0x10;
pub const PADMUX1: usize = SOC_CONTROL_ADDR + 0x14;

pub const SS_RESET_EN: usize = SOC_CONTROL_ADDR + 0xb0;
pub const SS_CLK_CTRL1: usize = SOC_CONTROL_ADDR + 0x98;
pub const SS_CLK_CTRL2: usize = SOC_CONTROL_ADDR + 0x9c;
pub const SS_CLK_CTRL3: usize = SOC_CONTROL_ADDR + 0xb8;

pub const PERIPH_CLK_DIV: usize = SOC_CONTROL_ADDR + 0xA8;

pub const SDRAM_CFG_DDR_INIT_DONE_ADDR: usize = SDRAM_BASE_ADDR + 0x04;
pub const SDRAM_CFG_AXI_DDR_MODE_ADDR: usize = SDRAM_BASE_ADDR + 0x08;
pub const SDRAM_CFG_AXI_ENABLE_ADDR: usize = SDRAM_BASE_ADDR + 0x0C;
