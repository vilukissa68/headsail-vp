#![no_std]

extern crate alloc;

mod mmap;

use alloc::vec::*;
use core::ptr;
use headsail_bsp::{sprint, sprintln};
use mmap::*;

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
    return value & 0x3
}

fn u4_to_u8(value: u8) -> u8 {
    return value & 0xF
}


pub fn dla_write_str(s: &str) {
    for b in s.as_bytes() {
        unsafe { ptr::write_volatile(DLA0_ADDR as *mut u8, *b) };
    }
}
pub fn dla_write(offset: usize, value: u8) {
    unsafe { ptr::write_volatile((offset) as *mut u8, value) };
}

pub fn dla_write_reg(offset: usize, value: u32) {
    unsafe { ptr::write_volatile((DLA0_ADDR + offset) as *mut u32, value) }
}

pub fn dla_read_reg(offset: usize) -> u32 {
    unsafe { ptr::read_volatile((DLA0_ADDR + offset) as *mut u32) }
}

pub fn dla_read(buf: &mut [u8], len: usize, offset: usize) {
    for i in 0..len {
        unsafe { buf[i] = ptr::read_volatile((DLA0_ADDR + offset + i) as *mut u8) }
    }
}

pub fn dla_write_data_bank(offset: usize, buf: &mut [u8]) {
    //sprintln!("\nWrite to bank {:#x}, data: {:?}", offset, buf);
    for (i, b) in buf.iter().enumerate() {
        unsafe { ptr::write_volatile((MEMORY_BANK_BASE_ADDR + offset + i) as *mut u8, *b) };
    }
}

