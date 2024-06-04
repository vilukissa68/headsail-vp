//! # DLA driver
//!
//! Implements driver for sochub headsail SoC's deep learning accelerator.
#![no_std]

extern crate alloc;

mod mmap;

pub use mmap::{
    DLA0_ADDR, MEMORY_BANK_0_OFFSET, MEMORY_BANK_10_OFFSET, MEMORY_BANK_11_OFFSET,
    MEMORY_BANK_12_OFFSET, MEMORY_BANK_13_OFFSET, MEMORY_BANK_14_OFFSET, MEMORY_BANK_15_OFFSET,
    MEMORY_BANK_1_OFFSET, MEMORY_BANK_2_OFFSET, MEMORY_BANK_3_OFFSET, MEMORY_BANK_4_OFFSET,
    MEMORY_BANK_5_OFFSET, MEMORY_BANK_6_OFFSET, MEMORY_BANK_7_OFFSET, MEMORY_BANK_8_OFFSET,
    MEMORY_BANK_9_OFFSET, MEMORY_BANK_BASE_ADDR,
};

const DEFAULT_INPUT_BANK: MemoryBank = MemoryBank::Bank0;
const DEFAULT_KERNEL_BANK: MemoryBank = MemoryBank::Bank8;
const DEFAULT_OUTPUT_BANK: MemoryBank = MemoryBank::Bank12;
const DEFAULT_BIAS_ADDR: u32 = 0x0;
const DEFAULT_KERNEL_SIZE: KernelSize = KernelSize {
    s_channels: 1,
    kernels: 1,
    width: 2,
    height: 2,
};
const DEFAULT_INPUT_SIZE: InputSize = InputSize {
    channels: 1,
    width: 8,
    height: 8,
};
const DEFAULT_PADDING: PaddingConfig = PaddingConfig {
    top: 0,
    right: 0,
    left: 0,
    bottom: 0,
    padding_value: 0,
};
const DEFAULT_STRIDE: StrideConfig = StrideConfig { x: 1, y: 1 };
const DEFAULT_MAC_CLIP: u32 = 8;
const DEFAULT_PP_CLIP: u32 = 8;
const DEFAULT_SIMD_MODE: SimdBitMode = SimdBitMode::EightBits;

use alloc::vec::Vec;
use core::{ptr, result};
use headsail_bsp::{sprint, sprintln};
use mmap::*;

/// Clip error type
struct InvalidClip(u32);

/// Dimensions of kernel
pub struct KernelSize {
    pub s_channels: u32,
    pub kernels: u32,
    pub height: u32,
    pub width: u32,
}

/// Dimensions of inputs
pub struct InputSize {
    pub channels: u32,
    pub width: u32,
    pub height: u32,
}

/// Conv2d padding
/// Value used for padding the matrix
///
/// # Examples
///
/// Non functional example, padding is done in hardware
/// ```
/// let padding = PaddingConfig {top:1, right:1, left:1, bottom:1, padding_value: 7};
/// matrix = [[1,2], [3,4]];
/// pretty_print_matrix(matrix)
/// 1 2
/// 3 4
/// padded_matrix = applyPadding(matrix, padding);
/// pretty_print_matrix(padded_matrix);
/// 7 7 7 7
/// 7 1 2 7
/// 7 3 4 7
/// 7 7 7 7
/// ```
pub struct PaddingConfig {
    pub top: u32,
    pub right: u32,
    pub left: u32,
    pub bottom: u32,
    pub padding_value: u32,
}

/// Conv2d stride
pub struct StrideConfig {
    pub x: u32,
    pub y: u32,
}

/// Configures DLA for performing calculation for layers
pub struct LayerConfig {
    pub input_bank: Option<MemoryBank>,
    pub kernel_bank: Option<MemoryBank>,
    pub output_bank: Option<MemoryBank>,
    pub bias_addr: Option<u32>,
    pub pp_enabled: bool,
    pub relu_enabled: bool,
    pub bias_enabled: bool,
    pub input_size: Option<InputSize>,
    pub kernel_size: Option<KernelSize>,
    pub padding: Option<PaddingConfig>,
    pub stride: Option<StrideConfig>,
    pub mac_clip: Option<u32>,
    pub pp_clip: Option<u32>,
    pub simd_mode: Option<SimdBitMode>,
}

