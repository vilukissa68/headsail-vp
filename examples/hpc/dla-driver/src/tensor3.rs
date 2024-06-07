#![no_std]
#![no_main]

use alloc::vec::*;
use headsail_bsp::ufmt::uDisplay;
use headsail_bsp::{sprint, sprintln};
use ndarray::{Array, Array3};

#[derive(Clone, Copy, Debug)]
pub enum Order {
    CHW,
    CWH,
    HWC,
    HCW,
    WHC,
    WCH,
}

impl From<Order> for [usize; 3] {
    fn from(order: Order) -> Self {
        match order {
            Order::CHW => [0, 1, 2],
            Order::CWH => [0, 2, 1],
            Order::HWC => [1, 2, 0],
            Order::HCW => [1, 0, 2],
            Order::WHC => [2, 1, 0],
            Order::WCH => [2, 0, 1],
        }
    }
}

pub struct Tensor3<T> {
    data: Array3<T>,
    pub channels: usize,
    pub height: usize,
    pub width: usize,
    order: Order,
}

impl<T: Clone + uDisplay> Tensor3<T> {
    // Creates a new Tensor3 with the specified dimensions, initial value, and order
    pub fn new(
        channels: usize,
        height: usize,
        width: usize,
        initial_value: T,
        order: Order,
    ) -> Self {
        let data = Array::from_elem((channels, height, width), initial_value);
        Tensor3 {
            data,
            channels,
            height,
            width,
            order,
        }
    }

    // Creates a new Tensor3 from a data buffer with the specified order
    pub fn from_data_buffer(
        channels: usize,
        height: usize,
        width: usize,
        data_buffer: Vec<T>,
        order: Order,
    ) -> Result<Self, &'static str> {
        if data_buffer.len() != channels * height * width {
            return Err("Data buffer size does not match specified dimensions");
        }

        let data = Array::from_shape_vec((channels, height, width), data_buffer)
            .map_err(|_| "Failed to create array from data buffer")?;

        Ok(Tensor3 {
            data,
            channels,
            height,
            width,
            order,
        })
    }

    /// Matches order field value to height, width and channels parameters
    fn get_dimension_order_values(&self) -> [usize; 3] {
        let mut out = [0; 3];
        let order: [usize; 3] = self.order.into();
        for (i, x) in order.into_iter().enumerate() {
            let param = match x {
                0 => self.channels,
                1 => self.height,
                2 => self.width,
                _ => unimplemented!(),
            };
            out[i] = param;
        }
        out
    }

    // Returns a reference to the element at the specified position
    pub fn get(&self, channel: usize, row: usize, col: usize) -> Option<&T> {
        self.data.get((channel, row, col))
    }

    // Returns a mutable reference to the element at the specified position
    pub fn get_mut(&mut self, channel: usize, row: usize, col: usize) -> Option<&mut T> {
        self.data.get_mut((channel, row, col))
    }

    // Sets the element at the specified position
    pub fn set(
        &mut self,
        channel: usize,
        row: usize,
        col: usize,
        value: T,
    ) -> Result<(), &'static str> {
        if let Some(elem) = self.data.get_mut((channel, row, col)) {
            *elem = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    // Returns the dimensions of the array
    pub fn dimensions(&self) -> (usize, usize, usize) {
        (self.channels, self.height, self.width)
    }

    // Gets the current order of the array
    pub fn order(&self) -> Order {
        self.order
    }

    // Sets a new order for the array
    pub fn set_order(&mut self, order: Order) {
        self.order = order;
    }

    // Converts the internal buffer to CHW order
    fn to_chw_buffer(&self) -> Vec<T> {
        let mut buffer = Vec::with_capacity(self.channels * self.height * self.width);

        let dim_order_values = self.get_dimension_order_values();

        for i in 0..dim_order_values[0] {
            for j in 0..dim_order_values[1] {
                for k in 0..dim_order_values[2] {
                    buffer.push(self.data[(i, j, k)].clone());
                }
            }
        }
        buffer
    }

    // Converts the 3D array to a linear buffer according to the current order
    pub fn to_buffer(&self) -> Vec<T> {
        self.to_buffer_with_order(self.order)
    }

    // Converts the 3D array to a linear buffer according to the specified order
    pub fn to_buffer_with_order(&self, order: Order) -> Vec<T> {
        // Convert current matrix to standard ordered vector
        let chw_buffer = self.to_chw_buffer(); // Use common format
        let data = Array::from_shape_vec((self.channels, self.height, self.width), chw_buffer)
            .expect("Failed to reshape buffer to CHW order");

        let mut buffer = Vec::with_capacity(self.channels * self.height * self.width);

        let dim_order_values = self.get_dimension_order_values();
        let dim_order: [usize; 3] = self.order.into();

        for i in 0..dim_order_values[0] {
            for j in 0..dim_order_values[1] {
                for k in 0..dim_order_values[2] {
                    // Match iterators i,j,k to current ordering scheme to find index data in standard CHW order
                    let c = [i, j, k][dim_order.iter().position(|&r| r == 0).unwrap()];
                    let h = [i, j, k][dim_order.iter().position(|&r| r == 1).unwrap()];
                    let w = [i, j, k][dim_order.iter().position(|&r| r == 2).unwrap()];
                    buffer.push(data[(c, h, w)].clone());
                }
            }
        }

        buffer
    }

    pub fn print_tensor(&self) {
        let chw_buffer = self.to_chw_buffer(); // Use common format
        let data = Array::from_shape_vec((self.channels, self.height, self.width), chw_buffer)
            .expect("Failed to reshape buffer to CHW order");
        let dim_order: [usize; 3] = self.order.into();
        let dim_order_values = self.get_dimension_order_values();
        for i in 0..dim_order_values[0] {
            for j in 0..dim_order_values[1] {
                for k in 0..dim_order_values[2] {
                    // Match iterators i,j,k to current ordering scheme to find index data in standard CHW order
                    let c = [i, j, k][dim_order.iter().position(|&r| r == 0).unwrap()];
                    let h = [i, j, k][dim_order.iter().position(|&r| r == 1).unwrap()];
                    let w = [i, j, k][dim_order.iter().position(|&r| r == 2).unwrap()];
                    sprint!("{} ", data[(c, h, w)]);
                }
                sprintln!("")
            }
            sprintln!("")
        }
    }
}
