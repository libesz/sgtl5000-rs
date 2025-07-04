#![no_std]

pub use registers::I2sDataLength;
pub use registers::SampleRate;
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

    pub fn init(&mut self, sample_rate: SampleRate, data_length: I2sDataLength) -> Result<(), I2C::Error> {
        self.write_register(CHIP_ANA_POWER, ChipAnaPower::bootstrap_default().0)?;
        self.write_register(CHIP_LINREG_CTRL, ChipLinregCtrl::default().0)?;
        self.write_register(CHIP_REF_CTRL, ChipRefCtrl::default().0)?;
        self.write_register(CHIP_LINE_OUT_CTRL, ChipLineOutCtrl::default().0)?;
        self.write_register(CHIP_SHORT_CTRL, ChipShortCtrl::default().0)?;
        self.write_register(CHIP_ANA_CTRL, ChipAnaCtrl::default().0)?;
        self.write_register(CHIP_ANA_POWER, ChipAnaPower::default().0)?;
        self.write_register(CHIP_DIG_POWER, ChipDigPower::default().0)?;
        self.write_register(CHIP_LINE_OUT_CTRL, ChipLineOutCtrl::default().0)?;
        self.write_register(CHIP_LINE_OUT_VOL, ChipLineOutVol::default().0)?;
        self.set_sample_rate(sample_rate)?;
        self.set_i2s_details(I2sMode::I2s, data_length)?;
        self.write_register(CHIP_SSS_CTRL, ChipSssCtrl::default().0)?;
        self.write_register(CHIP_ADCDAC_CTRL, ChipAdcdacCtrl::default().0)?;
        self.write_register(CHIP_DAC_VOL, ChipDacVol::default().0)?;
        self.write_register(CHIP_ANA_HP_CTRL, ChipAnaHpCtrl::default().0)?;
        self.write_register(CHIP_ANA_ADC_CTRL, ChipAnaAdcCtrl::default().0)?;
        self.write_register(CHIP_ANA_CTRL, ChipAnaCtrl::default().0)?;
        Ok(())
    }

    pub fn set_i2s_details(&mut self, mode: I2sMode, data_length: I2sDataLength) -> Result<(), I2C::Error> {
        let mut chip_i2s_ctrl = ChipI2sCtrl(self.read_register(registers::CHIP_I2S_CTRL)?);
        let mode_internal: InternalI2sMode = mode.into();
        chip_i2s_ctrl.set_i2s_mode(mode_internal.i2s_mode);
        chip_i2s_ctrl.set_lralign(mode_internal.lralign);
        chip_i2s_ctrl.set_dlen(data_length);
        self.write_register(registers::CHIP_I2S_CTRL, chip_i2s_ctrl.0)?;
        Ok(())
    }
    
    pub fn set_sample_rate(&mut self, rate: SampleRate) -> Result<(), I2C::Error> {
        let mut chip_clk_ctrl = ChipClkCtrl(self.read_register(CHIP_CLK_CTRL)?);
        chip_clk_ctrl.set_sys_fs(rate);
        self.write_register(CHIP_CLK_CTRL, chip_clk_ctrl.0)?;
        Ok(())
    }

    pub fn dump_device_config(&mut self) {
        use core::fmt::Write;
        let mut buf = heapless::String::<256>::new();
        write!(buf, "[").unwrap();
        for i in 0..=30 {
            if let Ok(result) = self.read_register(i * 2) {
                write!(buf, "{:#06X}, ", result).unwrap();
            }
        }
        write!(buf, "]").unwrap();
        log::info!("SGTL5000 config: {}", buf);

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
            Transaction::write(0x0A, vec![0x00, 0x06, 0x00, 0x30]),
        ];

        let mock_i2c = I2cMock::new(&expectations);
        let mut codec = SGTL5000::new(mock_i2c, SGTL5000_I2C_DEFAULT_ADDR);
        codec.set_i2s_details(I2sMode::I2s, I2sDataLength::Bits16).unwrap();
        codec.release().done();
    }
}
