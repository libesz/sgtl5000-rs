extern crate std;
use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction};
use sgtl5000::*;
use std::vec;

#[test]
fn test_set_i2s_mode() {
    let expectations = [
        Transaction::write_read(0x0A, vec![0x00, 0x06], vec![0x00, 0x00]),
        Transaction::write(0x0A, vec![0x00, 0x06, 0x00, 0x30]),
    ];

    let mock_i2c = I2cMock::new(&expectations);
    let mut codec = SGTL5000::new(mock_i2c, SGTL5000_I2C_DEFAULT_ADDR);
    codec
        .set_i2s_details(crate::I2sMode::I2s, I2sDataLength::Bits16)
        .unwrap();
    codec.release().done();
}
