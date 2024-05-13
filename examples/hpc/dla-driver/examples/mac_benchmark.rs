#![no_std]
#![no_main]

extern crate alloc;

use dla_driver::*;
use headsail_bsp::{init_alloc, rt::entry, sprint, sprintln};
use panic_halt as _;

use rand::rngs::SmallRng;
use rand::RngCore;
use rand::SeedableRng;

use alloc::vec::Vec;

fn calculate_conv2d_out_param_dim(
    input: (usize, usize),
    kernel: (usize, usize),
    padding: (usize, usize),
    dilation: (usize, usize),
    stride: (usize, usize),
) -> (usize, usize) {
    let output_width = (input.0 + 2 * padding.0 - dilation.0 * (kernel.0 - 1) - 1) / stride.0 + 1;
    let output_height = (input.1 + 2 * padding.1 - dilation.1 * (kernel.1 - 1) - 1) / stride.1 + 1;
    (output_width, output_height)
}

fn generate_random_array(buffer: &mut [u8], size: usize) {
    let mut rng = SmallRng::seed_from_u64(1234567890);
    for i in 0..size {
        buffer[i] = rng.next_u64() as u8;
    }
}

fn generate_random_matrix(height: usize, width: usize, seed: u64) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    let mut rng = SmallRng::seed_from_u64(seed);
    for _ in 0..(height * width) {
        res.push((rng.next_u64() & 0xFF) as u8);
    }
    res
}

fn generate_random_matrix_small(height: usize, width: usize, seed: u64) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    let mut rng = SmallRng::seed_from_u64(seed);
    for _ in 0..(height * width) {
        res.push((rng.next_u64() & 0x1) as u8);
    }
    res
}

fn run_random_layer(
    dla: &mut Dla,
    input_width: usize,
    input_height: usize,
    kernel_width: usize,
    kernel_height: usize,
    seed: u64,
) -> Vec<u8> {
    // Generate input and kernel
    dla.init_layer();

    let mut input = generate_random_matrix(input_width, input_height, seed);
    let mut kernel = generate_random_matrix_small(kernel_width, kernel_height, seed * 2);

    dla.set_kernel_size(1, kernel_width, kernel_height);
    dla.set_input_size(1, input_width, input_height);

    dla.write_input(&mut input);
    dla.write_kernel(&mut kernel);

    // Calculate output size
    let (output_width, output_height) = calculate_conv2d_out_param_dim(
        (input_width, input_height),
        (kernel_width, kernel_height),
        (0, 0),
        (1, 1),
        (1, 1),
    );

    dla.kernel_data_ready(true);
    dla.input_data_ready(true);

    // Print the matrix
    sprintln!("Waiting for calculation");
    while !dla.handle_handshake() {}
    sprintln!("Calculation ready");
    let output: Vec<u8> = dla.read_output(output_width * output_height);
    output
}

#[entry]
fn main() -> ! {
    init_alloc();

    let mut dla = Dla::new();
    sprintln!("Starting benchmark..");

    dla.set_mac_clip(8);
    dla.set_pp_clip(8);

    for x in 0..2 {
        let res = run_random_layer(&mut dla, 8, 8, 2, 2, x * x);
        for x in res {
            sprint!("{:?} ", x);
        }
        sprint!("\n\n");
    }
    loop {}
}
