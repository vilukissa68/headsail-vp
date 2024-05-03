#![no_std]
#![no_main]

extern crate alloc;

use headsail_bsp::{rt::entry, sprint, sprintln, init_alloc};
use dla_driver::*;
use panic_halt as _;

use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use rand::RngCore;

use alloc::vec::*;

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

fn conv2d_output_parameters(input: (usize, usize), kernel: (usize, usize), padding: (usize, usize),
                            dilation: (usize, usize), stride: (usize, usize)) -> (usize, usize) {
    let w_out = (input.0 + 2 * padding.0 - dilation.0 * (kernel.0 - 1) - 1) / stride.0 + 1;
    let h_out = (input.1 + 2 * padding.1 - dilation.1 * (kernel.1 - 1) - 1) / stride.1 + 1;
    (w_out, h_out)

}

fn generate_random_array(buffer: &mut [u8], size: usize) {
    let mut rng = SmallRng::seed_from_u64(1234567890);
    for i in 0..size {
        buffer[i] = rng.next_u64() as u8;
    }
}

fn generate_random_matrix(height: usize, width: usize) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    let mut rng = SmallRng::seed_from_u64(1234567890);
    for i in 0..(height*width) {
        res.push((rng.next_u64() & 0xFF) as u8);
    }
    res
}

fn run_random_layer(in_w: usize, in_h: usize, k_w: usize, k_h: usize) -> Vec<u8> {
    // Generate input and kernel
    let mut input = generate_random_matrix(in_w, in_h);
    let mut kernel = generate_random_matrix(k_w, k_h);

    dla_set_kernel_size(1, k_w, k_h);
    dla_set_input_size(1, in_w, in_h);

    dla_write_input(&mut input);
    dla_write_kernel(&mut kernel);

    // Calculate output size
    let (w_out, h_out) = conv2d_output_parameters((in_w, in_h), (k_w, k_h), (0,0), (1,1), (1,1));

    dla_kernel_data_ready(true);
    dla_input_data_ready(true);

    // Print the matrix
    sprintln!("Waiting for calculation");
    while !dla_is_ready() {
    }
    sprintln!("Calculation ready");

    let output: Vec<u8> =  dla_read_result(w_out * h_out);
    output

}

#[entry]
fn main() -> ! {
    init_alloc();
    sprintln!("Starting benchmark..");

    dla_init();

    dla_set_mac_clip(8);
    dla_set_pp_clip(8);

    for x in 0..1 {
        let res = run_random_layer(8,8,2,2);
        sprint!("Res:");
        for entry in res {
            sprint!("{} ", entry);
        }
    }

    loop {}
}