#[derive(Clone, Copy)]
#[rustfmt::skip]
/// Data banks in DLA's memory buffer, stores inputs, kernels and outputs.
pub enum MemoryBank {
    Bank0, Bank1, Bank2, Bank3, Bank4, Bank5, Bank6, Bank7, Bank8, Bank9,
    Bank10, Bank11, Bank12, Bank13, Bank14, Bank15,
}

impl TryFrom<u32> for MemoryBank {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MemoryBank::Bank0),
            1 => Ok(MemoryBank::Bank1),
            2 => Ok(MemoryBank::Bank2),
            3 => Ok(MemoryBank::Bank3),
            4 => Ok(MemoryBank::Bank4),
            5 => Ok(MemoryBank::Bank5),
            6 => Ok(MemoryBank::Bank6),
            7 => Ok(MemoryBank::Bank7),
            8 => Ok(MemoryBank::Bank8),
            9 => Ok(MemoryBank::Bank9),
            10 => Ok(MemoryBank::Bank10),
            11 => Ok(MemoryBank::Bank11),
            12 => Ok(MemoryBank::Bank12),
            13 => Ok(MemoryBank::Bank13),
            14 => Ok(MemoryBank::Bank14),
            15 => Ok(MemoryBank::Bank15),
            _ => Err(()),
        }
    }
}

impl From<MemoryBank> for usize {
    fn from(val: MemoryBank) -> Self {
        match val {
            MemoryBank::Bank0 => 0,
            MemoryBank::Bank1 => 1,
            MemoryBank::Bank2 => 2,
            MemoryBank::Bank3 => 3,
            MemoryBank::Bank4 => 4,
            MemoryBank::Bank5 => 5,
            MemoryBank::Bank6 => 6,
            MemoryBank::Bank7 => 7,
            MemoryBank::Bank8 => 8,
            MemoryBank::Bank9 => 9,
            MemoryBank::Bank10 => 10,
            MemoryBank::Bank11 => 11,
            MemoryBank::Bank12 => 12,
            MemoryBank::Bank13 => 13,
            MemoryBank::Bank14 => 14,
            MemoryBank::Bank15 => 15,
        }
    }
}

impl MemoryBank {
    fn offset(&self) -> usize {
        match self {
            MemoryBank::Bank0 => MEMORY_BANK_0_OFFSET,
            MemoryBank::Bank1 => MEMORY_BANK_1_OFFSET,
            MemoryBank::Bank2 => MEMORY_BANK_2_OFFSET,
            MemoryBank::Bank3 => MEMORY_BANK_3_OFFSET,
            MemoryBank::Bank4 => MEMORY_BANK_4_OFFSET,
            MemoryBank::Bank5 => MEMORY_BANK_5_OFFSET,
            MemoryBank::Bank6 => MEMORY_BANK_6_OFFSET,
            MemoryBank::Bank7 => MEMORY_BANK_7_OFFSET,
            MemoryBank::Bank8 => MEMORY_BANK_8_OFFSET,
            MemoryBank::Bank9 => MEMORY_BANK_9_OFFSET,
            MemoryBank::Bank10 => MEMORY_BANK_10_OFFSET,
            MemoryBank::Bank11 => MEMORY_BANK_11_OFFSET,
            MemoryBank::Bank12 => MEMORY_BANK_12_OFFSET,
            MemoryBank::Bank13 => MEMORY_BANK_13_OFFSET,
            MemoryBank::Bank14 => MEMORY_BANK_14_OFFSET,
            MemoryBank::Bank15 => MEMORY_BANK_15_OFFSET,
        }
    }
}

#[derive(Copy, Clone)]
/// DLA support three SIMD modes
pub enum SimdBitMode {
    EightBits = 0,
    FourBits = 1,
    TwoBits = 2,
}

macro_rules! set_bits {
    ($offset:expr, $mask:expr, $reg:expr, $value:expr) => {
        (($reg & !($mask as u32)) | ($value << $offset) as u32) as u32
    };
}

macro_rules! get_bits {
    ($reg:expr, $mask:expr) => {
        ($reg & ($mask as u32)) as u32
    };
}

/// DLA driver struct
pub struct Dla {}

impl Default for Dla {
    fn default() -> Self {
        Self::new()
    }
}

