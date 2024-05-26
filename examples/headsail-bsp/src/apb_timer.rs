/*!
 * Date: 6/5/2024
 * Author: Andreas Stergiopoulos (andreas.stergiopoulos@tuni.fi)
 *
 * This is the driver for the PULP APB Timer. The documentation
 * for this peripheral is the register map provided in the Headsail
 * gitlab pages.
 */
use crate::{mmap::*, read_u32, write_u32};

use bit_field::BitField;

/**
 * Relocatable driver for the PULP APB Timer IP. The generic represents the
 * base address for the timer. This driver is ASIC only. For the Virtual
 * Prototype driver, please enable the "vp" feature.
 */
pub struct ApbTimer<const BASE_ADDRESS: usize>;

impl<const BASE_ADDRESS: usize> ApbTimer<BASE_ADDRESS> {
    const TIMER_COUNTER_REG_OFFSET: usize = 0x0;
    const TIMER_CTRL_REG_OFFSET: usize = 0x4;
    // const TIMER_CMP_REG_OFFSET: usize = 0x8;
    const TIMER_ENABLE_BIT: usize = 0b0;

    /**
     * Enables the timer (starts counting).
     */
    #[inline]
    pub fn enable() {
        let mut reg = read_u32(BASE_ADDRESS + Self::TIMER_CTRL_REG_OFFSET);
        // Make enable bit 1
        reg.set_bit(Self::TIMER_ENABLE_BIT, true);
        write_u32(BASE_ADDRESS + Self::TIMER_CTRL_REG_OFFSET, reg);
    }

    /**
     * Disables the timer (stops counting).
     */
    #[inline]
    pub fn disable() {
        let mut reg = read_u32(BASE_ADDRESS + Self::TIMER_CTRL_REG_OFFSET);
        // Write 0 to bit 0 but leave all other bits untouched
        reg.set_bit(Self::TIMER_ENABLE_BIT, false);
        write_u32(BASE_ADDRESS + Self::TIMER_CTRL_REG_OFFSET, reg);
    }

    /**
     * Returns the timer counter (tick value).
     */
    #[inline]
    pub fn get_count() -> u32 {
        return read_u32(BASE_ADDRESS + Self::TIMER_COUNTER_REG_OFFSET);
    }

    /**
     * Debug only method used to peek into the timer's control register.
     */
    #[inline]
    #[cfg(debug_assertions)]
    pub fn get_ctrl_reg() -> u32 {
        return read_u32(BASE_ADDRESS + Self::TIMER_CTRL_REG_OFFSET);
    }
}

///Type alias that should be used to interface timer 0.
pub type Timer0 = ApbTimer<TIMER0_ADDR>;
///Type alias that should be used to interface timer 1.
pub type Timer1 = ApbTimer<TIMER1_ADDR>;
///Type alias that should be used to interface timer 2.
pub type Timer2 = ApbTimer<TIMER2_ADDR>;
///Type alias that should be used to interface timer 3.
pub type Timer3 = ApbTimer<TIMER3_ADDR>;
