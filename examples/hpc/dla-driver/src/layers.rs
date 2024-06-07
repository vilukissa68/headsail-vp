#![no_std]

pub fn dla_conv2d(input: Tensor3, kernels: Tensor4, padding: Padding, stride: Stride) -> Tensor3 {
    let output_size = calculate_conv2d_out_param_dim(
        (input.width, input.height),
        (kernels.width, kernels.height),
        (padding.top, padding.right),
        (stride.x, stride.y),
        (1, 1),
    );
    let dla = DLA::new();

    // Calculate needed space
    let input_size = input.get_size();
    let kernels_size = kernels.get_size();

    // Initalize layer
    let config = LayerConfig {
        input_bank: Some(MemoryBank::Bank8),  // b
        kernel_bank: Some(MemoryBank::Bank0), // a
        output_bank: Some(MemoryBank::Bank10),
        bias_addr: Some(0),
        pp_enabled: false,
        relu_enabled: false,
        bias_enabled: false,
        input_size: Some(InputSize {
            channels: input.channels,
            width: input.width,
            height: input.height,
        }),
        kernel_size: Some(KernelSize {
            s_channels: 1,
            kernels: kernels.kernels,
            width: kernels.width,
            height: kernels.height,
        }),
        padding: Padding,
        stride: Stride,
        mac_clip: Some(0),
        pp_clip: Some(8),
        simd_mode: Some(SimdBitMode::EightBits),
    };

    dla.init_layer(config);

    dla.write_input(&mut input.to_buffer_with_order(Order3::HWC));
    dla.write_kernel(&mut kernels.to_buffer_with_order(Order4::HWKC));

    // Mark data ready to start calculations
    dla.kernel_data_ready(true);
    dla.input_data_ready(true);

    while !dla.handle_handshake() {}
    let output_buffer = dla.read_output_i32(output_size.0 * output_size.1 * 2);
    let output = Tensor3::from_data_buffer();

    loop {}
}
