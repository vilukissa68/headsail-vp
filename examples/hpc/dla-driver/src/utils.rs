#![no_std]

use crate::mmap::MEMORY_BANK_SIZE;
use crate::tensor3::{Order3, Tensor3};
use crate::tensor4::Tensor4;
use crate::{MemoryBank, Padding, Stride, DEFAULT_PADDING, DEFAULT_STRIDE, MEMORY_BANK_BASE_ADDR};
use alloc::vec::Vec;
use headsail_bsp::sprintln;

pub fn calculate_conv2d_out_param_dim(
    input: (u32, u32),
    kernel: (u32, u32),
    padding: Option<Padding>,
    stride: Option<Stride>,
) -> (usize, usize) {
    let padding = padding.unwrap_or(DEFAULT_PADDING);
    let stride = stride.unwrap_or(DEFAULT_STRIDE);

    let output_width =
        (input.0 + padding.right + padding.left - 1 * (kernel.0 - 1) - 1) / stride.x + 1;
    let output_height =
        (input.1 + padding.bottom + padding.top - 1 * (kernel.1 - 1) - 1) / stride.y + 1;
    (output_width as usize, output_height as usize)
}

pub fn generate_output_tensor<T, H, J: Clone + headsail_bsp::ufmt::uDisplay>(
    input: &Tensor3<T>,
    kernel: &Tensor4<H>,
    output_buf: Vec<J>,
    order: Order3,
    padding: Option<Padding>,
    stride: Option<Stride>,
) -> Tensor3<J> {
    let output_size = calculate_conv2d_out_param_dim(
        (input.width as u32, input.height as u32),
        (kernel.width as u32, kernel.height as u32),
        padding,
        stride,
    );
    let dout_tensor = Tensor3::from_data_buffer(
        kernel.kernels,
        output_size.1,
        output_size.0,
        output_buf.clone(),
        order,
    )
    .unwrap();
    dout_tensor
}

pub fn calculate_number_of_banks_needed(bytes: usize) -> usize {
    // Take ceil
    (bytes + (MEMORY_BANK_SIZE - 1)) / MEMORY_BANK_SIZE
}

pub fn get_banks_for_layer(
    input_size: usize,
    kernels_size: usize,
    output_size: usize,
    bias_size: Option<usize>,
) -> (MemoryBank, MemoryBank, MemoryBank, Option<u32>) {
    let no_input_banks = calculate_number_of_banks_needed(input_size);
    let no_kernel_banks = calculate_number_of_banks_needed(kernels_size);
    let no_output_banks = calculate_number_of_banks_needed(output_size);

    let input_bank = MemoryBank::Bank0;
    let kernel_bank = input_bank + no_input_banks;
    let output_bank = kernel_bank + no_kernel_banks;

    let bias_bank = match bias_size {
        Some(_bias_size) => {
            Some((MEMORY_BANK_BASE_ADDR + (output_bank + no_output_banks).offset()) as u32)
        }
        None => None,
    };
    sprintln!(
        "input bank: {:x}, kernel bank: {:x}, output bank: {:x}, bias bank: {:x}",
        input_bank.offset(),
        kernel_bank.offset(),
        output_bank.offset(),
        bias_bank.unwrap()
    );

    return (input_bank, kernel_bank, output_bank, bias_bank);
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
