use core::marker::PhantomData;

use super::{Disabled, Enabled};
use crate::pac;
use embedded_io::ErrorType;

pub const SPI_CMD_SOT: u32 = 0x10000000;
pub const SPI_CMD_EOT: u32 = 0x90000000;
pub const SPI_CMD_SEND_CMD_BASE: u32 = 0x20070000;
pub const SPI_CMD_DUMMY: u32 = 0x400F0000;
pub const SPI_CMD_RX_CHECK: u32 = 0xB0200000;
pub const SPI_CMD_RX_DATA: u32 = 0x74000000;
pub const SPI_CMD_TX_DATA: u32 = 0x64000000;
pub const SPI_CMD_SETUP_UCA: u32 = 0xD0000000;
pub const SPI_CMD_SETUP_UCS: u32 = 0xE0000000;

/// Obtain an instance by calling [Udma::split]
pub struct UdmaSpim<'u, UdmaPeriphState>(
    pub(crate) &'u pac::sysctrl::Udma,
    pub(crate) PhantomData<UdmaPeriphState>,
);

impl<'u> UdmaSpim<'u, Disabled> {
    /// Enables the uDMA clock gate for SPI-M
    #[inline(always)]
    pub fn enable(self) -> UdmaSpim<'u, Enabled> {
        let spim = &self.0;

        // Turn on the clock gates for SPIM
        spim.ctrl_cfg_cg().modify(|_r, w| w.cg_spim().set_bit());

        UdmaSpim::<Enabled>(self.0, PhantomData)
    }
}

