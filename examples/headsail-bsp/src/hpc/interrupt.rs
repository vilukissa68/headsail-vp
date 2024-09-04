use crate::HartId;
use riscv_pac::PriorityNumber;

// HPC-SS specifies that priorities go up to 7
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Priority {
    P0 = 0,
    P1 = 1,
    P2 = 2,
    P3 = 3,
    P4 = 4,
    P5 = 5,
    P6 = 6,
    P7 = 7,
}

unsafe impl PriorityNumber for Priority {
    const MAX_PRIORITY_NUMBER: u8 = 7;

    #[inline]
    fn number(self) -> u8 {
        self as _
    }

    #[inline]
    fn from_number(number: u8) -> Result<Self, u8> {
        if number > Self::MAX_PRIORITY_NUMBER {
            Err(number)
        } else {
            // SAFETY: valid priority number
            Ok(unsafe { core::mem::transmute::<u8, Priority>(number) })
        }
    }
}

riscv_peripheral::clint_codegen!(
    base 0x60000,
    freq 32_768,
    mtimecmps [
        mtimecmp0 = (HartId::H0, "`H0`"),
        mtimecmp1 = (HartId::H1, "`H1`"),
        mtimecmp2 = (HartId::H2, "`H2`"),
        mtimecmp3 = (HartId::H3, "`H3`")
    ],
    msips [
        msip0 = (HartId::H0, "`H0`"),
        msip1 = (HartId::H1, "`H1`"),
        msip2 = (HartId::H2, "`H2`"),
        msip3 = (HartId::H3, "`H3`")
    ],
);

// ???: ASIC developer beware
//
// Some addresses were tightened to save space for Headsail's PLIC. That means that the ASIC will be
// different from the sim, and that this `riscv_peripheral` provided driver won't work as-is on
// ASIC, while it works perfectly well with the sim.
riscv_peripheral::plic_codegen!(
    base 0x80000,
    ctxs [
        ctx0 = (HartId::H0, "`H0M`"),
        ctx1 = (HartId::H0, "`H0S`"),
        ctx2 = (HartId::H1, "`H1M`"),
        ctx3 = (HartId::H1, "`H1S`"),
        ctx4 = (HartId::H2, "`H2M`"),
        ctx5 = (HartId::H2, "`H2S`"),
        ctx6 = (HartId::H3, "`H3M`"),
        ctx7 = (HartId::H3, "`H3S`")
    ],
);
