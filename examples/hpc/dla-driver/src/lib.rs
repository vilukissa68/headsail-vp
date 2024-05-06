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

use alloc::vec::*;
use core::ptr;
use headsail_bsp::{sprint, sprintln};
use mmap::*;

pub struct Dla {
    simd_mode: SimdBitMode,
    input_bank: MemoryBank,
    kernel_bank: MemoryBank,
    output_addr: usize,
}

pub enum MemoryBank {
    BANK0,
    BANK1,
    BANK2,
    BANK3,
    BANK4,
    BANK5,
    BANK6,
    BANK7,
    BANK8,
    BANK9,
    BANK10,
    BANK11,
    BANK12,
    BANK13,
    BANK14,
    BANK15,
}

impl MemoryBank {
    fn addr(&self) -> usize {
        match self {
                MemoryBank::BANK0 => MEMORY_BANK_0_OFFSET,
                MemoryBank::BANK1 => MEMORY_BANK_1_OFFSET,
                MemoryBank::BANK2 => MEMORY_BANK_2_OFFSET,
                MemoryBank::BANK3 => MEMORY_BANK_3_OFFSET,
                MemoryBank::BANK4 => MEMORY_BANK_4_OFFSET,
                MemoryBank::BANK5 => MEMORY_BANK_5_OFFSET,
                MemoryBank::BANK6 => MEMORY_BANK_6_OFFSET,
                MemoryBank::BANK7 => MEMORY_BANK_7_OFFSET,
                MemoryBank::BANK8 => MEMORY_BANK_8_OFFSET,
                MemoryBank::BANK9 => MEMORY_BANK_9_OFFSET,
                MemoryBank::BANK10 => MEMORY_BANK_10_OFFSET,
                MemoryBank::BANK11 => MEMORY_BANK_11_OFFSET,
                MemoryBank::BANK12 => MEMORY_BANK_12_OFFSET,
                MemoryBank::BANK13 => MEMORY_BANK_13_OFFSET,
                MemoryBank::BANK14 => MEMORY_BANK_14_OFFSET,
                MemoryBank::BANK15 => MEMORY_BANK_15_OFFSET,
                _ => 0,
        }
    }
    fn value(&self) -> usize {
        match self {
            MemoryBank::BANK0 => 0,
            MemoryBank::BANK1 => 1,
            MemoryBank::BANK2 => 2,
            MemoryBank::BANK3 => 3,
            MemoryBank::BANK4 => 4,
            MemoryBank::BANK5 => 5,
            MemoryBank::BANK6 => 6,
            MemoryBank::BANK7 => 7,
            MemoryBank::BANK8 => 8,
            MemoryBank::BANK9 => 9,
            MemoryBank::BANK10 => 10,
            MemoryBank::BANK11 => 11,
            MemoryBank::BANK12 => 12,
            MemoryBank::BANK13 => 13,
            MemoryBank::BANK14 => 14,
            MemoryBank::BANK15 => 15,
            _ => 0,
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
    ($mask:expr, $reg:expr) => {
        ($reg & ($mask as u32)) as u32
    };
}

fn u2_to_u8(value: u8) -> u8 {
    return value & 0x3;
}

fn u4_to_u8(value: u8) -> u8 {
    return value & 0xF;
}

impl Dla {
    pub fn new() -> Self {
        return Dla {
            simd_mode: SimdBitMode::EightBits,
            input_bank: MemoryBank::BANK0,
            kernel_bank: MemoryBank::BANK8,
            output_addr: MEMORY_BANK_12_OFFSET + MEMORY_BANK_BASE_ADDR,
        };
    }
    pub fn write_str(&self, s: &str) {
        for b in s.as_bytes() {
            unsafe { ptr::write_volatile(DLA0_ADDR as *mut u8, *b) };
        }
    }
    pub fn write(&self, offset: usize, value: u8) {
        unsafe { ptr::write_volatile((offset) as *mut u8, value) };
    }

    pub fn write_reg(&self, offset: usize, value: u32) {
        unsafe { ptr::write_volatile((DLA0_ADDR + offset) as *mut u32, value) }
    }

    pub fn read_reg(&self, offset: usize) -> u32 {
        unsafe { ptr::read_volatile((DLA0_ADDR + offset) as *mut u32) }
    }

    pub fn read(&self, buf: &mut [u8], len: usize, offset: usize) {
        for i in 0..len {
            unsafe { buf[i] = ptr::read_volatile((DLA0_ADDR + offset + i) as *mut u8) }
        }
    }

    pub fn write_data_bank(&self, offset: usize, buf: &mut [u8]) {
        //sprintln!("\nWrite to bank {:#x}, data: {:?}", offset, buf);
        for (i, b) in buf.iter().enumerate() {
            unsafe { ptr::write_volatile((MEMORY_BANK_BASE_ADDR + offset + i) as *mut u8, *b) };
        }
    }

    pub fn read_data_bank_offset(&self, offset: usize) -> u128 {
        // NOTE: this function enforces the 128-bit addressing
        if cfg!(feature = "vp") {
            let mut result: u128 = 0;
            for i in 0..4 {
                result |= (unsafe {
                    ptr::read_volatile((MEMORY_BANK_BASE_ADDR + offset + (i * 4)) as *mut u32)
                } as u128)
                    << (32 * i)
            }
            result
        } else {
            unsafe { ptr::read_volatile((MEMORY_BANK_BASE_ADDR + (offset & !0xF)) as *mut u128) }
        }
    }

    pub fn read_data_bank(&self, offset: usize, len: usize) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();

        let mut next_bank_offset = offset;
        while res.len() < len {
            let data = self.read_data_bank_offset(next_bank_offset);
            let bytes_remaining = len - res.len();
            let bytes_to_copy = core::cmp::min(16, bytes_remaining);

            // Copy everything from one 128-bit address
            for i in 0..bytes_to_copy {
                let byte = ((data >> (i * 8)) & 0xFF) as u8;
                res.push(byte)
            }
            next_bank_offset = offset + 0x10;
        }
        res
    }

    pub fn read_output(&self, len: usize) -> Vec<u8> {
        // VP only support reading from banks
        if cfg!(feature = "vp") {
            return self.read_data_bank(self.output_addr - MEMORY_BANK_BASE_ADDR, len);
        }
        self.read_data_bank(MEMORY_BANK_0_OFFSET, len)
    }

    pub fn read_input_bank(&self, len: usize) -> Vec<u8> {
        self.read_data_bank(self.input_bank.addr(), len)
    }

    pub fn read_weight_bank(&self, len: usize) -> Vec<u8> {
        self.read_data_bank(self.kernel_bank.addr(), len)
    }

    pub fn write_input(&self, input: &mut [u8]) {
        // TODO optimize memory bank logic
        self.write_data_bank(self.input_bank.addr(), input)
    }

    pub fn write_kernel(&self, kernel: &mut [u8]) {
        // TODO optimize memory bank logic
        self.write_data_bank(self.kernel_bank.addr(), kernel)
    }

    pub fn set_input_data_bank(&mut self, bank: MemoryBank) {
        self.input_bank = bank;
        let mut reg = self.read_reg(DLA_BUF_DATA_BANK);
        reg = set_bits!(
            DLA_BUF_DATA_BANK_B_OFFSET,
            DLA_BUF_DATA_BANK_B_BITMASK,
            reg,
            self.input_bank.value()
        );
        self.write_reg(DLA_BUF_DATA_BANK, reg);
    }

    pub fn set_kernel_data_bank(&mut self, bank: MemoryBank) {
        self.kernel_bank = bank;
        let mut reg = self.read_reg(DLA_BUF_DATA_BANK);
        reg = set_bits!(
            DLA_BUF_DATA_BANK_A_OFFSET,
            DLA_BUF_DATA_BANK_A_BITMASK,
            reg,
            self.kernel_bank.value()
        );
        self.write_reg(DLA_BUF_DATA_BANK, reg);
    }

    pub fn set_kernel_output_addr(&mut self, addr: usize) {
        self.output_addr = addr;
        let mut reg = self.read_reg(DLA_PP_AXI_WRITE);
        reg = set_bits!(
            DLA_PP_AXI_WRITE_ADDRESS_OFFSET,
            DLA_PP_AXI_WRITE_ADDRESS_BITMASK,
            reg,
            self.output_addr
        );
        self.write_reg(DLA_PP_AXI_WRITE, reg);
    }

    pub fn set_input_size(&self, channels: usize, width: usize, height: usize) {
        let mut reg = 0;
        reg = set_bits!(
            DLA_BUF_INPUT_CHANNELS_OFFSET,
            DLA_BUF_INPUT_CHANNELS_BITMASK,
            reg,
            channels - 1
        );
        reg = set_bits!(
            DLA_BUF_INPUT_WIDTH_OFFSET,
            DLA_BUF_INPUT_WIDTH_BITMASK,
            reg,
            width - 1
        );
        reg = set_bits!(
            DLA_BUF_INPUT_HEIGHT_OFFSET,
            DLA_BUF_INPUT_HEIGHT_BITMASK,
            reg,
            height - 1
        );
        self.write_reg(DLA_BUF_INPUT, reg);
    }

    pub fn set_kernel_size(&self, channels: usize, width: usize, height: usize) {
        let mut reg = 0;
        reg = set_bits!(
            DLA_BUF_KERNEL_0_S_CHANNELS_OFFSET,
            DLA_BUF_KERNEL_0_S_CHANNELS_BITMASK,
            reg,
            channels - 1
        );
        reg = set_bits!(
            DLA_BUF_KERNEL_0_WIDTH_OFFSET,
            DLA_BUF_KERNEL_0_WIDTH_BITMASK,
            reg,
            width - 1
        );
        reg = set_bits!(
            DLA_BUF_KERNEL_0_HEIGHT_OFFSET,
            DLA_BUF_KERNEL_0_HEIGHT_BITMASK,
            reg,
            height - 1
        );
        self.write_reg(DLA_BUF_KERNEL_0, reg);
    }

    pub fn input_data_ready(&self, ready: bool) {
        let mut reg = self.read_reg(DLA_BUF_CTRL);
        reg = set_bits!(
            DLA_READ_B_VALID_OFFSET,
            DLA_READ_B_VALID_BITMASK,
            reg,
            ready as usize
        );
        self.write_reg(DLA_BUF_CTRL, reg);
    }

    pub fn kernel_data_ready(&self, ready: bool) {
        let mut reg = self.read_reg(DLA_BUF_CTRL);
        reg = set_bits!(
            DLA_READ_A_VALID_OFFSET,
            DLA_READ_A_VALID_BITMASK,
            reg,
            ready as usize
        );
        self.write_reg(DLA_BUF_CTRL, reg);
    }

    pub fn enable_pp(&self, enable: bool) {
        let mut reg = self.read_reg(DLA_HANDSHAKE);
        reg = set_bits!(
            DLA_HANDSHAKE_BYPASS_ENABLE_OFFSET,
            DLA_HANDSHAKE_BYPASS_ENABLE_BITMASK,
            reg,
            enable as usize
        );
        self.write_reg(DLA_HANDSHAKE, reg);
    }

    pub fn enable_relu(&self, enable: bool) {
        let mut reg = self.read_reg(DLA_HANDSHAKE);
        reg = set_bits!(
            DLA_HANDSHAKE_ACTIVE_ENABLE_OFFSET,
            DLA_HANDSHAKE_ACTIVE_ENABLE_BITMASK,
            reg,
            enable as usize
        );
        self.write_reg(DLA_HANDSHAKE, reg);
    }

    pub fn enable_bias(&self, enable: bool) {
        let mut reg = self.read_reg(DLA_HANDSHAKE);
        reg = set_bits!(
            DLA_HANDSHAKE_BIAS_ENABLE_OFFSET,
            DLA_HANDSHAKE_BIAS_ENABLE_BITMASK,
            reg,
            enable as usize
        );
        self.write_reg(DLA_HANDSHAKE, reg);
    }

    pub fn set_input_padding(
        &self,
        top: usize,
        right: usize,
        bottom: usize,
        left: usize,
        value: usize,
    ) {
        let mut reg = 0;
        reg = set_bits!(DLA_BUF_PAD_TOP_OFFSET, DLA_BUF_PAD_TOP_BITMASK, reg, top);
        reg = set_bits!(
            DLA_BUF_PAD_RIGHT_OFFSET,
            DLA_BUF_PAD_RIGHT_BITMASK,
            reg,
            right
        );
        reg = set_bits!(
            DLA_BUF_PAD_BOTTOM_OFFSET,
            DLA_BUF_PAD_BOTTOM_BITMASK,
            reg,
            bottom
        );
        reg = set_bits!(DLA_BUF_PAD_LEFT_OFFSET, DLA_BUF_PAD_LEFT_BITMASK, reg, left);
        reg = set_bits!(
            DLA_BUF_PAD_VALUE_OFFSET,
            DLA_BUF_PAD_VALUE_BITMASK,
            reg,
            value
        );
        self.write_reg(DLA_BUF_PAD, reg);
    }

    pub fn set_stride(&self, x: usize, y: usize) {
        let mut reg = 0;
        reg = set_bits!(
            DLA_BUF_STRIDE_X_OFFSET,
            DLA_BUF_STRIDE_X_BITMASK,
            reg,
            x - 1
        );
        reg = set_bits!(
            DLA_BUF_STRIDE_Y_OFFSET,
            DLA_BUF_STRIDE_Y_BITMASK,
            reg,
            y - 1
        );
        self.write_reg(DLA_BUF_STRIDE, reg);
    }

    pub fn set_bias_address(&self, addr: usize) {
        let reg = set_bits!(
            DLA_PP_AXI_READ_ADDRESS_OFFSET,
            DLA_PP_AXI_READ_ADDRESS_BITMASK,
            0,
            addr
        );
        self.write_reg(DLA_PP_AXI_READ, reg);
    }

    pub fn get_status(&self) -> u32 {
        return self.read_reg(DLA_STATUS_ADDR);
    }

    pub fn set_simd_select(&mut self, mode: SimdBitMode) {
        self.simd_mode = mode.clone();
        let mut reg = self.read_reg(DLA_MAC_CTRL);
        reg = set_bits!(
            DLA_SIMD_SELECT_OFFSET,
            DLA_SIMD_SELECT_BITMASK,
            reg,
            mode as usize
        );
        self.write_reg(DLA_MAC_CTRL, reg)
    }

    pub fn get_simd_format(&self) -> SimdBitMode {
        let mut reg = self.read_reg(DLA_MAC_CTRL);
        reg = get_bits!(DLA_SIMD_SELECT_BITMASK, reg);
        match reg {
            0 => SimdBitMode::EightBits,
            1 => SimdBitMode::FourBits,
            2 => SimdBitMode::TwoBits,
            _ => SimdBitMode::EightBits,
        }
    }

    pub fn set_mac_clip(&self, clip_amount: usize) {
        let mut reg = self.read_reg(DLA_MAC_CTRL);
        // Cap clipping amount
        if clip_amount > 21 {
            reg = set_bits!(DLA_MAC_CLIP_OFFSET, DLA_MAC_CLIP_BITMASK, reg, 0x1F);
        } else {
            reg = set_bits!(DLA_MAC_CLIP_OFFSET, DLA_MAC_CLIP_BITMASK, reg, clip_amount);
        }
        self.write_reg(DLA_MAC_CTRL, reg)
    }

    pub fn set_pp_clip(&self, clip_amount: usize) {
        let mut reg = self.read_reg(DLA_PP_CTRL);
        // Cap clipping amount
        if clip_amount > 0x1F {
            reg = set_bits!(DLA_PP_CLIP_OFFSET, DLA_PP_CLIP_BITMASK, reg, 0x1F);
        } else {
            reg = set_bits!(DLA_PP_CLIP_OFFSET, DLA_PP_CLIP_BITMASK, reg, clip_amount);
        }
        self.write_reg(DLA_PP_CTRL, reg)
    }

    pub fn set_pp_rounding(&self, enable: bool) {
        let mut reg = self.read_reg(DLA_PP_CTRL);
        reg = set_bits!(
            DLA_ROUNDING_OFFSET,
            DLA_ROUNDING_BITMASK,
            reg,
            enable as usize
        );
        self.write_reg(DLA_PP_CTRL, reg);
    }

    pub fn is_ready(&self) -> bool {
        let status = self.read_reg(DLA_STATUS_ADDR);
        return !get_bits!(DLA_BUF_DONE_BITMASK, status) != 0;
    }

    pub fn set_bias_addr(&self, addr: u32) {
        self.write_reg(DLA_PP_AXI_READ, addr);
    }

    pub fn is_enabled(&self) -> bool {
        let handshake_reg = self.read_reg(DLA_HANDSHAKE);
        let buf_enabled = get_bits!(DLA_HANDSHAKE_BUFFER_ENABLE_BITMASK, handshake_reg) != 0;
        let mac_enabled = get_bits!(DLA_HANDSHAKE_MAC_ENABLE_BITMASK, handshake_reg) != 0;
        let active_enabled = get_bits!(DLA_HANDSHAKE_ACTIVE_ENABLE_BITMASK, handshake_reg) != 0;
        buf_enabled & mac_enabled & active_enabled
    }

    fn handshake_disable_hw(&self) {
        let mut handshake_reg = self.read_reg(DLA_HANDSHAKE);
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

        self.write_reg(DLA_HANDSHAKE, handshake_reg);
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

        let mut handshake_reg = self.read_reg(DLA_HANDSHAKE);
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

        self.write_reg(DLA_HANDSHAKE, handshake_reg);
        return true;
    }

    pub fn init_layer(&mut self) {
        let mut reg = self.read_reg(DLA_HANDSHAKE);
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
        self.write_reg(DLA_HANDSHAKE, reg);

        self.set_input_data_bank(MemoryBank::BANK0);
        self.set_kernel_data_bank(MemoryBank::BANK8);
        self.set_kernel_output_addr(MEMORY_BANK_12_OFFSET + MEMORY_BANK_BASE_ADDR);
        self.set_simd_select(SimdBitMode::EightBits);

        self.enable_pp(true);
        self.enable_relu(true);
        self.enable_bias(true);
    }
}
