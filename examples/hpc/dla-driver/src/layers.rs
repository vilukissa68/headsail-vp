#![no_std]

use headsail_bsp::{sprint, sprintln};

use crate::tensor3::{Order3, Tensor3};
use crate::tensor4::{Order4, Tensor4};
use crate::{
    Dla, InputSize, KernelSize, LayerConfig, MemoryBank, Padding, SimdBitMode, Stride,
    MEMORY_BANK_10_OFFSET, MEMORY_BANK_BASE_ADDR,
};
use alloc::vec::Vec;

use crate::utils::{
    calculate_conv2d_out_param_dim, calculate_number_of_banks_needed, get_banks_for_layer,
};

pub fn dense(outputs: usize, input: Tensor3<i8>, weights: Vec<i8>) -> Vec<i32> {
    // Build kernels to produce 1 to 1 mac operation
    let kernels_wrap = Tensor4::from_data_buffer(
        outputs,
        input.channels,
        input.height,
        input.width,
        weights,
        Order4::KCHW,
    );

    let kernels = match kernels_wrap {
        Ok(kernels) => kernels,
        Err(_e) => return [0].to_vec(),
    };

    let output = conv2d(input, kernels, None, None, None, None, None);
    output.to_buffer()
}

pub fn conv2d(
    input: Tensor3<i8>,
    kernels: Tensor4<i8>,
    padding: Option<Padding>,
    stride: Option<Stride>,
    mac_clip: Option<u32>,
    pp_clip: Option<u32>,
    simd_mode: Option<SimdBitMode>,
) -> Tensor3<i32> {
    let dla = Dla::new();
    let output_size = calculate_conv2d_out_param_dim(
        (input.width as u32, input.height as u32),
        (kernels.width as u32, kernels.height as u32),
        padding.clone(),
        stride.clone(),
    );

    // Calculate needed space
    let input_size = input.get_size();
    let kernels_size = kernels.get_size();

    let no_input_banks = calculate_number_of_banks_needed(input_size);
    let no_kernel_banks = calculate_number_of_banks_needed(kernels_size);

    let input_bank = MemoryBank::Bank0;
    let kernel_bank = input_bank + no_input_banks;
    let output_bank = kernel_bank + no_kernel_banks;

    // Initalize layer
    let config = LayerConfig {
        input_bank: Some(input_bank),   // b
        kernel_bank: Some(kernel_bank), // a
        output_bank: Some(output_bank),
        bias_addr: Some(0),
        pp_enabled: false,
        relu_enabled: false,
        bias_enabled: false,
        input_size: Some(InputSize {
            channels: input.channels as u32,
            width: input.width as u32,
            height: input.height as u32,
        }),
        kernel_size: Some(KernelSize {
            s_channels: 1,
            kernels: kernels.kernels as u32,
            width: kernels.width as u32,
            height: kernels.height as u32,
        }),
        padding,
        stride,
        mac_clip,
        pp_clip,
        simd_mode,
    };

    dla.init_layer(config);

    input.print_tensor();
    kernels.print_tensor();

    dla.write_input(&mut input.to_buffer_with_order(Order3::HWC));
    dla.write_kernel(&mut kernels.to_buffer_with_order(Order4::HWKC));

    // Mark data ready to start calculations
    dla.kernel_data_ready(true);
    dla.input_data_ready(true);

    while !dla.handle_handshake() {}
    let output_buffer = dla.read_output_i32(output_size.0 * output_size.1 * kernels.kernels);

    let output: Tensor3<i32> = Tensor3::from_data_buffer(
        kernels.kernels,
        output_size.1,
        output_size.0,
        output_buffer,
        Order3::HWC, // NOTE: (20240610 vaino-waltteri.granat@tuni.fi) This might not be true on ASIC
    )
    .unwrap();

    output
}

pub fn relu(input: Tensor3<i8>, pp_clip: Option<u32>) -> Tensor3<i8> {
    let output_size = calculate_conv2d_out_param_dim(
        (input.width as u32, input.height as u32),
        (1, 1),
        None,
        None,
    );
    let dla = Dla::new();

    // Calculate needed space
    let input_size = input.get_size();
    let kernels_size = input.get_size() * input.channels; // Channels doubled

    let no_input_banks = calculate_number_of_banks_needed(input_size);
    let no_kernel_banks = calculate_number_of_banks_needed(kernels_size);

    let input_bank = MemoryBank::Bank0;
    let kernel_bank = input_bank + no_input_banks;
    let output_bank = kernel_bank + no_kernel_banks;

    let mut kernels = vec![1; kernels_size]; // 1 filled kernels for constant conv2d

    // Initalize layer
    let config = LayerConfig {
        input_bank: Some(input_bank),   // b
        kernel_bank: Some(kernel_bank), // a
        output_bank: Some(output_bank),
        bias_addr: Some(0),
        pp_enabled: true,
        relu_enabled: true,
        bias_enabled: false,
        input_size: Some(InputSize {
            channels: input.channels as u32,
            width: input.width as u32,
            height: input.height as u32,
        }),
        kernel_size: Some(KernelSize {
            s_channels: 1,
            kernels: input.channels as u32,
            width: input.width as u32,
            height: input.height as u32,
        }),
        padding: None,
        stride: None,
        mac_clip: Some(0),
        pp_clip,
        simd_mode: Some(SimdBitMode::EightBits),
    };

    dla.init_layer(config);

    input.print_tensor();

    dla.write_input(&mut input.to_buffer_with_order(Order3::HWC));
    dla.write_kernel(&mut kernels);

    // Mark data ready to start calculations
    dla.kernel_data_ready(true);
    dla.input_data_ready(true);

    while !dla.handle_handshake() {}
    let output_buffer = dla.read_output_i8(output_size.0 * output_size.1 * input.channels);

    let output: Tensor3<i8> = Tensor3::from_data_buffer(
        input.channels,
        output_size.1,
        output_size.0,
        output_buffer,
        Order3::HWC, // NOTE: (20240610 vaino-waltteri.granat@tuni.fi) This might not be true on ASIC
    )
    .unwrap();
    output
}

