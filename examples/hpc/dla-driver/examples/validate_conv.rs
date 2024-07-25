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
use dla_driver::tensor3::{Order3, Tensor3};
use dla_driver::tensor4::{Order4, Tensor4};

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

    sprintln!("Inputs read\r\n");

    let din_tensor: Tensor3<i8> = Tensor3::from_data_buffer(3, 5, 5, din, Order3::HWC).unwrap();
    let wgt_tensor: Tensor4<i8> = Tensor4::from_data_buffer(2, 3, 3, 3, wgt, Order4::HWKC).unwrap();
    let mut dout_tensor: Tensor3<i32> =
        Tensor3::from_data_buffer(2, 3, 3, dout_i32.clone(), Order3::HWC).unwrap();

    let padding = Padding {
        top: 0,
        left: 0,
        right: 0,
        bottom: 0,
        padding_value: 0,
    };
    let stride = Stride { x: 1, y: 1 };

    let output = dla_driver::layers::conv2d(din_tensor, wgt_tensor, padding, stride);
    output.print_tensor();

    sprint!("\ndla out | dout\n");
    let dout_tensor_buf = dout_tensor.to_buffer();
    let output_tensor_buf = output.to_buffer();
    for (i, x) in output_tensor_buf.into_iter().enumerate() {
        if x != dout_i32[i] {
            sprintln!("{}: {}=/={}", i, x, dout_tensor_buf[i])
        }
    }

    output.to_buffer() == dout_i32
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

    let din_tensor: Tensor3<i8> = Tensor3::from_data_buffer(16, 16, 16, din, Order3::HWC).unwrap();
    let wgt_tensor: Tensor4<i8> =
        Tensor4::from_data_buffer(16, 16, 3, 3, wgt, Order4::HWKC).unwrap();
    let dout_tensor: Tensor3<i32> =
        Tensor3::from_data_buffer(16, 14, 14, dout_i32.clone(), Order3::HWC).unwrap();

    let padding = Padding {
        top: 0,
        left: 0,
        right: 0,
        bottom: 0,
        padding_value: 0,
    };
    let stride = Stride { x: 1, y: 1 };

    let output = dla_driver::layers::conv2d(din_tensor, wgt_tensor, padding, stride);

    sprint!("\n");
    let dout_tensor_buf = dout_tensor.to_buffer();
    let output_tensor_buf = output.to_buffer();

    dout_tensor_buf == output_tensor_buf
}
fn validate_conv2d_bias() -> bool {
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
    let dout: Vec<i8> = uart_read_to_heap(1024)
        .into_iter()
        .map(|x| x as i8)
        .collect();

    sprintln!("bias\r\n");
    let bias: Vec<u8> = uart_read_to_heap(16 * 2);
    let mut bias_i16: Vec<i16> = Vec::with_capacity(16);

    for chunk in bias.chunks(2) {
        let mut value: u16 = 0;
        for (i, &byte) in chunk.iter().rev().enumerate() {
            value |= (byte as u16) << (8 * i);
        }
        bias_i16.push(value as i16);
    }

    // Calculate output size
    let output_size = calculate_conv2d_out_param_dim((16, 16), (3, 3), (1, 1), (1, 1), (2, 2));

    // Initalize layer
    let config = LayerConfig {
        input_bank: Some(MemoryBank::Bank8),  // b
        kernel_bank: Some(MemoryBank::Bank0), // a
        output_bank: Some(MemoryBank::Bank10),
        bias_addr: Some((MEMORY_BANK_12_OFFSET + MEMORY_BANK_BASE_ADDR) as u32),
        pp_enabled: true,
        relu_enabled: false,
        bias_enabled: true,
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
            right: 1,
            left: 0,
            bottom: 1,
            padding_value: 0,
        }),
        stride: Some(StrideConfig { x: 2, y: 2 }),
        mac_clip: Some(6),
        pp_clip: Some(4),
        simd_mode: Some(SimdBitMode::EightBits),
    };

    dla.init_layer(config);

    dla.write_input(&mut din);
    dla.write_kernel(&mut wgt);
    dla.write_bias(&mut bias_i16);

    // Mark data ready to start calculations
    dla.kernel_data_ready(true);
    dla.input_data_ready(true);

    while !dla.handle_handshake() {}
    let output = dla.read_output_i8(output_size.0 * output_size.1 * 16);

    for (i, x) in output.iter().enumerate() {
        if output[i] != dout[i] {
            sprint!("Diff found at {}, expected {}, found {}.", i, dout[i], x);
            return false;
        }
    }

    output == dout
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

    if validate_conv2d_bias() {
        report_ok();
        sprintln!(" 16x16x16_3x3 conv2d bias test succesful");
        succesful_test += 1;
    } else {
        report_fail();
        sprintln!(" 16x16x16_3x3 conv2d bias test failed");
    }

    if succesful_test == 3 {
        report_pass();
        sprintln!(" All tests succesful!\r\n");
    } else {
        report_fail();
        sprintln!(" Not all tests succesful!\r\n");
    }

    loop {}
}
