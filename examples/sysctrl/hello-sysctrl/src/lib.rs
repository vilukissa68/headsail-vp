#![no_std]
#![no_main]

/// Print the name of the current file, i.e., test name.
///
/// This must be a macro to make sure core::file matches the file this is
/// invoked in.
#[macro_export]
macro_rules! print_example_name {
    () => {
        use headsail_bsp::sprintln;
        sprintln!("[{}]", core::file!());
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
