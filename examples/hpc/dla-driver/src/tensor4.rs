#![no_std]
#![no_main]

use alloc::vec::*;
use headsail_bsp::ufmt::uDisplay;
use headsail_bsp::{sprint, sprintln};
use ndarray::{Array, Array4};

#[derive(Clone, Copy, Debug)]
pub enum Order4 {
    KCHW,
    KCWH,
    KHWC,
    KHCW,
    KWHC,
    KWCH,
    CKHW,
    CKWH,
    CHWK,
    CHKW,
    CWKH,
    CWHK,
    HKCW,
    HKWC,
    HCKW,
    HCWK,
    HWCK,
    HWKC,
    WKCH,
    WKHC,
    WCKH,
    WCHK,
    WHCK,
    WHKC,
}

impl From<Order4> for [usize; 4] {
    fn from(order: Order4) -> Self {
        match order {
            Order4::KCHW => [0, 1, 2, 3],
            Order4::KCWH => [0, 1, 3, 2],
            Order4::KHWC => [0, 2, 3, 1],
            Order4::KHCW => [0, 2, 1, 3],
            Order4::KWCH => [0, 3, 1, 2],
            Order4::KWHC => [0, 3, 2, 1],
            Order4::CKHW => [1, 0, 2, 3],
            Order4::CKWH => [1, 0, 3, 2],
            Order4::CHWK => [1, 2, 0, 3],
            Order4::CHKW => [1, 2, 3, 0],
            Order4::CWKH => [1, 3, 0, 2],
            Order4::CWHK => [1, 3, 2, 0],
            Order4::HKCW => [2, 0, 1, 3],
            Order4::HKWC => [2, 0, 3, 1],
            Order4::HCKW => [2, 1, 0, 3],
            Order4::HCWK => [2, 1, 3, 0],
            Order4::HWCK => [2, 3, 0, 1],
            Order4::HWKC => [2, 3, 1, 0],
            Order4::WKCH => [3, 0, 1, 2],
            Order4::WKHC => [3, 0, 2, 1],
            Order4::WCKH => [3, 1, 0, 2],
            Order4::WCHK => [3, 1, 2, 0],
            Order4::WHCK => [3, 2, 0, 1],
            Order4::WHKC => [3, 2, 1, 0],
        }
    }
}

pub struct Tensor4<T> {
    data: Array4<T>,
    channels: usize,
    kernels: usize,
    height: usize,
    width: usize,
    order: Order4,
}

impl<T: Clone + uDisplay> Tensor4<T> {
    // Creates a new Tensor4 with the specified dimensions, initial value, and order
    pub fn new(
        kernels: usize,
        channels: usize,
        height: usize,
        width: usize,
        initial_value: T,
        order: Order4,
    ) -> Self {
        let data = Array::from_elem((kernels, channels, height, width), initial_value);
        Tensor4 {
            data,
            kernels,
            channels,
            height,
            width,
            order,
        }
    }

    // Creates a new Tensor4 from a data buffer with the specified order
    pub fn from_data_buffer(
        kernels: usize,
        channels: usize,
        height: usize,
        width: usize,
        data_buffer: Vec<T>,
        order: Order4,
    ) -> Result<Self, &'static str> {
        if data_buffer.len() != kernels * channels * height * width {
            return Err("Data buffer size does not match specified dimensions");
        }

        let data = Array::from_shape_vec((kernels, channels, height, width), data_buffer)
            .map_err(|_| "Failed to create array from data buffer")?;

