#![no_std]

use headsail_bsp::{sprint, sprintln};

use crate::tensor3::{Order3, Tensor3};
use crate::tensor4::{Order4, Tensor4};
use crate::{Dla, InputSize, KernelSize, LayerConfig, MemoryBank, Padding, SimdBitMode, Stride};

use crate::utils::{calculate_conv2d_out_param_dim, calculate_number_of_banks_needed};

pub fn conv2d(
    input: Tensor3<i8>,
    kernels: Tensor4<i8>,
    padding: Padding,
    stride: Stride,
) -> Tensor3<i32> {
    let output_size = calculate_conv2d_out_param_dim(
        (input.width as u32, input.height as u32),
        (kernels.width as u32, kernels.height as u32),
        (padding.top, padding.right),
        (stride.x, stride.y),
        (1, 1),
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
        padding: Some(padding),
        stride: Some(stride),
        mac_clip: Some(0),
        pp_clip: Some(8),
        simd_mode: Some(SimdBitMode::EightBits),
    };

    dla.init_layer(config);

    input.print_tensor();
    kernels.print_tensor();

    dla.write_input(&mut input.to_buffer_with_order(Order3::HWC));
    dla.write_kernel(&mut kernels.to_buffer_with_order(Order4::HWCK));

    // Mark data ready to start calculations
    dla.kernel_data_ready(true);
    dla.input_data_ready(true);

    while !dla.handle_handshake() {}
    let output_buffer = dla.read_output_i32(output_size.0 * output_size.1 * kernels.kernels);

    for x in &output_buffer {
        sprint!("{} ", x)
    }

    let output: Tensor3<i32> = Tensor3::from_data_buffer(
        kernels.kernels,
        output_size.1,
        output_size.0,
        output_buffer,
        Order3::WHC, // NOTE: (20240610 vaino-waltteri.granat@tuni.fi) This might not be true on ASIC
    )
    .unwrap();
    output
}