pub fn conv2d_relu(
    input: Tensor3<i8>,
    kernels: Tensor4<i8>,
    padding: Option<Padding>,
    stride: Option<Stride>,
    mac_clip: Option<u32>,
    pp_clip: Option<u32>,
    simd_mode: Option<SimdBitMode>,
) -> Tensor3<i8> {
    let output_size = calculate_conv2d_out_param_dim(
        (input.width as u32, input.height as u32),
        (kernels.width as u32, kernels.height as u32),
        padding.clone(),
        stride.clone(),
    );

    let dla = Dla::new();

    // Calculate needed space
    let input_size = input.get_size();
    let kernels_size = kernels.get_size();

    let no_input_banks = calculate_number_of_banks_needed(input_size);
    let no_kernel_banks = calculate_number_of_banks_needed(kernels_size);

    let input_bank = MemoryBank::Bank0;
    let kernel_bank = input_bank + no_input_banks;
    let output_bank = kernel_bank + no_kernel_banks;

    // Initalize layer
    let config = LayerConfig {
        input_bank: Some(input_bank),   // b
        kernel_bank: Some(kernel_bank), // a
        output_bank: Some(output_bank),
        bias_addr: Some(0),
        pp_enabled: true,
        relu_enabled: true,
        bias_enabled: false,
        input_size: Some(InputSize {
            channels: input.channels as u32,
            width: input.width as u32,
            height: input.height as u32,
        }),
        kernel_size: Some(KernelSize {
            s_channels: 1,
            kernels: kernels.kernels as u32,
            width: kernels.width as u32,
            height: kernels.height as u32,
        }),
        padding,
        stride,
        mac_clip,
        pp_clip,
        simd_mode,
    };

    dla.init_layer(config);

    input.print_tensor();
    kernels.print_tensor();

    dla.write_input(&mut input.to_buffer_with_order(Order3::HWC));
    dla.write_kernel(&mut kernels.to_buffer_with_order(Order4::HWKC));

    // Mark data ready to start calculations
    dla.kernel_data_ready(true);
    dla.input_data_ready(true);

    while !dla.handle_handshake() {}
    let output_buffer = dla.read_output_i8(output_size.0 * output_size.1 * kernels.kernels);

    let output: Tensor3<i8> = Tensor3::from_data_buffer(
        kernels.kernels,
        output_size.1,
        output_size.0,
        output_buffer,
        Order3::HWC, // NOTE: (20240610 vaino-waltteri.granat@tuni.fi) This might not be true on ASIC
    )
    .unwrap();
    output
}

pub fn conv2d_bias(
    input: Tensor3<i8>,
    kernels: Tensor4<i8>,
    bias: Vec<i16>,
    padding: Option<Padding>,
    stride: Option<Stride>,
    mac_clip: Option<u32>,
    pp_clip: Option<u32>,
    simd_mode: Option<SimdBitMode>,
) -> Tensor3<i8> {
    let output_size = calculate_conv2d_out_param_dim(
        (input.width as u32, input.height as u32),
        (kernels.width as u32, kernels.height as u32),
        padding.clone(),
        stride.clone(),
    );

    let dla = Dla::new();

    let banks = get_banks_for_layer(
        input.get_size(),
        kernels.get_size(),
        output_size.0 * output_size.1,
        Some(bias.len()),
    );

    // Initalize layer
    let config = LayerConfig {
        //input_bank: Some(banks.0),  // b
        //kernel_bank: Some(banks.1), // a
        //output_bank: Some(banks.2),
        //bias_addr: banks.3,
        input_bank: Some(MemoryBank::Bank0),  // b
        kernel_bank: Some(MemoryBank::Bank1), // a
        output_bank: Some(MemoryBank::Bank12),
        bias_addr: Some((MEMORY_BANK_BASE_ADDR + MEMORY_BANK_10_OFFSET) as u32),
        pp_enabled: true,
        relu_enabled: false,
        bias_enabled: true,
        input_size: Some(InputSize {
            channels: input.channels as u32,
            width: input.width as u32,
            height: input.height as u32,
        }),
        kernel_size: Some(KernelSize {
            s_channels: 1,
            kernels: kernels.kernels as u32,
            width: kernels.width as u32,
            height: kernels.height as u32,
        }),
        padding,
        stride,
        mac_clip,
        pp_clip,
        simd_mode,
    };

    dla.init_layer(config);

    input.print_tensor();
    kernels.print_tensor();

    dla.write_input(&mut input.to_buffer_with_order(Order3::HWC));
    dla.write_kernel(&mut kernels.to_buffer_with_order(Order4::HWKC));
    dla.write_bias(&bias);

    // Mark data ready to start calculations
    dla.kernel_data_ready(true);
    dla.input_data_ready(true);

    while !dla.handle_handshake() {}
    let output_buffer = dla.read_output_i8(output_size.0 * output_size.1 * kernels.kernels);

    let output: Tensor3<i8> = Tensor3::from_data_buffer(
        kernels.kernels,
        output_size.1,
        output_size.0,
        output_buffer,
        Order3::HWC, // NOTE: (20240610 vaino-waltteri.granat@tuni.fi) This might not be true on ASIC
    )
    .unwrap();
    output
}