        Ok(Tensor4 {
            data,
            kernels,
            channels,
            height,
            width,
            order,
        })
    }

    /// Matches order field value to height, width, channels and kernels parameters
    fn get_dimension_order_values(&self) -> [usize; 4] {
        let mut out = [0; 4];
        let order: [usize; 4] = self.order.into();
        for (i, x) in order.into_iter().enumerate() {
            let param = match x {
                0 => self.kernels,
                1 => self.channels,
                2 => self.height,
                3 => self.width,
                _ => unimplemented!(),
            };
            out[i] = param;
        }
        out
    }

    // Returns a reference to the element at the specified position
    pub fn get(&self, kernel: usize, channel: usize, row: usize, col: usize) -> Option<&T> {
        self.data.get((kernel, channel, row, col))
    }

    // Returns a mutable reference to the element at the specified position
    pub fn get_mut(
        &mut self,
        kernel: usize,
        channel: usize,
        row: usize,
        col: usize,
    ) -> Option<&mut T> {
        self.data.get_mut((kernel, channel, row, col))
    }

    // Sets the element at the specified position
    pub fn set(
        &mut self,
        kernel: usize,
        channel: usize,
        row: usize,
        col: usize,
        value: T,
    ) -> Result<(), &'static str> {
        if let Some(elem) = self.data.get_mut((kernel, channel, row, col)) {
            *elem = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    // Returns the dimensions of the array
    pub fn dimensions(&self) -> (usize, usize, usize, usize) {
        (self.kernels, self.channels, self.height, self.width)
    }

    // Sets a new order for the array
    pub fn set_order(&mut self, order: Order4) {
        self.order = order;
    }

    fn to_kchw_buffer(&self) -> Vec<T> {
        let mut buffer =
            Vec::with_capacity(self.kernels * self.channels * self.height * self.width);
        sprint!("This1");

        let dim_order_values: [usize; 4] = self.get_dimension_order_values();

        for x in dim_order_values {
            sprint!("{}", x)
        }

        sprint!("\nThis2");
        for i in 0..dim_order_values[0] {
            for j in 0..dim_order_values[1] {
                for k in 0..dim_order_values[2] {
                    for l in 0..dim_order_values[3] {
                        buffer.push(self.data[(i, j, k, l)].clone());
                    }
                    sprint!("This3");
                }

                sprint!("This4");
            }
            sprint!("This5");
        }
        sprint!("This6");
        buffer
    }

    // Converts the 4D array to a linear buffer according to the current order
    pub fn to_buffer(&self) -> Vec<T> {
        self.to_buffer_with_order(self.order)
    }

    // Converts the 4D array to a linear buffer according to the specified order
    pub fn to_buffer_with_order(&self, order: Order4) -> Vec<T> {
        // Convert current matrix to standard ordered vector
        let kchw_buffer = self.to_kchw_buffer();
        let data = Array::from_shape_vec(
            (self.kernels, self.channels, self.height, self.width),
            kchw_buffer,
        )
        .expect("Failed to reshape buffer to KCHW order");

        let mut buffer =
            Vec::with_capacity(self.kernels * self.channels * self.height * self.width);

        let dim_order_values = self.get_dimension_order_values();
        let dim_order: [usize; 4] = self.order.into();

        for i in 0..dim_order_values[0] {
            for j in 0..dim_order_values[1] {
                for k in 0..dim_order_values[2] {
                    for l in 0..dim_order_values[3] {
                        let ker = [i, j, k, l][dim_order.iter().position(|&r| r == 0).unwrap()];
                        let c = [i, j, k, l][dim_order.iter().position(|&r| r == 1).unwrap()];
                        let h = [i, j, k, l][dim_order.iter().position(|&r| r == 2).unwrap()];
                        let w = [i, j, k, l][dim_order.iter().position(|&r| r == 3).unwrap()];
                        buffer.push(data[(ker, c, h, w)].clone());
                    }
                }
            }
        }
        buffer
    }

    pub fn print_tensor(&self) {
        // Convert current matrix to standard ordered vector
        sprint!("Here 0");
        let kchw_buffer = self.to_kchw_buffer();
        sprint!("Here 1");
        let data = Array::from_shape_vec(
            (self.kernels, self.channels, self.height, self.width),
            kchw_buffer,
        )
        .expect("Failed to reshape buffer to KCHW order");

        sprint!("Here 2");
        let dim_order_values = self.get_dimension_order_values();
        sprint!("Here 3");
        let dim_order: [usize; 4] = self.order.into();

        sprint!("Here 4");
        for x in dim_order_values {
            sprint!("{} ", x)
        }

        for i in 0..dim_order_values[0] {
            for j in 0..dim_order_values[1] {
                for k in 0..dim_order_values[2] {
                    for l in 0..dim_order_values[3] {
                        // Match iterators i,j,k to current ordering scheme to find index data in standard KCHW order
                        let ker = [i, j, k, l][dim_order.iter().position(|&r| r == 0).unwrap()];
                        let c = [i, j, k, l][dim_order.iter().position(|&r| r == 1).unwrap()];
                        let h = [i, j, k, l][dim_order.iter().position(|&r| r == 2).unwrap()];
                        let w = [i, j, k, l][dim_order.iter().position(|&r| r == 3).unwrap()];
                        sprint!("{} ", data[(ker, c, h, w)]);
                    }
                    sprintln!("");
                }
                sprintln!("");
            }
            sprintln!("");
        }
    }
}
