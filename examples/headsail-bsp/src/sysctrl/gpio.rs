use core::marker::PhantomData;

use super::{mmap, soc_ctrl};
use crate::{mask_u32, toggle_u32, unmask_u32};

/// Type-state trait for GPIO in different states
pub trait GpioState {}

pub struct Uninit;
impl GpioState for Uninit {}

pub struct Input;
impl GpioState for Input {}

pub struct Output;
impl GpioState for Output {}

/// To obtain an instance:
///
/// 1. Obtain [Pads] with [sysctrl::soc_ctrl::Pads::take]
/// 2. Pick pin: `pads.p9`
/// 3. Convert pin into GPIO [systcrl::soc_ctrl::Pad::into_gpio]
pub struct Gpio<const IDX: u32, State: GpioState = Uninit> {
    _pd: PhantomData<State>,
}

pub type Gpio9<S> = Gpio<9, S>;

impl<const IDX: u32> Gpio<IDX, Uninit> {
    pub(crate) fn new() -> Self {
        Self { _pd: PhantomData }
    }

    pub fn into_input(self) -> Gpio<IDX, Input> {
        unmask_u32(mmap::GPIO_DIR, 1 << IDX);

        Gpio { _pd: PhantomData }
    }

    pub fn into_output(self) -> Gpio<IDX, Output> {
        mask_u32(mmap::GPIO_DIR, 1 << IDX);

        Gpio { _pd: PhantomData }
    }
}

impl<const IDX: u32> Gpio<IDX, Output> {
    pub fn toggle(&mut self) {
        toggle_u32(mmap::GPIO_OUT, 1 << IDX);
    }

    pub fn set_high(&mut self) {
        mask_u32(mmap::GPIO_OUT, 1 << IDX);
    }

    pub fn set_low(&mut self) {
        unmask_u32(mmap::GPIO_OUT, 1 << IDX);
    }
}

impl<const IDX: u32, S: GpioState> Gpio<IDX, S> {
    /// Release pad back to its original function
    ///
    /// Pins can be released in any state. GPIO register configurations will
    /// retain their state (until overridden again by HAL methods).
    pub fn release(self) -> soc_ctrl::Pad<IDX> {
        unmask_u32(
            if IDX <= 15 {
                mmap::PADMUX0
            } else {
                mmap::PADMUX1
            },
            (soc_ctrl::PadFn::Gpio as u32) << (IDX * 2),
        );

        soc_ctrl::Pad::<IDX> {}
    }

    /// # Safety
    ///
    /// This will not configure the pin in any way (i.e., for use as GPIO or
    /// direction).
    pub unsafe fn steal() -> Self {
        Self { _pd: PhantomData }
    }
}
