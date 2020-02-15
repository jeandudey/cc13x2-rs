#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 160usize],
    #[doc = "0xa0 - Misc configurations"]
    pub misc_conf_1: MISC_CONF_1,
    #[doc = "0xa4 - Internal. Only to be used through TI provided API."]
    pub misc_conf_2: MISC_CONF_2,
    _reserved2: [u8; 28usize],
    #[doc = "0xc4 - Internal. Only to be used through TI provided API."]
    pub config_cc26_fe: CONFIG_CC26_FE,
    #[doc = "0xc8 - Internal. Only to be used through TI provided API."]
    pub config_cc13_fe: CONFIG_CC13_FE,
    #[doc = "0xcc - Internal. Only to be used through TI provided API."]
    pub config_rf_common: CONFIG_RF_COMMON,
    #[doc = "0xd0 - Internal. Only to be used through TI provided API."]
    pub config_synth_div2_cc26_2g4: CONFIG_SYNTH_DIV2_CC26_2G4,
    #[doc = "0xd4 - Internal. Only to be used through TI provided API."]
    pub config_synth_div2_cc13_2g4: CONFIG_SYNTH_DIV2_CC13_2G4,
    #[doc = "0xd8 - Internal. Only to be used through TI provided API."]
    pub config_synth_div2_cc26_1g: CONFIG_SYNTH_DIV2_CC26_1G,
    #[doc = "0xdc - Internal. Only to be used through TI provided API."]
    pub config_synth_div2_cc13_1g: CONFIG_SYNTH_DIV2_CC13_1G,
    #[doc = "0xe0 - Internal. Only to be used through TI provided API."]
    pub config_synth_div4_cc26: CONFIG_SYNTH_DIV4_CC26,
    #[doc = "0xe4 - Internal. Only to be used through TI provided API."]
    pub config_synth_div4_cc13: CONFIG_SYNTH_DIV4_CC13,
    #[doc = "0xe8 - Internal. Only to be used through TI provided API."]
    pub config_synth_div5: CONFIG_SYNTH_DIV5,
    #[doc = "0xec - Internal. Only to be used through TI provided API."]
    pub config_synth_div6_cc26: CONFIG_SYNTH_DIV6_CC26,
    #[doc = "0xf0 - Internal. Only to be used through TI provided API."]
    pub config_synth_div6_cc13: CONFIG_SYNTH_DIV6_CC13,
    #[doc = "0xf4 - Internal. Only to be used through TI provided API."]
    pub config_synth_div10: CONFIG_SYNTH_DIV10,
    #[doc = "0xf8 - Internal. Only to be used through TI provided API."]
    pub config_synth_div12_cc26: CONFIG_SYNTH_DIV12_CC26,
    #[doc = "0xfc - Internal. Only to be used through TI provided API."]
    pub config_synth_div12_cc13: CONFIG_SYNTH_DIV12_CC13,
    #[doc = "0x100 - Internal. Only to be used through TI provided API."]
    pub config_synth_div15: CONFIG_SYNTH_DIV15,
    #[doc = "0x104 - Internal. Only to be used through TI provided API."]
    pub config_synth_div30: CONFIG_SYNTH_DIV30,
    _reserved19: [u8; 92usize],
    #[doc = "0x164 - Flash information"]
    pub flash_number: FLASH_NUMBER,
    _reserved20: [u8; 4usize],
    #[doc = "0x16c - Flash information"]
    pub flash_coordinate: FLASH_COORDINATE,
    #[doc = "0x170 - Internal. Only to be used through TI provided API."]
    pub flash_e_p: FLASH_E_P,
    #[doc = "0x174 - Internal. Only to be used through TI provided API."]
    pub flash_c_e_p_r: FLASH_C_E_P_R,
    #[doc = "0x178 - Internal. Only to be used through TI provided API."]
    pub flash_p_r_pv: FLASH_P_R_PV,
    #[doc = "0x17c - Internal. Only to be used through TI provided API."]
    pub flash_eh_seq: FLASH_EH_SEQ,
    #[doc = "0x180 - Internal. Only to be used through TI provided API."]
    pub flash_vhv_e: FLASH_VHV_E,
    #[doc = "0x184 - Internal. Only to be used through TI provided API."]
    pub flash_pp: FLASH_PP,
    #[doc = "0x188 - Internal. Only to be used through TI provided API."]
    pub flash_prog_ep: FLASH_PROG_EP,
    #[doc = "0x18c - Internal. Only to be used through TI provided API."]
    pub flash_era_pw: FLASH_ERA_PW,
    #[doc = "0x190 - Internal. Only to be used through TI provided API."]
    pub flash_vhv: FLASH_VHV,
    #[doc = "0x194 - Internal. Only to be used through TI provided API."]
    pub flash_vhv_pv: FLASH_VHV_PV,
    #[doc = "0x198 - Internal. Only to be used through TI provided API."]
    pub flash_v: FLASH_V,
    _reserved32: [u8; 248usize],
    #[doc = "0x294 - User Identification. Reading this register and the FCFG1:ICEPICK_DEVICE_ID register is the only supported way of identifying a device. The value of this register will be written to AON_PMCTL:JTAGUSERCODE by boot FW while in safezone."]
    pub user_id: USER_ID,
    _reserved33: [u8; 24usize],
    #[doc = "0x2b0 - Internal. Only to be used through TI provided API."]
    pub flash_otp_data3: FLASH_OTP_DATA3,
    #[doc = "0x2b4 - Internal. Only to be used through TI provided API."]
    pub ana2_trim: ANA2_TRIM,
    #[doc = "0x2b8 - Internal. Only to be used through TI provided API."]
    pub ldo_trim: LDO_TRIM,
    _reserved36: [u8; 44usize],
    #[doc = "0x2e8 - MAC BLE Address 0"]
    pub mac_ble_0: MAC_BLE_0,
    #[doc = "0x2ec - MAC BLE Address 1"]
    pub mac_ble_1: MAC_BLE_1,
    #[doc = "0x2f0 - MAC IEEE 802.15.4 Address 0"]
    pub mac_15_4_0: MAC_15_4_0,
    #[doc = "0x2f4 - MAC IEEE 802.15.4 Address 1"]
    pub mac_15_4_1: MAC_15_4_1,
    _reserved40: [u8; 16usize],
    #[doc = "0x308 - Internal. Only to be used through TI provided API."]
    pub flash_otp_data4: FLASH_OTP_DATA4,
    #[doc = "0x30c - Miscellaneous Trim Parameters"]
    pub misc_trim: MISC_TRIM,
    #[doc = "0x310 - Internal. Only to be used through TI provided API."]
    pub rcosc_hf_tempcomp: RCOSC_HF_TEMPCOMP,
    _reserved43: [u8; 4usize],
    #[doc = "0x318 - IcePick Device Identification Reading this register and the FCFG1:USER_ID register is the only supported way of identifying a device."]
    pub icepick_device_id: ICEPICK_DEVICE_ID,
    #[doc = "0x31c - Factory Configuration (FCFG1) Revision"]
    pub fcfg1_revision: FCFG1_REVISION,
    #[doc = "0x320 - Misc OTP Data"]
    pub misc_otp_data: MISC_OTP_DATA,
    _reserved46: [u8; 32usize],
    #[doc = "0x344 - IO Configuration"]
    pub ioconf: IOCONF,
    _reserved47: [u8; 4usize],
    #[doc = "0x34c - Internal. Only to be used through TI provided API."]
    pub config_if_adc: CONFIG_IF_ADC,
    #[doc = "0x350 - Internal. Only to be used through TI provided API."]
    pub config_osc_top: CONFIG_OSC_TOP,
    _reserved49: [u8; 8usize],
    #[doc = "0x35c - AUX_ADC Gain in Absolute Reference Mode"]
    pub soc_adc_abs_gain: SOC_ADC_ABS_GAIN,
    #[doc = "0x360 - AUX_ADC Gain in Relative Reference Mode"]
    pub soc_adc_rel_gain: SOC_ADC_REL_GAIN,
    _reserved51: [u8; 4usize],
    #[doc = "0x368 - AUX_ADC Temperature Offsets in Absolute Reference Mode"]
    pub soc_adc_offset_int: SOC_ADC_OFFSET_INT,
    #[doc = "0x36c - Internal. Only to be used through TI provided API."]
    pub soc_adc_ref_trim_and_offset_ext: SOC_ADC_REF_TRIM_AND_OFFSET_EXT,
    #[doc = "0x370 - Internal. Only to be used through TI provided API."]
    pub ampcomp_th1: AMPCOMP_TH1,
    #[doc = "0x374 - Internal. Only to be used through TI provided API."]
    pub ampcomp_th2: AMPCOMP_TH2,
    #[doc = "0x378 - Internal. Only to be used through TI provided API."]
    pub ampcomp_ctrl1: AMPCOMP_CTRL1,
    #[doc = "0x37c - Internal. Only to be used through TI provided API."]
    pub anabypass_value2: ANABYPASS_VALUE2,
    _reserved57: [u8; 8usize],
    #[doc = "0x388 - Internal. Only to be used through TI provided API."]
    pub volt_trim: VOLT_TRIM,
    #[doc = "0x38c - OSC Configuration"]
    pub osc_conf: OSC_CONF,
    #[doc = "0x390 - Internal. Only to be used through TI provided API."]
    pub freq_offset: FREQ_OFFSET,
    _reserved60: [u8; 4usize],
    #[doc = "0x398 - Internal. Only to be used through TI provided API."]
    pub misc_otp_data_1: MISC_OTP_DATA_1,
    _reserved61: [u8; 52usize],
    #[doc = "0x3d0 - Shadow of DIE_ID_0 register in eFuse"]
    pub shdw_die_id_0: SHDW_DIE_ID_0,
    #[doc = "0x3d4 - Shadow of DIE_ID_1 register in eFuse"]
    pub shdw_die_id_1: SHDW_DIE_ID_1,
    #[doc = "0x3d8 - Shadow of DIE_ID_2 register in eFuse"]
    pub shdw_die_id_2: SHDW_DIE_ID_2,
    #[doc = "0x3dc - Shadow of DIE_ID_3 register in eFuse"]
    pub shdw_die_id_3: SHDW_DIE_ID_3,
    _reserved65: [u8; 24usize],
    #[doc = "0x3f8 - Internal. Only to be used through TI provided API."]
    pub shdw_osc_bias_ldo_trim: SHDW_OSC_BIAS_LDO_TRIM,
    #[doc = "0x3fc - Internal. Only to be used through TI provided API."]
    pub shdw_ana_trim: SHDW_ANA_TRIM,
    _reserved67: [u8; 12usize],
    #[doc = "0x40c - Internal. Only to be used through TI provided API."]
    pub dac_bias_cnf: DAC_BIAS_CNF,
    _reserved68: [u8; 8usize],
    #[doc = "0x418 - Internal. Only to be used through TI provided API."]
    pub tfw_probe: TFW_PROBE,
    #[doc = "0x41c - Internal. Only to be used through TI provided API."]
    pub tfw_ft: TFW_FT,
    #[doc = "0x420 - Internal. Only to be used through TI provided API."]
    pub dac_cal0: DAC_CAL0,
    #[doc = "0x424 - Internal. Only to be used through TI provided API."]
    pub dac_cal1: DAC_CAL1,
    #[doc = "0x428 - Internal. Only to be used through TI provided API."]
    pub dac_cal2: DAC_CAL2,
    #[doc = "0x42c - Internal. Only to be used through TI provided API."]
    pub dac_cal3: DAC_CAL3,
    #[doc = "0x430 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved_n: RESERVED_N,
}
#[doc = "Misc configurations\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_conf_1](misc_conf_1) module"]
pub type MISC_CONF_1 = crate::Reg<u32, _MISC_CONF_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC_CONF_1;
#[doc = "`read()` method returns [misc_conf_1::R](misc_conf_1::R) reader structure"]
impl crate::Readable for MISC_CONF_1 {}
#[doc = "`write(|w| ..)` method takes [misc_conf_1::W](misc_conf_1::W) writer structure"]
impl crate::Writable for MISC_CONF_1 {}
#[doc = "Misc configurations"]
pub mod misc_conf_1;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_conf_2](misc_conf_2) module"]
pub type MISC_CONF_2 = crate::Reg<u32, _MISC_CONF_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC_CONF_2;
#[doc = "`read()` method returns [misc_conf_2::R](misc_conf_2::R) reader structure"]
impl crate::Readable for MISC_CONF_2 {}
#[doc = "`write(|w| ..)` method takes [misc_conf_2::W](misc_conf_2::W) writer structure"]
impl crate::Writable for MISC_CONF_2 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod misc_conf_2;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_cc26_fe](config_cc26_fe) module"]
pub type CONFIG_CC26_FE = crate::Reg<u32, _CONFIG_CC26_FE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_CC26_FE;
#[doc = "`read()` method returns [config_cc26_fe::R](config_cc26_fe::R) reader structure"]
impl crate::Readable for CONFIG_CC26_FE {}
#[doc = "`write(|w| ..)` method takes [config_cc26_fe::W](config_cc26_fe::W) writer structure"]
impl crate::Writable for CONFIG_CC26_FE {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_cc26_fe;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_cc13_fe](config_cc13_fe) module"]
pub type CONFIG_CC13_FE = crate::Reg<u32, _CONFIG_CC13_FE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_CC13_FE;
#[doc = "`read()` method returns [config_cc13_fe::R](config_cc13_fe::R) reader structure"]
impl crate::Readable for CONFIG_CC13_FE {}
#[doc = "`write(|w| ..)` method takes [config_cc13_fe::W](config_cc13_fe::W) writer structure"]
impl crate::Writable for CONFIG_CC13_FE {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_cc13_fe;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_rf_common](config_rf_common) module"]
pub type CONFIG_RF_COMMON = crate::Reg<u32, _CONFIG_RF_COMMON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_RF_COMMON;
#[doc = "`read()` method returns [config_rf_common::R](config_rf_common::R) reader structure"]
impl crate::Readable for CONFIG_RF_COMMON {}
#[doc = "`write(|w| ..)` method takes [config_rf_common::W](config_rf_common::W) writer structure"]
impl crate::Writable for CONFIG_RF_COMMON {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_rf_common;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_synth_div2_cc26_2g4](config_synth_div2_cc26_2g4) module"]
pub type CONFIG_SYNTH_DIV2_CC26_2G4 = crate::Reg<u32, _CONFIG_SYNTH_DIV2_CC26_2G4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_SYNTH_DIV2_CC26_2G4;
#[doc = "`read()` method returns [config_synth_div2_cc26_2g4::R](config_synth_div2_cc26_2g4::R) reader structure"]
impl crate::Readable for CONFIG_SYNTH_DIV2_CC26_2G4 {}
#[doc = "`write(|w| ..)` method takes [config_synth_div2_cc26_2g4::W](config_synth_div2_cc26_2g4::W) writer structure"]
impl crate::Writable for CONFIG_SYNTH_DIV2_CC26_2G4 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div2_cc26_2g4;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_synth_div2_cc13_2g4](config_synth_div2_cc13_2g4) module"]
pub type CONFIG_SYNTH_DIV2_CC13_2G4 = crate::Reg<u32, _CONFIG_SYNTH_DIV2_CC13_2G4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_SYNTH_DIV2_CC13_2G4;
#[doc = "`read()` method returns [config_synth_div2_cc13_2g4::R](config_synth_div2_cc13_2g4::R) reader structure"]
impl crate::Readable for CONFIG_SYNTH_DIV2_CC13_2G4 {}
#[doc = "`write(|w| ..)` method takes [config_synth_div2_cc13_2g4::W](config_synth_div2_cc13_2g4::W) writer structure"]
impl crate::Writable for CONFIG_SYNTH_DIV2_CC13_2G4 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div2_cc13_2g4;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_synth_div2_cc26_1g](config_synth_div2_cc26_1g) module"]
pub type CONFIG_SYNTH_DIV2_CC26_1G = crate::Reg<u32, _CONFIG_SYNTH_DIV2_CC26_1G>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_SYNTH_DIV2_CC26_1G;
#[doc = "`read()` method returns [config_synth_div2_cc26_1g::R](config_synth_div2_cc26_1g::R) reader structure"]
impl crate::Readable for CONFIG_SYNTH_DIV2_CC26_1G {}
#[doc = "`write(|w| ..)` method takes [config_synth_div2_cc26_1g::W](config_synth_div2_cc26_1g::W) writer structure"]
impl crate::Writable for CONFIG_SYNTH_DIV2_CC26_1G {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div2_cc26_1g;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_synth_div2_cc13_1g](config_synth_div2_cc13_1g) module"]
pub type CONFIG_SYNTH_DIV2_CC13_1G = crate::Reg<u32, _CONFIG_SYNTH_DIV2_CC13_1G>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_SYNTH_DIV2_CC13_1G;
#[doc = "`read()` method returns [config_synth_div2_cc13_1g::R](config_synth_div2_cc13_1g::R) reader structure"]
impl crate::Readable for CONFIG_SYNTH_DIV2_CC13_1G {}
#[doc = "`write(|w| ..)` method takes [config_synth_div2_cc13_1g::W](config_synth_div2_cc13_1g::W) writer structure"]
impl crate::Writable for CONFIG_SYNTH_DIV2_CC13_1G {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div2_cc13_1g;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_synth_div4_cc26](config_synth_div4_cc26) module"]
pub type CONFIG_SYNTH_DIV4_CC26 = crate::Reg<u32, _CONFIG_SYNTH_DIV4_CC26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_SYNTH_DIV4_CC26;
#[doc = "`read()` method returns [config_synth_div4_cc26::R](config_synth_div4_cc26::R) reader structure"]
impl crate::Readable for CONFIG_SYNTH_DIV4_CC26 {}
#[doc = "`write(|w| ..)` method takes [config_synth_div4_cc26::W](config_synth_div4_cc26::W) writer structure"]
impl crate::Writable for CONFIG_SYNTH_DIV4_CC26 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div4_cc26;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_synth_div4_cc13](config_synth_div4_cc13) module"]
pub type CONFIG_SYNTH_DIV4_CC13 = crate::Reg<u32, _CONFIG_SYNTH_DIV4_CC13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_SYNTH_DIV4_CC13;
#[doc = "`read()` method returns [config_synth_div4_cc13::R](config_synth_div4_cc13::R) reader structure"]
impl crate::Readable for CONFIG_SYNTH_DIV4_CC13 {}
#[doc = "`write(|w| ..)` method takes [config_synth_div4_cc13::W](config_synth_div4_cc13::W) writer structure"]
impl crate::Writable for CONFIG_SYNTH_DIV4_CC13 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div4_cc13;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_synth_div5](config_synth_div5) module"]
pub type CONFIG_SYNTH_DIV5 = crate::Reg<u32, _CONFIG_SYNTH_DIV5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_SYNTH_DIV5;
#[doc = "`read()` method returns [config_synth_div5::R](config_synth_div5::R) reader structure"]
impl crate::Readable for CONFIG_SYNTH_DIV5 {}
#[doc = "`write(|w| ..)` method takes [config_synth_div5::W](config_synth_div5::W) writer structure"]
impl crate::Writable for CONFIG_SYNTH_DIV5 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div5;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_synth_div6_cc26](config_synth_div6_cc26) module"]
pub type CONFIG_SYNTH_DIV6_CC26 = crate::Reg<u32, _CONFIG_SYNTH_DIV6_CC26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_SYNTH_DIV6_CC26;
#[doc = "`read()` method returns [config_synth_div6_cc26::R](config_synth_div6_cc26::R) reader structure"]
impl crate::Readable for CONFIG_SYNTH_DIV6_CC26 {}
#[doc = "`write(|w| ..)` method takes [config_synth_div6_cc26::W](config_synth_div6_cc26::W) writer structure"]
impl crate::Writable for CONFIG_SYNTH_DIV6_CC26 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div6_cc26;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_synth_div6_cc13](config_synth_div6_cc13) module"]
pub type CONFIG_SYNTH_DIV6_CC13 = crate::Reg<u32, _CONFIG_SYNTH_DIV6_CC13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_SYNTH_DIV6_CC13;
#[doc = "`read()` method returns [config_synth_div6_cc13::R](config_synth_div6_cc13::R) reader structure"]
impl crate::Readable for CONFIG_SYNTH_DIV6_CC13 {}
#[doc = "`write(|w| ..)` method takes [config_synth_div6_cc13::W](config_synth_div6_cc13::W) writer structure"]
impl crate::Writable for CONFIG_SYNTH_DIV6_CC13 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div6_cc13;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_synth_div10](config_synth_div10) module"]
pub type CONFIG_SYNTH_DIV10 = crate::Reg<u32, _CONFIG_SYNTH_DIV10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_SYNTH_DIV10;
#[doc = "`read()` method returns [config_synth_div10::R](config_synth_div10::R) reader structure"]
impl crate::Readable for CONFIG_SYNTH_DIV10 {}
#[doc = "`write(|w| ..)` method takes [config_synth_div10::W](config_synth_div10::W) writer structure"]
impl crate::Writable for CONFIG_SYNTH_DIV10 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div10;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_synth_div12_cc26](config_synth_div12_cc26) module"]
pub type CONFIG_SYNTH_DIV12_CC26 = crate::Reg<u32, _CONFIG_SYNTH_DIV12_CC26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_SYNTH_DIV12_CC26;
#[doc = "`read()` method returns [config_synth_div12_cc26::R](config_synth_div12_cc26::R) reader structure"]
impl crate::Readable for CONFIG_SYNTH_DIV12_CC26 {}
#[doc = "`write(|w| ..)` method takes [config_synth_div12_cc26::W](config_synth_div12_cc26::W) writer structure"]
impl crate::Writable for CONFIG_SYNTH_DIV12_CC26 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div12_cc26;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_synth_div12_cc13](config_synth_div12_cc13) module"]
pub type CONFIG_SYNTH_DIV12_CC13 = crate::Reg<u32, _CONFIG_SYNTH_DIV12_CC13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_SYNTH_DIV12_CC13;
#[doc = "`read()` method returns [config_synth_div12_cc13::R](config_synth_div12_cc13::R) reader structure"]
impl crate::Readable for CONFIG_SYNTH_DIV12_CC13 {}
#[doc = "`write(|w| ..)` method takes [config_synth_div12_cc13::W](config_synth_div12_cc13::W) writer structure"]
impl crate::Writable for CONFIG_SYNTH_DIV12_CC13 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div12_cc13;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_synth_div15](config_synth_div15) module"]
pub type CONFIG_SYNTH_DIV15 = crate::Reg<u32, _CONFIG_SYNTH_DIV15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_SYNTH_DIV15;
#[doc = "`read()` method returns [config_synth_div15::R](config_synth_div15::R) reader structure"]
impl crate::Readable for CONFIG_SYNTH_DIV15 {}
#[doc = "`write(|w| ..)` method takes [config_synth_div15::W](config_synth_div15::W) writer structure"]
impl crate::Writable for CONFIG_SYNTH_DIV15 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div15;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_synth_div30](config_synth_div30) module"]
pub type CONFIG_SYNTH_DIV30 = crate::Reg<u32, _CONFIG_SYNTH_DIV30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_SYNTH_DIV30;
#[doc = "`read()` method returns [config_synth_div30::R](config_synth_div30::R) reader structure"]
impl crate::Readable for CONFIG_SYNTH_DIV30 {}
#[doc = "`write(|w| ..)` method takes [config_synth_div30::W](config_synth_div30::W) writer structure"]
impl crate::Writable for CONFIG_SYNTH_DIV30 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div30;
#[doc = "Flash information\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_number](flash_number) module"]
pub type FLASH_NUMBER = crate::Reg<u32, _FLASH_NUMBER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_NUMBER;
#[doc = "`read()` method returns [flash_number::R](flash_number::R) reader structure"]
impl crate::Readable for FLASH_NUMBER {}
#[doc = "`write(|w| ..)` method takes [flash_number::W](flash_number::W) writer structure"]
impl crate::Writable for FLASH_NUMBER {}
#[doc = "Flash information"]
pub mod flash_number;
#[doc = "Flash information\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_coordinate](flash_coordinate) module"]
pub type FLASH_COORDINATE = crate::Reg<u32, _FLASH_COORDINATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_COORDINATE;
#[doc = "`read()` method returns [flash_coordinate::R](flash_coordinate::R) reader structure"]
impl crate::Readable for FLASH_COORDINATE {}
#[doc = "`write(|w| ..)` method takes [flash_coordinate::W](flash_coordinate::W) writer structure"]
impl crate::Writable for FLASH_COORDINATE {}
#[doc = "Flash information"]
pub mod flash_coordinate;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_e_p](flash_e_p) module"]
pub type FLASH_E_P = crate::Reg<u32, _FLASH_E_P>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_E_P;
#[doc = "`read()` method returns [flash_e_p::R](flash_e_p::R) reader structure"]
impl crate::Readable for FLASH_E_P {}
#[doc = "`write(|w| ..)` method takes [flash_e_p::W](flash_e_p::W) writer structure"]
impl crate::Writable for FLASH_E_P {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_e_p;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_c_e_p_r](flash_c_e_p_r) module"]
pub type FLASH_C_E_P_R = crate::Reg<u32, _FLASH_C_E_P_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_C_E_P_R;
#[doc = "`read()` method returns [flash_c_e_p_r::R](flash_c_e_p_r::R) reader structure"]
impl crate::Readable for FLASH_C_E_P_R {}
#[doc = "`write(|w| ..)` method takes [flash_c_e_p_r::W](flash_c_e_p_r::W) writer structure"]
impl crate::Writable for FLASH_C_E_P_R {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_c_e_p_r;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_p_r_pv](flash_p_r_pv) module"]
pub type FLASH_P_R_PV = crate::Reg<u32, _FLASH_P_R_PV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_P_R_PV;
#[doc = "`read()` method returns [flash_p_r_pv::R](flash_p_r_pv::R) reader structure"]
impl crate::Readable for FLASH_P_R_PV {}
#[doc = "`write(|w| ..)` method takes [flash_p_r_pv::W](flash_p_r_pv::W) writer structure"]
impl crate::Writable for FLASH_P_R_PV {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_p_r_pv;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_eh_seq](flash_eh_seq) module"]
pub type FLASH_EH_SEQ = crate::Reg<u32, _FLASH_EH_SEQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_EH_SEQ;
#[doc = "`read()` method returns [flash_eh_seq::R](flash_eh_seq::R) reader structure"]
impl crate::Readable for FLASH_EH_SEQ {}
#[doc = "`write(|w| ..)` method takes [flash_eh_seq::W](flash_eh_seq::W) writer structure"]
impl crate::Writable for FLASH_EH_SEQ {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_eh_seq;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_vhv_e](flash_vhv_e) module"]
pub type FLASH_VHV_E = crate::Reg<u32, _FLASH_VHV_E>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_VHV_E;
#[doc = "`read()` method returns [flash_vhv_e::R](flash_vhv_e::R) reader structure"]
impl crate::Readable for FLASH_VHV_E {}
#[doc = "`write(|w| ..)` method takes [flash_vhv_e::W](flash_vhv_e::W) writer structure"]
impl crate::Writable for FLASH_VHV_E {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_vhv_e;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_pp](flash_pp) module"]
pub type FLASH_PP = crate::Reg<u32, _FLASH_PP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_PP;
#[doc = "`read()` method returns [flash_pp::R](flash_pp::R) reader structure"]
impl crate::Readable for FLASH_PP {}
#[doc = "`write(|w| ..)` method takes [flash_pp::W](flash_pp::W) writer structure"]
impl crate::Writable for FLASH_PP {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_pp;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_prog_ep](flash_prog_ep) module"]
pub type FLASH_PROG_EP = crate::Reg<u32, _FLASH_PROG_EP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_PROG_EP;
#[doc = "`read()` method returns [flash_prog_ep::R](flash_prog_ep::R) reader structure"]
impl crate::Readable for FLASH_PROG_EP {}
#[doc = "`write(|w| ..)` method takes [flash_prog_ep::W](flash_prog_ep::W) writer structure"]
impl crate::Writable for FLASH_PROG_EP {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_prog_ep;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_era_pw](flash_era_pw) module"]
pub type FLASH_ERA_PW = crate::Reg<u32, _FLASH_ERA_PW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_ERA_PW;
#[doc = "`read()` method returns [flash_era_pw::R](flash_era_pw::R) reader structure"]
impl crate::Readable for FLASH_ERA_PW {}
#[doc = "`write(|w| ..)` method takes [flash_era_pw::W](flash_era_pw::W) writer structure"]
impl crate::Writable for FLASH_ERA_PW {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_era_pw;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_vhv](flash_vhv) module"]
pub type FLASH_VHV = crate::Reg<u32, _FLASH_VHV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_VHV;
#[doc = "`read()` method returns [flash_vhv::R](flash_vhv::R) reader structure"]
impl crate::Readable for FLASH_VHV {}
#[doc = "`write(|w| ..)` method takes [flash_vhv::W](flash_vhv::W) writer structure"]
impl crate::Writable for FLASH_VHV {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_vhv;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_vhv_pv](flash_vhv_pv) module"]
pub type FLASH_VHV_PV = crate::Reg<u32, _FLASH_VHV_PV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_VHV_PV;
#[doc = "`read()` method returns [flash_vhv_pv::R](flash_vhv_pv::R) reader structure"]
impl crate::Readable for FLASH_VHV_PV {}
#[doc = "`write(|w| ..)` method takes [flash_vhv_pv::W](flash_vhv_pv::W) writer structure"]
impl crate::Writable for FLASH_VHV_PV {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_vhv_pv;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_v](flash_v) module"]
pub type FLASH_V = crate::Reg<u32, _FLASH_V>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_V;
#[doc = "`read()` method returns [flash_v::R](flash_v::R) reader structure"]
impl crate::Readable for FLASH_V {}
#[doc = "`write(|w| ..)` method takes [flash_v::W](flash_v::W) writer structure"]
impl crate::Writable for FLASH_V {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_v;
#[doc = "User Identification. Reading this register and the FCFG1:ICEPICK_DEVICE_ID register is the only supported way of identifying a device. The value of this register will be written to AON_PMCTL:JTAGUSERCODE by boot FW while in safezone.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [user_id](user_id) module"]
pub type USER_ID = crate::Reg<u32, _USER_ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER_ID;
#[doc = "`read()` method returns [user_id::R](user_id::R) reader structure"]
impl crate::Readable for USER_ID {}
#[doc = "`write(|w| ..)` method takes [user_id::W](user_id::W) writer structure"]
impl crate::Writable for USER_ID {}
#[doc = "User Identification. Reading this register and the FCFG1:ICEPICK_DEVICE_ID register is the only supported way of identifying a device. The value of this register will be written to AON_PMCTL:JTAGUSERCODE by boot FW while in safezone."]
pub mod user_id;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_otp_data3](flash_otp_data3) module"]
pub type FLASH_OTP_DATA3 = crate::Reg<u32, _FLASH_OTP_DATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_OTP_DATA3;
#[doc = "`read()` method returns [flash_otp_data3::R](flash_otp_data3::R) reader structure"]
impl crate::Readable for FLASH_OTP_DATA3 {}
#[doc = "`write(|w| ..)` method takes [flash_otp_data3::W](flash_otp_data3::W) writer structure"]
impl crate::Writable for FLASH_OTP_DATA3 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_otp_data3;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana2_trim](ana2_trim) module"]
pub type ANA2_TRIM = crate::Reg<u32, _ANA2_TRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANA2_TRIM;
#[doc = "`read()` method returns [ana2_trim::R](ana2_trim::R) reader structure"]
impl crate::Readable for ANA2_TRIM {}
#[doc = "`write(|w| ..)` method takes [ana2_trim::W](ana2_trim::W) writer structure"]
impl crate::Writable for ANA2_TRIM {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ana2_trim;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldo_trim](ldo_trim) module"]
pub type LDO_TRIM = crate::Reg<u32, _LDO_TRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LDO_TRIM;
#[doc = "`read()` method returns [ldo_trim::R](ldo_trim::R) reader structure"]
impl crate::Readable for LDO_TRIM {}
#[doc = "`write(|w| ..)` method takes [ldo_trim::W](ldo_trim::W) writer structure"]
impl crate::Writable for LDO_TRIM {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ldo_trim;
#[doc = "MAC BLE Address 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_ble_0](mac_ble_0) module"]
pub type MAC_BLE_0 = crate::Reg<u32, _MAC_BLE_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_BLE_0;
#[doc = "`read()` method returns [mac_ble_0::R](mac_ble_0::R) reader structure"]
impl crate::Readable for MAC_BLE_0 {}
#[doc = "`write(|w| ..)` method takes [mac_ble_0::W](mac_ble_0::W) writer structure"]
impl crate::Writable for MAC_BLE_0 {}
#[doc = "MAC BLE Address 0"]
pub mod mac_ble_0;
#[doc = "MAC BLE Address 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_ble_1](mac_ble_1) module"]
pub type MAC_BLE_1 = crate::Reg<u32, _MAC_BLE_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_BLE_1;
#[doc = "`read()` method returns [mac_ble_1::R](mac_ble_1::R) reader structure"]
impl crate::Readable for MAC_BLE_1 {}
#[doc = "`write(|w| ..)` method takes [mac_ble_1::W](mac_ble_1::W) writer structure"]
impl crate::Writable for MAC_BLE_1 {}
#[doc = "MAC BLE Address 1"]
pub mod mac_ble_1;
#[doc = "MAC IEEE 802.15.4 Address 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_15_4_0](mac_15_4_0) module"]
pub type MAC_15_4_0 = crate::Reg<u32, _MAC_15_4_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_15_4_0;
#[doc = "`read()` method returns [mac_15_4_0::R](mac_15_4_0::R) reader structure"]
impl crate::Readable for MAC_15_4_0 {}
#[doc = "`write(|w| ..)` method takes [mac_15_4_0::W](mac_15_4_0::W) writer structure"]
impl crate::Writable for MAC_15_4_0 {}
#[doc = "MAC IEEE 802.15.4 Address 0"]
pub mod mac_15_4_0;
#[doc = "MAC IEEE 802.15.4 Address 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_15_4_1](mac_15_4_1) module"]
pub type MAC_15_4_1 = crate::Reg<u32, _MAC_15_4_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_15_4_1;
#[doc = "`read()` method returns [mac_15_4_1::R](mac_15_4_1::R) reader structure"]
impl crate::Readable for MAC_15_4_1 {}
#[doc = "`write(|w| ..)` method takes [mac_15_4_1::W](mac_15_4_1::W) writer structure"]
impl crate::Writable for MAC_15_4_1 {}
#[doc = "MAC IEEE 802.15.4 Address 1"]
pub mod mac_15_4_1;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_otp_data4](flash_otp_data4) module"]
pub type FLASH_OTP_DATA4 = crate::Reg<u32, _FLASH_OTP_DATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_OTP_DATA4;
#[doc = "`read()` method returns [flash_otp_data4::R](flash_otp_data4::R) reader structure"]
impl crate::Readable for FLASH_OTP_DATA4 {}
#[doc = "`write(|w| ..)` method takes [flash_otp_data4::W](flash_otp_data4::W) writer structure"]
impl crate::Writable for FLASH_OTP_DATA4 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_otp_data4;
#[doc = "Miscellaneous Trim Parameters\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_trim](misc_trim) module"]
pub type MISC_TRIM = crate::Reg<u32, _MISC_TRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC_TRIM;
#[doc = "`read()` method returns [misc_trim::R](misc_trim::R) reader structure"]
impl crate::Readable for MISC_TRIM {}
#[doc = "`write(|w| ..)` method takes [misc_trim::W](misc_trim::W) writer structure"]
impl crate::Writable for MISC_TRIM {}
#[doc = "Miscellaneous Trim Parameters"]
pub mod misc_trim;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcosc_hf_tempcomp](rcosc_hf_tempcomp) module"]
pub type RCOSC_HF_TEMPCOMP = crate::Reg<u32, _RCOSC_HF_TEMPCOMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCOSC_HF_TEMPCOMP;
#[doc = "`read()` method returns [rcosc_hf_tempcomp::R](rcosc_hf_tempcomp::R) reader structure"]
impl crate::Readable for RCOSC_HF_TEMPCOMP {}
#[doc = "`write(|w| ..)` method takes [rcosc_hf_tempcomp::W](rcosc_hf_tempcomp::W) writer structure"]
impl crate::Writable for RCOSC_HF_TEMPCOMP {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod rcosc_hf_tempcomp;
#[doc = "IcePick Device Identification Reading this register and the FCFG1:USER_ID register is the only supported way of identifying a device.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icepick_device_id](icepick_device_id) module"]
pub type ICEPICK_DEVICE_ID = crate::Reg<u32, _ICEPICK_DEVICE_ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICEPICK_DEVICE_ID;
#[doc = "`read()` method returns [icepick_device_id::R](icepick_device_id::R) reader structure"]
impl crate::Readable for ICEPICK_DEVICE_ID {}
#[doc = "`write(|w| ..)` method takes [icepick_device_id::W](icepick_device_id::W) writer structure"]
impl crate::Writable for ICEPICK_DEVICE_ID {}
#[doc = "IcePick Device Identification Reading this register and the FCFG1:USER_ID register is the only supported way of identifying a device."]
pub mod icepick_device_id;
#[doc = "Factory Configuration (FCFG1) Revision\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfg1_revision](fcfg1_revision) module"]
pub type FCFG1_REVISION = crate::Reg<u32, _FCFG1_REVISION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCFG1_REVISION;
#[doc = "`read()` method returns [fcfg1_revision::R](fcfg1_revision::R) reader structure"]
impl crate::Readable for FCFG1_REVISION {}
#[doc = "`write(|w| ..)` method takes [fcfg1_revision::W](fcfg1_revision::W) writer structure"]
impl crate::Writable for FCFG1_REVISION {}
#[doc = "Factory Configuration (FCFG1) Revision"]
pub mod fcfg1_revision;
#[doc = "Misc OTP Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_otp_data](misc_otp_data) module"]
pub type MISC_OTP_DATA = crate::Reg<u32, _MISC_OTP_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC_OTP_DATA;
#[doc = "`read()` method returns [misc_otp_data::R](misc_otp_data::R) reader structure"]
impl crate::Readable for MISC_OTP_DATA {}
#[doc = "`write(|w| ..)` method takes [misc_otp_data::W](misc_otp_data::W) writer structure"]
impl crate::Writable for MISC_OTP_DATA {}
#[doc = "Misc OTP Data"]
pub mod misc_otp_data;
#[doc = "IO Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioconf](ioconf) module"]
pub type IOCONF = crate::Reg<u32, _IOCONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCONF;
#[doc = "`read()` method returns [ioconf::R](ioconf::R) reader structure"]
impl crate::Readable for IOCONF {}
#[doc = "`write(|w| ..)` method takes [ioconf::W](ioconf::W) writer structure"]
impl crate::Writable for IOCONF {}
#[doc = "IO Configuration"]
pub mod ioconf;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_if_adc](config_if_adc) module"]
pub type CONFIG_IF_ADC = crate::Reg<u32, _CONFIG_IF_ADC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_IF_ADC;
#[doc = "`read()` method returns [config_if_adc::R](config_if_adc::R) reader structure"]
impl crate::Readable for CONFIG_IF_ADC {}
#[doc = "`write(|w| ..)` method takes [config_if_adc::W](config_if_adc::W) writer structure"]
impl crate::Writable for CONFIG_IF_ADC {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_if_adc;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_osc_top](config_osc_top) module"]
pub type CONFIG_OSC_TOP = crate::Reg<u32, _CONFIG_OSC_TOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_OSC_TOP;
#[doc = "`read()` method returns [config_osc_top::R](config_osc_top::R) reader structure"]
impl crate::Readable for CONFIG_OSC_TOP {}
#[doc = "`write(|w| ..)` method takes [config_osc_top::W](config_osc_top::W) writer structure"]
impl crate::Writable for CONFIG_OSC_TOP {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_osc_top;
#[doc = "AUX_ADC Gain in Absolute Reference Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soc_adc_abs_gain](soc_adc_abs_gain) module"]
pub type SOC_ADC_ABS_GAIN = crate::Reg<u32, _SOC_ADC_ABS_GAIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOC_ADC_ABS_GAIN;
#[doc = "`read()` method returns [soc_adc_abs_gain::R](soc_adc_abs_gain::R) reader structure"]
impl crate::Readable for SOC_ADC_ABS_GAIN {}
#[doc = "`write(|w| ..)` method takes [soc_adc_abs_gain::W](soc_adc_abs_gain::W) writer structure"]
impl crate::Writable for SOC_ADC_ABS_GAIN {}
#[doc = "AUX_ADC Gain in Absolute Reference Mode"]
pub mod soc_adc_abs_gain;
#[doc = "AUX_ADC Gain in Relative Reference Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soc_adc_rel_gain](soc_adc_rel_gain) module"]
pub type SOC_ADC_REL_GAIN = crate::Reg<u32, _SOC_ADC_REL_GAIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOC_ADC_REL_GAIN;
#[doc = "`read()` method returns [soc_adc_rel_gain::R](soc_adc_rel_gain::R) reader structure"]
impl crate::Readable for SOC_ADC_REL_GAIN {}
#[doc = "`write(|w| ..)` method takes [soc_adc_rel_gain::W](soc_adc_rel_gain::W) writer structure"]
impl crate::Writable for SOC_ADC_REL_GAIN {}
#[doc = "AUX_ADC Gain in Relative Reference Mode"]
pub mod soc_adc_rel_gain;
#[doc = "AUX_ADC Temperature Offsets in Absolute Reference Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soc_adc_offset_int](soc_adc_offset_int) module"]
pub type SOC_ADC_OFFSET_INT = crate::Reg<u32, _SOC_ADC_OFFSET_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOC_ADC_OFFSET_INT;
#[doc = "`read()` method returns [soc_adc_offset_int::R](soc_adc_offset_int::R) reader structure"]
impl crate::Readable for SOC_ADC_OFFSET_INT {}
#[doc = "`write(|w| ..)` method takes [soc_adc_offset_int::W](soc_adc_offset_int::W) writer structure"]
impl crate::Writable for SOC_ADC_OFFSET_INT {}
#[doc = "AUX_ADC Temperature Offsets in Absolute Reference Mode"]
pub mod soc_adc_offset_int;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soc_adc_ref_trim_and_offset_ext](soc_adc_ref_trim_and_offset_ext) module"]
pub type SOC_ADC_REF_TRIM_AND_OFFSET_EXT = crate::Reg<u32, _SOC_ADC_REF_TRIM_AND_OFFSET_EXT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOC_ADC_REF_TRIM_AND_OFFSET_EXT;
#[doc = "`read()` method returns [soc_adc_ref_trim_and_offset_ext::R](soc_adc_ref_trim_and_offset_ext::R) reader structure"]
impl crate::Readable for SOC_ADC_REF_TRIM_AND_OFFSET_EXT {}
#[doc = "`write(|w| ..)` method takes [soc_adc_ref_trim_and_offset_ext::W](soc_adc_ref_trim_and_offset_ext::W) writer structure"]
impl crate::Writable for SOC_ADC_REF_TRIM_AND_OFFSET_EXT {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod soc_adc_ref_trim_and_offset_ext;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ampcomp_th1](ampcomp_th1) module"]
pub type AMPCOMP_TH1 = crate::Reg<u32, _AMPCOMP_TH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AMPCOMP_TH1;
#[doc = "`read()` method returns [ampcomp_th1::R](ampcomp_th1::R) reader structure"]
impl crate::Readable for AMPCOMP_TH1 {}
#[doc = "`write(|w| ..)` method takes [ampcomp_th1::W](ampcomp_th1::W) writer structure"]
impl crate::Writable for AMPCOMP_TH1 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ampcomp_th1;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ampcomp_th2](ampcomp_th2) module"]
pub type AMPCOMP_TH2 = crate::Reg<u32, _AMPCOMP_TH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AMPCOMP_TH2;
#[doc = "`read()` method returns [ampcomp_th2::R](ampcomp_th2::R) reader structure"]
impl crate::Readable for AMPCOMP_TH2 {}
#[doc = "`write(|w| ..)` method takes [ampcomp_th2::W](ampcomp_th2::W) writer structure"]
impl crate::Writable for AMPCOMP_TH2 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ampcomp_th2;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ampcomp_ctrl1](ampcomp_ctrl1) module"]
pub type AMPCOMP_CTRL1 = crate::Reg<u32, _AMPCOMP_CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AMPCOMP_CTRL1;
#[doc = "`read()` method returns [ampcomp_ctrl1::R](ampcomp_ctrl1::R) reader structure"]
impl crate::Readable for AMPCOMP_CTRL1 {}
#[doc = "`write(|w| ..)` method takes [ampcomp_ctrl1::W](ampcomp_ctrl1::W) writer structure"]
impl crate::Writable for AMPCOMP_CTRL1 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ampcomp_ctrl1;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [anabypass_value2](anabypass_value2) module"]
pub type ANABYPASS_VALUE2 = crate::Reg<u32, _ANABYPASS_VALUE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANABYPASS_VALUE2;
#[doc = "`read()` method returns [anabypass_value2::R](anabypass_value2::R) reader structure"]
impl crate::Readable for ANABYPASS_VALUE2 {}
#[doc = "`write(|w| ..)` method takes [anabypass_value2::W](anabypass_value2::W) writer structure"]
impl crate::Writable for ANABYPASS_VALUE2 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod anabypass_value2;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [volt_trim](volt_trim) module"]
pub type VOLT_TRIM = crate::Reg<u32, _VOLT_TRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VOLT_TRIM;
#[doc = "`read()` method returns [volt_trim::R](volt_trim::R) reader structure"]
impl crate::Readable for VOLT_TRIM {}
#[doc = "`write(|w| ..)` method takes [volt_trim::W](volt_trim::W) writer structure"]
impl crate::Writable for VOLT_TRIM {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod volt_trim;
#[doc = "OSC Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc_conf](osc_conf) module"]
pub type OSC_CONF = crate::Reg<u32, _OSC_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSC_CONF;
#[doc = "`read()` method returns [osc_conf::R](osc_conf::R) reader structure"]
impl crate::Readable for OSC_CONF {}
#[doc = "`write(|w| ..)` method takes [osc_conf::W](osc_conf::W) writer structure"]
impl crate::Writable for OSC_CONF {}
#[doc = "OSC Configuration"]
pub mod osc_conf;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [freq_offset](freq_offset) module"]
pub type FREQ_OFFSET = crate::Reg<u32, _FREQ_OFFSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FREQ_OFFSET;
#[doc = "`read()` method returns [freq_offset::R](freq_offset::R) reader structure"]
impl crate::Readable for FREQ_OFFSET {}
#[doc = "`write(|w| ..)` method takes [freq_offset::W](freq_offset::W) writer structure"]
impl crate::Writable for FREQ_OFFSET {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod freq_offset;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_otp_data_1](misc_otp_data_1) module"]
pub type MISC_OTP_DATA_1 = crate::Reg<u32, _MISC_OTP_DATA_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC_OTP_DATA_1;
#[doc = "`read()` method returns [misc_otp_data_1::R](misc_otp_data_1::R) reader structure"]
impl crate::Readable for MISC_OTP_DATA_1 {}
#[doc = "`write(|w| ..)` method takes [misc_otp_data_1::W](misc_otp_data_1::W) writer structure"]
impl crate::Writable for MISC_OTP_DATA_1 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod misc_otp_data_1;
#[doc = "Shadow of DIE_ID_0 register in eFuse\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shdw_die_id_0](shdw_die_id_0) module"]
pub type SHDW_DIE_ID_0 = crate::Reg<u32, _SHDW_DIE_ID_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHDW_DIE_ID_0;
#[doc = "`read()` method returns [shdw_die_id_0::R](shdw_die_id_0::R) reader structure"]
impl crate::Readable for SHDW_DIE_ID_0 {}
#[doc = "`write(|w| ..)` method takes [shdw_die_id_0::W](shdw_die_id_0::W) writer structure"]
impl crate::Writable for SHDW_DIE_ID_0 {}
#[doc = "Shadow of DIE_ID_0 register in eFuse"]
pub mod shdw_die_id_0;
#[doc = "Shadow of DIE_ID_1 register in eFuse\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shdw_die_id_1](shdw_die_id_1) module"]
pub type SHDW_DIE_ID_1 = crate::Reg<u32, _SHDW_DIE_ID_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHDW_DIE_ID_1;
#[doc = "`read()` method returns [shdw_die_id_1::R](shdw_die_id_1::R) reader structure"]
impl crate::Readable for SHDW_DIE_ID_1 {}
#[doc = "`write(|w| ..)` method takes [shdw_die_id_1::W](shdw_die_id_1::W) writer structure"]
impl crate::Writable for SHDW_DIE_ID_1 {}
#[doc = "Shadow of DIE_ID_1 register in eFuse"]
pub mod shdw_die_id_1;
#[doc = "Shadow of DIE_ID_2 register in eFuse\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shdw_die_id_2](shdw_die_id_2) module"]
pub type SHDW_DIE_ID_2 = crate::Reg<u32, _SHDW_DIE_ID_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHDW_DIE_ID_2;
#[doc = "`read()` method returns [shdw_die_id_2::R](shdw_die_id_2::R) reader structure"]
impl crate::Readable for SHDW_DIE_ID_2 {}
#[doc = "`write(|w| ..)` method takes [shdw_die_id_2::W](shdw_die_id_2::W) writer structure"]
impl crate::Writable for SHDW_DIE_ID_2 {}
#[doc = "Shadow of DIE_ID_2 register in eFuse"]
pub mod shdw_die_id_2;
#[doc = "Shadow of DIE_ID_3 register in eFuse\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shdw_die_id_3](shdw_die_id_3) module"]
pub type SHDW_DIE_ID_3 = crate::Reg<u32, _SHDW_DIE_ID_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHDW_DIE_ID_3;
#[doc = "`read()` method returns [shdw_die_id_3::R](shdw_die_id_3::R) reader structure"]
impl crate::Readable for SHDW_DIE_ID_3 {}
#[doc = "`write(|w| ..)` method takes [shdw_die_id_3::W](shdw_die_id_3::W) writer structure"]
impl crate::Writable for SHDW_DIE_ID_3 {}
#[doc = "Shadow of DIE_ID_3 register in eFuse"]
pub mod shdw_die_id_3;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shdw_osc_bias_ldo_trim](shdw_osc_bias_ldo_trim) module"]
pub type SHDW_OSC_BIAS_LDO_TRIM = crate::Reg<u32, _SHDW_OSC_BIAS_LDO_TRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHDW_OSC_BIAS_LDO_TRIM;
#[doc = "`read()` method returns [shdw_osc_bias_ldo_trim::R](shdw_osc_bias_ldo_trim::R) reader structure"]
impl crate::Readable for SHDW_OSC_BIAS_LDO_TRIM {}
#[doc = "`write(|w| ..)` method takes [shdw_osc_bias_ldo_trim::W](shdw_osc_bias_ldo_trim::W) writer structure"]
impl crate::Writable for SHDW_OSC_BIAS_LDO_TRIM {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod shdw_osc_bias_ldo_trim;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shdw_ana_trim](shdw_ana_trim) module"]
pub type SHDW_ANA_TRIM = crate::Reg<u32, _SHDW_ANA_TRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHDW_ANA_TRIM;
#[doc = "`read()` method returns [shdw_ana_trim::R](shdw_ana_trim::R) reader structure"]
impl crate::Readable for SHDW_ANA_TRIM {}
#[doc = "`write(|w| ..)` method takes [shdw_ana_trim::W](shdw_ana_trim::W) writer structure"]
impl crate::Writable for SHDW_ANA_TRIM {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod shdw_ana_trim;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_bias_cnf](dac_bias_cnf) module"]
pub type DAC_BIAS_CNF = crate::Reg<u32, _DAC_BIAS_CNF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC_BIAS_CNF;
#[doc = "`read()` method returns [dac_bias_cnf::R](dac_bias_cnf::R) reader structure"]
impl crate::Readable for DAC_BIAS_CNF {}
#[doc = "`write(|w| ..)` method takes [dac_bias_cnf::W](dac_bias_cnf::W) writer structure"]
impl crate::Writable for DAC_BIAS_CNF {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod dac_bias_cnf;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tfw_probe](tfw_probe) module"]
pub type TFW_PROBE = crate::Reg<u32, _TFW_PROBE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TFW_PROBE;
#[doc = "`read()` method returns [tfw_probe::R](tfw_probe::R) reader structure"]
impl crate::Readable for TFW_PROBE {}
#[doc = "`write(|w| ..)` method takes [tfw_probe::W](tfw_probe::W) writer structure"]
impl crate::Writable for TFW_PROBE {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod tfw_probe;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tfw_ft](tfw_ft) module"]
pub type TFW_FT = crate::Reg<u32, _TFW_FT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TFW_FT;
#[doc = "`read()` method returns [tfw_ft::R](tfw_ft::R) reader structure"]
impl crate::Readable for TFW_FT {}
#[doc = "`write(|w| ..)` method takes [tfw_ft::W](tfw_ft::W) writer structure"]
impl crate::Writable for TFW_FT {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod tfw_ft;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_cal0](dac_cal0) module"]
pub type DAC_CAL0 = crate::Reg<u32, _DAC_CAL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC_CAL0;
#[doc = "`read()` method returns [dac_cal0::R](dac_cal0::R) reader structure"]
impl crate::Readable for DAC_CAL0 {}
#[doc = "`write(|w| ..)` method takes [dac_cal0::W](dac_cal0::W) writer structure"]
impl crate::Writable for DAC_CAL0 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod dac_cal0;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_cal1](dac_cal1) module"]
pub type DAC_CAL1 = crate::Reg<u32, _DAC_CAL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC_CAL1;
#[doc = "`read()` method returns [dac_cal1::R](dac_cal1::R) reader structure"]
impl crate::Readable for DAC_CAL1 {}
#[doc = "`write(|w| ..)` method takes [dac_cal1::W](dac_cal1::W) writer structure"]
impl crate::Writable for DAC_CAL1 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod dac_cal1;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_cal2](dac_cal2) module"]
pub type DAC_CAL2 = crate::Reg<u32, _DAC_CAL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC_CAL2;
#[doc = "`read()` method returns [dac_cal2::R](dac_cal2::R) reader structure"]
impl crate::Readable for DAC_CAL2 {}
#[doc = "`write(|w| ..)` method takes [dac_cal2::W](dac_cal2::W) writer structure"]
impl crate::Writable for DAC_CAL2 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod dac_cal2;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_cal3](dac_cal3) module"]
pub type DAC_CAL3 = crate::Reg<u32, _DAC_CAL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC_CAL3;
#[doc = "`read()` method returns [dac_cal3::R](dac_cal3::R) reader structure"]
impl crate::Readable for DAC_CAL3 {}
#[doc = "`write(|w| ..)` method takes [dac_cal3::W](dac_cal3::W) writer structure"]
impl crate::Writable for DAC_CAL3 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod dac_cal3;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved_n](reserved_n) module"]
pub type RESERVED_N = crate::Reg<u32, _RESERVED_N>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED_N;
#[doc = "`read()` method returns [reserved_n::R](reserved_n::R) reader structure"]
impl crate::Readable for RESERVED_N {}
#[doc = "`write(|w| ..)` method takes [reserved_n::W](reserved_n::W) writer structure"]
impl crate::Writable for RESERVED_N {}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved_n;
