// allow(dead_code):
//
// It's common to have unused symbols in the memory maps and in this case we don't feel that's an
// issue. If this library ever becomes "finished", we can remove this allow definition.
#![allow(dead_code)]

#[cfg(feature = "hpc")]
pub const DLA0_ADDR: usize = 0x1FF700000;
#[cfg(not(feature = "hpc"))]
pub const DLA0_ADDR: usize = 0xFF700000;

#[cfg(feature = "hpc")]
pub const MEMORY_BANK_BASE_ADDR: usize = 0x170000000;
#[cfg(not(feature = "hpc"))]
pub const MEMORY_BANK_BASE_ADDR: usize = 0x70000000;

#[cfg(feature = "hpc")]
pub const EXTERNAL_BIT: usize = 0x100000000;
#[cfg(not(feature = "hpc"))]
pub const EXTERNAL_BIT: usize = 0x0;

pub const MEMORY_BANK_SIZE: usize = 0x8000;
pub const MEMORY_BANK_0_OFFSET: usize = 0x00000;
pub const MEMORY_BANK_1_OFFSET: usize = 0x08000;
pub const MEMORY_BANK_2_OFFSET: usize = 0x10000;
pub const MEMORY_BANK_3_OFFSET: usize = 0x18000;
pub const MEMORY_BANK_4_OFFSET: usize = 0x20000;
pub const MEMORY_BANK_5_OFFSET: usize = 0x28000;
pub const MEMORY_BANK_6_OFFSET: usize = 0x30000;
pub const MEMORY_BANK_7_OFFSET: usize = 0x38000;
pub const MEMORY_BANK_8_OFFSET: usize = 0x40000;
pub const MEMORY_BANK_9_OFFSET: usize = 0x48000;
pub const MEMORY_BANK_10_OFFSET: usize = 0x50000;
pub const MEMORY_BANK_11_OFFSET: usize = 0x58000;
pub const MEMORY_BANK_12_OFFSET: usize = 0x60000;
pub const MEMORY_BANK_13_OFFSET: usize = 0x68000;
pub const MEMORY_BANK_14_OFFSET: usize = 0x70000;
pub const MEMORY_BANK_15_OFFSET: usize = 0x78000;

pub(crate) const DLA_STATUS_ADDR: usize = 0x0;
pub(crate) const DLA_BUF_DONE_OFFSET: usize = 0x0;
pub(crate) const DLA_MAC_DONE_OFFSET: usize = 0x1;
pub(crate) const DLA_PP_DONE_OFFSET: usize = 0x2;
pub(crate) const DLA_DMA_IRQ_OFFSET: usize = 0x3;
pub(crate) const DLA_BUF_DONE_BITMASK: usize = 0b1;
pub(crate) const DLA_MAC_DONE_BITMASK: usize = 0b10;
pub(crate) const DLA_PP_DONE_BITMASK: usize = 0b100;
pub(crate) const DLA_DMA_IRQ_BITMASK: usize = 0b1000;

pub(crate) const DLA_CTRL_ADDR: usize = 0x4;
pub(crate) const DLA_CPU_FE_OFFSET: usize = 0x0;
pub(crate) const DLA_HP_RST_OFFSET: usize = 0x4;
pub(crate) const DLA_SW_IRQ_OFFSET: usize = 0x8;
pub(crate) const DLA_CPU_FE_BITMASK: usize = 0b1;
pub(crate) const DLA_HP_RST_BITMASK: usize = 0b10000;
pub(crate) const DLA_SW_IRQ_BITMASK: usize = 0b1 << 8;

pub(crate) const DLA_BUF_CTRL: usize = 0x8;
pub(crate) const DLA_CONV_MODE_OFFSET: usize = 0x0;
pub(crate) const DLA_READ_A_VALID_OFFSET: usize = 0x4;
pub(crate) const DLA_READ_B_VALID_OFFSET: usize = 0x8;
pub(crate) const DLA_CONV_MODE_BITMASK: usize = 0b1111;
pub(crate) const DLA_READ_A_VALID_BITMASK: usize = 0b10000;
pub(crate) const DLA_READ_B_VALID_BITMASK: usize = 0b1 << 8;

pub(crate) const DLA_MAC_CTRL: usize = 0xC;
pub(crate) const DLA_SIMD_SELECT_OFFSET: usize = 0x1;
pub(crate) const DLA_MAC_CLIP_OFFSET: usize = 0x8;
pub(crate) const DLA_SIMD_SELECT_BITMASK: usize = 0x11;
pub(crate) const DLA_MAC_CLIP_BITMASK: usize = 0b11111 << 8;

