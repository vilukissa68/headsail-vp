use crate::tensor3::{Order3, Tensor3};
use crate::tensor4::{Order4, Tensor4};
use crate::{Dla, InputSize, KernelSize, LayerConfig, Padding, SimdBitMode, Stride};
use alloc::vec::Vec;

use headsail_bsp::sprintln;

use crate::utils::{calculate_conv2d_out_param_dim, get_banks_for_layer};

// Define a trait for output handling
pub trait DlaOutput: Sized {
    fn read_output(dla: &Dla, size: usize) -> Vec<Self>;
}

// Implement the trait for i8
impl DlaOutput for i8 {
    fn read_output(dla: &Dla, size: usize) -> Vec<Self> {
        dla.read_output_i8(size)
    }
}

// Implement the trait for i16
impl DlaOutput for i16 {
    fn read_output(dla: &Dla, size: usize) -> Vec<Self> {
        dla.read_output_i16(size)
    }
}

// Implement the trait for i32
impl DlaOutput for i32 {
    fn read_output(dla: &Dla, size: usize) -> Vec<Self> {
        dla.read_output_i32(size)
    }
}

pub fn dense(outputs: usize, input: Tensor3<i8>, weights: Vec<i8>) -> Vec<i32> {
    // Build kernels to produce 1 to 1 mac operation
    let kernels_wrap = Tensor4::from_data_buffer(
        outputs,
        input.channels(),
        input.height(),
        input.width(),
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

pub fn conv2d<T: DlaOutput + Clone>(
    input: Tensor3<i8>,
    kernels: Tensor4<i8>,
    padding: Option<Padding>,
    stride: Option<Stride>,
    mac_clip: Option<u32>,
    pp_clip: Option<u32>,
    simd_mode: Option<SimdBitMode>,
) -> Tensor3<T> {
    run_layers(
        input, kernels, None, false, false, padding, stride, mac_clip, pp_clip, simd_mode,
    )
}

pub fn relu(input: Tensor3<i8>, pp_clip: Option<u32>) -> Tensor3<i8> {
    let kernel_buf = vec![1; input.get_size() * input.channels()]; // 1 filled kernels for constant conv2d
    let kernels: Tensor4<i8> = Tensor4::from_data_buffer(
        input.channels(),
        input.channels(),
        input.height(),
        input.width(),
        kernel_buf,
        Order4::HWKC,
    )
    .unwrap();

    run_layers(
        input,
        kernels,
        None,
        false,
        true,
        None,
        None,
        Some(0),
        pp_clip,
        Some(SimdBitMode::EightBits),
    )
}

pub fn bias(input: Tensor3<i8>, bias: Vec<i16>, pp_clip: Option<u32>) -> Tensor3<i8> {
    let kernel_buf = vec![1; input.get_size() * input.channels()]; // 1 filled kernels for constant conv2d
    let kernels: Tensor4<i8> = Tensor4::from_data_buffer(
        input.channels(),
        input.channels(),
        input.height(),
        input.width(),
        kernel_buf,
        Order4::HWKC,
    )
    .unwrap();

    run_layers(
        input,
        kernels,
        Some(bias),
        true,
        false,
        None,
        None,
        Some(0),
        pp_clip,
        Some(SimdBitMode::EightBits),
    )
}

pub fn conv2d_relu<T: DlaOutput + Clone>(
    input: Tensor3<i8>,
    kernels: Tensor4<i8>,
    padding: Option<Padding>,
    stride: Option<Stride>,
    mac_clip: Option<u32>,
    pp_clip: Option<u32>,
    simd_mode: Option<SimdBitMode>,
) -> Tensor3<T> {
    run_layers(
        input, kernels, None, false, true, padding, stride, mac_clip, pp_clip, simd_mode,
    )
}

pub fn conv2d_bias<T: DlaOutput + Clone>(
    input: Tensor3<i8>,
    kernels: Tensor4<i8>,
    bias: Vec<i16>,
    padding: Option<Padding>,
    stride: Option<Stride>,
    mac_clip: Option<u32>,
    pp_clip: Option<u32>,
    simd_mode: Option<SimdBitMode>,
) -> Tensor3<T> {
    run_layers(
        input,
        kernels,
        Some(bias),
        true,
        false,
        padding,
        stride,
        mac_clip,
        pp_clip,
        simd_mode,
    )
}

pub fn conv2d_bias_relu<T: DlaOutput + Clone>(
    input: Tensor3<i8>,
    kernels: Tensor4<i8>,
    bias: Vec<i16>,
    padding: Option<Padding>,
    stride: Option<Stride>,
    mac_clip: Option<u32>,
    pp_clip: Option<u32>,
    simd_mode: Option<SimdBitMode>,
) -> Tensor3<T> {
    run_layers(
        input,
        kernels,
        Some(bias),
        true,
        true,
        padding,
        stride,
        mac_clip,
        pp_clip,
        simd_mode,
    )
}

pub fn grouped_conv2d<T: DlaOutput + Clone>(
    input: Tensor3<i8>,
    kernels: Tensor4<i8>,
    bias: Vec<i16>,
    padding: Option<Padding>,
    stride: Option<Stride>,
    mac_clip: Option<u32>,
    pp_clip: Option<u32>,
    simd_mode: Option<SimdBitMode>,
    groups: usize
) -> Tensor3<T> {
    let total_in_channels = input.channels();
    let total_out_channels = kernels.kernels();
    let group_in_channels = total_in_channels / groups;
    let group_out_channels = kernels.kernels() / groups;

    // Placeholder for the output tensor
    let mut output_tensors = Vec::new();

    for g in 0..groups {
        let input_group = input.slice_channels(g * group_in_channels..(g + 1)*group_in_channels);
        let kernels_group = kernels.slice_channels(g * group_in_channels..(g + 1)*group_in_channels);
        let bias_group = bias[g * group_out_channels..(g + 1) * group_out_channels].to_vec();

        let output_group = run_layers(
            input_group,
            kernels_group,
            Some(bias_group),
            true,
            false,
            padding.clone(),
            stride.clone(),
            mac_clip,
            pp_clip,
            simd_mode,
        );

        output_tensors.push(output_group);
    }

    // Find channel axis
    //let axis = output_tensors[0].

    // Concatenate the output tensors along the channel dimension
    let res = Tensor3::concat_interleaved(output_tensors, 2);
    //let res = Tensor3::concat(output_tensors, 0);

    sprintln!("Res shape: {} {} {}", res.dimensions().0, res.dimensions().1, res.dimensions().2);
    res

}


fn run_layers<T: DlaOutput + Clone>(
    input: Tensor3<i8>,
    kernels: Tensor4<i8>,
    bias: Option<Vec<i16>>,
    bias_enabled: bool,
    relu_enabled: bool,
    padding: Option<Padding>,
    stride: Option<Stride>,
    mac_clip: Option<u32>,
    pp_clip: Option<u32>,
    simd_mode: Option<SimdBitMode>,
) -> Tensor3<T> {
    let output_size = calculate_conv2d_out_param_dim(
        (input.width() as u32, input.height() as u32),
        (kernels.width() as u32, kernels.height() as u32),
        padding.clone(),
        stride.clone(),
    );

    let dla = Dla::new();

    let banks = get_banks_for_layer(
        input.get_size(),
        kernels.get_size(),
        output_size.0 * output_size.1,
    );

    // Initalize layer
    let config = LayerConfig {
        input_bank: Some(banks.0),  // b
        kernel_bank: Some(banks.1), // a
        output_bank: Some(banks.2),
        bias_addr: banks.3,
        pp_enabled: relu_enabled || bias_enabled,
        relu_enabled,
        bias_enabled,
        input_size: Some(InputSize {
            channels: input.channels() as u32,
            width: input.width() as u32,
            height: input.height() as u32,
        }),
        kernel_size: Some(KernelSize {
            s_channels: 1,
            kernels: kernels.kernels() as u32,
            width: kernels.width() as u32,
            height: kernels.height() as u32,
        }),
        padding,
        stride,
        mac_clip,
        pp_clip,
        simd_mode,
    };

    dla.init_layer(config);

    dla.write_input(&mut input.to_buffer_with_order(Order3::HWC));
    dla.write_kernel(&mut kernels.to_buffer_with_order(Order4::HWKC));

    if let Some(bias) = bias {
        dla.write_bias(&bias)
    }

    // Mark data ready to start calculations
    dla.kernel_data_ready(true);
    dla.input_data_ready(true);

    while !dla.handle_handshake() {}

    let output_buffer = T::read_output(&dla, output_size.0 * output_size.1 * kernels.kernels());

    Tensor3::from_data_buffer(
        kernels.kernels(),
        output_size.1,
        output_size.0,
        output_buffer,
        Order3::HWC, // NOTE: (20240610 vaino-waltteri.granat@tuni.fi) This might not be true on ASIC
    )
    .unwrap()
}
