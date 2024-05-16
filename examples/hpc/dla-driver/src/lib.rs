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
    channels: 1,
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
    value: 0,
};
const DEFAULT_STRIDE: StrideConfig = StrideConfig { x: 1, y: 1 };
const DEFAULT_MAC_CLIP: u32 = 8;
const DEFAULT_PP_CLIP: u32 = 8;
const DEFAULT_SIMD_MODE: SimdBitMode = SimdBitMode::EightBits;

use alloc::vec::Vec;
use core::ptr;
use headsail_bsp::{sprint, sprintln};
use mmap::*;

pub struct KernelSize {
    pub channels: u32,
    pub width: u32,
    pub height: u32,
}

pub struct InputSize {
    pub channels: u32,
    pub width: u32,
    pub height: u32,
}

pub struct PaddingConfig {
    pub top: u32,
    pub right: u32,
    pub left: u32,
    pub bottom: u32,
    pub value: u32,
}

pub struct StrideConfig {
    pub x: u32,
    pub y: u32,
}

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
pub enum MemoryBank {
    Bank0, Bank1, Bank2, Bank3, Bank4, Bank5, Bank6, Bank7, Bank8, Bank9,
    Bank10, Bank11, Bank12, Bank13, Bank14, Bank15,
}

impl MemoryBank {
    fn from_u32(value: u32) -> MemoryBank {
        match value {
            0 => MemoryBank::Bank0,
            1 => MemoryBank::Bank1,
            2 => MemoryBank::Bank2,
            3 => MemoryBank::Bank3,
            4 => MemoryBank::Bank4,
            5 => MemoryBank::Bank5,
            6 => MemoryBank::Bank6,
            7 => MemoryBank::Bank7,
            8 => MemoryBank::Bank8,
            9 => MemoryBank::Bank9,
            10 => MemoryBank::Bank10,
            11 => MemoryBank::Bank11,
            12 => MemoryBank::Bank12,
            13 => MemoryBank::Bank13,
            14 => MemoryBank::Bank14,
            15 => MemoryBank::Bank15,
            _ => MemoryBank::Bank0,
        }
    }

