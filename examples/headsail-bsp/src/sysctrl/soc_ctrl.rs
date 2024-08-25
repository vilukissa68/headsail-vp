use riscv::interrupt;

use super::{gpio::Gpio, mmap};
use crate::{mask_u32, write_u32};

#[repr(u32)]
pub(crate) enum PadFn {
    //Default = 0,
    Gpio = 1,
}

/// ~Pin
pub struct Pad<const IDX: u32> {}

pub struct Pads {
    pub p0: Pad<0>,
    pub p1: Pad<1>,
    pub p2: Pad<2>,
    pub p3: Pad<3>,
    pub p4: Pad<4>,
    pub p5: Pad<5>,
    pub p6: Pad<6>,
    pub p7: Pad<7>,
    pub p8: Pad<8>,
    pub p9: Pad<9>,
    pub p10: Pad<10>,
    pub p11: Pad<11>,
    pub p12: Pad<12>,
    pub p13: Pad<13>,
    pub p14: Pad<14>,
    pub p15: Pad<15>,
    pub p16: Pad<16>,
    pub p17: Pad<17>,
    pub p18: Pad<18>,
}

/// Set to `true` when `take` or `steal` was called to make `Peripherals` a singleton.
static mut PADS_TAKEN: bool = false;

impl Pads {
    #[inline]
    pub fn take() -> Option<Self> {
        interrupt::free(|| {
            if unsafe { PADS_TAKEN } {
                None
            } else {
                Some(unsafe { Self::steal() })
            }
        })
    }

    #[inline]
    pub unsafe fn steal() -> Self {
        Self {
            p0: Pad {},
            p1: Pad {},
            p2: Pad {},
            p3: Pad {},
            p4: Pad {},
            p5: Pad {},
            p6: Pad {},
            p7: Pad {},
            p8: Pad {},
            p9: Pad {},
            p10: Pad {},
            p11: Pad {},
            p12: Pad {},
            p13: Pad {},
            p14: Pad {},
            p15: Pad {},
            p16: Pad {},
            p17: Pad {},
            p18: Pad {},
        }
    }
}

impl<const IDX: u32> Pad<IDX> {
    pub fn into_gpio(self) -> Gpio<IDX> {
        mask_u32(
            if IDX <= 15 {
                mmap::PADMUX0
            } else {
                mmap::PADMUX1
            },
            (PadFn::Gpio as u32) << (IDX * 2),
        );

        Gpio::<IDX>::new()
    }
}

pub fn ss_enable(ss_bits: u32) {
    write_u32(mmap::SS_RESET_EN, ss_bits);
}

pub fn clk2_set(conf_val: u32) {
    write_u32(mmap::SS_CLK_CTRL2, conf_val);
}

pub fn clk3_set(conf_val: u32) {
    write_u32(mmap::SS_CLK_CTRL3, conf_val);
}

/// # Parameters
///
/// * `div` - value to set the `div` register to. Divider will be 1 << `div`
///   (unverified).
pub fn periph_clk_div_set(div: u32) {
    let valid_bit = 0x400;
    write_u32(mmap::PERIPH_CLK_DIV, valid_bit | div)
}