pub(crate) const DLA_PP_CTRL: usize = 0x10;
pub(crate) const DLA_ACTIVE_MODE_OFFSET: usize = 0x0;
pub(crate) const DLA_RELU_OFFSET_UNUSED: usize = 0x2;
pub(crate) const DLA_MAX_OFFSET_UNUSED: usize = 0x4;
pub(crate) const DLA_PP_SELECT_OFFSET: usize = 0x6;
pub(crate) const DLA_POOL_MODE_OFFSET_UNUSED: usize = 0x7;
pub(crate) const DLA_ROUNDING_OFFSET: usize = 0x9;
pub(crate) const DLA_CTRL_VLD_OFFSET_UNUSED: usize = 0xA;
pub(crate) const DLA_PP_CLIP_OFFSET: usize = 0x10;
pub(crate) const DLA_ACTIVE_MODE_BITMASK: usize = 0b11;
pub(crate) const DLA_RELU_BITMASK_UNUSED: usize = 0b1100;
pub(crate) const DLA_MAX_BITMASK_UNUSED: usize = 0b110000;
pub(crate) const DLA_PP_SELECT_BITMASK: usize = 0b1 << 6;
pub(crate) const DLA_POOL_MODE_BITMASK_UNUSED: usize = 0b11 << 7;
pub(crate) const DLA_ROUNDING_BITMASK: usize = 0b1 << 9;
pub(crate) const DLA_CTRL_VLD_BITMASK_UNUSED: usize = 0b1 << 10;
pub(crate) const DLA_PP_CLIP_BITMASK: usize = 0b11111 << 16;

pub(crate) const DLA_BUF_INPUT: usize = 0x14;
pub(crate) const DLA_BUF_INPUT_WIDTH_OFFSET: usize = 0;
pub(crate) const DLA_BUF_INPUT_HEIGHT_OFFSET: usize = 9;
pub(crate) const DLA_BUF_INPUT_CHANNELS_OFFSET: usize = 18;
pub(crate) const DLA_BUF_INPUT_WIDTH_BITMASK: usize = 0b111111111;
pub(crate) const DLA_BUF_INPUT_HEIGHT_BITMASK: usize = 0b111111111 << 9;
pub(crate) const DLA_BUF_INPUT_CHANNELS_BITMASK: usize = 0b111111111111 << 18;

pub(crate) const DLA_BUF_KERNEL_0: usize = 0x18;
pub(crate) const DLA_BUF_KERNEL_0_WIDTH_OFFSET: usize = 0;
pub(crate) const DLA_BUF_KERNEL_0_HEIGHT_OFFSET: usize = 4;
pub(crate) const DLA_BUF_KERNEL_0_S_CHANNELS_OFFSET: usize = 8;
pub(crate) const DLA_BUF_KERNEL_0_WIDTH_BITMASK: usize = 0b1111;
pub(crate) const DLA_BUF_KERNEL_0_HEIGHT_BITMASK: usize = 0b1111 << 4;
pub(crate) const DLA_BUF_KERNEL_0_S_CHANNELS_BITMASK: usize = 0b111111111111 << 8;

pub(crate) const DLA_BUF_KERNEL_1: usize = 0x1C;
pub(crate) const DLA_BUF_KERNEL_1_NUM_OFFSET: usize = 0x0;
pub(crate) const DLA_BUF_KERNEL_1_NUM_BITMASK: usize = 0b111111111111;

pub(crate) const DLA_BUF_PAD: usize = 0x20;
pub(crate) const DLA_BUF_PAD_TOP_OFFSET: usize = 0x0;
pub(crate) const DLA_BUF_PAD_RIGHT_OFFSET: usize = 0x4;
pub(crate) const DLA_BUF_PAD_BOTTOM_OFFSET: usize = 0x8;
pub(crate) const DLA_BUF_PAD_LEFT_OFFSET: usize = 0xC;
pub(crate) const DLA_BUF_PAD_VALUE_OFFSET: usize = 0x10;
pub(crate) const DLA_BUF_PAD_TOP_BITMASK: usize = 0b1111;
pub(crate) const DLA_BUF_PAD_RIGHT_BITMASK: usize = 0b1111 << 4;
pub(crate) const DLA_BUF_PAD_BOTTOM_BITMASK: usize = 0b1111 << 8;
pub(crate) const DLA_BUF_PAD_LEFT_BITMASK: usize = 0b1111 << 12;
pub(crate) const DLA_BUF_PAD_VALUE_BITMASK: usize = 0b1111 << 16;

