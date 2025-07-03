use bitfield::bitfield;

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

pub trait BootstrapDefault: Sized{
    fn bootstrap_default() -> Self;
}

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

impl Default for ChipI2sCtrl {
    fn default() -> Self {
        Self(0x0030)
    }
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

impl From<I2sMode> for InternalI2sMode {
    fn from(val: I2sMode) -> Self {
        match val {
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

bitfield! {
    pub(crate) struct ChipSssCtrl(u16);
    impl Debug;
    
    pub i2_s_select, set_i2_s_select: 1, 0;
    pub dac_select, set_dac_select: 5, 4;
    pub dap_select, set_dap_select: 7, 6;
    pub dap_mix_select, set_dap_mix_select: 9, 8;
    pub i2_s_lrswap, set_i2_s_lrswap: 10;
    pub dac_lrswap, set_dac_lrswap: 12;
    pub dap_lrswap, set_dap_lrswap: 13;
    pub dap_mix_lrswa, set_dap_mix_lrswa: 14;
}

impl Default for ChipSssCtrl {
    fn default() -> Self {
        Self(0x0010)
    }
}

bitfield! {
    pub(crate) struct ChipAdcdacCtrl(u16);
    impl Debug;

    pub adc_hpf_bypass, set_adc_hpf_bypass: 0;
    pub adc_hpf_freeze, set_adc_hpf_freeze: 1;
    pub dac_mute_left, set_dac_mute_left: 2;
    pub dac_mute_right, set_dac_mute_right: 3;
    pub vol_expo_ramp, set_vol_expo_ramp: 8;
    pub vol_ramp_en, set_vol_ramp_en: 9;
    pub vol_busy_dac_left, set_vol_busy_dac_left: 12;
    pub vol_busy_dac_right, set_vol_busy_dac_right: 13;
}

impl Default for ChipAdcdacCtrl {
    fn default() -> Self {
        Self(0x0055)
    }
}

bitfield! {
    pub(crate) struct ChipAnaHpCtrl(u16);
    impl Debug;

    pub hp_vol_left, set_hp_vol_left: 6, 0;
    pub hp_vol_right, set_hp_vol_right: 14, 8;
}

impl Default for ChipAnaHpCtrl{
    fn default() -> Self {
        Self(0x4040)
    }
}

bitfield! {
    pub(crate) struct ChipAnaAdcCtrl(u16);
    impl Debug;

    pub adc_vol_left, set_adc_vol_left: 3, 0;
    pub adc_vol_right, set_adc_vol_right: 7, 4;
    pub adc_vol_m6db, set_adc_vol_m6db: 8;
}

impl Default for ChipAnaAdcCtrl{
    fn default() -> Self {
        Self(0x0055)
    }
}

bitfield! {
    pub(crate) struct ChipDacVol(u16);
    impl Debug;

    pub dac_vol_left, set_dac_vol_left: 7, 0;
    pub dac_vol_right, set_dac_vol_right: 15, 8;
}

impl Default for ChipDacVol {
    fn default() -> Self {
        Self(0x3C3C)
    }
}

bitfield! {
    pub(crate) struct ChipClkCtrl(u16);
    impl Debug;

    pub mclk_freq, set_mclk_freq: 1, 0;
    pub sys_fs, set_sys_fs: 3, 2;
    pub rate_mode, set_rate_mode: 5, 4;
}

impl Default for ChipClkCtrl{
    fn default() -> Self {
        Self(0x0008)
    }
}

bitfield! {
    pub(crate) struct ChipAnaPower(u16);
    impl Debug;

    pub lineout_powerup, set_lineout_powerup: 0;
    pub adc_powerup, set_adc_powerup: 1;
    pub capless_headphone_powerup, set_capless_headphone_powerup: 2;
    pub dac_powerup, set_dav_powerup: 3;
    pub headphone_powerup, set_headphone_powerup: 4;
    pub reftop_powerup, set_reftop_powerup: 5;
    pub adc_mono, set_adc_mono: 6;
    pub vag_powerup, set_vag_powerup: 7;
    pub vcoamp_powerup, set_vcoamp_powerup: 8;
    pub linereg_d_powerup, set_linereg_d_powerup: 9;
    pub pll_powerup, set_pll_powerup: 10;
    pub vddc_chrgpmp_powerup, set_vddc_chrgpmp_powerup: 11;
    pub startup_powerup, set_startup_powerup: 12;
    pub linreg_simple_powerup, set_linreg_simple_powerup: 13;
    pub dac_mono, set_dac_mono: 14;
}

impl Default for ChipAnaPower {
    fn default() -> Self {
        Self(0x40ff)
    }
}

impl BootstrapDefault for ChipAnaPower {
    fn bootstrap_default() -> Self {
        Self(0x4060)
    }
}

bitfield!{
    pub(crate) struct ChipDigPower(u16);
    impl Debug;

    pub i2s_in_powerup, set_i2s_in_powerup: 0;
    pub i2s_out_powerup, set_i2s_out_powerup: 1;
    pub dap_powerup, set_dap_powerup: 4;
    pub dac_powerup, set_dac_powerup: 5;
    pub adc_powerup, set_adc_powerup: 6;
}

impl Default for ChipDigPower{
    fn default() -> Self {
        Self(0x0073)
    }
}

bitfield! {
    pub(crate) struct ChipLinregCtrl(u16);
    impl Debug;

    pub d_programming, set_d_programming: 3, 0;
    pub vddc_assn_ovrd, set_vddc_assn_ovrd: 5;
    pub vddc_man_assn, set_vddc_man_assn: 6;
}

impl Default for ChipLinregCtrl {
    fn default() -> Self {
        Self(0x006c)
    }
}

bitfield! {
    pub(crate) struct ChipRefCtrl(u16);
    impl Debug;

    pub small_pop, set_small_pop: 0;
    pub bias_ctrl, set_bias_ctrl: 3, 1;
    pub vag_val, set_vag_val: 8, 4;
}

impl Default for ChipRefCtrl {
    fn default() -> Self {
        Self(0x01f2)
    }
}

bitfield! {
    pub(crate) struct ChipLineOutCtrl(u16);
    impl Debug;

    pub lo_vagcntrl, set_lo_vagcntrl: 5, 0;
    pub out_current, set_out_current: 11, 8;
}

impl Default for ChipLineOutCtrl {
    fn default() -> Self {
        Self(0x0F22)
    }
}

bitfield! {
    pub(crate) struct ChipShortCtrl(u16);
    impl Debug;

    pub mode_cm, set_mode_cm: 1, 0;
    pub mode_lr, set_mode_lr: 3, 2;
    pub lvladjc, set_lvladjc: 6, 4;
    pub lvladjl, set_lvladjl: 10, 8;
    pub lvladjr, set_lvladjr: 14, 12;
}

impl Default for ChipShortCtrl {
    fn default() -> Self {
        Self(0x4446)
    }
}

bitfield! {
    pub(crate) struct ChipAnaCtrl(u16);
    impl Debug;

    pub mute_adc, set_mute_adc: 0;
    pub en_zcd_adc, set_en_zcd_adc: 1;
    pub select_adc, set_select_adc: 2;
    pub mute_hp, set_mute_hp: 4;
    pub en_zcd_hp, set_en_zcd_hp: 5;
    pub select_hp, set_select_hp: 6;
    pub mute_lo, set_mute_lo: 8;
}

impl Default for ChipAnaCtrl {
    fn default() -> Self {
        Self(0x0036)
    }
}

bitfield!{
    pub(crate) struct ChipLineOutVol(u16);
    impl Debug;

    pub lo_vol_left, set_lo_vol_left: 4, 0;
    pub lo_vol_right, set_lo_vol_right: 12, 8;
}

impl Default for ChipLineOutVol{
    fn default() -> Self {
        Self(0x1D1D)
    }
}
