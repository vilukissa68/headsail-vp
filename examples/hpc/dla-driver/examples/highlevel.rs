#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;

use dla_driver::{Padding, Stride};
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

    let mut din_tensor: Tensor3<i8> = Tensor3::from_data_buffer(3, 5, 5, din, Order3::CHW).unwrap();
    let mut wgt_tensor: Tensor4<i8> =
        Tensor4::from_data_buffer(2, 3, 3, 3, wgt, Order4::KCHW).unwrap();

    let padding = Padding {
        top: 0,
        left: 0,
        right: 0,
        bottom: 0,
        padding_value: 0,
    };
    let stride = Stride { x: 1, y: 1 };
    let mut output = dla_driver::layers::conv2d(din_tensor, wgt_tensor, padding, stride);
    output.transmute(Order3::CHW);
    sprint!("here");
    output.print_tensor();
    sprint!("here end");

    loop {}
}