pub(crate) const DLA_BUF_STRIDE: usize = 0x24;
pub(crate) const DLA_BUF_STRIDE_X_OFFSET: usize = 0x0;
pub(crate) const DLA_BUF_STRIDE_Y_OFFSET: usize = 0x10;
pub(crate) const DLA_BUF_STRIDE_X_BITMASK: usize = 0b1111;
pub(crate) const DLA_BUF_STRIDE_Y_BITMASK: usize = 0b1111 << 16;

pub(crate) const DLA_PP_INPUT: usize = 0x28;
pub(crate) const DLA_PP_INPUT_WIDTH_OFFSET: usize = 0x0;
pub(crate) const DLA_PP_INPUT_HEIGHT_OFFSET: usize = 0x10;
pub(crate) const DLA_PP_INPUT_WIDTH_BITMASK: usize = 0b111111111;
pub(crate) const DLA_PP_INPUT_HEIGHT_BITMASK: usize = 0b111111111 << 16;

pub(crate) const DLA_BUF_DATA_BANK: usize = 0x2C;
pub(crate) const DLA_BUF_DATA_BANK_A_OFFSET: usize = 0x0;
pub(crate) const DLA_BUF_DATA_BANK_B_OFFSET: usize = 16;
pub(crate) const DLA_BUF_DATA_BANK_A_BITMASK: usize = 0b1111;
pub(crate) const DLA_BUF_DATA_BANK_B_BITMASK: usize = 0b1111 << 16;

pub(crate) const DLA_BUF_DATA_WAIT_A: usize = 0x30;
pub(crate) const DLA_BUF_DATA_WAIT_A_OFFSET: usize = 0x0;
pub(crate) const DLA_BUF_DATA_WAIT_A_BITMASK: usize = 0xFFFFFFFF;

pub(crate) const DLA_BUF_DATA_WAIT_B: usize = 0x34;
pub(crate) const DLA_BUF_DATA_WAIT_B_OFFSET: usize = 0x0;
pub(crate) const DLA_BUF_DATA_WAIT_B_BITMASK: usize = 0xFFFFFFFF;

pub(crate) const DLA_BUF_PIPE_STALL_STALL_CYCLES: usize = 0x38;
pub(crate) const DLA_BUF_PIPE_STALL_STALL_CYCLES_OFFSET: usize = 0x0;
pub(crate) const DLA_BUF_PIPE_STALL_STALL_CYCLES_BITMASK: usize = 0xFFFFFFFF;

pub(crate) const DLA_POWER_CTRL: usize = 0x4C;
pub(crate) const DLA_POWER_CTRL_DOWN_0_OFFSET: usize = 0x0;
pub(crate) const DLA_POWER_CTRL_DOWN_1_OFFSET: usize = 0x1;
pub(crate) const DLA_POWER_CTRL_DOWN_2_OFFSET: usize = 0x2;
pub(crate) const DLA_POWER_CTRL_ISO_OFFSET: usize = 0x3;
pub(crate) const DLA_POWER_CTRL_DOWN_0_BITMASK: usize = 0b1;
pub(crate) const DLA_POWER_CTRL_DOWN_1_BITMASK: usize = 0b10;
pub(crate) const DLA_POWER_CTRL_DOWN_2_BITMASK: usize = 0b100;
pub(crate) const DLA_POWER_CTRL_ISO_BITMASK: usize = 0b1000;

pub(crate) const DLA_POWER_STAT: usize = 0x50;
pub(crate) const DLA_POWER_STAT_ACK_0_OFFSET: usize = 0x0;
pub(crate) const DLA_POWER_STAT_ACK_1_OFFSET: usize = 0x1;
pub(crate) const DLA_POWER_STAT_ACK_2_OFFSET: usize = 0x2;
pub(crate) const DLA_POWER_STAT_ACK_0_BITMASK: usize = 0b1;
pub(crate) const DLA_POWER_STAT_ACK_1_BITMASK: usize = 0b10;
pub(crate) const DLA_POWER_STAT_ACK_2_BITMASK: usize = 0b100;

