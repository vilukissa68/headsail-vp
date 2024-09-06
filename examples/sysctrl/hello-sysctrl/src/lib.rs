#![no_std]
#![no_main]
use headsail_bsp::pac::Sysctrl;

/// Print the name of the current file, i.e., test name.
///
/// This must be a macro to make sure core::file matches the file this is
/// invoked in.
#[macro_export]
macro_rules! print_example_name {
    () => {
        use $crate::sysctrl_print;
        sysctrl_print(b"[");
        sysctrl_print(core::file!().as_bytes());
        sysctrl_print(b"]");
    };
}

// Number of nops SysCtrl is capable of executing at 30 MHz reference clocks
pub const NOPS_PER_SEC: usize = match () {
    #[cfg(debug_assertions)]
    // This is an experimentally found value
    () => 2_000_000 / 9,
    #[cfg(not(debug_assertions))]
    // This is just a guess for now (10x debug)
    () => 20_000_000 / 9,
};

/// Make sure to enable uDMA UART prior to using this function
pub fn sysctrl_print(buf: &[u8]) {
    let sysctrl = Sysctrl::ptr();
    let udma = unsafe { (*sysctrl).udma() };

    udma.uart_tx_saddr()
        .write(|w| unsafe { w.bits(buf.as_ptr() as u32) });
    udma.uart_tx_size()
        .write(|w| unsafe { w.bits(buf.len() as u32) });

    // (3) Dispatch transmission
    udma.uart_tx_cfg().write(
        |w| w.en().set_bit(), // If we want "continuous mode". In continuous mode, uDMA reloads the address and transmits it again
                              //.continous().set_bit()
    );

    // (4) Poll until finished
    while udma.uart_tx_saddr().read().bits() != 0 {}
}
