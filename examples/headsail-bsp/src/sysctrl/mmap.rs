/// SysCtrl-specific memory maps

pub(crate) const SYSCTRL_ADDR: usize = 0x1a10_0000;

pub(crate) const GPIO_ADDR: usize = SYSCTRL_ADDR + 0x1000;
pub(crate) const GPIO_DIR: usize = GPIO_ADDR + 0x0;
pub(crate) const GPIO_OUT: usize = GPIO_ADDR + 0xc;

pub(crate) const SOC_CONTROL_ADDR: usize = SYSCTRL_ADDR + 0x4000;
pub(crate) const PADMUX0: usize = SOC_CONTROL_ADDR + 0x10;
pub(crate) const PADMUX1: usize = SOC_CONTROL_ADDR + 0x14;

pub(crate) const SS_RESET_EN: usize = SOC_CONTROL_ADDR + 0xb0;
pub(crate) const SS_CLK_CTRL2: usize = SOC_CONTROL_ADDR + 0x9c;
pub(crate) const SS_CLK_CTRL3: usize = SOC_CONTROL_ADDR + 0xb8;
