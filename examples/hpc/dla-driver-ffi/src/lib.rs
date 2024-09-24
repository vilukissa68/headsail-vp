//! # DLA driver FFI
//!
//! Makes DLA's highlevel API availeable from C via FFI.
#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;
use core::ffi::{c_char, CStr};
use core::slice;
use dla_driver::layers::{conv2d, conv2d_bias, conv2d_bias_relu, conv2d_relu};
use dla_driver::tensor3::{rescale, Order3, Tensor3};
use dla_driver::tensor4::{Order4, Tensor4};
use dla_driver::{Padding, Stride};
use headsail_bsp::{sprint, sprintln};

/// Converts C-types to DLA Tensors for use with the highlevel layer
unsafe fn ffi_data_import(
    input_data: *const i8,
    input_channels: usize,
    input_height: usize,
    input_width: usize,
    input_order: *const c_char,
    input_zero: i16,
    kernel_data: *const i8,
    kernel_amount: usize,
    kernel_channels: usize,
    kernel_height: usize,
    kernel_width: usize,
    kernel_order: *const c_char,
) -> (Tensor3<i8>, Tensor4<i8>) {
    let mut input_data: Vec<i8> = unsafe {
        slice::from_raw_parts(input_data, input_channels * input_height * input_width).to_vec()
    };

    // Input zero point shift
    input_data = input_data
        .into_iter()
        .map(|x| scale_as_i16(x, input_zero))
        .collect();

    let input_order_string = unsafe { CStr::from_ptr(input_order).to_str().unwrap_unchecked() };
    let input_tensor = unsafe {
        Tensor3::from_data_buffer(
            input_channels,
            input_height,
            input_width,
            input_data,
            Order3::try_from(input_order_string).unwrap_unchecked(),
        )
        .unwrap_unchecked()
    };

    let kernels_data: Vec<i8> = unsafe {
        slice::from_raw_parts(
            kernel_data,
            kernel_amount * kernel_channels * kernel_height * kernel_width,
        )
        .to_vec()
    };

    let kernel_order_string = unsafe { CStr::from_ptr(kernel_order).to_str().unwrap_unchecked() };
    let kernels_tensor = unsafe {
        Tensor4::from_data_buffer(
            kernel_amount,
            kernel_channels,
            kernel_height,
            kernel_width,
            kernels_data,
            Order4::try_from(kernel_order_string).unwrap_unchecked(),
        )
        .unwrap_unchecked()
    };

    (input_tensor, kernels_tensor)
}

/// Initializes DLA by setting up necessary heap allocator from headsail-bsp. This should be called only once in the program.
#[no_mangle]
pub unsafe extern "C" fn dla_init() {
    headsail_bsp::init_alloc();
}

/// Executes Conv2D on DLA with given parameters and writes result to output buffer.
#[no_mangle]
pub unsafe extern "C" fn dla_conv2d(
    input_data: *const i8,
    kernel_data: *const i8,
    output: *mut i8,
    input_channels: usize,
    input_height: usize,
    input_width: usize,
    input_order: *const c_char,
    kernel_amount: usize,
    kernel_channels: usize,
    kernel_height: usize,
    kernel_width: usize,
    kernel_order: *const c_char,
    pad_top: u32,
    pad_right: u32,
    pad_left: u32,
    pad_bottom: u32,
    pad_value: i32,
    stride_x: u32,
    stride_y: u32,
    mac_clip: u32,
    pp_clip: u32,
) {
    let (input_tensor, kernels_tensor) = unsafe {
        ffi_data_import(
            input_data,
            input_channels,
            input_height,
            input_width,
            input_order,
            0,
            kernel_data,
            kernel_amount,
            kernel_channels,
            kernel_height,
            kernel_width,
            kernel_order,
        )
    };

    let result: Tensor3<i8> = conv2d(
        input_tensor,
        kernels_tensor,
        Some(Padding {
            top: pad_top,
            right: pad_right,
            left: pad_left,
            bottom: pad_bottom,
            padding_value: pad_value,
        }),
        Some(Stride {
            x: stride_x,
            y: stride_y,
        }),
        Some(mac_clip),
        Some(pp_clip),
        None,
    );
    unsafe {
        core::ptr::copy_nonoverlapping(result.to_buffer().as_mut_ptr(), output, result.get_size())
    };
}

