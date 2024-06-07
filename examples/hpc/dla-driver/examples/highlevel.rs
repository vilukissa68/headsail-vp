#![no_std]
#![no_main]

extern crate alloc;

use dla_driver::{LayerConfig, DLA};
use headsail_bsp::{init_alloc, rt::entry, sprint, sprintln};
use panic_halt as _;

use dla_driver::tensor3::{Order3, Tensor3};
use dla_driver::tensor4::{Order4, Tensor4};
use dla_driver::utils::calculate_conv2d_out_param_dim;
use rand::rngs::SmallRng;
use rand::RngCore;
use rand::SeedableRng;

use alloc::vec::Vec;

#[entry]
fn main() -> ! {
    init_alloc();

    let din: Vec<i8> = [
        0, 0, 0, 2, 0, 0, 1, 2, 1, 2, 0, 0, 1, 2, 0, 1, 0, 0, 0, 2, 0, 0, 1, 0, 1, 2, 0, 1, 0, 1,
        0, 0, 2, 2, 1, 1, 0, 2, 1, 1, 2, 1, 2, 2, 1, 0, 0, 1, 1, 2, 0, 1, 1, 1, 0, 0, 2, 0, 1, 2,
        1, 0, 0, 1, 2, 1, 1, 1, 0, 0, 1, 1, 2, 0, 2,
    ]
    .to_vec();
    let wgt: Vec<i8> = [
        -1, -1, 0, -1, 0, 0, -1, -1, 1, 0, 0, 1, 1, -1, -1, 1, -1, 0, 1, 0, -1, -1, 1, -1, -1, 0,
        -1, 1, 0, 0, -1, 0, 1, 0, -1, 1, 0, 1, -1, -1, 0, 0, 0, -1, -1, 0, -1, 1, -1, -1, -1, 0, 1,
        0,
    ]
    .to_vec();

    let mut din_tensor: Tensor3<i8> = Tensor3::from_data_buffer(3, 5, 5, din, Order3::CWH).unwrap();

    din_tensor.set_order(Order3::WHC);

    din_tensor.print_tensor();

    // for x in din_tensor.to_buffer_with_order(Order::CWH) {
    //     sprint!("{} ", x)
    // }

    let mut wgt_tensor: Tensor4<i8> =
        Tensor4::from_data_buffer(2, 3, 3, 3, wgt, Order4::KCHW).unwrap();

    wgt_tensor.set_order(Order4::WHKC);

    wgt_tensor.print_tensor();
    let output_size = calculate_conv2d_out_param_dim((5, 5), (3, 3), (0, 0), (1, 1), (1, 1));

    let dla = DLA::new();

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

    dla.write_input(&mut din_tensor.to_buffer_with_order(Order3::HWC));
    dla.write_kernel(&mut wgt_tensor.to_buffer_with_order(Order4::HWKC));

    // Mark data ready to start calculations
    dla.kernel_data_ready(true);
    dla.input_data_ready(true);

    while !dla.handle_handshake() {}
    let output = dla.read_output_i32(output_size.0 * output_size.1 * 2);

    loop {}
}