impl Dla {
    pub fn new() -> Self {
        Dla {}
    }
    /// Writes u32 to dla configuration registers at offset
    fn write_u32(&self, offset: usize, value: u32) {
        unsafe { ptr::write_volatile((DLA0_ADDR + offset) as *mut _, value) }
    }

    /// Reads u32 from dla configuration registers at offset
    fn read_u32(&self, offset: usize) -> u32 {
        unsafe { ptr::read_volatile((DLA0_ADDR + offset) as *const _) }
    }

    /// Writes buffer DLA's data bank(s) based on offset
    pub fn write_data_bank(&self, offset: usize, buf: &mut [i8]) {
        /* NOTE:(20240604 vaino-waltteri.granat@tuni.fi)
         * After RTL test examination, it was found that DLA needs to
         * be written by reversing the order of bytes in each 64-bit chunk
         */
        for (cidx, chunk) in buf.chunks(8).enumerate() {
            for (i, b) in chunk.iter().rev().enumerate() {
                unsafe {
                    ptr::write_volatile(
                        (MEMORY_BANK_BASE_ADDR + offset + cidx * 8 + i) as *mut _,
                        *b,
                    )
                };
            }
        }
    }

    /// Read register from one of the DLA's data banks
    fn read_data_bank_offset(&self, bank: MemoryBank, offset: usize) -> u128 {
        if cfg!(feature = "vp") {
            let mut result: u128 = 0;
            for i in 0..4 {
                result |= (unsafe {
                    ptr::read_volatile(
                        (MEMORY_BANK_BASE_ADDR + bank.offset() + offset + (i * 4)) as *mut u32,
                    )
                } as u128)
                    << (32 * i)
            }
            result
        } else {
            unsafe {
                ptr::read_volatile((MEMORY_BANK_BASE_ADDR + bank.offset() + offset) as *const _)
            }
        }
    }

    /// Reads len number of bytes from DLA's memory banks, starting from bank given as parameter
    fn read_data_bank(&self, bank: MemoryBank, len: usize) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::with_capacity(len);

