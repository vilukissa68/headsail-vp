/**
 * Date: 6/5/2024
 * Author: Andreas Stergiopoulos (andreas.stergiopoulos@tuni.fi)
 *
 * This is the driver for the PULP APB Timer. The documentation
 * for this peripheral is the register map provided in the Headsail
 * gitlab pages.
 */
use crate::{mmap::TIMER0_ADDR, read_u32, write_u32};

use bit_field::BitField;

const TIMER0_COUNTER_REG_OFFSET: usize = 0x0;
const TIMER0_CTRL_REG_OFFSET: usize = 0x4;
const TIMER0_CMP_REG_OFFSET: usize = 0x8;
const TIMER0_ENABLE_BIT: usize = 0b0;

/**
 * Enables the timer (starts counting).
 */
#[inline]
pub fn timer0_enable() {
    // Read register
    let mut reg = read_u32(TIMER0_ADDR + TIMER0_CTRL_REG_OFFSET);
    // Make enable bit 1
    reg.set_bit(TIMER0_ENABLE_BIT, true);
    // Write register back
    write_u32(TIMER0_ADDR + TIMER0_CTRL_REG_OFFSET, reg);
}

/**
 * Disables the timer (stops counting).
 */
#[inline]
pub fn timer0_disable() {
    // Read register
    let mut reg = read_u32(TIMER0_ADDR + TIMER0_CTRL_REG_OFFSET);
    // Write 0 to bit 0 but leave all other bits untouched
    reg.set_bit(TIMER0_ENABLE_BIT, false);
    // Write register back
    write_u32(TIMER0_ADDR + TIMER0_CTRL_REG_OFFSET, reg);
}

/**
 * Returns the timer counter (tick value).
 */
#[inline]
pub fn timer0_get_count() -> u32 {
    return read_u32(TIMER0_ADDR + TIMER0_COUNTER_REG_OFFSET);
}

#[inline]
#[cfg(debug_assertions)]
pub fn timer0_get_ctrl_reg() -> u32 {
    return read_u32(TIMER0_ADDR + TIMER0_CTRL_REG_OFFSET);
}
