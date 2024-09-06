pub mod uart;

use core::marker::PhantomData;

use crate::pac;
pub use uart::UdmaUart;

/// Type-state trait for uDMA peripherals in different states
pub trait UdmaPeriphState {}

pub struct Enabled;
impl UdmaPeriphState for Enabled {}

pub struct Disabled;
impl UdmaPeriphState for Disabled {}

/// Relocatable driver for uDMA IP
pub struct Udma<'u>(pub &'u pac::sysctrl::Udma);

pub struct UdmaParts<'u> {
    pub uart: UdmaUart<'u, Disabled>,
}

impl<'u> Udma<'u> {
    pub fn split(self) -> UdmaParts<'u> {
        UdmaParts {
            uart: UdmaUart::<Disabled>(self.0, PhantomData),
        }
    }
}
