#![no_std]
#![no_main]

use alloc::vec::*;
use headsail_bsp::ufmt::uDisplay;
use headsail_bsp::{sprint, sprintln};
use ndarray::{Array, Array3};

#[derive(Clone, Copy, Debug)]
pub enum Order3 {
    CHW,
    CWH,
    HWC,
    HCW,
    WHC,
    WCH,
}

impl From<Order3> for [usize; 3] {
    fn from(order: Order3) -> Self {
        match order {
            Order3::CHW => [0, 1, 2],
            Order3::CWH => [0, 2, 1],
            Order3::HWC => [1, 2, 0],
            Order3::HCW => [1, 0, 2],
            Order3::WHC => [2, 1, 0],
            Order3::WCH => [2, 0, 1],
        }
    }
}

pub struct Tensor3<T> {
    data: Array3<T>,
    pub channels: usize,
    pub height: usize,
    pub width: usize,
    order: Order3,
    internal_order: Order3,
}

impl<T: Clone + uDisplay> Tensor3<T> {
    // Creates a new Tensor3 with the specified dimensions, initial value, and order
    pub fn new(
        channels: usize,
        height: usize,
        width: usize,
        initial_value: T,
        order: Order3,
    ) -> Self {
        let data = Array::from_elem((channels, height, width), initial_value);
        Tensor3 {
            data,
            channels,
            height,
            width,
            order,
            internal_order: order,
        }
    }

    /// Creates a new Tensor3 from a data buffer with the specified order
    pub fn from_array3(data: Array3<T>, order: Order3) -> Self {
        let shape = data.shape();

        let dim_order: [usize; 3] = order.into();
        let channels = shape[dim_order.iter().position(|&r| r == 0).unwrap()];
        let height = shape[dim_order.iter().position(|&r| r == 1).unwrap()];
        let width = shape[dim_order.iter().position(|&r| r == 2).unwrap()];

        Tensor3 {
            data,
            channels,
            height,
            width,
            order,
            internal_order: order,
        }
    }

    /// Creates a new Tensor3 from a data buffer with the specified order
    pub fn from_data_buffer(
        channels: usize,
        height: usize,
        width: usize,
        data_buffer: Vec<T>,
        order: Order3,
    ) -> Result<Self, &'static str> {
        if data_buffer.len() != channels * height * width {
            return Err("Data buffer size does not match specified dimensions");
        }

        let standard_shape = [channels, height, width];
        let dim_order: [usize; 3] = order.into();
        let channels_ordered = standard_shape[dim_order.iter().position(|&r| r == 0).unwrap()];
        let height_ordered = standard_shape[dim_order.iter().position(|&r| r == 1).unwrap()];
        let width_ordered = standard_shape[dim_order.iter().position(|&r| r == 2).unwrap()];

        let data = Array::from_shape_vec(
            (channels_ordered, height_ordered, width_ordered),
            data_buffer,
        )
        .map_err(|_| "Failed to create array from data buffer")?;

        Ok(Tensor3 {
            data,
            channels,
            height,
            width,
            order,
            internal_order: order,
        })
    }

    /// Get the number of element in ndarray
    pub fn get_size(&self) -> usize {
        self.data.len()
    }

    /// Matches order field value to height, width and channels parameters
    fn get_dimension_order_values(&self, order: Option<Order3>) -> [usize; 3] {
        let mut out = [0; 3];

        // Use self value if no order was given
        let order: [usize; 3] = match order {
            Some(order) => order.into(),
            None => self.order.into(),
        };

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

    /// Returns a reference to the element at the specified position
    pub fn get(&self, channel: usize, row: usize, col: usize) -> Option<&T> {
        self.data.get((channel, row, col))
    }

    /// Returns a mutable reference to the element at the specified position
    pub fn get_mut(&mut self, channel: usize, row: usize, col: usize) -> Option<&mut T> {
        self.data.get_mut((channel, row, col))
    }

    /// Sets the element at the specified position
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

    /// Returns the dimensions of the array
    pub fn dimensions(&self) -> (usize, usize, usize) {
        (self.channels, self.height, self.width)
    }

    /// Gets the current order of the array
    pub fn order(&self) -> Order3 {
        self.order
    }

    /// Sets a new order for the array
    pub fn set_order(&mut self, order: Order3) {
        self.order = order;
    }

    /// Converts the internal buffer to CHW order
    fn to_chw_buffer(&self) -> Vec<T> {
        let mut buffer = Vec::with_capacity(self.channels * self.height * self.width);

        let dim_order: [usize; 3] = Order3::CHW.into();
        let dim_order_values = self.get_dimension_order_values(Some(self.internal_order));

        for x in dim_order_values {
            sprint!("{} ", x)
        }

        for i in 0..dim_order_values[0] {
            for j in 0..dim_order_values[1] {
                for k in 0..dim_order_values[2] {
                    // Match iterators i,j,k to current ordering scheme to find index data in standard CHW order
                    let c = [i, j, k][dim_order.iter().position(|&r| r == 0).unwrap()];
                    let h = [i, j, k][dim_order.iter().position(|&r| r == 1).unwrap()];
                    let w = [i, j, k][dim_order.iter().position(|&r| r == 2).unwrap()];
                    buffer.push(self.data[(c, h, w)].clone());
                }
            }
        }
        buffer
    }

    /// Converts the 3D array to a linear buffer according to the current order
    pub fn to_buffer(&self) -> Vec<T> {
        self.to_buffer_with_order(self.order)
    }

    /// Converts the 3D array to a linear buffer according to the specified order
    pub fn to_buffer_with_order(&self, order: Order3) -> Vec<T> {
        // Convert current matrix to standard ordered vector
        let chw_buffer = self.to_chw_buffer(); // Use common format
        let data = Array::from_shape_vec((self.channels, self.height, self.width), chw_buffer)
            .expect("Failed to reshape buffer to CHW order");

        let mut buffer = Vec::with_capacity(self.channels * self.height * self.width);

        let dim_order_values = self.get_dimension_order_values(Some(order));
        let dim_order: [usize; 3] = order.into();

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

    /// Prints tensor in current order
    pub fn print_tensor(&self) {
        sprintln!("Channels: {}", self.channels);
        sprintln!("Height: {}", self.height);
        sprintln!("Width: {}", self.width);
        let chw_buffer = self.to_chw_buffer(); // Use common format
        let data = Array::from_shape_vec((self.channels, self.height, self.width), chw_buffer)
            .expect("Failed to reshape buffer to CHW order");

        let dim_order: [usize; 3] = self.order.into();
        let dim_order_values = self.get_dimension_order_values(None);
        // for x in dim_order_values {
        //     sprint!("{} ", x)
        // }

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
