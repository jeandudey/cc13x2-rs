#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved_0: RESERVED_0,
    _reserved1: [u8; 8100usize],
    #[doc = "0x1fa8 - Extern LF clock configuration"]
    pub ext_lf_clk: EXT_LF_CLK,
    #[doc = "0x1fac - Mode Configuration 1"]
    pub mode_conf_1: MODE_CONF_1,
    #[doc = "0x1fb0 - CCFG Size and Disable Flags"]
    pub size_and_dis_flags: SIZE_AND_DIS_FLAGS,
    #[doc = "0x1fb4 - Mode Configuration 0"]
    pub mode_conf: MODE_CONF,
    #[doc = "0x1fb8 - Voltage Load 0 Enabled by MODE_CONF.VDDR_EXT_LOAD."]
    pub volt_load_0: VOLT_LOAD_0,
    #[doc = "0x1fbc - Voltage Load 1 Enabled by MODE_CONF.VDDR_EXT_LOAD."]
    pub volt_load_1: VOLT_LOAD_1,
    #[doc = "0x1fc0 - Real Time Clock Offset Enabled by MODE_CONF.RTC_COMP."]
    pub rtc_offset: RTC_OFFSET,
    #[doc = "0x1fc4 - Frequency Offset"]
    pub freq_offset: FREQ_OFFSET,
    #[doc = "0x1fc8 - IEEE MAC Address 0"]
    pub ieee_mac_0: IEEE_MAC_0,
    #[doc = "0x1fcc - IEEE MAC Address 1"]
    pub ieee_mac_1: IEEE_MAC_1,
    #[doc = "0x1fd0 - IEEE BLE Address 0"]
    pub ieee_ble_0: IEEE_BLE_0,
    #[doc = "0x1fd4 - IEEE BLE Address 1"]
    pub ieee_ble_1: IEEE_BLE_1,
    #[doc = "0x1fd8 - Bootloader Configuration Configures the functionality of the ROM boot loader. If both the boot loader is enabled by the BOOTLOADER_ENABLE field and the boot loader backdoor is enabled by the BL_ENABLE field it is possible to force entry of the ROM boot loader even if a valid image is present in flash."]
    pub bl_config: BL_CONFIG,
    #[doc = "0x1fdc - Erase Configuration"]
    pub erase_conf: ERASE_CONF,
    #[doc = "0x1fe0 - TI Options"]
    pub ccfg_ti_options: CCFG_TI_OPTIONS,
    #[doc = "0x1fe4 - Test Access Points Enable 0"]
    pub ccfg_tap_dap_0: CCFG_TAP_DAP_0,
    #[doc = "0x1fe8 - Test Access Points Enable 1"]
    pub ccfg_tap_dap_1: CCFG_TAP_DAP_1,
    #[doc = "0x1fec - Image Valid"]
    pub image_valid_conf: IMAGE_VALID_CONF,
    #[doc = "0x1ff0 - Protect Sectors 0-31 Each bit write protects one 8KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect."]
    pub ccfg_prot_31_0: CCFG_PROT_31_0,
    #[doc = "0x1ff4 - Protect Sectors 32-63 Each bit write protects one 8KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect."]
    pub ccfg_prot_63_32: CCFG_PROT_63_32,
    #[doc = "0x1ff8 - Protect Sectors 64-95 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use."]
    pub ccfg_prot_95_64: CCFG_PROT_95_64,
    #[doc = "0x1ffc - Protect Sectors 96-127 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use."]
    pub ccfg_prot_127_96: CCFG_PROT_127_96,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved_0](reserved_0) module"]
