#![no_std]
use embedded_hal::spi::{SpiBus, SpiDevice};
const CMD_READ_ID: u8 = 0x9f;

/// Nand flash on SPI.
pub struct SpiNand<SPI> {
    spi: SPI,
}

impl<SPI> SpiNand<SPI>
where
    SPI: SpiDevice,
    SPI::Bus: SpiBus,
{
    /// Identify the NAND flash device.
    #[inline]
    pub fn read_id(&mut self) -> Result<Id, SPI::Error> {
        let mut buf = [0u8; 3];
        self.spi.transfer(&mut buf, &[CMD_READ_ID])?;
        // buf[0] is dummy byte, discard
        let (manufacturer, device) = (buf[1], buf[2]);
        Ok(Id {
            manufacturer,
            device,
        })
    }
}

/// Nand flash identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Id {
    /// Device manufacturer byte
    pub manufacturer: u8,
    /// Device identifier byte
    pub device: u8,
}

// pub struct SpiNor
