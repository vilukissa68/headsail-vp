#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;

use headsail_bsp::{init_alloc, rt::entry, sprint, sprintln};
use panic_halt as _;

use dla_driver::tensor3::{Order3, Tensor3};
use dla_driver::tensor4::{Order4, Tensor4};

use alloc::vec::Vec;

fn conv_test() {
    sprintln!("conv_test: enter");

	#[rustfmt:skip]
    let din: Vec<i8> = vec![
        1, 2, 3, 4, 5,
		1, 2, 3, 4, 5,
		1, 2, 3, 4, 5,
		1, 2, 3, 4, 5,
		1, 2, 3, 4, 5,

		5, 4, 3, 2, 1,
		5, 4, 3, 2, 1,
		5, 4, 3, 2, 1,
		5, 4, 3, 2, 1,
		5, 4, 3, 2, 1,

		3, 2, 1, 4, 5,
		3, 2, 1, 4, 5,
		3, 2, 1, 4, 5,
		3, 2, 1, 4, 5,
		3, 2, 1, 4, 5,

		2, 4, 5, 1, 3,
		2, 4, 5, 1, 3,
		2, 4, 5, 1, 3,
		2, 4, 5, 1, 3,
		2, 4, 5, 1, 3,
    ];

	#[rustfmt:skip]
    let wgt: Vec<i8> = vec![
		1,2,3,
		4,5,6,
		7,8,9,

		9,8,7,
		4,5,6,
		1,2,3,

		1,4,9,
		2,5,8,
		3,6,7,

		9,4,1,
		8,5,2,
		7,6,3,

		1,2,3,
		4,5,6,
		7,8,9,

		9,8,7,
		4,5,6,
		1,2,3,

		1,4,9,
		2,5,8,
		3,6,7,

		9,4,1,
		8,5,2,
		7,6,3,
	];

	let bias: Vec<i16> = vec![-16, 16, 0, 0, 8, -8, -36, 36];

    let din_tensor: Tensor3<i8> = Tensor3::from_data_buffer(4, 5, 5, din, Order3::CHW).unwrap();
    let wgt_tensor: Tensor4<i8> = Tensor4::from_data_buffer(2, 4, 3, 3, wgt, Order4::KCHW).unwrap();

    sprintln!("Data loaded");
    let mut output: Tensor3<i8> =
        dla_driver::layers::grouped_conv2d(din_tensor, wgt_tensor, bias, None, None, None, None, None, 4);
    output.permute(Order3::CWH);

    sprintln!("Output dim: {} {} {}", output.dimensions().0, output.dimensions().1, output.dimensions().2);
	for x in output.to_buffer() {
		sprint!(" {}", x);
	}
    sprintln!("\nconv_test: leave");
}

#[entry]
fn main() -> ! {
    init_alloc();
    conv_test();

    loop {}
}
