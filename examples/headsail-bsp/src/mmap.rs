#[cfg(feature = "hpc")]
pub(crate) const UART0_ADDR: usize = 0x1FFF00000;
#[cfg(not(feature = "hpc"))]
pub(crate) const UART0_ADDR: usize = 0xFFF00000;

#[cfg(feature = "hpc")]
pub(crate) const UART1_ADDR: usize = 0x1FFF01000;
#[cfg(not(feature = "hpc"))]
pub(crate) const UART1_ADDR: usize = 0xFFF01000;

pub(crate) const UART_THR_OFFSET: usize = 0;

// NOTE: (20240614 vaino-waltteri.granat@tuni.fi) This applies to renode NS16550 uart, but might not apply to headsail ASIC
pub(crate) const UART_DATA_READY_OFFSET: usize = 5;

#[cfg(feature = "asic")]
mod asic_uart {
    pub(crate) const UART_DIV_LSB_OFFSET: usize = 0;
    pub(crate) const UART_DIV_MSB_OFFSET: usize = 1;
    pub(crate) const UART_INTERRUPT_ENABLE_OFFSET: usize = 1;
    pub(crate) const UART_FIFO_CONTROL_OFFSET: usize = 2;
    pub(crate) const UART_LINE_CONTROL_OFFSET: usize = 3;
    pub(crate) const UART_MODEM_CONTROL_OFFSET: usize = 4;
    pub(crate) const UART_LINE_STATUS_OFFSET: usize = 5;
}
#[cfg(feature = "asic")]
pub(crate) use self::asic_uart::*;

pub(crate) const TIMER0_ADDR: usize = 0x5_0000;
pub(crate) const TIMER1_ADDR: usize = 0x5_0010;
pub(crate) const TIMER2_ADDR: usize = 0x5_0020;
pub(crate) const TIMER3_ADDR: usize = 0x5_0030;
