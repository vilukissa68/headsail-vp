pub(crate) const UART0_ADDR: usize = 0xFFF00000;
// NOTE: (20240614 vaino-waltteri.granat@tuni.fi) This applies to renode NS16550 uart, but might not apply to headsail ASIC
pub(crate) const UART_DATA_READY_OFFSET: usize = 5;
pub(crate) const TIMER0_ADDR: usize = 0x5_0000;
pub(crate) const TIMER1_ADDR: usize = 0x5_0010;
pub(crate) const TIMER2_ADDR: usize = 0x5_0020;
pub(crate) const TIMER3_ADDR: usize = 0x5_0030;
