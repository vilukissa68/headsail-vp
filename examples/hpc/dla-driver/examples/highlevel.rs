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

fn dense_test() {
    sprintln!("Starting dense test");
    let din: Vec<i8> = vec![
        1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 1, 2, 3, 4, 5,
    ];
    let mut din_tensor: Tensor3<i8> = Tensor3::from_data_buffer(1, 5, 5, din, Order3::CHW).unwrap();
    sprintln!("Din tensor created");

    let dense_weight_size = din_tensor.height * din_tensor.width * din_tensor.channels * 5;

    let mut weight: Vec<i8> = vec![];
    for _ in 0..dense_weight_size {
        weight.push(1)
    }
    sprintln!("Weight tensor created with length {}", weight.len());

    let output = dla_driver::layers::dense(5, din_tensor, weight);
    sprint!("Finished");
}

fn conv_test() {
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

    let mut output =
        dla_driver::layers::conv2d(din_tensor, wgt_tensor, None, None, None, None, None);
    //output.transmute(Order3::CWH);
    output.print_tensor();
    dout_tensor.print_tensor();

    // Larger
    let mut din_large: Vec<i8> = conv_16x16x16_3x3_din::DATA
        .iter()
        .map(|&x| x as i8)
        .collect();
    let mut dout_large: Vec<i32> = conv_16x16x16_3x3_dout::DATA
        .iter()
        .map(|&x| x as i32)
        .collect();
    let mut wgt_large: Vec<i8> = conv_16x16x16_3x3_wgt::DATA
        .iter()
        .map(|&x| x as i8)
        .collect();
    let mut din_large_tensor: Tensor3<i8> =
        Tensor3::from_data_buffer(16, 16, 16, din_large, Order3::CHW).unwrap();
    let mut wgt_large_tensor: Tensor4<i8> =
        Tensor4::from_data_buffer(16, 16, 3, 3, wgt_large, Order4::KCHW).unwrap();

    let mut output = dla_driver::layers::conv2d(
        din_large_tensor,
        wgt_large_tensor,
        None,
        None,
        None,
        None,
        None,
    );
    output.transmute(Order3::CWH);
    output.print_tensor();

    let test: Vec<i8> = vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26,
    ];
    let mut test_tensor: Tensor3<i8> =
        Tensor3::from_data_buffer(3, 3, 3, test, Order3::HCW).unwrap();
    test_tensor.transmute(Order3::CHW);
    test_tensor.print_tensor();
}

#[entry]
fn main() -> ! {
    init_alloc();
    dense_test();
    conv_test();

    loop {}
}
