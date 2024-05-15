use dla_driver::*;
use headsail_bsp::{init_alloc, rt::entry, sprint, sprintln};
use panic_halt as _;

use alloc::vec::Vec;

fn calculate_conv2d_out_param_dim(
    input: (u32, u32),
    kernel: (u32, u32),
    padding: (u32, u32),
    dilation: (u32, u32),
    stride: (u32, u32),
) -> (u32, u32) {
    let output_width = (input.0 + 2 * padding.0 - dilation.0 * (kernel.0 - 1) - 1) / stride.0 + 1;
    let output_height = (input.1 + 2 * padding.1 - dilation.1 * (kernel.1 - 1) - 1) / stride.1 + 1;
    (output_width, output_height)
}
pub struct Vec3<T> {
    elems: Vec3<T>,
    channels: usize,
    layer_width: usize,
    layer_height: usize,
}

pub struct Vec2<T> {
    elems: Vec<T>,
    width: usize,
    height: usize,
}

pub fn dla_conv2d(input: Vec3<i8>, kernel: Vec3<i8>) -> Vec3<i8> {
    let mut dla = Dla::new();
    // Calculate output size
    let (output_width, output_height) = calculate_conv2d_out_param_dim(
        (input.layer_width, input.layer_height),
        (kernel.layer_width, kernel.layer_height),
        (0, 0),
        (1, 1),
        (1, 1),
    );

    // Initalize layer
    let config = LayerConfig {
        input_bank: Some(MemoryBank::Bank0),
        kernel_bank: Some(MemoryBank::Bank8),
        output_bank: Some(MemoryBank::Bank12),
        bias_addr: 0,
        pp_enabled: false,
        relu_enabled: false,
        bias_enabled: false,
        input_size: Some(InputSize {
            channels: input.channels,
            width: input.layer_width,
            height: input.layer_height,
        }),
        kernel_size: Some(KernelSize {
            channels: kernel.channels,
            width: kernel.layer_width,
            height: kernel.layer_height,
        }),
        padding: Some(PaddingConfig {
            top: 0,
            right: 0,
            left: 0,
            bottom: 0,
            value: 0,
        }),
        stride: Some(StrideConfig { x: 1, y: 1 }),
        mac_clip: Some(8),
        pp_clip: Some(8),
        simd_mode: Some(SimdBitMode::EightBits),
    };

    dla.init_layer(config);

    dla.write_input(input.elems);
    dla.write_kernel(kernel.elems);
    // Mark data ready to start calculations
    dla.kernel_data_ready(true);
    dla.input_data_ready(true);

    // Print the matrix
    sprintln!("Waiting for calculation");
    while !dla.handle_handshake() {}
    sprintln!("Calculation ready");

    let output_channels: usize = input.channels * kernel.channels;
    let output: Vec<i8> = dla.read_output(output_channels * output_width as usize * output_height as usize);
    Vec3 { elems: output, channels: output_channels, layer_width: output_width, layer_height: output_height }
}

pub fn dla_conv2d_relu(input: Vec3<i8>, kernel: Vec3<i8>) -> Vec3<i8> {
    let mut dla = Dla::new();
    // Calculate output size
    let (output_width, output_height) = calculate_conv2d_out_param_dim(
        (input.layer_width, input.layer_height),
        (kernel.layer_width, kernel.layer_height),
        (0, 0),
        (1, 1),
        (1, 1),
    );

    // Initalize layer
    let config = LayerConfig {
        input_bank: Some(MemoryBank::Bank0),
        kernel_bank: Some(MemoryBank::Bank8),
        output_bank: Some(MemoryBank::Bank12),
        bias_addr: 0,
        pp_enabled: true,
        relu_enabled: true,
        bias_enabled: false,
        input_size: Some(InputSize {
            channels: input.channels,
            width: input.layer_width,
            height: input.layer_height,
        }),
        kernel_size: Some(KernelSize {
            channels: kernel.channels,
            width: kernel.layer_width,
            height: kernel.layer_height,
        }),
        padding: Some(PaddingConfig {
            top: 0,
            right: 0,
            left: 0,
            bottom: 0,
            value: 0,
        }),
        stride: Some(StrideConfig { x: 1, y: 1 }),
        mac_clip: Some(8),
        pp_clip: Some(8),
        simd_mode: Some(SimdBitMode::EightBits),
    };

    dla.init_layer(config);

    dla.write_input(input.elems);
    dla.write_kernel(kernel.elems);
    // Mark data ready to start calculations
    dla.kernel_data_ready(true);
    dla.input_data_ready(true);

    // Print the matrix
    sprintln!("Waiting for calculation");
    while !dla.handle_handshake() {}
    sprintln!("Calculation ready");

    let output_channels: usize = input.channels * kernel.channels;
    let output: Vec<i8> = dla.read_output(output_channels * output_width as usize * output_height as usize);
    Vec3 { elems: output, channels: output_channels, layer_width: output_width, layer_height: output_height }
}

