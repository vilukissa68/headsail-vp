use alloc::vec::*;
use ndarray::{Array, Array4};

#[derive(Clone, Copy, Debug, PartialEq)]
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

impl Order4 {
    fn into_position(self) -> [usize; 4] {
        match self {
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
            Order4::HWCK => [2, 3, 1, 0],
            Order4::HWKC => [2, 3, 0, 1],
            Order4::WKCH => [3, 0, 1, 2],
            Order4::WKHC => [3, 0, 2, 1],
            Order4::WCKH => [3, 1, 0, 2],
            Order4::WCHK => [3, 1, 2, 0],
            Order4::WHCK => [3, 2, 0, 1],
            Order4::WHKC => [3, 2, 1, 0],
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tensor4<T> {
    data: Array4<T>,
    order: Order4,
}

impl<T: Clone> Tensor4<T> {
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
        Tensor4 { data, order }
    }
    pub fn kernels(&self) -> usize {
        let dim_order: [usize; 4] = self.order.into_position();
        let position = unsafe { dim_order.iter().position(|&r| r == 0).unwrap_unchecked() };
        self.data.raw_dim()[position]
    }
    pub fn channels(&self) -> usize {
        let dim_order: [usize; 4] = self.order.into_position();
        let position = unsafe { dim_order.iter().position(|&r| r == 1).unwrap_unchecked() };
        self.data.raw_dim()[position]
    }

    pub fn height(&self) -> usize {
        let dim_order: [usize; 4] = self.order.into_position();
        let position = unsafe { dim_order.iter().position(|&r| r == 2).unwrap_unchecked() };
        self.data.raw_dim()[position]
    }

    pub fn width(&self) -> usize {
        let dim_order: [usize; 4] = self.order.into_position();
        let position = unsafe { dim_order.iter().position(|&r| r == 3).unwrap_unchecked() };
        self.data.raw_dim()[position]
    }

    /// Creates a new Tensor4 from a data buffer with the specified order
    pub fn from_array4(data: Array4<T>, order: Order4) -> Self {
        Tensor4 { data, order }
    }

    /// Get the number of element in ndarray
    pub fn get_size(&self) -> usize {
        self.data.len()
    }

    /// Creates a new Tensor4 from a data buffer with the specified order
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

        let standard_shape = [kernels, channels, height, width];
        let dim_order: [usize; 4] = order.into_position();
        let kernels_ordered =
            standard_shape[unsafe { dim_order.iter().position(|&r| r == 0).unwrap_unchecked() }];
        let channels_ordered =
            standard_shape[unsafe { dim_order.iter().position(|&r| r == 1).unwrap_unchecked() }];
        let height_ordered =
            standard_shape[unsafe { dim_order.iter().position(|&r| r == 2).unwrap_unchecked() }];
        let width_ordered =
            standard_shape[unsafe { dim_order.iter().position(|&r| r == 3).unwrap_unchecked() }];

        let data = Array::from_shape_vec(
            (
                kernels_ordered,
                channels_ordered,
                height_ordered,
                width_ordered,
            ),
            data_buffer,
        )
        .map_err(|_| "Failed to create array from data buffer")?;

        Ok(Tensor4 { data, order })
    }

    /// Matches order field value to height, width, channels and kernels parameters
    fn get_dimension_order_values(&self, order: Option<Order4>) -> [usize; 4] {
        let mut out = [0; 4];

        // Use self value if no order was given
        let order: [usize; 4] = match order {
            Some(order) => order.into_position(),
            None => self.order.into_position(),
        };

        for (i, x) in order.into_iter().enumerate() {
            let param = match x {
                0 => self.kernels(),
                1 => self.channels(),
                2 => self.height(),
                3 => self.width(),
                _ => unimplemented!(),
            };
            out[i] = param;
        }
        out
    }

    /// Returns a reference to the element at the specified position
    pub fn get(&self, kernel: usize, channel: usize, row: usize, col: usize) -> Option<&T> {
        self.data.get((kernel, channel, row, col))
    }

    /// Returns a mutable reference to the element at the specified position
    pub fn get_mut(
        &mut self,
        kernel: usize,
        channel: usize,
        row: usize,
        col: usize,
    ) -> Option<&mut T> {
        self.data.get_mut((kernel, channel, row, col))
    }

    /// Sets the element at the specified position
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

    /// Returns the dimensions of the array
    pub fn dimensions(&self) -> (usize, usize, usize, usize) {
        (self.kernels(), self.channels(), self.height(), self.width())
    }

    /// Sets a new order for the array
    pub fn permute(&mut self, order: Order4) {
        // Early return if already in order
        if self.order == order {
            return;
        }

        if self.order == Order4::KCHW {
            // Transmute to target order
            let new_order: [usize; 4] = order.into_position();
            self.data = self.data.clone().permuted_axes(new_order);
            self.order = order;
            return;
        }

        // Transmute to standard order
        let std_order: [usize; 4] = self.order.into_position();
        let std = self.data.clone().permuted_axes(std_order);

        // Transmute to target order
        let new_order: [usize; 4] = order.into_position();
        self.data = std.permuted_axes(new_order);
        self.order = order;
    }

    /// Transforms data to standard format
    fn to_kchw_buffer(&self) -> Vec<T> {
        self.to_buffer_with_order(Order4::KCHW)
    }

    /// Converts the 4D array to a linear buffer according to the current order
    pub fn to_buffer(&self) -> Vec<T> {
        let mut buffer = Vec::with_capacity(self.get_size());
        for x in Array::from_iter(self.data.iter().cloned()) {
            buffer.push(x)
        }
        buffer
    }

    /// Converts the 4D array to a linear buffer according to the specified order
    pub fn to_buffer_with_order(&self, order: Order4) -> Vec<T> {
        // If order is correct no need to permute
        if order == self.order {
            return self.to_buffer();
        }
        let mut data = self.clone();
        data.permute(order);
        data.to_buffer()
    }
}
