use crate::HartId;
use riscv_pac::{result::Error, PriorityNumber};

// HPC-SS specifies that priorities go up to 7
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(usize)]
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
    const MAX_PRIORITY_NUMBER: usize = 7;

    #[inline]
    fn number(self) -> usize {
        self as _
    }

    #[inline]
    fn from_number(number: usize) -> Result<Self, Error> {
        match number {
            0 => Ok(Priority::P0),
            1 => Ok(Priority::P1),
            2 => Ok(Priority::P2),
            3 => Ok(Priority::P3),
            4 => Ok(Priority::P4),
            5 => Ok(Priority::P5),
            6 => Ok(Priority::P6),
            7 => Ok(Priority::P7),
            _ => Err(Error::IndexOutOfBounds {
                index: number,
                min: 0,
                max: Self::MAX_PRIORITY_NUMBER,
            }),
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
