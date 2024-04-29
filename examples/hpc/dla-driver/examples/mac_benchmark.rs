#![no_std]
#![no_main]

use headsail_bsp::{rt::entry, sprint, sprintln};
use dla_driver::*;
use panic_halt as _;

use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use rand::RngCore;

macro_rules! conv2d_out_parameters_height {
    (($input_h:expr, $kernel_h:expr, $padding_h:expr, $dilation_h:expr, $stride_h:expr)) => {
        {
            ($input_h + 2 * $padding_h - $dilation_h * ($kernel_h - 1) - 1) / $stride_h + 1
        }
    };
}

macro_rules! conv2d_out_parameters_width {
    (($input_w:expr, $kernel_w:expr, $padding_w:expr, $dilation_w:expr, $stride_w:expr)) => {
        {
            ($input_w + 2 * $padding_w - $dilation_w * ($kernel_w - 1) - 1) / $stride_w + 1
        }
    };
}

fn generate_random_array(buffer: &mut [u8], size: usize) {
    let mut rng = SmallRng::seed_from_u64(1234567890);
    for i in 0..size {
        buffer[i] = rng.next_u64() as u8;
    }
}

#[entry]
fn main() -> ! {
    sprintln!("Starting benchmark..");

    dla_init();

    const INPUT_WIDTH: usize = 10;
    const INPUT_HEIGHT: usize = 10;

    const KERNEL_WIDTH: usize = 3;
    const KERNEL_HEIGHT: usize = 3;

    const PADDING_HEIGHT: usize = 0;
    const PADDING_WIDTH: usize = 0;
    const DILATION_HEIGHT: usize = 1;
    const DILATION_WIDTH: usize = 1;
    const STRIDE_HEIGHT: usize = 1;
    const STRIDE_WIDTH: usize = 1;

    // Calculate output size
    const H_OUT: usize = conv2d_out_parameters_height!(
        (INPUT_HEIGHT, KERNEL_HEIGHT, PADDING_HEIGHT, DILATION_HEIGHT, STRIDE_HEIGHT)
    );

    const W_OUT: usize = conv2d_out_parameters_width!(
        (INPUT_WIDTH, KERNEL_WIDTH, PADDING_WIDTH, DILATION_WIDTH, STRIDE_WIDTH)
    );

    // Generate a random input matrix
    let mut input: [u8; INPUT_WIDTH * INPUT_HEIGHT] = [0; INPUT_WIDTH * INPUT_HEIGHT];
    // Generate a random kernel matrix
    let mut kernel: [u8; KERNEL_WIDTH * KERNEL_HEIGHT] = [0; KERNEL_WIDTH * KERNEL_HEIGHT];

    let mut output: [u8; H_OUT * W_OUT] = [0; H_OUT * W_OUT];

    generate_random_array(&mut input, INPUT_WIDTH * INPUT_HEIGHT);
    generate_random_array(&mut kernel, KERNEL_WIDTH * KERNEL_HEIGHT);

    dla_set_kernel_size(1, KERNEL_WIDTH, KERNEL_HEIGHT);
    dla_set_input_size(1, INPUT_WIDTH, INPUT_HEIGHT);

    dla_write_input(&mut input);
    dla_write_kernel(&mut kernel);

    dla_set_mac_clip(8);
    dla_set_pp_clip(8);

    dla_kernel_data_ready(true);
    dla_input_data_ready(true);

    // Print the matrix
    sprintln!("Waiting for calculation");
    while !dla_is_ready() {
    }
    sprintln!("Calculation ready");

    output =  dla_read_input_bank(H_OUT * W_OUT);
    for b in output.iter() {
        sprint!("{:?} ", b)
    }
    sprint!("\n");
    sprintln!("Result read");
    loop {}
}