pub type RESERVED_0 = crate::Reg<u32, _RESERVED_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED_0;
#[doc = "`read()` method returns [reserved_0::R](reserved_0::R) reader structure"]
impl crate::Readable for RESERVED_0 {}
#[doc = "`write(|w| ..)` method takes [reserved_0::W](reserved_0::W) writer structure"]
impl crate::Writable for RESERVED_0 {}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved_0;
#[doc = "Extern LF clock configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_lf_clk](ext_lf_clk) module"]
pub type EXT_LF_CLK = crate::Reg<u32, _EXT_LF_CLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXT_LF_CLK;
#[doc = "`read()` method returns [ext_lf_clk::R](ext_lf_clk::R) reader structure"]
impl crate::Readable for EXT_LF_CLK {}
#[doc = "`write(|w| ..)` method takes [ext_lf_clk::W](ext_lf_clk::W) writer structure"]
impl crate::Writable for EXT_LF_CLK {}
#[doc = "Extern LF clock configuration"]
pub mod ext_lf_clk;
#[doc = "Mode Configuration 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode_conf_1](mode_conf_1) module"]
pub type MODE_CONF_1 = crate::Reg<u32, _MODE_CONF_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE_CONF_1;
#[doc = "`read()` method returns [mode_conf_1::R](mode_conf_1::R) reader structure"]
impl crate::Readable for MODE_CONF_1 {}
#[doc = "`write(|w| ..)` method takes [mode_conf_1::W](mode_conf_1::W) writer structure"]
impl crate::Writable for MODE_CONF_1 {}
#[doc = "Mode Configuration 1"]
pub mod mode_conf_1;
#[doc = "CCFG Size and Disable Flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [size_and_dis_flags](size_and_dis_flags) module"]
pub type SIZE_AND_DIS_FLAGS = crate::Reg<u32, _SIZE_AND_DIS_FLAGS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIZE_AND_DIS_FLAGS;
#[doc = "`read()` method returns [size_and_dis_flags::R](size_and_dis_flags::R) reader structure"]
impl crate::Readable for SIZE_AND_DIS_FLAGS {}
#[doc = "`write(|w| ..)` method takes [size_and_dis_flags::W](size_and_dis_flags::W) writer structure"]
impl crate::Writable for SIZE_AND_DIS_FLAGS {}
#[doc = "CCFG Size and Disable Flags"]
pub mod size_and_dis_flags;
#[doc = "Mode Configuration 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode_conf](mode_conf) module"]
pub type MODE_CONF = crate::Reg<u32, _MODE_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE_CONF;
#[doc = "`read()` method returns [mode_conf::R](mode_conf::R) reader structure"]
impl crate::Readable for MODE_CONF {}
#[doc = "`write(|w| ..)` method takes [mode_conf::W](mode_conf::W) writer structure"]
impl crate::Writable for MODE_CONF {}
#[doc = "Mode Configuration 0"]
pub mod mode_conf;
#[doc = "Voltage Load 0 Enabled by MODE_CONF.VDDR_EXT_LOAD.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [volt_load_0](volt_load_0) module"]
pub type VOLT_LOAD_0 = crate::Reg<u32, _VOLT_LOAD_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VOLT_LOAD_0;
#[doc = "`read()` method returns [volt_load_0::R](volt_load_0::R) reader structure"]
impl crate::Readable for VOLT_LOAD_0 {}
#[doc = "`write(|w| ..)` method takes [volt_load_0::W](volt_load_0::W) writer structure"]
impl crate::Writable for VOLT_LOAD_0 {}
#[doc = "Voltage Load 0 Enabled by MODE_CONF.VDDR_EXT_LOAD."]
pub mod volt_load_0;
#[doc = "Voltage Load 1 Enabled by MODE_CONF.VDDR_EXT_LOAD.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [volt_load_1](volt_load_1) module"]
pub type VOLT_LOAD_1 = crate::Reg<u32, _VOLT_LOAD_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VOLT_LOAD_1;
#[doc = "`read()` method returns [volt_load_1::R](volt_load_1::R) reader structure"]
impl crate::Readable for VOLT_LOAD_1 {}
#[doc = "`write(|w| ..)` method takes [volt_load_1::W](volt_load_1::W) writer structure"]
impl crate::Writable for VOLT_LOAD_1 {}
#[doc = "Voltage Load 1 Enabled by MODE_CONF.VDDR_EXT_LOAD."]
pub mod volt_load_1;
#[doc = "Real Time Clock Offset Enabled by MODE_CONF.RTC_COMP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_offset](rtc_offset) module"]
pub type RTC_OFFSET = crate::Reg<u32, _RTC_OFFSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_OFFSET;
#[doc = "`read()` method returns [rtc_offset::R](rtc_offset::R) reader structure"]
impl crate::Readable for RTC_OFFSET {}
#[doc = "`write(|w| ..)` method takes [rtc_offset::W](rtc_offset::W) writer structure"]
impl crate::Writable for RTC_OFFSET {}
#[doc = "Real Time Clock Offset Enabled by MODE_CONF.RTC_COMP."]
pub mod rtc_offset;
#[doc = "Frequency Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [freq_offset](freq_offset) module"]
pub type FREQ_OFFSET = crate::Reg<u32, _FREQ_OFFSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FREQ_OFFSET;
#[doc = "`read()` method returns [freq_offset::R](freq_offset::R) reader structure"]
impl crate::Readable for FREQ_OFFSET {}
#[doc = "`write(|w| ..)` method takes [freq_offset::W](freq_offset::W) writer structure"]
impl crate::Writable for FREQ_OFFSET {}
#[doc = "Frequency Offset"]
pub mod freq_offset;
#[doc = "IEEE MAC Address 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ieee_mac_0](ieee_mac_0) module"]
pub type IEEE_MAC_0 = crate::Reg<u32, _IEEE_MAC_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEEE_MAC_0;
#[doc = "`read()` method returns [ieee_mac_0::R](ieee_mac_0::R) reader structure"]
impl crate::Readable for IEEE_MAC_0 {}
#[doc = "`write(|w| ..)` method takes [ieee_mac_0::W](ieee_mac_0::W) writer structure"]
impl crate::Writable for IEEE_MAC_0 {}
#[doc = "IEEE MAC Address 0"]
pub mod ieee_mac_0;
#[doc = "IEEE MAC Address 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ieee_mac_1](ieee_mac_1) module"]
pub type IEEE_MAC_1 = crate::Reg<u32, _IEEE_MAC_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEEE_MAC_1;
#[doc = "`read()` method returns [ieee_mac_1::R](ieee_mac_1::R) reader structure"]
impl crate::Readable for IEEE_MAC_1 {}
#[doc = "`write(|w| ..)` method takes [ieee_mac_1::W](ieee_mac_1::W) writer structure"]
impl crate::Writable for IEEE_MAC_1 {}
#[doc = "IEEE MAC Address 1"]
pub mod ieee_mac_1;
#[doc = "IEEE BLE Address 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ieee_ble_0](ieee_ble_0) module"]
pub type IEEE_BLE_0 = crate::Reg<u32, _IEEE_BLE_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEEE_BLE_0;
#[doc = "`read()` method returns [ieee_ble_0::R](ieee_ble_0::R) reader structure"]
impl crate::Readable for IEEE_BLE_0 {}
#[doc = "`write(|w| ..)` method takes [ieee_ble_0::W](ieee_ble_0::W) writer structure"]
impl crate::Writable for IEEE_BLE_0 {}
#[doc = "IEEE BLE Address 0"]
pub mod ieee_ble_0;
#[doc = "IEEE BLE Address 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ieee_ble_1](ieee_ble_1) module"]
pub type IEEE_BLE_1 = crate::Reg<u32, _IEEE_BLE_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEEE_BLE_1;
#[doc = "`read()` method returns [ieee_ble_1::R](ieee_ble_1::R) reader structure"]
impl crate::Readable for IEEE_BLE_1 {}
#[doc = "`write(|w| ..)` method takes [ieee_ble_1::W](ieee_ble_1::W) writer structure"]
impl crate::Writable for IEEE_BLE_1 {}
#[doc = "IEEE BLE Address 1"]
pub mod ieee_ble_1;
#[doc = "Bootloader Configuration Configures the functionality of the ROM boot loader. If both the boot loader is enabled by the BOOTLOADER_ENABLE field and the boot loader backdoor is enabled by the BL_ENABLE field it is possible to force entry of the ROM boot loader even if a valid image is present in flash.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bl_config](bl_config) module"]
pub type BL_CONFIG = crate::Reg<u32, _BL_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BL_CONFIG;
#[doc = "`read()` method returns [bl_config::R](bl_config::R) reader structure"]
impl crate::Readable for BL_CONFIG {}
#[doc = "`write(|w| ..)` method takes [bl_config::W](bl_config::W) writer structure"]
impl crate::Writable for BL_CONFIG {}
#[doc = "Bootloader Configuration Configures the functionality of the ROM boot loader. If both the boot loader is enabled by the BOOTLOADER_ENABLE field and the boot loader backdoor is enabled by the BL_ENABLE field it is possible to force entry of the ROM boot loader even if a valid image is present in flash."]
pub mod bl_config;
#[doc = "Erase Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erase_conf](erase_conf) module"]
pub type ERASE_CONF = crate::Reg<u32, _ERASE_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERASE_CONF;
#[doc = "`read()` method returns [erase_conf::R](erase_conf::R) reader structure"]
impl crate::Readable for ERASE_CONF {}
#[doc = "`write(|w| ..)` method takes [erase_conf::W](erase_conf::W) writer structure"]
impl crate::Writable for ERASE_CONF {}
#[doc = "Erase Configuration"]
pub mod erase_conf;
#[doc = "TI Options\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_ti_options](ccfg_ti_options) module"]
pub type CCFG_TI_OPTIONS = crate::Reg<u32, _CCFG_TI_OPTIONS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCFG_TI_OPTIONS;
#[doc = "`read()` method returns [ccfg_ti_options::R](ccfg_ti_options::R) reader structure"]
impl crate::Readable for CCFG_TI_OPTIONS {}
#[doc = "`write(|w| ..)` method takes [ccfg_ti_options::W](ccfg_ti_options::W) writer structure"]
impl crate::Writable for CCFG_TI_OPTIONS {}
#[doc = "TI Options"]
pub mod ccfg_ti_options;
#[doc = "Test Access Points Enable 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_tap_dap_0](ccfg_tap_dap_0) module"]
pub type CCFG_TAP_DAP_0 = crate::Reg<u32, _CCFG_TAP_DAP_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCFG_TAP_DAP_0;
#[doc = "`read()` method returns [ccfg_tap_dap_0::R](ccfg_tap_dap_0::R) reader structure"]
impl crate::Readable for CCFG_TAP_DAP_0 {}
#[doc = "`write(|w| ..)` method takes [ccfg_tap_dap_0::W](ccfg_tap_dap_0::W) writer structure"]
impl crate::Writable for CCFG_TAP_DAP_0 {}
#[doc = "Test Access Points Enable 0"]
pub mod ccfg_tap_dap_0;
#[doc = "Test Access Points Enable 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_tap_dap_1](ccfg_tap_dap_1) module"]
pub type CCFG_TAP_DAP_1 = crate::Reg<u32, _CCFG_TAP_DAP_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCFG_TAP_DAP_1;
#[doc = "`read()` method returns [ccfg_tap_dap_1::R](ccfg_tap_dap_1::R) reader structure"]
impl crate::Readable for CCFG_TAP_DAP_1 {}
#[doc = "`write(|w| ..)` method takes [ccfg_tap_dap_1::W](ccfg_tap_dap_1::W) writer structure"]
impl crate::Writable for CCFG_TAP_DAP_1 {}
#[doc = "Test Access Points Enable 1"]
pub mod ccfg_tap_dap_1;
#[doc = "Image Valid\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [image_valid_conf](image_valid_conf) module"]
pub type IMAGE_VALID_CONF = crate::Reg<u32, _IMAGE_VALID_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMAGE_VALID_CONF;
#[doc = "`read()` method returns [image_valid_conf::R](image_valid_conf::R) reader structure"]
impl crate::Readable for IMAGE_VALID_CONF {}
#[doc = "`write(|w| ..)` method takes [image_valid_conf::W](image_valid_conf::W) writer structure"]
impl crate::Writable for IMAGE_VALID_CONF {}
#[doc = "Image Valid"]
pub mod image_valid_conf;
#[doc = "Protect Sectors 0-31 Each bit write protects one 8KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_prot_31_0](ccfg_prot_31_0) module"]
pub type CCFG_PROT_31_0 = crate::Reg<u32, _CCFG_PROT_31_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCFG_PROT_31_0;
#[doc = "`read()` method returns [ccfg_prot_31_0::R](ccfg_prot_31_0::R) reader structure"]
impl crate::Readable for CCFG_PROT_31_0 {}
#[doc = "`write(|w| ..)` method takes [ccfg_prot_31_0::W](ccfg_prot_31_0::W) writer structure"]
impl crate::Writable for CCFG_PROT_31_0 {}
#[doc = "Protect Sectors 0-31 Each bit write protects one 8KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect."]
pub mod ccfg_prot_31_0;
#[doc = "Protect Sectors 32-63 Each bit write protects one 8KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_prot_63_32](ccfg_prot_63_32) module"]
pub type CCFG_PROT_63_32 = crate::Reg<u32, _CCFG_PROT_63_32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCFG_PROT_63_32;
#[doc = "`read()` method returns [ccfg_prot_63_32::R](ccfg_prot_63_32::R) reader structure"]
impl crate::Readable for CCFG_PROT_63_32 {}
#[doc = "`write(|w| ..)` method takes [ccfg_prot_63_32::W](ccfg_prot_63_32::W) writer structure"]
impl crate::Writable for CCFG_PROT_63_32 {}
#[doc = "Protect Sectors 32-63 Each bit write protects one 8KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect."]
pub mod ccfg_prot_63_32;
#[doc = "Protect Sectors 64-95 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_prot_95_64](ccfg_prot_95_64) module"]
pub type CCFG_PROT_95_64 = crate::Reg<u32, _CCFG_PROT_95_64>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCFG_PROT_95_64;
#[doc = "`read()` method returns [ccfg_prot_95_64::R](ccfg_prot_95_64::R) reader structure"]
impl crate::Readable for CCFG_PROT_95_64 {}
#[doc = "`write(|w| ..)` method takes [ccfg_prot_95_64::W](ccfg_prot_95_64::W) writer structure"]
impl crate::Writable for CCFG_PROT_95_64 {}
#[doc = "Protect Sectors 64-95 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use."]
pub mod ccfg_prot_95_64;
#[doc = "Protect Sectors 96-127 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_prot_127_96](ccfg_prot_127_96) module"]
pub type CCFG_PROT_127_96 = crate::Reg<u32, _CCFG_PROT_127_96>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCFG_PROT_127_96;
#[doc = "`read()` method returns [ccfg_prot_127_96::R](ccfg_prot_127_96::R) reader structure"]
impl crate::Readable for CCFG_PROT_127_96 {}
#[doc = "`write(|w| ..)` method takes [ccfg_prot_127_96::W](ccfg_prot_127_96::W) writer structure"]
impl crate::Writable for CCFG_PROT_127_96 {}
#[doc = "Protect Sectors 96-127 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use."]
pub mod ccfg_prot_127_96;