    fn addr(&self) -> usize {
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
    fn value(&self) -> usize {
        match self {
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

#[derive(Copy, Clone)]
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

pub struct Dla {}

impl Dla {
    pub fn new() -> Self {
        Dla {}
    }
    pub fn write_u8(&self, offset: usize, value: u8) {
        unsafe { ptr::write_volatile((offset) as *mut _, value) };
    }

    fn write_u32(&self, offset: usize, value: u32) {
        unsafe { ptr::write_volatile((DLA0_ADDR + offset) as *mut _, value) }
    }

    fn read_u32(&self, offset: usize) -> u32 {
        unsafe { ptr::read_volatile((DLA0_ADDR + offset) as *mut _) }
    }

    pub fn read_bytes(&self, offset: usize, len: usize, buf: &mut [u8]) {
        for i in 0..len {
            unsafe { buf[i] = ptr::read_volatile((DLA0_ADDR + offset + i) as *mut _) }
        }
    }

    pub fn write_data_bank(&self, offset: usize, buf: &mut [i8]) {
        //sprintln!("\nWrite to bank {:#x}, data: {:?}", offset, buf);
        for (i, b) in buf.iter().enumerate() {
            unsafe { ptr::write_volatile((MEMORY_BANK_BASE_ADDR + offset + i) as *mut _, *b) };
        }
    }

    fn read_data_bank_offset(&self, bank: &MemoryBank, offset: usize) -> u128 {
        // NOTE: this function enforces the 128-bit addressing
        if cfg!(feature = "vp") {
            let mut result: u128 = 0;
            for i in 0..4 {
                result |= (unsafe {
                    ptr::read_volatile(
                        (MEMORY_BANK_BASE_ADDR + bank.addr() + offset + (i * 4)) as *mut u32,
                    )
                } as u128)
                    << (32 * i)
            }
            result
        } else {
            unsafe {
                ptr::read_volatile((MEMORY_BANK_BASE_ADDR + bank.addr() + offset) as *mut _)
            }
        }
    }

    fn read_data_bank(&self, bank: &MemoryBank, len: usize) -> Vec<i8> {
        let mut res: Vec<i8> = Vec::new();

        let mut next_bank_offset = 0;
        while res.len() < len {
            let data = self.read_data_bank_offset(bank, next_bank_offset);
            let bytes_remaining = len - res.len();
            let bytes_to_copy = core::cmp::min(16, bytes_remaining);

            // Copy everything from one 128-bit address
            for i in 0..bytes_to_copy {
                let byte = ((data >> (i * 8)) & 0xFF) as i8;
                res.push(byte)
            }
            next_bank_offset = next_bank_offset + 0x10;
        }
        res
    }

    pub fn read_output(&self, len: usize) -> Vec<i8> {
        // VP only support reading from banks
        if cfg!(feature = "vp") {
            return self.read_data_bank(&self.get_output_bank(), len);
        }
        self.read_data_bank(&MemoryBank::Bank0, len)
    }

    pub fn read_input_bank(&self, len: usize) -> Vec<i8> {
        self.read_data_bank(&self.get_input_bank(), len)
    }

    pub fn read_weight_bank(&self, len: usize) -> Vec<i8> {
        self.read_data_bank(&self.get_kernel_bank(), len)
    }

    pub fn write_input(&self, input: &mut [i8]) {
        // TODO optimize memory bank logic
        self.write_data_bank(self.get_input_bank().addr(), input)
    }

    pub fn write_kernel(&self, kernel: &mut [i8]) {
        // TODO optimize memory bank logic
        self.write_data_bank(self.get_kernel_bank().addr(), kernel)
    }

    fn set_input_data_bank(&self, bank: MemoryBank) {
        let mut reg = self.read_u32(DLA_BUF_DATA_BANK);
        reg = set_bits!(
            DLA_BUF_DATA_BANK_B_OFFSET,
            DLA_BUF_DATA_BANK_B_BITMASK,
            reg,
            bank.value()
        );
        self.write_u32(DLA_BUF_DATA_BANK, reg);
    }

    fn set_kernel_data_bank(&self, bank: MemoryBank) {
        let mut reg = self.read_u32(DLA_BUF_DATA_BANK);
        reg = set_bits!(
            DLA_BUF_DATA_BANK_A_OFFSET,
            DLA_BUF_DATA_BANK_A_BITMASK,
            reg,
            bank.value()
        );
        self.write_u32(DLA_BUF_DATA_BANK, reg);
    }

    fn set_output_bank(&self, bank: MemoryBank) {
        let mut reg = self.read_u32(DLA_PP_AXI_WRITE);
        reg = set_bits!(
            DLA_PP_AXI_WRITE_ADDRESS_OFFSET,
            DLA_PP_AXI_WRITE_ADDRESS_BITMASK,
            reg,
            bank.addr() + MEMORY_BANK_BASE_ADDR
        );
        self.write_u32(DLA_PP_AXI_WRITE, reg);
    }

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

    fn set_kernel_size(&self, kernel_size: KernelSize) {
        let mut reg = 0;
        reg = set_bits!(
            DLA_BUF_KERNEL_0_S_CHANNELS_OFFSET,
            DLA_BUF_KERNEL_0_S_CHANNELS_BITMASK,
            reg,
            kernel_size.channels - 1
        );
        reg = set_bits!(
            DLA_BUF_KERNEL_0_WIDTH_OFFSET,
            DLA_BUF_KERNEL_0_WIDTH_BITMASK,
            reg,
            kernel_size.width - 1
        );
        reg = set_bits!(
            DLA_BUF_KERNEL_0_HEIGHT_OFFSET,
            DLA_BUF_KERNEL_0_HEIGHT_BITMASK,
            reg,
            kernel_size.height - 1
        );
        self.write_u32(DLA_BUF_KERNEL_0, reg);
    }

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
            padding.value
        );
        self.write_u32(DLA_BUF_PAD, reg);
    }

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

    pub fn get_status(&self) -> u32 {
        return self.read_u32(DLA_STATUS_ADDR);
    }

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

    fn get_input_bank(&self) -> MemoryBank {
        let mut reg = self.read_u32(DLA_BUF_DATA_BANK);
        reg = get_bits!(reg, DLA_BUF_DATA_BANK_B_BITMASK);
        MemoryBank::from_u32(reg)
    }

    fn get_kernel_bank(&self) -> MemoryBank {
        let mut reg = self.read_u32(DLA_BUF_DATA_BANK);
        reg = get_bits!(reg, DLA_BUF_DATA_BANK_A_BITMASK);
        MemoryBank::from_u32(reg)
    }

    fn get_output_bank(&self) -> MemoryBank {
        let reg = self.read_u32(DLA_PP_AXI_WRITE);
        let bank_idx: u32 = (reg - MEMORY_BANK_BASE_ADDR as u32) / MEMORY_BANK_SIZE as u32;
        MemoryBank::from_u32(bank_idx)
    }

    fn set_mac_clip(&self, clip_amount: u32) {
        let mut reg = self.read_u32(DLA_MAC_CTRL);
        // Cap clipping amount
        if clip_amount > 21 {
            reg = set_bits!(DLA_MAC_CLIP_OFFSET, DLA_MAC_CLIP_BITMASK, reg, 0x1F);
        } else {
            reg = set_bits!(DLA_MAC_CLIP_OFFSET, DLA_MAC_CLIP_BITMASK, reg, clip_amount);
        }
        self.write_u32(DLA_MAC_CTRL, reg)
    }

    fn set_pp_clip(&self, clip_amount: u32) {
        let mut reg = self.read_u32(DLA_PP_CTRL);
        // Cap clipping amount
        if clip_amount > 0x1F {
            reg = set_bits!(DLA_PP_CLIP_OFFSET, DLA_PP_CLIP_BITMASK, reg, 0x1F);
        } else {
            reg = set_bits!(DLA_PP_CLIP_OFFSET, DLA_PP_CLIP_BITMASK, reg, clip_amount);
        }
        self.write_u32(DLA_PP_CTRL, reg)
    }

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

    pub fn is_ready(&self) -> bool {
        let status = self.read_u32(DLA_STATUS_ADDR);
        return !get_bits!(status, DLA_BUF_DONE_BITMASK) != 0;
    }

    fn set_bias_addr(&self, addr: u32) {
        self.write_u32(DLA_PP_AXI_READ, addr);
    }

    pub fn is_enabled(&self) -> bool {
        let handshake_reg = self.read_u32(DLA_HANDSHAKE);
        let buf_enabled = get_bits!(handshake_reg, DLA_HANDSHAKE_BUFFER_ENABLE_BITMASK) != 0;
        let mac_enabled = get_bits!(handshake_reg, DLA_HANDSHAKE_MAC_ENABLE_BITMASK) != 0;
        let active_enabled = get_bits!(handshake_reg, DLA_HANDSHAKE_ACTIVE_ENABLE_BITMASK) != 0;
        buf_enabled & mac_enabled & active_enabled
    }

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

    pub fn handle_handshake(&self) -> bool {
        // Handshake only if dla status is done
        if !self.is_ready() {
            return false;
        }

        if self.is_enabled() {
            self.handshake_disable_hw();
            return false;
        }

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
        return true;
    }

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
        self.set_mac_clip(config.mac_clip.unwrap_or(DEFAULT_MAC_CLIP));
        self.set_pp_clip(config.pp_clip.unwrap_or(DEFAULT_PP_CLIP));
    }
}
