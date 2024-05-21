use riscv_pac::HartIdNumber;

/// HPC has 4 HARTs
#[repr(u16)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HartId {
    H0 = 0,
    H1 = 1,
    H2 = 2,
    H3 = 3,
}

unsafe impl HartIdNumber for HartId {
    const MAX_HART_ID_NUMBER: u16 = 3;

    #[inline]
    fn number(self) -> u16 {
        self as _
    }

    #[inline]
    fn from_number(number: u16) -> Result<Self, u16> {
        if number > Self::MAX_HART_ID_NUMBER {
            Err(number)
        } else {
            // SAFETY: valid context number
            Ok(unsafe { core::mem::transmute(number) })
        }
    }
}