/// Executes Conv2D + ReLU on DLA with given parameters and writes result to output buffer.
#[no_mangle]
pub unsafe extern "C" fn dla_conv2d_relu(
    input_data: *const i8,
    kernel_data: *const i8,
    output: *mut i8,
    input_channels: usize,
    input_height: usize,
    input_width: usize,
    input_order: *const c_char,
    kernel_amount: usize,
    kernel_channels: usize,
    kernel_height: usize,
    kernel_width: usize,
    kernel_order: *const c_char,
    pad_top: u32,
    pad_right: u32,
    pad_left: u32,
    pad_bottom: u32,
    pad_value: i32,
    stride_x: u32,
    stride_y: u32,
    mac_clip: u32,
    pp_clip: u32,
) {
    let (input_tensor, kernels_tensor) = unsafe {
        ffi_data_import(
            input_data,
            input_channels,
            input_height,
            input_width,
            input_order,
            0,
            kernel_data,
            kernel_amount,
            kernel_channels,
            kernel_height,
            kernel_width,
            kernel_order,
        )
    };

    let result: Tensor3<i8> = conv2d_relu(
        input_tensor,
        kernels_tensor,
        Some(Padding {
            top: pad_top,
            right: pad_right,
            left: pad_left,
            bottom: pad_bottom,
            padding_value: pad_value,
        }),
        Some(Stride {
            x: stride_x,
            y: stride_y,
        }),
        Some(mac_clip),
        Some(pp_clip),
        None,
    );
    unsafe {
        core::ptr::copy_nonoverlapping(result.to_buffer().as_mut_ptr(), output, result.get_size())
    };
}

/// Executes Conv2D + Bias on DLA with given parameters and writes result to output buffer.
#[no_mangle]
pub unsafe extern "C" fn dla_conv2d_bias(
    input_data: *const i8,
    kernel_data: *const i8,
    bias: *const i32, // NOTE: bias is actually i16 in hardware, here we use 32 for TVM compatability
    output: *mut i8,
    input_channels: usize,
    input_height: usize,
    input_width: usize,
    input_order: *const c_char,
    kernel_amount: usize,
    kernel_channels: usize,
    kernel_height: usize,
    kernel_width: usize,
    kernel_order: *const c_char,
    bias_length: usize,
    pad_top: u32,
    pad_right: u32,
    pad_left: u32,
    pad_bottom: u32,
    pad_value: i32,
    stride_x: u32,
    stride_y: u32,
    mac_clip: u32,
    pp_clip: u32,
) {
    let (input_tensor, kernels_tensor) = unsafe {
        ffi_data_import(
            input_data,
            input_channels,
            input_height,
            input_width,
            input_order,
            0,
            kernel_data,
            kernel_amount,
            kernel_channels,
            kernel_height,
            kernel_width,
            kernel_order,
        )
    };

    let bias: Vec<i16> = unsafe { slice::from_raw_parts(bias as *const i16, bias_length).to_vec() };

    let result = conv2d_bias(
        input_tensor,
        kernels_tensor,
        bias,
        Some(Padding {
            top: pad_top,
            right: pad_right,
            left: pad_left,
            bottom: pad_bottom,
            padding_value: pad_value,
        }),
        Some(Stride {
            x: stride_x,
            y: stride_y,
        }),
        Some(mac_clip),
        Some(pp_clip),
        None,
    );
    unsafe {
        core::ptr::copy_nonoverlapping(result.to_buffer().as_mut_ptr(), output, result.get_size())
    };
}

/// Executes Conv2D + Bias + ReLU on DLA with given parameters and writes result to output buffer.
#[no_mangle]
pub unsafe extern "C" fn dla_conv2d_bias_relu(
    input_data: *const i8,
    kernel_data: *const i8,
    bias: *const i32,
    output: *mut i8,
    input_channels: usize,
    input_height: usize,
    input_width: usize,
    input_order: *const c_char,
    kernel_amount: usize,
    kernel_channels: usize,
    kernel_height: usize,
    kernel_width: usize,
    kernel_order: *const c_char,
    bias_length: usize,
    pad_top: u32,
    pad_right: u32,
    pad_left: u32,
    pad_bottom: u32,
    pad_value: i32,
    stride_x: u32,
    stride_y: u32,
    mac_clip: u32,
    pp_clip: u32,
) {
    let (input_tensor, kernels_tensor) = unsafe {
        ffi_data_import(
            input_data,
            input_channels,
            input_height,
            input_width,
            input_order,
            0,
            kernel_data,
            kernel_amount,
            kernel_channels,
            kernel_height,
            kernel_width,
            kernel_order,
        )
    };

    let bias: Vec<i32> = unsafe { slice::from_raw_parts(bias as *const i32, bias_length).to_vec() };
    let bias_i16: Vec<i16> = bias.into_iter().map(|x| clip_i32_to_i16(x)).collect();

    let result = conv2d_bias_relu(
        input_tensor,
        kernels_tensor,
        bias_i16,
        Some(Padding {
            top: pad_top,
            right: pad_right,
            left: pad_left,
            bottom: pad_bottom,
            padding_value: pad_value,
        }),
        Some(Stride {
            x: stride_x,
            y: stride_y,
        }),
        Some(mac_clip),
        Some(pp_clip),
        None,
    );

    let input_order_string = unsafe { CStr::from_ptr(input_order).to_str().unwrap_unchecked() };
    unsafe {
        core::ptr::copy_nonoverlapping(
            result
                .to_buffer_with_order(Order3::try_from(input_order_string).unwrap_unchecked())
                .as_mut_ptr(),
            output,
            result.get_size(),
        )
    };
}

