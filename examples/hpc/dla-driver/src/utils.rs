#![no_std]

use crate::mmap::MEMORY_BANK_SIZE;
use crate::Padding;

pub fn calculate_conv2d_out_param_dim(
    input: (u32, u32),
    kernel: (u32, u32),
    padding: (u32, u32),
    dilation: (u32, u32),
    stride: (u32, u32),
) -> (usize, usize) {
    let output_width = (input.0 + 2 * padding.0 - dilation.0 * (kernel.0 - 1) - 1) / stride.0 + 1;
    let output_height = (input.1 + 2 * padding.1 - dilation.1 * (kernel.1 - 1) - 1) / stride.1 + 1;
    (output_width as usize, output_height as usize)
}

pub fn calculate_number_of_banks_needed(bytes: usize) -> usize {
    // Take ceil
    (bytes + (MEMORY_BANK_SIZE - 1)) / MEMORY_BANK_SIZE
}

fn ceil<T>(x: T, y: T) -> T
where
    T: core::ops::Add<Output = T>
        + core::ops::Sub<Output = T>
        + core::ops::Div<Output = T>
        + From<u8>
        + Copy,
{
    (x + y - T::from(1)) / y
}

pub fn calculate_valid_output_size(
    input: (u32, u32),
    kernel: (u32, u32),
    stride: (u32, u32),
) -> (usize, usize) {
    let output_width = ceil(input.0 - kernel.0 + 1, stride.0);
    let output_height = ceil(input.1 - kernel.1 + 1, stride.1);
    (output_width as usize, output_height as usize)
}

pub fn calculate_same_output_size(input: (u32, u32), stride: (u32, u32)) -> (usize, usize) {
    let output_width = ceil(input.0, stride.0);
    let output_height = ceil(input.1, stride.1);
    (output_width as usize, output_height as usize)
}

fn calculate_same_padding(input: (u32, u32), kernel: (u32, u32), stride: (u32, u32)) -> Padding {
    let padding_width = ((input.0 - 1) * stride.0 + kernel.0).saturating_sub(input.0);
    let padding_height = ((input.1 - 1) * stride.1 + kernel.1).saturating_sub(input.1);

    Padding {
        top: padding_height / 2,
        bottom: padding_height - padding_height / 2,
        left: padding_width / 2,
        right: padding_width - padding_width / 2,
        padding_value: 0,
    }
}