        let mut next_bank_offset = 0;
        while res.len() < len {
            let data = self.read_data_bank_offset(bank, next_bank_offset);
            let bytes_remaining = len - res.len();
            let bytes_to_copy = core::cmp::min(16, bytes_remaining);

            // Copy everything from one 128-bit address
            for i in 0..bytes_to_copy {
                let byte = ((data >> (i * 8)) & 0xFF) as u8;
                res.push(byte)
            }
            next_bank_offset += 0x10;
        }
        res
    }

    /// Reads len amount of bytes from DLA's output bank(s)
    pub fn read_output_i32(&self, len: usize) -> Vec<i32> {
        let bytes = self.read_data_bank(self.get_output_bank(), len * 4);
        let mut result = Vec::with_capacity(bytes.len() / 4);

        for pair in bytes.chunks_exact(4) {
            let combined = ((pair[0] as i32) << 24)
                | ((pair[1] as i32) << 16)
                | ((pair[2] as i32) << 8)
                | (pair[3] as i32);
            result.push(combined);
        }
        result
    }

    /// Reads len amount of bytes from DLA's output bank(s)
    pub fn read_output_i16(&self, len: usize) -> Vec<i16> {
        let bytes = self.read_data_bank(self.get_output_bank(), len * 2);
        let mut result = Vec::with_capacity(bytes.len() / 2);

        for pair in bytes.chunks_exact(2) {
            let combined = ((pair[0] as i16) << 8) | (pair[1] as i16 & 0xFF);
            result.push(combined);
        }
        result
    }

    /// Reads len amount of bytes from DLA's output bank(s)
    pub fn read_output_i8(&self, len: usize) -> Vec<i8> {
        let bytes = self.read_data_bank(self.get_output_bank(), len);
        bytes.iter().map(|&x| x as i8).collect()
    }

    /// Reads len amount of bytes from DLA's output bank(s)
    pub fn read_output_i4(&self, len: usize) -> Vec<i8> {
        let bytes = self.read_data_bank(self.get_output_bank(), len);
        let mut result = Vec::with_capacity(bytes.len() * 2);
        for &byte in bytes.iter() {
            // Extract the upper 4 bits and sign-extend to i8
            let upper = (byte as u8 & 0xF0) >> 4;
            let upper_sign_extended = if upper & 0x08 != 0 {
                upper | 0xF0
            } else {
                upper
            } as i8;

            // Extract the lower 4 bits and sign-extend to i8
            let lower = byte & 0x0F;
            let lower_sign_extended = if lower & 0x08 != 0 {
                (lower | 0xF0) as i8
            } else {
                lower as i8
            } as i8;

            result.push(upper_sign_extended);
            result.push(lower_sign_extended);
        }
        result
    }

    /// Reads len amount of bytes from DLA's input bank(s)
    pub fn read_input_bank(&self, len: usize) -> Vec<i8> {
        let bytes = self.read_data_bank(self.get_input_bank(), len);
        bytes.iter().map(|&x| x as i8).collect()
    }

    /// Reads len amount of bytes from DLA's weight bank(s)
    pub fn read_weight_bank(&self, len: usize) -> Vec<i8> {
        let bytes = self.read_data_bank(self.get_kernel_bank(), len);
        bytes.iter().map(|&x| x as i8).collect()
    }

    /// Writes buffer to DLA's input bank(s)
    pub fn write_input(&self, input: &mut [i8]) {
        // TODO optimize memory bank logic
        let offset = self.get_input_bank().offset();
        self.write_data_bank(offset, input)
    }

    /// Writes buffer to DLA's kernel bank(s)
    pub fn write_kernel(&self, kernel: &mut [i8]) {
        // TODO optimize memory bank logic
        self.write_data_bank(self.get_kernel_bank().offset(), kernel)
    }

    /// Sets one of the DLA's memory banks as starting bank for inputs
    fn set_input_data_bank(&self, bank: MemoryBank) {
        let mut reg = self.read_u32(DLA_BUF_DATA_BANK);
        let b: usize = bank.into();
        reg = set_bits!(
            DLA_BUF_DATA_BANK_B_OFFSET,
            DLA_BUF_DATA_BANK_B_BITMASK,
            reg,
            b
        );
        self.write_u32(DLA_BUF_DATA_BANK, reg);
    }

    /// Sets one of the DLA's memory banks as starting bank for kernels
    fn set_kernel_data_bank(&self, bank: MemoryBank) {
        let mut reg = self.read_u32(DLA_BUF_DATA_BANK);
        let b: usize = bank.into();
        reg = set_bits!(
            DLA_BUF_DATA_BANK_A_OFFSET,
            DLA_BUF_DATA_BANK_A_BITMASK,
            reg,
            b
        );
        self.write_u32(DLA_BUF_DATA_BANK, reg);
    }

    /// Sets one of the DLA's memory banks as starting bank for outputs
    fn set_output_bank(&self, bank: MemoryBank) {
        let mut reg = self.read_u32(DLA_PP_AXI_WRITE);
        let b: usize = bank.offset();
        reg = set_bits!(
            DLA_PP_AXI_WRITE_ADDRESS_OFFSET,
            DLA_PP_AXI_WRITE_ADDRESS_BITMASK,
            reg,
            b + MEMORY_BANK_BASE_ADDR
        );
        self.write_u32(DLA_PP_AXI_WRITE, reg);
    }

    /// Sets dimensions for inputs in convolution
    fn set_input_size(&self, input_size: InputSize) {
        let mut reg = 0;
        reg = set_bits!(
            DLA_BUF_INPUT_CHANNELS_OFFSET,
            DLA_BUF_INPUT_CHANNELS_BITMASK,
            reg,
            input_size.channels - 1
        );
        reg = set_bits!(
            DLA_BUF_INPUT_WIDTH_OFFSET,
            DLA_BUF_INPUT_WIDTH_BITMASK,
            reg,
            input_size.width - 1
        );
        reg = set_bits!(
            DLA_BUF_INPUT_HEIGHT_OFFSET,
            DLA_BUF_INPUT_HEIGHT_BITMASK,
            reg,
            input_size.height - 1
        );
        self.write_u32(DLA_BUF_INPUT, reg);
    }

    /// Sets dimensions for filters in convolution
    fn set_kernel_size(&self, kernel_size: KernelSize) {
        let mut buf_kernel_0 = 0;
        // Set BUF_KERNEL_0
        buf_kernel_0 = set_bits!(
            DLA_BUF_KERNEL_0_S_CHANNELS_OFFSET,
            DLA_BUF_KERNEL_0_S_CHANNELS_BITMASK,
            buf_kernel_0,
            kernel_size.s_channels - 1
        );
        buf_kernel_0 = set_bits!(
            DLA_BUF_KERNEL_0_WIDTH_OFFSET,
            DLA_BUF_KERNEL_0_WIDTH_BITMASK,
            buf_kernel_0,
            kernel_size.width - 1
        );
        buf_kernel_0 = set_bits!(
            DLA_BUF_KERNEL_0_HEIGHT_OFFSET,
            DLA_BUF_KERNEL_0_HEIGHT_BITMASK,
            buf_kernel_0,
            kernel_size.height - 1
        );
        self.write_u32(DLA_BUF_KERNEL_0, buf_kernel_0);
        // Set BUF_KERNEL_1
        let mut buf_kernel_1 = 0;
        buf_kernel_1 = set_bits!(
            DLA_BUF_KERNEL_1_NUM_OFFSET,
            DLA_BUF_KERNEL_1_NUM_BITMASK,
            buf_kernel_1,
            kernel_size.kernels - 1
        );
        self.write_u32(DLA_BUF_KERNEL_1, buf_kernel_1);
    }

    /// Signals to DLA that all input data has been set
    pub fn input_data_ready(&self, ready: bool) {
        let mut reg = self.read_u32(DLA_BUF_CTRL);
        reg = set_bits!(
            DLA_READ_B_VALID_OFFSET,
            DLA_READ_B_VALID_BITMASK,
            reg,
            ready as usize
        );
        self.write_u32(DLA_BUF_CTRL, reg);
    }

    /// Signals to DLA that all kernel/filter/weight data has been set
    pub fn kernel_data_ready(&self, ready: bool) {
        let mut reg = self.read_u32(DLA_BUF_CTRL);
        reg = set_bits!(
            DLA_READ_A_VALID_OFFSET,
            DLA_READ_A_VALID_BITMASK,
            reg,
            ready as usize
        );
        self.write_u32(DLA_BUF_CTRL, reg);
    }

    /// Enables post-processing
    fn enable_pp(&self, enable: bool) {
        let mut reg = self.read_u32(DLA_HANDSHAKE);
        reg = set_bits!(
            DLA_HANDSHAKE_BYPASS_ENABLE_OFFSET,
            DLA_HANDSHAKE_BYPASS_ENABLE_BITMASK,
            reg,
            enable as usize
        );
        self.write_u32(DLA_HANDSHAKE, reg);
    }

    /// Enables ReLU in post-processing. Post-processing needs to be enabled
    fn enable_relu(&self, enable: bool) {
        let mut reg = self.read_u32(DLA_HANDSHAKE);
        reg = set_bits!(
            DLA_HANDSHAKE_ACTIVE_ENABLE_OFFSET,
            DLA_HANDSHAKE_ACTIVE_ENABLE_BITMASK,
            reg,
            enable as usize
        );
        self.write_u32(DLA_HANDSHAKE, reg);
    }

    /// Enables bias in post-processing. Post-processing needs to be enabled
    fn enable_bias(&self, enable: bool) {
        let mut reg = self.read_u32(DLA_HANDSHAKE);
        reg = set_bits!(
            DLA_HANDSHAKE_BIAS_ENABLE_OFFSET,
            DLA_HANDSHAKE_BIAS_ENABLE_BITMASK,
            reg,
            enable as usize
        );
        self.write_u32(DLA_HANDSHAKE, reg);
    }

    /// Sets padding paramters for convolution
    fn set_input_padding(&self, padding: PaddingConfig) {
        let mut reg = 0;
        reg = set_bits!(
            DLA_BUF_PAD_TOP_OFFSET,
            DLA_BUF_PAD_TOP_BITMASK,
            reg,
            padding.top
        );
        reg = set_bits!(
            DLA_BUF_PAD_RIGHT_OFFSET,
            DLA_BUF_PAD_RIGHT_BITMASK,
            reg,
            padding.right
        );
        reg = set_bits!(
            DLA_BUF_PAD_BOTTOM_OFFSET,
            DLA_BUF_PAD_BOTTOM_BITMASK,
            reg,
            padding.bottom
        );
        reg = set_bits!(
            DLA_BUF_PAD_LEFT_OFFSET,
            DLA_BUF_PAD_LEFT_BITMASK,
            reg,
            padding.left
        );
        reg = set_bits!(
            DLA_BUF_PAD_VALUE_OFFSET,
            DLA_BUF_PAD_VALUE_BITMASK,
            reg,
            padding.padding_value
        );
        self.write_u32(DLA_BUF_PAD, reg);
    }

    /// Sets stride paramters for convolution
    fn set_stride(&self, stride: StrideConfig) {
        let mut reg = 0;
        reg = set_bits!(
            DLA_BUF_STRIDE_X_OFFSET,
            DLA_BUF_STRIDE_X_BITMASK,
            reg,
            stride.x - 1
        );
        reg = set_bits!(
            DLA_BUF_STRIDE_Y_OFFSET,
            DLA_BUF_STRIDE_Y_BITMASK,
            reg,
            stride.y - 1
        );
        self.write_u32(DLA_BUF_STRIDE, reg);
    }

    /// Get status of calculation from DLA
    pub fn get_status(&self) -> u32 {
        self.read_u32(DLA_STATUS_ADDR)
    }

    /// Sets simd mode for conv2d
    fn set_simd_mode(&self, mode: SimdBitMode) {
        let mut reg = self.read_u32(DLA_MAC_CTRL);
        reg = set_bits!(
            DLA_SIMD_SELECT_OFFSET,
            DLA_SIMD_SELECT_BITMASK,
            reg,
            mode as usize
        );
        self.write_u32(DLA_MAC_CTRL, reg)
    }

    /// Gets simd mode for conv2d
    fn get_simd_mode(&self) -> SimdBitMode {
        let mut reg = self.read_u32(DLA_MAC_CTRL);
        reg = get_bits!(reg, DLA_SIMD_SELECT_BITMASK);
        match reg {
            0 => SimdBitMode::EightBits,
            1 => SimdBitMode::FourBits,
            2 => SimdBitMode::TwoBits,
            _ => SimdBitMode::EightBits,
        }
    }

    /// Reads index of the first input bank
    fn get_input_bank(&self) -> MemoryBank {
        let mut reg = self.read_u32(DLA_BUF_DATA_BANK);
        reg = get_bits!(reg, DLA_BUF_DATA_BANK_B_BITMASK);
        // Shift value back here
        reg = reg >> 16;
        MemoryBank::try_from(reg).unwrap()
    }

    /// Reads index of the first kernel bank
    fn get_kernel_bank(&self) -> MemoryBank {
        let mut reg = self.read_u32(DLA_BUF_DATA_BANK);
        reg = get_bits!(reg, DLA_BUF_DATA_BANK_A_BITMASK);
        MemoryBank::try_from(reg).unwrap()
    }

    /// Reads index of the first output bank
    fn get_output_bank(&self) -> MemoryBank {
        let reg = self.read_u32(DLA_PP_AXI_WRITE);
        let bank_idx: u32 = (reg - MEMORY_BANK_BASE_ADDR as u32) / MEMORY_BANK_SIZE as u32;
        MemoryBank::try_from(bank_idx).unwrap()
    }

    /// Reads kernel parameters from DLA
    fn get_kernel_size(&self) -> KernelSize {
        let reg0 = self.read_u32(DLA_BUF_KERNEL_0);
        let reg1 = self.read_u32(DLA_BUF_KERNEL_1);
        let width = get_bits!(reg0, DLA_BUF_KERNEL_0_WIDTH_BITMASK) + 1;
        let height = (get_bits!(reg0, DLA_BUF_KERNEL_0_HEIGHT_BITMASK)
            >> DLA_BUF_KERNEL_0_HEIGHT_OFFSET)
            + 1;
        let s_channels = get_bits!(reg0, DLA_BUF_KERNEL_0_S_CHANNELS_BITMASK) + 1;
        let kernels = get_bits!(reg1, DLA_BUF_KERNEL_1_NUM_BITMASK) + 1;

        KernelSize {
            s_channels,
            kernels,
            height,
            width,
        }
    }

    /// Reads input parameters from DLA
    fn get_input_size(&self) -> InputSize {
        let reg = self.read_u32(DLA_BUF_INPUT);
        let width = get_bits!(reg, DLA_BUF_INPUT_WIDTH_BITMASK) + 1;
        let height =
            (get_bits!(reg, DLA_BUF_INPUT_HEIGHT_BITMASK) >> DLA_BUF_INPUT_HEIGHT_OFFSET) + 1;
        let channels =
            (get_bits!(reg, DLA_BUF_INPUT_CHANNELS_BITMASK) >> DLA_BUF_INPUT_CHANNELS_OFFSET) + 1;

        InputSize {
            channels,
            height,
            width,
        }
    }

    /// Sets clipping after conv2d
    fn set_mac_clip(&self, clip_amount: u32) -> Result<(), InvalidClip> {
        // Cap clipping amount
        if clip_amount > 21 {
            return Err(InvalidClip(clip_amount));
        }
        let mut reg = self.read_u32(DLA_MAC_CTRL);
        reg = set_bits!(DLA_MAC_CLIP_OFFSET, DLA_MAC_CLIP_BITMASK, reg, clip_amount);
        self.write_u32(DLA_MAC_CTRL, reg);
        Ok(())
    }

    /// Sets clipping after post-processing
    fn set_pp_clip(&self, clip_amount: u32) -> Result<(), InvalidClip> {
        // Cap clipping amount
        if clip_amount > 0x1F {
            return Err(InvalidClip(clip_amount));
        }
        let mut reg = self.read_u32(DLA_PP_CTRL);
        reg = set_bits!(DLA_PP_CLIP_OFFSET, DLA_PP_CLIP_BITMASK, reg, clip_amount);
        self.write_u32(DLA_PP_CTRL, reg);
        Ok(())
    }

    /// Sets rounding after post-processing
    fn set_pp_rounding(&self, enable: bool) {
        let mut reg = self.read_u32(DLA_PP_CTRL);
        reg = set_bits!(
            DLA_ROUNDING_OFFSET,
            DLA_ROUNDING_BITMASK,
            reg,
            enable as usize
        );
        self.write_u32(DLA_PP_CTRL, reg);
    }

    /// Checks if calculations are ready in DLA
    pub fn is_ready(&self) -> bool {
        let status = self.read_u32(DLA_STATUS_ADDR);
        !get_bits!(status, DLA_BUF_DONE_BITMASK) != 0
    }

    /// Sets external memory address containing bias data for post-processing
    fn set_bias_addr(&self, addr: u32) {
        self.write_u32(DLA_PP_AXI_READ, addr);
    }

    /// Checks if all functions have been enabled
    pub fn is_enabled(&self) -> bool {
        let handshake_reg = self.read_u32(DLA_HANDSHAKE);
        let buf_enabled = get_bits!(handshake_reg, DLA_HANDSHAKE_BUFFER_ENABLE_BITMASK) != 0;
        let mac_enabled = get_bits!(handshake_reg, DLA_HANDSHAKE_MAC_ENABLE_BITMASK) != 0;
        let active_enabled = get_bits!(handshake_reg, DLA_HANDSHAKE_ACTIVE_ENABLE_BITMASK) != 0;
        buf_enabled || mac_enabled || active_enabled
    }

    /// Responds to DLA handshake by disabling hardware
    fn handshake_disable_hw(&self) {
        let mut handshake_reg = self.read_u32(DLA_HANDSHAKE);
        handshake_reg = set_bits!(
            DLA_HANDSHAKE_BUFFER_ENABLE_OFFSET,
            DLA_HANDSHAKE_BUFFER_ENABLE_BITMASK,
            handshake_reg,
            0
        );
        handshake_reg = set_bits!(
            DLA_HANDSHAKE_MAC_ENABLE_OFFSET,
            DLA_HANDSHAKE_MAC_ENABLE_BITMASK,
            handshake_reg,
            0
        );
        handshake_reg = set_bits!(
            DLA_HANDSHAKE_ACTIVE_ENABLE_OFFSET,
            DLA_HANDSHAKE_ACTIVE_ENABLE_BITMASK,
            handshake_reg,
            0
        );
        handshake_reg = set_bits!(
            DLA_HANDSHAKE_BIAS_ENABLE_OFFSET,
            DLA_HANDSHAKE_BIAS_ENABLE_BITMASK,
            handshake_reg,
            0
        );
        handshake_reg = set_bits!(
            DLA_HANDSHAKE_BYPASS_ENABLE_OFFSET,
            DLA_HANDSHAKE_BYPASS_ENABLE_BITMASK,
            handshake_reg,
            0
        );

        self.write_u32(DLA_HANDSHAKE, handshake_reg);
    }

    /// Performs handshake with DLA
    pub fn handle_handshake(&self) -> bool {
        // Handshake only if dla status is done
        if !self.is_ready() {
            sprintln!("Result not ready");
            return false;
        }

        if self.is_enabled() {
            sprintln!("DLA still enabled");
            self.handshake_disable_hw();
            return false;
        }
        sprintln!("Finishing handshake");

        let mut handshake_reg = self.read_u32(DLA_HANDSHAKE);
        handshake_reg = set_bits!(
            DLA_HANDSHAKE_BUFFER_VALID_OFFSET,
            DLA_HANDSHAKE_BUFFER_VALID_BITMASK,
            handshake_reg,
            1
        );
        handshake_reg = set_bits!(
            DLA_HANDSHAKE_MAC_VALID_OFFSET,
            DLA_HANDSHAKE_MAC_VALID_BITMASK,
            handshake_reg,
            1
        );
        handshake_reg = set_bits!(
            DLA_HANDSHAKE_ACTIVE_VALID_OFFSET,
            DLA_HANDSHAKE_ACTIVE_VALID_BITMASK,
            handshake_reg,
            1
        );

        self.write_u32(DLA_HANDSHAKE, handshake_reg);
        true
    }

    /// Prepares DLA for receiveing configuration for next layer
    fn handshake_next_layer(&self) {
        let mut reg = self.read_u32(DLA_HANDSHAKE);
        reg = set_bits!(
            DLA_HANDSHAKE_BUFFER_ENABLE_OFFSET,
            DLA_HANDSHAKE_BUFFER_ENABLE_BITMASK,
            reg,
            1
        );
        reg = set_bits!(
            DLA_HANDSHAKE_MAC_ENABLE_OFFSET,
            DLA_HANDSHAKE_MAC_ENABLE_BITMASK,
            reg,
            1
        );
        reg = set_bits!(
            DLA_HANDSHAKE_BYPASS_ENABLE_OFFSET,
            DLA_HANDSHAKE_BYPASS_ENABLE_BITMASK,
            reg,
            1
        );
        self.write_u32(DLA_HANDSHAKE, reg);
    }

    /// Configures the next layer in dla
    ///
    /// # Examples
    ///
    /// ```
    /// let dla = Dla::new();
    /// let layer = LayerConfig {...};
    /// dla.init_layer(layer)
    /// ```
    pub fn init_layer(&self, config: LayerConfig) {
        // Handshake for next layer
        self.handshake_next_layer();

        // Set memory banks
        self.set_input_data_bank(config.input_bank.unwrap_or(DEFAULT_INPUT_BANK));
        self.set_kernel_data_bank(config.kernel_bank.unwrap_or(DEFAULT_KERNEL_BANK));
        self.set_output_bank(config.output_bank.unwrap_or(DEFAULT_OUTPUT_BANK));

        // Set bias address
        self.set_bias_addr(config.bias_addr.unwrap_or(DEFAULT_BIAS_ADDR));

        // Enable post processor
        self.enable_pp(config.pp_enabled);
        self.enable_relu(config.relu_enabled);
        self.enable_bias(config.bias_enabled);

        // Set input and kernel dimensions
        self.set_kernel_size(config.kernel_size.unwrap_or(DEFAULT_KERNEL_SIZE));

        self.set_input_size(config.input_size.unwrap_or(DEFAULT_INPUT_SIZE));

        // Set simd
        self.set_simd_mode(config.simd_mode.unwrap_or(DEFAULT_SIMD_MODE));

        // Set padding
        self.set_input_padding(config.padding.unwrap_or(DEFAULT_PADDING));

        // Set stride
        self.set_stride(config.stride.unwrap_or(DEFAULT_STRIDE));

        // Set clipping
        match self.set_mac_clip(config.mac_clip.unwrap_or(DEFAULT_MAC_CLIP)) {
            Ok(_) => (),
            Err(_) => sprintln!("Mac clip value, exceeds allowed maximum of 21"),
        }
        match self.set_pp_clip(config.pp_clip.unwrap_or(DEFAULT_PP_CLIP)) {
            Ok(_) => (),
            Err(_) => sprintln!("PP clip value, exceeds allowed maximum of 21"),
        }
    }
}
