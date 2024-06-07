#![no_std]

use crate::mmap::MEMORY_BANK_SIZE;

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
