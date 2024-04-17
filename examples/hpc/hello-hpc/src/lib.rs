#![no_std]

mod mmap;

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