#[no_mangle]
pub unsafe extern "C" fn dla_tvm_qnn_conv2d(
    input_data: *const i8,
    kernel_data: *const i8,
    bias: *const i32,
    output: *mut i8,
    output_scale: *const f32,
    output_zero: *const i32,
    input_scale: *const f32,
    input_zero: *const i32,
    input_channels: usize,
    input_height: usize,
    input_width: usize,
    input_order: *const c_char,
    kernel_amount: usize,
    kernel_channels: usize,
    kernel_height: usize,
    kernel_width: usize,
    kernel_order: *const c_char,
    bias_length: usize,
    pad_top: u32,
    pad_right: u32,
    pad_left: u32,
    pad_bottom: u32,
    pad_value: i32,
    stride_x: u32,
    stride_y: u32,
    mac_clip: u32,
    pp_clip: u32,
) {
    let input_scale: Vec<f32> =
        unsafe { slice::from_raw_parts(input_scale as *const f32, 1).to_vec() };

    let input_zero: Vec<i32> =
        unsafe { slice::from_raw_parts(input_zero as *const i32, 1).to_vec() };

    let output_scale: Vec<f32> =
        unsafe { slice::from_raw_parts(output_scale as *const f32, kernel_amount).to_vec() };

    let output_zero: Vec<i32> =
        unsafe { slice::from_raw_parts(output_zero as *const i32, 1).to_vec() };

    let (input_tensor, kernels_tensor) = unsafe {
        ffi_data_import(
            input_data,
            input_channels,
            input_height,
            input_width,
            input_order,
            (-1 * input_zero[0]) as i16,
            kernel_data,
            kernel_amount,
            kernel_channels,
            kernel_height,
            kernel_width,
            kernel_order,
        )
    };

    let bias: Vec<i32> = unsafe { slice::from_raw_parts(bias as *const i32, bias_length).to_vec() };
    let bias_i16: Vec<i16> = bias.into_iter().map(|x| clip_i32_to_i16(x)).collect();

    let mut result = conv2d_bias_relu(
        input_tensor,
        kernels_tensor,
        bias_i16,
        Some(Padding {
            top: pad_top,
            right: pad_right,
            left: pad_left,
            bottom: pad_bottom,
            padding_value: pad_value,
        }),
        Some(Stride {
            x: stride_x,
            y: stride_y,
        }),
        Some(mac_clip),
        Some(pp_clip),
        None,
    );

    // TVM requantization and clip
    rescale(
        &mut result,
        u32::pow(2, pp_clip) as f32, //NOTE:(20240924 vaino-waltteri.granat@tuni.fi) Mitigate pp downscale
        input_zero[0],
        output_zero[0],
        input_scale[0],
        output_scale,
    );

    let input_order_string = unsafe { CStr::from_ptr(input_order).to_str().unwrap_unchecked() };

    unsafe {
        core::ptr::copy_nonoverlapping(
            result
                .to_buffer_with_order(Order3::try_from(input_order_string).unwrap_unchecked())
                .as_mut_ptr(),
            output,
            result.get_size(),
        )
    };
}
fn clip_i32_to_i16(value: i32) -> i16 {
    if value > i16::MAX as i32 {
        return i16::MAX;
    } else if value < i16::MIN as i32 {
        return i16::MIN;
    } else {
        return value as i16;
    }
}

fn scale_as_i16(input: i8, factor: i16) -> i8 {
    let new_value = input as i16 + factor;
    if new_value > 127 {
        return 127;
    } else if new_value < -128 {
        return -128;
    }
    return new_value as i8;
}