pub(crate) const DLA_DMA_CTRL: usize = 0x44;
pub(crate) const DLA_DMA_CTRL_READ_EVENT_OFFSET: usize = 0x0;
pub(crate) const DLA_DMA_CTRL_WRITE_EVENT_OFFSET: usize = 0x0;
pub(crate) const DLA_DMA_CTRL_READ_EVENT_BITMASK: usize = 0xFFFFFFFF;
pub(crate) const DLA_DMA_CTRL_WRITE_EVENT_BITMASK: usize = 0xFFFFFFFF;

pub(crate) const DLA_DMA_PAD_CONFIG: usize = 0x48;
pub(crate) const DLA_DMA_PAD_CONFIG_OFFSET: usize = 0x0;
pub(crate) const DLA_DMA_PAD_CONFIG_BITMASK: usize = 0xFFFFFFFF;

pub(crate) const DLA_MAC_SAT_MAX: usize = 0x54;
pub(crate) const DLA_MAC_SAT_MAX_OFFSET: usize = 0x0;
pub(crate) const DLA_MAC_SAT_MAX_BITMASK: usize = 0xFFFFFFFF;

pub(crate) const DLA_MAC_SAT_MIN: usize = 0x58;
pub(crate) const DLA_MAC_SAT_MIN_OFFSET: usize = 0x0;
pub(crate) const DLA_MAC_SAT_MIN_BITMASK: usize = 0xFFFFFFFF;

pub(crate) const DLA_PP_AXI_WRITE: usize = 0x5c;
pub(crate) const DLA_PP_AXI_WRITE_ADDRESS_OFFSET: usize = 0x00;
pub(crate) const DLA_PP_AXI_WRITE_ADDRESS_BITMASK: usize = 0xFFFFFFFF;

pub(crate) const DLA_PP_AXI_READ: usize = 0x60;
pub(crate) const DLA_PP_AXI_READ_ADDRESS_OFFSET: usize = 0x00;
pub(crate) const DLA_PP_AXI_READ_ADDRESS_BITMASK: usize = 0xFFFFFFFF;

pub(crate) const DLA_HANDSHAKE: usize = 0x64;
pub(crate) const DLA_HANDSHAKE_BUFFER_VALID_OFFSET: usize = 0x0;
pub(crate) const DLA_HANDSHAKE_MAC_VALID_OFFSET: usize = 0x1;
pub(crate) const DLA_HANDSHAKE_POOL_VALID_OFFSET: usize = 0x2;
pub(crate) const DLA_HANDSHAKE_ACTIVE_VALID_OFFSET: usize = 0x3;
pub(crate) const DLA_HANDSHAKE_BUFFER_ENABLE_OFFSET: usize = 0x4;
pub(crate) const DLA_HANDSHAKE_MAC_ENABLE_OFFSET: usize = 0x5;
pub(crate) const DLA_HANDSHAKE_ACTIVE_ENABLE_OFFSET: usize = 0x6;
pub(crate) const DLA_HANDSHAKE_POOL_ENABLE_OFFSET: usize = 0x7;
pub(crate) const DLA_HANDSHAKE_BIAS_ENABLE_OFFSET: usize = 0x8;
pub(crate) const DLA_HANDSHAKE_BYPASS_ENABLE_OFFSET: usize = 0x9;
pub(crate) const DLA_HANDSHAKE_BUFFER_VALID_BITMASK: usize = 0b1;
pub(crate) const DLA_HANDSHAKE_MAC_VALID_BITMASK: usize = 0b1 << 1;
pub(crate) const DLA_HANDSHAKE_POOL_VALID_BITMASK: usize = 0b1 << 2;
pub(crate) const DLA_HANDSHAKE_ACTIVE_VALID_BITMASK: usize = 0b1 << 3;
pub(crate) const DLA_HANDSHAKE_BUFFER_ENABLE_BITMASK: usize = 0b1 << 4;
pub(crate) const DLA_HANDSHAKE_MAC_ENABLE_BITMASK: usize = 0b1 << 5;
pub(crate) const DLA_HANDSHAKE_ACTIVE_ENABLE_BITMASK: usize = 0b1 << 6;
pub(crate) const DLA_HANDSHAKE_POOL_ENABLE_BITMASK: usize = 0b1 << 7;
pub(crate) const DLA_HANDSHAKE_BIAS_ENABLE_BITMASK: usize = 0b1 << 8;
pub(crate) const DLA_HANDSHAKE_BYPASS_ENABLE_BITMASK: usize = 0b1 << 9;
