#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;

use dla_driver::*;
use headsail_bsp::{
    init_alloc, rt::entry, sprint, sprintln, tb::report_fail, tb::report_ok, tb::report_pass,
    uart::uart_read_to_heap,
};
use panic_halt as _;

use alloc::vec::Vec;

fn calculate_conv2d_out_param_dim(
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

fn validate_conv2d_tiny() -> bool {
    let dla = Dla::new();

    sprintln!("din\r\n");
    let mut din: Vec<i8> = uart_read_to_heap(75).into_iter().map(|x| x as i8).collect();

    sprintln!("wgt\r\n");
    let mut wgt: Vec<i8> = uart_read_to_heap(54).into_iter().map(|x| x as i8).collect();

    sprintln!("dout\r\n");
    let dout_i32: Vec<i32> = uart_read_to_heap(18)
        .into_iter()
        .map(|x| x as i8)
        .map(i32::from)
        .collect();

    // Calculate output size
    let output_size = calculate_conv2d_out_param_dim((5, 5), (3, 3), (0, 0), (1, 1), (1, 1));

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
            channels: 3,
            width: 5,
            height: 5,
        }),
        kernel_size: Some(KernelSize {
            s_channels: 1,
            kernels: 2,
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
        mac_clip: Some(0),
        pp_clip: Some(8),
        simd_mode: Some(SimdBitMode::EightBits),
    };

    dla.init_layer(config);

    dla.write_input(&mut din);
    dla.write_kernel(&mut wgt);

    // Mark data ready to start calculations
    dla.kernel_data_ready(true);
    dla.input_data_ready(true);

    while !dla.handle_handshake() {}
    let output: Vec<i32> = dla.read_output_i32(output_size.0 * output_size.1 * 2);

    output == dout_i32
}

fn validate_conv2d() -> bool {
    let mut dla = Dla::new();

    sprintln!("din\r\n");
    let mut din: Vec<i8> = uart_read_to_heap(4096)
        .into_iter()
        .map(|x| x as i8)
        .collect();

    sprintln!("wgt\r\n");
    let mut wgt: Vec<i8> = uart_read_to_heap(2304)
        .into_iter()
        .map(|x| x as i8)
        .collect();

    sprintln!("dout\r\n");
    let dout: Vec<u8> = uart_read_to_heap(3136 * 4);
    let mut dout_i32: Vec<i32> = Vec::with_capacity(3136);

    for chunk in dout.chunks(4) {
        let mut value: u32 = 0;
        for (i, &byte) in chunk.iter().rev().enumerate() {
            value |= (byte as u32) << (8 * i);
        }
        dout_i32.push(value as i32);
    }

    // Calculate output size
    let output_size = calculate_conv2d_out_param_dim((16, 16), (3, 3), (0, 0), (1, 1), (1, 1));

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
            channels: 16,
            width: 16,
            height: 16,
        }),
        kernel_size: Some(KernelSize {
            s_channels: 1,
            kernels: 16,
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
        mac_clip: Some(0),
        pp_clip: Some(8),
        simd_mode: Some(SimdBitMode::EightBits),
    };

    dla.init_layer(config);

    dla.write_input(&mut din);
    dla.write_kernel(&mut wgt);

    // Mark data ready to start calculations
    dla.kernel_data_ready(true);
    dla.input_data_ready(true);

    while !dla.handle_handshake() {}
    let output = dla.read_output_i32(output_size.0 * output_size.1 * 16);

    output == dout_i32
}

#[entry]
fn main() -> ! {
    init_alloc();
    sprintln!("Validate conv2d");
    let mut succesful_test = 0;
    if validate_conv2d_tiny() {
        report_ok();
        sprintln!(" Tiny test succesful");
        succesful_test += 1;
    } else {
        report_fail();
        sprintln!(" Tiny test failed");
    }
    if validate_conv2d() {
        report_ok();
        sprintln!(" 16x16x16_3x3 conv2d test succesful");
        succesful_test += 1;
    } else {
        report_fail();
        sprintln!(" 16x16x16_3x3 conv2d test failed");
    }

    if succesful_test == 2 {
        report_pass();
        sprintln!(" All tests succesful!\r\n");
    } else {
        report_fail();
        sprintln!(" Not all tests succesful!\r\n");
    }

    loop {}
}
