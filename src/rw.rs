use embedded_hal::blocking::spi::{Operation, Transactional, Transfer, Write};

use crate::registers::Registers;

pub trait ReadWrite {
    type Error;

    /// Direct write to RFM69 registers.
    fn write_many(&mut self, reg: Registers, data: &[u8]) -> core::result::Result<(), Self::Error>;

    /// Direct read from RFM69 registers.
    fn read_many(
        &mut self,
        reg: Registers,
        buffer: &mut [u8],
    ) -> core::result::Result<(), Self::Error>;
}

pub struct SpiTransactional<S>(pub(crate) S);

impl<S, E> ReadWrite for SpiTransactional<S>
where
    S: Transactional<u8, Error = E>,
{
    type Error = E;

    fn write_many(&mut self, reg: Registers, data: &[u8]) -> core::result::Result<(), E> {
        let mut write = [reg.write(), data[0]]; // TODO, use ALL data
        let mut operations = [Operation::Transfer(&mut write)];
        self.0.exec(&mut operations)?;
        Ok(())
    }

    fn read_many(&mut self, reg: Registers, buffer: &mut [u8]) -> core::result::Result<(), E> {
        let mut read = [reg.read(), buffer[0]]; // TODO, use ALL buffer
        let mut operations = [Operation::Transfer(&mut read)];
        self.0.exec(&mut operations)?;
        buffer[0] = read[1];
        Ok(())
    }
}

impl<S, E> ReadWrite for S
where
    S: Transfer<u8, Error = E>,
    S: Write<u8, Error = E>,
{
    type Error = E;

    fn write_many(&mut self, reg: Registers, data: &[u8]) -> core::result::Result<(), E> {
        let mut write = [reg.write(), data[0]]; // TODO, use ALL data
        self.transfer(&mut write)?;
        Ok(())
    }

    fn read_many(&mut self, reg: Registers, buffer: &mut [u8]) -> core::result::Result<(), E> {
        let mut read = [reg.read(), buffer[0]]; // TODO, use ALL buffer
        self.transfer(&mut read)?;
        buffer[0] = read[1];
        Ok(())
    }
}