pub fn dla_read_data_bank_offset(offset: usize) -> u128 {
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

pub fn dla_read_data_bank(offset: usize, len: usize) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    let mut next_bank_offset = offset;
    while res.len() < len {
        let data = dla_read_data_bank_offset(next_bank_offset);
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

pub fn dla_read_result(len: usize) -> Vec<u8> {
    // On VP we default for results to be located in Bank 12 onwards
    if cfg!(feature = "vp") {
        return dla_read_data_bank(MEMORY_BANK_12_OFFSET, len);
    }
    dla_read_data_bank(MEMORY_BANK_0_OFFSET, len)
}

pub fn dla_read_input_bank(len: usize) -> Vec<u8> {
    dla_read_data_bank(MEMORY_BANK_0_OFFSET, len)
}

pub fn dla_read_weight_bank(len: usize) -> Vec<u8> {
    dla_read_data_bank(MEMORY_BANK_8_OFFSET, len)
}

pub fn dla_write_input(input: &mut [u8]) {
    // TODO optimize memory bank logic
    dla_write_data_bank(MEMORY_BANK_0_OFFSET, input)
}

pub fn dla_write_kernel(kernel: &mut [u8]) {
    // TODO optimize memory bank logic
    dla_write_data_bank(MEMORY_BANK_8_OFFSET, kernel)
}

pub fn dla_set_input_data_bank(bank: usize) {
    let mut reg = dla_read_reg(DLA_BUF_DATA_BANK);
    reg = set_bits!(
        DLA_BUF_DATA_BANK_B_OFFSET,
        DLA_BUF_DATA_BANK_B_BITMASK,
        reg,
        bank
    );
    dla_write_reg(DLA_BUF_DATA_BANK, reg);
}

pub fn dla_set_kernel_data_bank(bank: usize) {
    let mut reg = dla_read_reg(DLA_BUF_DATA_BANK);
    reg = set_bits!(
        DLA_BUF_DATA_BANK_A_OFFSET,
        DLA_BUF_DATA_BANK_A_BITMASK,
        reg,
        bank
    );
    dla_write_reg(DLA_BUF_DATA_BANK, reg);
}

pub fn dla_set_input_size(channels: usize, width: usize, height: usize) {
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
    dla_write_reg(DLA_BUF_INPUT, reg);
}

pub fn dla_set_kernel_size(channels: usize, width: usize, height: usize) {
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
    dla_write_reg(DLA_BUF_KERNEL_0, reg);
}

pub fn dla_input_data_ready(ready: bool) {
    let mut reg = dla_read_reg(DLA_BUF_CTRL);
    reg = set_bits!(
        DLA_READ_B_VALID_OFFSET,
        DLA_READ_B_VALID_BITMASK,
        reg,
        ready as usize
    );
    dla_write_reg(DLA_BUF_CTRL, reg);
}

pub fn dla_kernel_data_ready(ready: bool) {
    let mut reg = dla_read_reg(DLA_BUF_CTRL);
    reg = set_bits!(
        DLA_READ_A_VALID_OFFSET,
        DLA_READ_A_VALID_BITMASK,
        reg,
        ready as usize
    );
    dla_write_reg(DLA_BUF_CTRL, reg);
}

pub fn dla_enable_pp(enable: bool) {
    let mut reg = dla_read_reg(DLA_HANDSHAKE);
    reg = set_bits!(
        DLA_HANDSHAKE_BYPASS_ENABLE_OFFSET,
        DLA_HANDSHAKE_BYPASS_ENABLE_BITMASK,
        reg,
        enable as usize
    );
    sprintln!("reg: {}", reg);
    dla_write_reg(DLA_HANDSHAKE, reg);
}

pub fn dla_enable_relu(enable: bool) {
    let mut reg = dla_read_reg(DLA_HANDSHAKE);
    reg = set_bits!(
        DLA_HANDSHAKE_ACTIVE_ENABLE_OFFSET,
        DLA_HANDSHAKE_ACTIVE_ENABLE_BITMASK,
        reg,
        enable as usize
    );
    dla_write_reg(DLA_HANDSHAKE, reg);
}

pub fn dla_enable_bias(enable: bool) {
    let mut reg = dla_read_reg(DLA_HANDSHAKE);
    reg = set_bits!(
        DLA_HANDSHAKE_BIAS_ENABLE_OFFSET,
        DLA_HANDSHAKE_BIAS_ENABLE_BITMASK,
        reg,
        enable as usize
    );
    dla_write_reg(DLA_HANDSHAKE, reg);
}

pub fn dla_set_input_padding(top: usize, right: usize, bottom: usize, left: usize, value: usize) {
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
    dla_write_reg(DLA_BUF_PAD, reg);
}

pub fn dla_set_stride(x: usize, y: usize) {
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
    dla_write_reg(DLA_BUF_STRIDE, reg);
}

pub fn dla_set_bias_address(addr: usize) {
    let reg = set_bits!(
        DLA_PP_AXI_READ_ADDRESS_OFFSET,
        DLA_PP_AXI_READ_ADDRESS_BITMASK,
        0,
        addr
    );
    dla_write_reg(DLA_PP_AXI_READ, reg);
}

pub fn dla_get_status() -> u32 {
    return dla_read_reg(DLA_STATUS_ADDR);
}

pub fn dla_set_simd_select(mode: SimdBitMode) {
    let mut reg = dla_read_reg(DLA_MAC_CTRL);
    reg = set_bits!(
        DLA_SIMD_SELECT_OFFSET,
        DLA_SIMD_SELECT_BITMASK,
        reg,
        mode as usize
    );
    dla_write_reg(DLA_MAC_CTRL, reg)
}

pub fn dla_get_simd_format() -> SimdBitMode {
    let mut reg = dla_read_reg(DLA_MAC_CTRL);
    reg = get_bits!(DLA_SIMD_SELECT_BITMASK, reg);
    match reg {
        0 => SimdBitMode::EightBits,
        1 => SimdBitMode::FourBits,
        2 => SimdBitMode::TwoBits,
        _ => SimdBitMode::EightBits,
    }
}

pub fn dla_set_mac_clip(clip_amount: usize) {
    let mut reg = dla_read_reg(DLA_MAC_CTRL);
    // Cap clipping amount
    if clip_amount > 21 {
        reg = set_bits!(DLA_MAC_CLIP_OFFSET, DLA_MAC_CLIP_BITMASK, reg, 0x1F);
    } else {
        reg = set_bits!(DLA_MAC_CLIP_OFFSET, DLA_MAC_CLIP_BITMASK, reg, clip_amount);
    }
    dla_write_reg(DLA_MAC_CTRL, reg)
}

pub fn dla_set_pp_clip(clip_amount: usize) {
    let mut reg = dla_read_reg(DLA_PP_CTRL);
    // Cap clipping amount
    if clip_amount > 0x1F {
        reg = set_bits!(DLA_PP_CLIP_OFFSET, DLA_PP_CLIP_BITMASK, reg, 0x1F);
    } else {
        reg = set_bits!(DLA_PP_CLIP_OFFSET, DLA_PP_CLIP_BITMASK, reg, clip_amount);
    }
    dla_write_reg(DLA_PP_CTRL, reg)
}

pub fn dla_set_pp_rounding(enable: bool) {
    let mut reg = dla_read_reg(DLA_PP_CTRL);
    reg = set_bits!(
        DLA_ROUNDING_OFFSET,
        DLA_ROUNDING_BITMASK,
        reg,
        enable as usize
    );
    dla_write_reg(DLA_PP_CTRL, reg);
}

pub fn dla_is_ready() -> bool {
    let status = dla_read_reg(DLA_STATUS_ADDR);
    return !get_bits!(DLA_BUF_DONE_BITMASK, status) != 0;
}

pub fn dla_set_bias_addr(addr: u32) {
    dla_write_reg(DLA_PP_AXI_READ, addr);
}

pub fn dla_is_enabled() -> bool {
    let handshake_reg = dla_read_reg(DLA_HANDSHAKE);
    let buf_enabled = get_bits!(DLA_HANDSHAKE_BUFFER_ENABLE_BITMASK, handshake_reg) != 0;
    let mac_enabled = get_bits!(DLA_HANDSHAKE_MAC_ENABLE_BITMASK, handshake_reg) != 0;
    let active_enabled = get_bits!(DLA_HANDSHAKE_ACTIVE_ENABLE_BITMASK, handshake_reg) != 0;
    buf_enabled & mac_enabled & active_enabled
}

fn dla_handshake_disable_hw() {
    let mut handshake_reg = dla_read_reg(DLA_HANDSHAKE);
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

    dla_write_reg(DLA_HANDSHAKE, handshake_reg);
}

pub fn dla_handle_handshake() -> bool {
    // Handshake only if dla status is done
    if !dla_is_ready() {
        return false
    }

    if dla_is_enabled() {
        dla_handshake_disable_hw();
        return false
    }

    let mut handshake_reg = dla_read_reg(DLA_HANDSHAKE);
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

    dla_write_reg(DLA_HANDSHAKE, handshake_reg);
    return true
}

pub fn dla_init() {
    let mut reg = dla_read_reg(DLA_HANDSHAKE);
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
    dla_write_reg(DLA_HANDSHAKE, reg);

    // This should not be done in allcases
    // dla_set_kernel_size(1, 3, 3);
    // dla_set_input_size(1, 5, 5);

    dla_set_input_data_bank(0);
    dla_set_kernel_data_bank(8);

    dla_enable_pp(true);
    dla_enable_relu(true);
    dla_enable_bias(true);

    // let mut A = [
    //     1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5,
    // ];
    // let mut B = [0, 1, 0, 1, 1, 1, 0, 1, 0];
    // dla_write_data_bank(MEMORY_BANK_0_OFFSET, &mut A);
    // dla_write_data_bank(MEMORY_BANK_8_OFFSET, &mut B);

    // dla_kernel_data_ready(true);
    // dla_input_data_ready(true);
}
