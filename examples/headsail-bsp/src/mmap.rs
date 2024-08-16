#[cfg(feature = "hpc")]
pub(crate) const UART0_ADDR: usize = 0x1FFF00000;
#[cfg(not(feature = "hpc"))]
pub(crate) const UART0_ADDR: usize = 0xFFF00000;
pub(crate) const UART0_THR: usize = UART0_ADDR;

// NOTE: (20240614 vaino-waltteri.granat@tuni.fi) This applies to renode NS16550 uart, but might not apply to headsail ASIC
pub(crate) const UART_DATA_READY_OFFSET: usize = 5;

#[cfg(feature = "asic")]
mod asic_uart {
    use super::UART0_ADDR;

    pub(crate) const UART0_DIV_LSB: usize = UART0_ADDR + 0;
    pub(crate) const UART0_DIV_MSB: usize = UART0_ADDR + 1;
    pub(crate) const UART0_INTERRUPT_ENABLE: usize = UART0_ADDR + 1;
    pub(crate) const UART0_FIFO_CONTROL: usize = UART0_ADDR + 2;
    pub(crate) const UART0_LINE_CONTROL: usize = UART0_ADDR + 3;
    pub(crate) const UART0_MODEM_CONTROL: usize = UART0_ADDR + 4;
    pub(crate) const UART0_LINE_STATUS: usize = UART0_ADDR + 5;
}
#[cfg(feature = "asic")]
pub(crate) use self::asic_uart::*;

pub(crate) const TIMER0_ADDR: usize = 0x5_0000;
pub(crate) const TIMER1_ADDR: usize = 0x5_0010;
pub(crate) const TIMER2_ADDR: usize = 0x5_0020;
pub(crate) const TIMER3_ADDR: usize = 0x5_0030;
