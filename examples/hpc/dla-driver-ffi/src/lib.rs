//! # DLA driver
//!
//! Implements driver for sochub headsail SoC's deep learning accelerator.
#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;
use core::ffi::c_char;
use core::slice;
use dla_driver::layers::conv2d;
use dla_driver::tensor3::{Order3, Tensor3};
use dla_driver::tensor4::{Order4, Tensor4};
use dla_driver::{Padding, SimdBitMode, Stride};

#[repr(C)]
pub struct COrder3 {
    order: [c_char; 3],
}

#[repr(C)]
pub struct COrder4 {
    order: [c_char; 4],
}

// FFI-compatible Tensor3 structure:
#[repr(C)]
pub struct CTensor3 {
    data: *const i8, // Pointer to the data
    channels: usize,
    height: usize,
    width: usize,
    order: COrder3,
}

// FFI-compatible Tensor4 structure:
#[repr(C)]
pub struct CTensor4 {
    data: *const i8, // Pointer to the data
    kernels: usize,
    channels: usize,
    height: usize,
    width: usize,
    order: COrder4,
}

#[repr(C)]
pub struct CPadding {
    top: u32,
    bottom: u32,
    left: u32,
    right: u32,
    padding_value: i32,
}

impl From<CPadding> for Padding {
    fn from(c_padding: CPadding) -> Self {
        Padding {
            top: c_padding.top,
            right: c_padding.right,
            left: c_padding.left,
            bottom: c_padding.bottom,
            padding_value: c_padding.padding_value,
        }
    }
}

impl From<Padding> for CPadding {
    fn from(padding: Padding) -> Self {
        CPadding {
            top: padding.top,
            right: padding.right,
            left: padding.left,
            bottom: padding.bottom,
            padding_value: padding.padding_value,
        }
    }
}

#[repr(C)]
pub struct CStride {
    x: u32,
    y: u32,
}

impl From<CStride> for Stride {
    fn from(c_stride: CStride) -> Self {
        Stride {
            x: c_stride.x,
            y: c_stride.y,
        }
    }
}

impl From<Stride> for CStride {
    fn from(stride: Stride) -> Self {
        CStride {
            x: stride.x,
            y: stride.y,
        }
    }
}

#[repr(C)]
pub enum CSimdBitMode {
    EightBits = 0,
    FourBits = 1,
    TwoBits = 2,
}

impl From<CSimdBitMode> for SimdBitMode {
    fn from(c_mode: CSimdBitMode) -> Self {
        match c_mode {
            CSimdBitMode::EightBits => SimdBitMode::EightBits,
            CSimdBitMode::FourBits => SimdBitMode::FourBits,
            CSimdBitMode::TwoBits => SimdBitMode::TwoBits,
        }
    }
}

impl From<SimdBitMode> for CSimdBitMode {
    fn from(mode: SimdBitMode) -> Self {
        match mode {
            SimdBitMode::EightBits => CSimdBitMode::EightBits,
            SimdBitMode::FourBits => CSimdBitMode::FourBits,
            SimdBitMode::TwoBits => CSimdBitMode::TwoBits,
        }
    }
}

#[no_mangle]
pub extern "C" fn conv2d_ffi(
    input: CTensor3,
    kernels: CTensor4,
    padding: CPadding,       // Assuming padding is a pointer (nullable)
    stride: CStride,         // Assuming stride is a pointer (nullable)
    mac_clip: u32,           // Nullable pointer for optional values
    pp_clip: u32,            // Nullable pointer for optional values
    simd_mode: CSimdBitMode, // Nullable pointer for optional values
    output: *mut CTensor3,   // Output pointer
) {
    let input_data: Vec<i8> = unsafe {
        slice::from_raw_parts(input.data, input.channels * input.height * input.width).to_vec()
    };
    let input_tensor = unsafe {
        Tensor3::from_data_buffer(
            input.channels,
            input.height,
            input.width,
            input_data,
            Order3::try_from(input.order.order).unwrap_unchecked(),
        )
        .unwrap_unchecked()
    };

    let kernels_data: Vec<i8> = unsafe {
        slice::from_raw_parts(
            kernels.data,
            kernels.kernels * kernels.channels * kernels.height * kernels.width,
        )
        .to_vec()
    };

    let kernels_tensor = unsafe {
        Tensor4::from_data_buffer(
            kernels.kernels,
            kernels.channels,
            kernels.height,
            kernels.width,
            kernels_data,
            Order4::try_from(kernels.order.order).unwrap_unchecked(),
        )
        .unwrap_unchecked()
    };

    let result = conv2d(
        input_tensor,
        kernels_tensor,
        Some(Padding::from(padding)),
        Some(Stride::from(stride)),
        Some(mac_clip),
        Some(pp_clip),
        Some(SimdBitMode::from(simd_mode)),
    );
}
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
