// SGTL5000 register address definitions

#[allow(dead_code)]
pub const CHIP_ID: u16 = 0x0000;
#[allow(dead_code)]
pub const CHIP_DIG_POWER: u16 = 0x0002;
#[allow(dead_code)]
pub const CHIP_CLK_CTRL: u16 = 0x0004;
pub const CHIP_I2S_CTRL: u16 = 0x0006;
#[allow(dead_code)]
pub const CHIP_SSS_CTRL: u16 = 0x000A;
#[allow(dead_code)]
pub const CHIP_ADCDAC_CTRL: u16 = 0x000E;
#[allow(dead_code)]
pub const CHIP_DAC_VOL: u16 = 0x0010;
#[allow(dead_code)]
pub const CHIP_PAD_STRENGTH: u16 = 0x0014;
#[allow(dead_code)]
pub const CHIP_ANA_ADC_CTRL: u16 = 0x0020;
#[allow(dead_code)]
pub const CHIP_ANA_HP_CTRL: u16 = 0x0022;
#[allow(dead_code)]
pub const CHIP_ANA_CTRL: u16 = 0x0024;
#[allow(dead_code)]
pub const CHIP_LINREG_CTRL: u16 = 0x0026;
#[allow(dead_code)]
pub const CHIP_REF_CTRL: u16 = 0x0028;
#[allow(dead_code)]
pub const CHIP_MIC_CTRL: u16 = 0x002A;
#[allow(dead_code)]
pub const CHIP_LINE_OUT_CTRL: u16 = 0x002C;
#[allow(dead_code)]
pub const CHIP_LINE_OUT_VOL: u16 = 0x002E;
#[allow(dead_code)]
pub const CHIP_ANA_POWER: u16 = 0x0030;
#[allow(dead_code)]
pub const CHIP_PLL_CTRL: u16 = 0x0032;
#[allow(dead_code)]
pub const CHIP_CLK_TOP_CTRL: u16 = 0x0034;
#[allow(dead_code)]
pub const CHIP_ANA_STATUS: u16 = 0x0036;
#[allow(dead_code)]
pub const CHIP_ANA_TEST1: u16 = 0x0048;
#[allow(dead_code)]
pub const CHIP_ANA_TEST2: u16 = 0x003A;
#[allow(dead_code)]
pub const CHIP_SHORT_CTRL: u16 = 0x003C;
#[allow(dead_code)]
pub const DAP_CONTROL: u16 = 0x0100;
#[allow(dead_code)]
pub const DAP_PEQ: u16 = 0x0102;
#[allow(dead_code)]
pub const DAP_BASS_ENHANCE: u16 = 0x0104;
#[allow(dead_code)]
pub const DAP_BASS_ENHANCE_CTRL: u16 = 0x0106;
#[allow(dead_code)]
pub const DAP_AUDIO_EQ: u16 = 0x0108;
#[allow(dead_code)]
pub const DAP_SGTL_SURROUND: u16 = 0x010A;
#[allow(dead_code)]
pub const DAP_FILTER_COEF_ACCESS: u16 = 0x010C;
#[allow(dead_code)]
pub const DAP_COEF_WR_B0_MSB: u16 = 0x010E;
#[allow(dead_code)]
pub const DAP_COEF_WR_B0_LSB: u16 = 0x0110;
#[allow(dead_code)]
pub const DAP_AUDIO_EQ_BASS_BAND0: u16 = 0x0116;
#[allow(dead_code)]
pub const DAP_AUDIO_EQ_BAND1: u16 = 0x0118;
#[allow(dead_code)]
pub const DAP_AUDIO_EQ_BAND2: u16 = 0x011A;
#[allow(dead_code)]
pub const DAP_AUDIO_EQ_BAND3: u16 = 0x011C;
#[allow(dead_code)]
pub const DAP_AUDIO_EQ_TREBLE_BAND4: u16 = 0x011E;
#[allow(dead_code)]
pub const DAP_MAIN_CHAN: u16 = 0x0120;
#[allow(dead_code)]
pub const DAP_MIX_CHAN: u16 = 0x0122;
#[allow(dead_code)]
pub const DAP_AVC_CTRL: u16 = 0x0124;
#[allow(dead_code)]
pub const DAP_AVC_THRESHOLD: u16 = 0x0126;
#[allow(dead_code)]
pub const DAP_AVC_ATTACK: u16 = 0x0128;
#[allow(dead_code)]
pub const DAP_AVC_DECAY: u16 = 0x012A;
#[allow(dead_code)]
pub const DAP_COEF_WR_B1_MSB: u16 = 0x012C;
#[allow(dead_code)]
pub const DAP_COEF_WR_B1_LSB: u16 = 0x012E;
#[allow(dead_code)]
pub const DAP_COEF_WR_B2_MSB: u16 = 0x0130;
#[allow(dead_code)]
pub const DAP_COEF_WR_B2_LSB: u16 = 0x0132;
#[allow(dead_code)]
pub const DAP_COEF_WR_A1_MSB: u16 = 0x0134;
#[allow(dead_code)]
pub const DAP_COEF_WR_A1_LSB: u16 = 0x0136;
#[allow(dead_code)]
pub const DAP_COEF_WR_A2_MSB: u16 = 0x0138;
#[allow(dead_code)]
pub const DAP_COEF_WR_A2_LSB: u16 = 0x013A;

use bitfield::bitfield;
bitfield! {
    pub(crate) struct ChipI2sCtrl(u16);
    impl Debug;

    pub lrpol, set_lrpol: 0;
    pub lralign, set_lralign: 1;
    pub i2s_mode, set_i2s_mode: 3, 2;
    pub dlen, set_dlen: 5, 4;
    pub sclk_inv, set_sclk_inv: 6;
    pub ms, set_ms: 7;
    pub sclkfreq, set_sclkfreq: 8;
}

pub enum I2sMode {
    I2s,
    LeftJustified,
    RightJustified,
    PcmA,
    PcmB,
}

pub(crate) struct InternalI2sMode {
    pub i2s_mode: u16,
    pub lralign: bool,
}

impl Into<InternalI2sMode> for I2sMode {
    fn into(self) -> InternalI2sMode {
        match self {
            I2sMode::I2s => InternalI2sMode {
                i2s_mode: 0x0,
                lralign: false,
            },
            I2sMode::LeftJustified => InternalI2sMode {
                i2s_mode: 0x0,
                lralign: true,
            },
            I2sMode::RightJustified => InternalI2sMode {
                i2s_mode: 0x01,
                lralign: false,
            },
            I2sMode::PcmA => InternalI2sMode {
                i2s_mode: 0x10,
                lralign: false,
            },
            I2sMode::PcmB => InternalI2sMode {
                i2s_mode: 0x10,
                lralign: true,
            },
        }
    }
}