pub fn dla_conv2d_bias(input: Vec3<i8>, kernel: Vec3<i8>, bias: Vec<i8>) -> Vec3<i8> {
    let mut dla = Dla::new();
    // Calculate output size
    let (output_width, output_height) = calculate_conv2d_out_param_dim(
        (input.layer_width, input.layer_height),
        (kernel.layer_width, kernel.layer_height),
        (0, 0),
        (1, 1),
        (1, 1),
    );

    // Initalize layer
    let config = LayerConfig {
        input_bank: Some(MemoryBank::Bank0),
        kernel_bank: Some(MemoryBank::Bank8),
        output_bank: Some(MemoryBank::Bank12),
        bias_addr: 0,
        pp_enabled: true,
        relu_enabled: false,
        bias_enabled: true,
        input_size: Some(InputSize {
            channels: input.channels,
            width: input.layer_width,
            height: input.layer_height,
        }),
        kernel_size: Some(KernelSize {
            channels: kernel.channels,
            width: kernel.layer_width,
            height: kernel.layer_height,
        }),
        padding: Some(PaddingConfig {
            top: 0,
            right: 0,
            left: 0,
            bottom: 0,
            value: 0,
        }),
        stride: Some(StrideConfig { x: 1, y: 1 }),
        mac_clip: Some(8),
        pp_clip: Some(8),
        simd_mode: Some(SimdBitMode::EightBits),
    };

    dla.init_layer(config);

    //dla.write_bias(bias)

    dla.write_input(input.elems);
    dla.write_kernel(kernel.elems);
    // Mark data ready to start calculations
    dla.kernel_data_ready(true);
    dla.input_data_ready(true);

    // Print the matrix
    sprintln!("Waiting for calculation");
    while !dla.handle_handshake() {}
    sprintln!("Calculation ready");

    let output_channels: usize = input.channels * kernel.channels;
    let output: Vec<i8> = dla.read_output(output_channels * output_width as usize * output_height as usize);
    Vec3 { elems: output, channels: output_channels, layer_width: output_width, layer_height: output_height }
}

pub fn dla_conv2d_bias_relu(input: Vec3<i8>, kernel: Vec3<i8>, bias: Vec<i8>) -> Vec3<i8> {
    let mut dla = Dla::new();
    // Calculate output size
    let (output_width, output_height) = calculate_conv2d_out_param_dim(
        (input.layer_width, input.layer_height),
        (kernel.layer_width, kernel.layer_height),
        (0, 0),
        (1, 1),
        (1, 1),
    );

    // Initalize layer
    let config = LayerConfig {
        input_bank: Some(MemoryBank::Bank0),
        kernel_bank: Some(MemoryBank::Bank8),
        output_bank: Some(MemoryBank::Bank12),
        bias_addr: 0,
        pp_enabled: true,
        relu_enabled: true,
        bias_enabled: true,
        input_size: Some(InputSize {
            channels: input.channels,
            width: input.layer_width,
            height: input.layer_height,
        }),
        kernel_size: Some(KernelSize {
            channels: kernel.channels,
            width: kernel.layer_width,
            height: kernel.layer_height,
        }),
        padding: Some(PaddingConfig {
            top: 0,
            right: 0,
            left: 0,
            bottom: 0,
            value: 0,
        }),
        stride: Some(StrideConfig { x: 1, y: 1 }),
        mac_clip: Some(8),
        pp_clip: Some(8),
        simd_mode: Some(SimdBitMode::EightBits),
    };

    dla.init_layer(config);

    //dla.write_bias(bias)

    dla.write_input(input.elems);
    dla.write_kernel(kernel.elems);
    // Mark data ready to start calculations
    dla.kernel_data_ready(true);
    dla.input_data_ready(true);

    // Print the matrix
    sprintln!("Waiting for calculation");
    while !dla.handle_handshake() {}
    sprintln!("Calculation ready");

    let output_channels: usize = input.channels * kernel.channels;
    let output: Vec<i8> = dla.read_output(output_channels * output_width as usize * output_height as usize);
    Vec3 { elems: output, channels: output_channels, layer_width: output_width, layer_height: output_height }
}
