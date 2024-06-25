#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;

use dla_driver::{Padding, Stride};
use headsail_bsp::{init_alloc, rt::entry, sprint, sprintln};
use panic_halt as _;

use dla_driver::tensor3::{Order3, Tensor3};
use dla_driver::tensor4::{Order4, Tensor4};
mod test_data;
use test_data::{conv_16x16x16_3x3_din, conv_16x16x16_3x3_dout, conv_16x16x16_3x3_wgt};

use alloc::vec::Vec;

#[entry]
fn main() -> ! {
    init_alloc();

    let din: Vec<i8> = vec![
        0, 0, 0, 2, 0, 0, 1, 2, 1, 2, 0, 0, 1, 2, 0, 1, 0, 0, 0, 2, 0, 0, 1, 0, 1, 2, 0, 1, 0, 1,
        0, 0, 2, 2, 1, 1, 0, 2, 1, 1, 2, 1, 2, 2, 1, 0, 0, 1, 1, 2, 0, 1, 1, 1, 0, 0, 2, 0, 1, 2,
        1, 0, 0, 1, 2, 1, 1, 1, 0, 0, 1, 1, 2, 0, 2,
    ];
    let wgt: Vec<i8> = vec![
        -1, -1, 0, -1, 0, 0, -1, -1, 1, 0, 0, 1, 1, -1, -1, 1, -1, 0, 1, 0, -1, -1, 1, -1, -1, 0,
        -1, 1, 0, 0, -1, 0, 1, 0, -1, 1, 0, 1, -1, -1, 0, 0, 0, -1, -1, 0, -1, 1, -1, -1, -1, 0, 1,
        0,
    ];
    let mut dout: Vec<i32> = vec![
        1, -10, -8, -3, -6, -5, -2, -7, -10, -2, -4, -10, -7, 0, -3, -7, -2, -1,
    ];

    let mut din_tensor: Tensor3<i8> = Tensor3::from_data_buffer(3, 5, 5, din, Order3::CHW).unwrap();
    let mut wgt_tensor: Tensor4<i8> =
        Tensor4::from_data_buffer(2, 3, 3, 3, wgt, Order4::KCHW).unwrap();
    let mut dout_tensor: Tensor3<i32> =
        Tensor3::from_data_buffer(2, 3, 3, dout, Order3::CHW).unwrap();

    let padding = Padding {
        top: 0,
        left: 0,
        right: 0,
        bottom: 0,
        padding_value: 0,
    };
    let stride = Stride { x: 1, y: 1 };
    let mut output = dla_driver::layers::conv2d(din_tensor, wgt_tensor, padding, stride);
    output.transmute(Order3::CWH);
    output.print_tensor();
    dout_tensor.print_tensor();

    // Larger
    // let mut din_large: Vec<i8> = conv_16x16x16_3x3_din::DATA
    //     .iter()
    //     .map(|&x| x as i8)
    //     .collect();
    // let mut dout_large: Vec<i32> = conv_16x16x16_3x3_dout::DATA
    //     .iter()
    //     .map(|&x| x as i32)
    //     .collect();
    // let mut wgt_large: Vec<i8> = conv_16x16x16_3x3_wgt::DATA
    //     .iter()
    //     .map(|&x| x as i8)
    //     .collect();
    // let mut din_large_tensor: Tensor3<i8> =
    //     Tensor3::from_data_buffer(16, 16, 16, din_large, Order3::CHW).unwrap();
    // let mut wgt_large_tensor: Tensor4<i8> =
    //     Tensor4::from_data_buffer(16, 16, 3, 3, wgt_large, Order4::KCHW).unwrap();
    // let padding = Padding {
    //     top: 0,
    //     left: 0,
    //     right: 0,
    //     bottom: 0,
    //     padding_value: 0,
    // };
    // let stride = Stride { x: 1, y: 1 };
    // let mut output =
    //     dla_driver::layers::conv2d(din_large_tensor, wgt_large_tensor, padding, stride);
    // output.transmute(Order3::CWH);
    // output.print_tensor();

    loop {}
}