impl<'u> UdmaSpim<'u, Enabled> {
    #[inline(always)]
    pub fn disable(self) -> UdmaSpim<'u, Disabled> {
        self.0.ctrl_cfg_cg().modify(|_r, w| w.cg_spim().clear_bit());
        UdmaSpim::<Disabled>(self.0, PhantomData)
    }

    /// # Safety
    ///
    /// This will not configure the SPI-M in any way.
    #[inline(always)]
    pub unsafe fn steal(udma: &'static pac::sysctrl::Udma) -> Self {
        Self(udma, PhantomData)
    }

    #[inline(always)]
    pub fn write_tx(&mut self, buf: &[u8]) {
        while !self.can_enqueue_tx() {}

        // SAFETY: we spin lock on spim_tx_saddr to make sure the transfer is complete before
        // dropping the stack frame.
        unsafe { self.enqueue_tx(buf) };

        let spim = &self.0;

        // Poll until finished (prevents `buf` leakage)
        while spim.spim_tx_saddr().read().bits() != 0 {}
    }

    /// A potential `impl<'u> embedded_io::Read for UdmaSpim<'u, Enabled> {}`
    #[deprecated = "UdmaSpim::read method is experimental and untested. If you see this method
                    does what it's supposed to do useful, remove this deprecation notice from the \
                    method definition."]
    #[inline(always)]
    pub fn read_rx(&mut self, buf: &mut [u8]) -> Result<usize, SpimError> {
        while !self.can_enqueue_rx() {}

        // SAFETY: we spin lock on spim_rx_saddr to make sure the transfer is complete before
        // dropping the stack frame.
        unsafe { self.enqueue_rx(buf) };

        // Poll until finished (prevents `buf` leakage)
        let spim = &self.0;
        while spim.spim_rx_saddr().read().bits() != 0 {}

        // TODO: is this a guarantee that we always read exactly `buf.len()`? Can we somehow
        // identify how many bytes were actually read. This does sound reasonable if we're always
        // doing block reads like we might do with an SD card.
        Ok(buf.len())
    }

    #[inline(always)]
    pub fn write_cmd(&mut self, buf: &[u8]) {
        while !self.can_enqueue_cmd() {}

        // SAFETY: we spin lock on spim_cmd_saddr to make sure the transfer is complete before
        // dropping the stack frame.
        unsafe { self.enqueue_cmd(buf) };

        // Poll until finished (prevents `buf` leakage)
        let spim = &self.0;
        while spim.spim_cmd_saddr().read().bits() != 0 {}
    }

    /// Send 'Start Of Transmission' (SOT) command
    #[inline(always)]
    pub fn send_sot(&mut self) {
        let sot_cmd: [u8; 4] = SPI_CMD_SOT.to_ne_bytes();
        self.write_cmd(&sot_cmd);
    }

    /// Send 'End Of Transmission' (EOT) command
    #[inline(always)]
    pub fn send_eot(&mut self) {
        let eot_cmd: [u8; 4] = (SPI_CMD_EOT).to_ne_bytes();
        self.write_cmd(&eot_cmd);
    }

    /// This function sends EOT (End Of Transmission) command but keeps the cs asserted.
    #[inline(always)]
    pub fn send_eot_keep_cs(&mut self) {
        let eot_cmd: [u8; 4] = (SPI_CMD_EOT | 0x03).to_ne_bytes();
        self.write_cmd(&eot_cmd);
    }

    /// This function sends one dummy byte (0xFF)
    ///
    /// TODO: this should be fixable so that the user can easily choose the
    /// number of repetition without using a for loop.
    ///
    /// # Examples
    ///
    /// ```
    /// for _i in 0..10 {
    ///     spim.sot();
    ///     pim.send_dummy();
    /// }
    /// ```
    #[inline(always)]
    pub fn write_dummy(&mut self) {
        let mut buffer: [u8; 4] = [0; 4];
        let cmd_cmd: [u8; 4] = (SPI_CMD_SEND_CMD_BASE | 0xFF).to_ne_bytes();

        buffer[0..4].copy_from_slice(&cmd_cmd[0..4]);
        self.write_cmd(&buffer);
    }

    /// This function sends data out. Use this transfer data via SPI to for example SD card.
    ///
    /// # Examples
    ///
    /// ```
    ///   let data: [u8; 2] = [0x01,0x02];
    ///   spim.sot();
    ///   spim.send(&data);
    ///   spim.eot();
    ///
    /// ```
    #[inline(always)]
    pub fn send_data(&mut self, data: &[u8]) {
        let mut cmd_data: [u8; 12] = [0; 12];

        cmd_data[0..4].copy_from_slice(
            &(SPI_CMD_SETUP_UCA | (data.as_ptr() as u32 & 0x0000FFFF)).to_ne_bytes(),
        );
        cmd_data[4..8]
            // 4 byte but change this to depend on data i.e: ((data.len() - 2) as u32)
            .copy_from_slice(&(SPI_CMD_SETUP_UCS | (data.len() - 2) as u32).to_ne_bytes());
        cmd_data[8..12].copy_from_slice(
            &(SPI_CMD_TX_DATA | (data.len() - 1) as u32 | (7 << 16)).to_ne_bytes(),
        );

        self.write_cmd(&cmd_data);
        self.write_tx(data);
    }

    /// This function receives data. Use this to receive data via SPI-M from for example SD card.
    ///
    /// # Examples
    ///
    /// ```
    ///   let data: [u8; 2] = [0;2];
    ///   spim.sot();
    ///   spim.receive(&data);
    ///   spim.eot();
    ///
    /// ```
    #[inline(always)]
    pub fn receive_data(&mut self, data: &mut [u8]) -> Result<usize, SpimError> {
        let mut cmd_data: [u8; 12] = [0; 12];

        cmd_data[0..4].copy_from_slice(
            &(SPI_CMD_SETUP_UCA | (data.as_ptr() as u32 & 0x0000FFFF)).to_ne_bytes(),
        );
        cmd_data[4..8]
            .copy_from_slice(&(SPI_CMD_SETUP_UCS | (data.len() - 2) as u32).to_ne_bytes());
        cmd_data[8..12].copy_from_slice(
            &(SPI_CMD_RX_DATA | (data.len() - 1) as u32 | (7 << 16)).to_ne_bytes(),
        );

        self.write_cmd(&cmd_data);

        self.read_rx(data)
    }

    /// Can a new transfer be enqueued to the CMD channel?
    ///
    /// Returns 1 if another transfer can be enqueued, 0 otherwise
    #[inline(always)]
    fn can_enqueue_cmd(&self) -> bool {
        let spim = &self.0;
        spim.spim_cmd_cfg().read().pending().bit_is_clear()
    }

    /// Can a new transfer be enqueued to the TX channel?
    ///
    /// Returns 1 if another transfer can be enqueued, 0 otherwise
    #[inline(always)]
    fn can_enqueue_tx(&self) -> bool {
        let spim = &self.0;
        spim.spim_tx_cfg().read().pending().bit_is_clear()
    }

    /// Can a new transfer be enqueued to the RX channel?
    ///
    /// Returns 1 if another transfer can be enqueued, 0 otherwise
    #[inline(always)]
    fn can_enqueue_rx(&self) -> bool {
        let spim = &self.0;
        spim.spim_rx_cfg().read().pending().bit_is_clear()
    }

    /// # Safety
    ///
    /// `buf` must outlive the transfer. Call `while spim.spim_*_saddr().read().bits() != 0 {}` or
    /// use an interrupt to determine when `buf` is safe to free.
    #[inline(always)]
    unsafe fn enqueue_cmd(&mut self, buf: &[u8]) {
        let spim = &self.0;

        // Write buffer location & len
        spim.spim_cmd_saddr()
            .write(|w| unsafe { w.bits(buf.as_ptr() as u32) });
        spim.spim_cmd_size()
            .write(|w| unsafe { w.bits(buf.len() as u32) });

        // Dispatch transmission
        spim.spim_cmd_cfg().modify(|_, w| w.en().set_bit());
    }

    /// # Safety
    ///
    /// `buf` must outlive the transfer. Call `while spim.spim_*_saddr().read().bits() != 0 {}` or
    /// use an interrupt to determine when `buf` is safe to free.
    #[inline(always)]
    unsafe fn enqueue_tx(&mut self, buf: &[u8]) {
        let spim = &self.0;

        // Write buffer location & len
        spim.spim_tx_saddr()
            .write(|w| unsafe { w.bits(buf.as_ptr() as u32) });
        spim.spim_tx_size()
            .write(|w| unsafe { w.bits(buf.len() as u32) });

        // Dispatch transmission
        spim.spim_tx_cfg().modify(|_, w| w.en().set_bit());
    }

    /// # Safety
    ///
    /// `buf` must outlive the transfer. Call `while spim.spim_*_saddr().read().bits() != 0 {}` or
    /// use an interrupt to determine when `buf` is safe to free.
    #[inline(always)]
    unsafe fn enqueue_rx(&mut self, buf: &mut [u8]) {
        let spim = &self.0;

        // Write buffer location & len
        spim.spim_rx_saddr()
            .write(|w| unsafe { w.bits(buf.as_ptr() as u32) });
        spim.spim_rx_size()
            .write(|w| unsafe { w.bits(buf.len() as u32) });

        // Dispatch transmission
        spim.spim_rx_cfg().modify(|_, w| w.en().set_bit());
    }
}

#[derive(Debug)]
pub struct SpimError;
impl embedded_io::Error for SpimError {
    fn kind(&self) -> embedded_io::ErrorKind {
        todo!()
    }
}

impl<'u> ErrorType for UdmaSpim<'u, Enabled> {
    type Error = SpimError;
}
