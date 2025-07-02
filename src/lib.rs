#![no_std]

mod registers;

use embedded_hal::i2c::{I2c, SevenBitAddress};
use registers::*;

pub const SGTL5000_I2C_DEFAULT_ADDR: u8 = 0x0A;

pub struct SGTL5000<I2C> {
    i2c: I2C,
    address: SevenBitAddress,
}

impl<I2C> SGTL5000<I2C>
where
    I2C: I2c<SevenBitAddress>,
{
    pub fn new(i2c: I2C, address: SevenBitAddress) -> Self {
        Self { i2c, address }
    }

    fn read_register(&mut self, reg: u16) -> Result<u16, I2C::Error> {
        let addr = [(reg >> 8) as u8, reg as u8];
        let mut buf = [0u8; 2];
        self.i2c.write_read(self.address, &addr, &mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    fn write_register(&mut self, reg: u16, value: u16) -> Result<(), I2C::Error> {
        let data = [(reg >> 8) as u8, reg as u8, (value >> 8) as u8, value as u8];
        self.i2c.write(self.address, &data)
    }

    pub fn set_i2s_mode(&mut self, mode: registers::I2sMode) -> Result<(), I2C::Error> {
        let mut chip_i2s_ctrl = ChipI2sCtrl(self.read_register(registers::CHIP_I2S_CTRL)?);
        let mode_internal: registers::InternalI2sMode = mode.into();
        chip_i2s_ctrl.set_i2s_mode(mode_internal.i2s_mode);
        chip_i2s_ctrl.set_lralign(mode_internal.lralign);
        self.write_register(registers::CHIP_I2S_CTRL, chip_i2s_ctrl.0)?;
        Ok(())
    }

    pub fn release(self) -> I2C {
        self.i2c
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction};
    use std::vec;

    #[test]
    fn test_set_i2s_mode() {
        let expectations = [
            Transaction::write_read(0x0A, vec![0x00, 0x06], vec![0x00, 0x00]),
            Transaction::write(0x0A, vec![0x00, 0x06, 0x00, 0x00]),
        ];

        let mock_i2c = I2cMock::new(&expectations);
        let mut codec = SGTL5000::new(mock_i2c, SGTL5000_I2C_DEFAULT_ADDR);
        codec.set_i2s_mode(I2sMode::I2s).unwrap();
        codec.release().done();
    }
}
