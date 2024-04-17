//! C API for `headsail-bsp`
#![no_std]
#![no_main]

/// Runtime shim
///
/// Calls external `cmain`.
#[cfg(feature = "rt")]
#[headsail_bsp::rt::entry]
unsafe fn rust_main() -> ! {
    extern "C" {
        fn cmain();
    }

    cmain();

    loop {}
}

// Make symbols available in the C header by making a symbol visible here, and according to the
// following format:
//
// - Function: `#[no_mangle] pub extern fn` ("functions")
// - Global: `#[no_mangle] pub static`
// - Constant: `pub const`
//
// To make types usable in C, make them `#[repr(C)]`, `#[repr(u8, u16, ... etc)]`, or
// `#[repr(transparent)]`.
//
// Find more documentation at <https://github.com/mozilla/cbindgen/blob/master/docs.md>

#[no_mangle]
pub extern "C" fn putc(byte: u8) {
    headsail_bsp::uart::putc(byte)
}
