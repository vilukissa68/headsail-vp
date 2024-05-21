#![no_std]
#![no_main]

extern crate alloc;

use dla_driver::*;
use headsail_bsp::{init_alloc, rt::entry, sprint, sprintln};
use panic_halt as _;

mod test_data;
use test_data::{conv_16x16x16_3x3_din, conv_16x16x16_3x3_dout, conv_16x16x16_3x3_wgt};

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

fn validate_conv2d() {
    let mut dla = Dla::new();

    let mut din: Vec<i8> = conv_16x16x16_3x3_din::DATA
        .iter()
        .map(|&x| x as i8)
        .collect();
    let mut dout: Vec<i32> = conv_16x16x16_3x3_dout::DATA
        .iter()
        .map(|&x| x as i32)
        .collect();
    let mut wgt: Vec<i8> = conv_16x16x16_3x3_wgt::DATA
        .iter()
        .map(|&x| x as i8)
        .collect();

    // Calculate output size
    let (output_width, output_height) =
        calculate_conv2d_out_param_dim((16, 16), (16, 16), (0, 0), (1, 1), (1, 1));

    // Initalize layer
    let config = LayerConfig {
        input_bank: Some(MemoryBank::Bank14), // b
        kernel_bank: Some(MemoryBank::Bank0), // a
        output_bank: Some(MemoryBank::Bank15),
        bias_addr: Some(0),
        pp_enabled: false,
        relu_enabled: false,
        bias_enabled: false,
        input_size: Some(InputSize {
            channels: 16,
            width: 16,
            height: 16,
        }),
        kernel_size: Some(KernelSize {
            s_channels: 1,
            channels: 16,
            width: 3,
            height: 3,
        }),
        padding: Some(PaddingConfig {
            top: 0,
            right: 0,
            left: 0,
            bottom: 0,
            padding_value: 0,
        }),
        stride: Some(StrideConfig { x: 1, y: 1 }),
        mac_clip: Some(8),
        pp_clip: Some(8),
        simd_mode: Some(SimdBitMode::EightBits),
    };

    dla.init_layer(config);
    sprintln!("Layer configured");

    dla.write_input(&mut din);
    dla.write_kernel(&mut wgt);

    // Mark data ready to start calculations
    dla.kernel_data_ready(true);
    dla.input_data_ready(true);

    while !dla.handle_handshake() {}
    let output = dla.read_output(output_width as usize * output_height as usize);
}
#[entry]
fn main() -> ! {
    init_alloc();
    sprint!("Validate conv2d");
    validate_conv2d();
    sprint!("Validation conv2d succesful");
    loop {}
}