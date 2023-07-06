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
        let mut write = data.to_vec();
        write.insert(0, reg.write());
        let mut operations = [Operation::Write(&write)];
        self.0.exec(&mut operations)
    }

    fn read_many(&mut self, reg: Registers, buffer: &mut [u8]) -> core::result::Result<(), E> {
        let mut read = buffer.to_vec();
        read.insert(0, reg.read());
        let mut operations = [Operation::Transfer(&mut read)];
        self.0.exec(&mut operations)?;
        buffer.copy_from_slice(&read[1..]);
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
        let mut write = data.to_vec();
        write.insert(0, reg.write());
        self.write(&write)
    }

    fn read_many(&mut self, reg: Registers, buffer: &mut [u8]) -> core::result::Result<(), E> {
        let mut read = buffer.to_vec();
        read.insert(0, reg.read());
        self.transfer(&mut read)?;
        buffer.copy_from_slice(&read[1..]);
        Ok(())
    }
}
