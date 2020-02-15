#[doc = "Reader of register MODE_CONF"]
pub type R = crate::R<u32, super::MODE_CONF>;
#[doc = "Writer for register MODE_CONF"]
pub type W = crate::W<u32, super::MODE_CONF>;
#[doc = "Register MODE_CONF `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MODE_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `VDDR_TRIM_SLEEP_DELTA`"]
pub type VDDR_TRIM_SLEEP_DELTA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VDDR_TRIM_SLEEP_DELTA`"]
pub struct VDDR_TRIM_SLEEP_DELTA_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_TRIM_SLEEP_DELTA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `DCDC_RECHARGE`"]
pub type DCDC_RECHARGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCDC_RECHARGE`"]
pub struct DCDC_RECHARGE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC_RECHARGE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `DCDC_ACTIVE`"]
pub type DCDC_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCDC_ACTIVE`"]
pub struct DCDC_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC_ACTIVE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `VDDR_EXT_LOAD`"]
pub type VDDR_EXT_LOAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDDR_EXT_LOAD`"]
pub struct VDDR_EXT_LOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_EXT_LOAD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `VDDS_BOD_LEVEL`"]
pub type VDDS_BOD_LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDDS_BOD_LEVEL`"]
pub struct VDDS_BOD_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDS_BOD_LEVEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "23:22\\]
Select source for SCLK_LF.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SCLK_LF_OPTION_A {
    #[doc = "3: Low frequency RCOSC (default)"]
    RCOSC_LF = 3,
    #[doc = "2: 32.768kHz low frequency XOSC"]
    XOSC_LF = 2,
    #[doc = "1: External low frequency clock on DIO defined by EXT_LF_CLK.DIO. The RTC tick speed AON_RTC:SUBSECINC is updated to EXT_LF_CLK.RTC_INCREMENT (done in the trimDevice() xxWare boot function). External clock must always be running when the chip is in standby for VDDR recharge timing."]
    EXTERNAL_LF = 1,
    #[doc = "0: 31.25kHz clock derived from 24MHz XOSC (dividing by 768 in HW). The RTC tick speed \\[AON_RTC.SUBSECINC.*\\]
is updated to 0x8637BD, corresponding to a 31.25kHz clock (done in the trimDevice() xxWare boot function). Standby power mode is not supported when using this clock source."]
    XOSC_HF_DLF = 0,
}
impl From<SCLK_LF_OPTION_A> for u8 {
    #[inline(always)]
    fn from(variant: SCLK_LF_OPTION_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SCLK_LF_OPTION`"]
pub type SCLK_LF_OPTION_R = crate::R<u8, SCLK_LF_OPTION_A>;
impl SCLK_LF_OPTION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLK_LF_OPTION_A {
        match self.bits {
            3 => SCLK_LF_OPTION_A::RCOSC_LF,
            2 => SCLK_LF_OPTION_A::XOSC_LF,
            1 => SCLK_LF_OPTION_A::EXTERNAL_LF,
            0 => SCLK_LF_OPTION_A::XOSC_HF_DLF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RCOSC_LF`"]
    #[inline(always)]
    pub fn is_rcosc_lf(&self) -> bool {
        *self == SCLK_LF_OPTION_A::RCOSC_LF
    }
    #[doc = "Checks if the value of the field is `XOSC_LF`"]
    #[inline(always)]
    pub fn is_xosc_lf(&self) -> bool {
        *self == SCLK_LF_OPTION_A::XOSC_LF
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_LF`"]
    #[inline(always)]
    pub fn is_external_lf(&self) -> bool {
        *self == SCLK_LF_OPTION_A::EXTERNAL_LF
    }
    #[doc = "Checks if the value of the field is `XOSC_HF_DLF`"]
    #[inline(always)]
    pub fn is_xosc_hf_dlf(&self) -> bool {
        *self == SCLK_LF_OPTION_A::XOSC_HF_DLF
    }
}
#[doc = "Write proxy for field `SCLK_LF_OPTION`"]
pub struct SCLK_LF_OPTION_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_LF_OPTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCLK_LF_OPTION_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Low frequency RCOSC (default)"]
    #[inline(always)]
    pub fn rcosc_lf(self) -> &'a mut W {
        self.variant(SCLK_LF_OPTION_A::RCOSC_LF)
    }
    #[doc = "32.768kHz low frequency XOSC"]
    #[inline(always)]
    pub fn xosc_lf(self) -> &'a mut W {
        self.variant(SCLK_LF_OPTION_A::XOSC_LF)
    }
    #[doc = "External low frequency clock on DIO defined by EXT_LF_CLK.DIO. The RTC tick speed AON_RTC:SUBSECINC is updated to EXT_LF_CLK.RTC_INCREMENT (done in the trimDevice() xxWare boot function). External clock must always be running when the chip is in standby for VDDR recharge timing."]
    #[inline(always)]
    pub fn external_lf(self) -> &'a mut W {
        self.variant(SCLK_LF_OPTION_A::EXTERNAL_LF)
    }
    #[doc = "31.25kHz clock derived from 24MHz XOSC (dividing by 768 in HW). The RTC tick speed \\[AON_RTC.SUBSECINC.*\\]
is updated to 0x8637BD, corresponding to a 31.25kHz clock (done in the trimDevice() xxWare boot function). Standby power mode is not supported when using this clock source."]
    #[inline(always)]
    pub fn xosc_hf_dlf(self) -> &'a mut W {
        self.variant(SCLK_LF_OPTION_A::XOSC_HF_DLF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `VDDR_TRIM_SLEEP_TC`"]
pub type VDDR_TRIM_SLEEP_TC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDDR_TRIM_SLEEP_TC`"]
pub struct VDDR_TRIM_SLEEP_TC_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_TRIM_SLEEP_TC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `RTC_COMP`"]
pub type RTC_COMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_COMP`"]
pub struct RTC_COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_COMP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "19:18\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum XOSC_FREQ_A {
    #[doc = "3: 24 MHz XOSC_HF"]
    _24M = 3,
    #[doc = "2: 48 MHz XOSC_HF"]
    _48M = 2,
    #[doc = "1: HPOSC"]
    HPOSC = 1,
}
impl From<XOSC_FREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: XOSC_FREQ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `XOSC_FREQ`"]
pub type XOSC_FREQ_R = crate::R<u8, XOSC_FREQ_A>;
impl XOSC_FREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, XOSC_FREQ_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(XOSC_FREQ_A::_24M),
            2 => Val(XOSC_FREQ_A::_48M),
            1 => Val(XOSC_FREQ_A::HPOSC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_24M`"]
    #[inline(always)]
    pub fn is_24m(&self) -> bool {
        *self == XOSC_FREQ_A::_24M
    }
    #[doc = "Checks if the value of the field is `_48M`"]
    #[inline(always)]
    pub fn is_48m(&self) -> bool {
        *self == XOSC_FREQ_A::_48M
    }
    #[doc = "Checks if the value of the field is `HPOSC`"]
    #[inline(always)]
    pub fn is_hposc(&self) -> bool {
        *self == XOSC_FREQ_A::HPOSC
    }
}
#[doc = "Write proxy for field `XOSC_FREQ`"]
pub struct XOSC_FREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_FREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XOSC_FREQ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "24 MHz XOSC_HF"]
    #[inline(always)]
    pub fn _24m(self) -> &'a mut W {
        self.variant(XOSC_FREQ_A::_24M)
    }
    #[doc = "48 MHz XOSC_HF"]
    #[inline(always)]
    pub fn _48m(self) -> &'a mut W {
        self.variant(XOSC_FREQ_A::_48M)
    }
    #[doc = "HPOSC"]
    #[inline(always)]
    pub fn hposc(self) -> &'a mut W {
        self.variant(XOSC_FREQ_A::HPOSC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `XOSC_CAP_MOD`"]
pub type XOSC_CAP_MOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSC_CAP_MOD`"]
pub struct XOSC_CAP_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_CAP_MOD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `HF_COMP`"]
pub type HF_COMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HF_COMP`"]
pub struct HF_COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> HF_COMP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `XOSC_CAPARRAY_DELTA`"]
pub type XOSC_CAPARRAY_DELTA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XOSC_CAPARRAY_DELTA`"]
pub struct XOSC_CAPARRAY_DELTA_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_CAPARRAY_DELTA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `VDDR_CAP`"]
pub type VDDR_CAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VDDR_CAP`"]
pub struct VDDR_CAP_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_CAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - 31:28\\]
Signed delta value to apply to the VDDR_TRIM_SLEEP target, minus one. See FCFG1:VOLT_TRIM.VDDR_TRIM_SLEEP_H. 0x8 (-8) : Delta = -7 ... 0xF (-1) : Delta = 0 0x0 (0) : Delta = +1 ... 0x7 (7) : Delta = +8"]
    #[inline(always)]
    pub fn vddr_trim_sleep_delta(&self) -> VDDR_TRIM_SLEEP_DELTA_R {
        VDDR_TRIM_SLEEP_DELTA_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bit 27 - 27:27\\]
DC/DC during recharge in powerdown. 0: Use the DC/DC during recharge in powerdown. 1: Do not use the DC/DC during recharge in powerdown (default). NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline(always)]
    pub fn dcdc_recharge(&self) -> DCDC_RECHARGE_R {
        DCDC_RECHARGE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
DC/DC in active mode. 0: Use the DC/DC during active mode. 1: Do not use the DC/DC during active mode (default). NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline(always)]
    pub fn dcdc_active(&self) -> DCDC_ACTIVE_R {
        DCDC_ACTIVE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_load(&self) -> VDDR_EXT_LOAD_R {
        VDDR_EXT_LOAD_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
VDDS BOD level. 0: VDDS BOD level is 2.0V (necessary for external load mode, or for maximum PA output power on CC13xx). 1: VDDS BOD level is 1.8V (or 1.65V for external regulator mode) (default)."]
    #[inline(always)]
    pub fn vdds_bod_level(&self) -> VDDS_BOD_LEVEL_R {
        VDDS_BOD_LEVEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Select source for SCLK_LF."]
    #[inline(always)]
    pub fn sclk_lf_option(&self) -> SCLK_LF_OPTION_R {
        SCLK_LF_OPTION_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
0x1: VDDR_TRIM_SLEEP_DELTA is not temperature compensated 0x0: RTOS/driver temperature compensates VDDR_TRIM_SLEEP_DELTA every time standby mode is entered. This improves low-temperature RCOSC_LF frequency stability in standby mode. When temperature compensation is performed, the delta is calculates this way: Delta = max (delta, min(8, floor(62-temp)/8)) Here, delta is given by VDDR_TRIM_SLEEP_DELTA, and temp is the current temperature in degrees C."]
    #[inline(always)]
    pub fn vddr_trim_sleep_tc(&self) -> VDDR_TRIM_SLEEP_TC_R {
        VDDR_TRIM_SLEEP_TC_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn rtc_comp(&self) -> RTC_COMP_R {
        RTC_COMP_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - 19:18\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn xosc_freq(&self) -> XOSC_FREQ_R {
        XOSC_FREQ_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 17 - 17:17\\]
Enable modification (delta) to XOSC cap-array. Value specified in XOSC_CAPARRAY_DELTA. 0: Apply cap-array delta 1: Do not apply cap-array delta (default)"]
    #[inline(always)]
    pub fn xosc_cap_mod(&self) -> XOSC_CAP_MOD_R {
        XOSC_CAP_MOD_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn hf_comp(&self) -> HF_COMP_R {
        HF_COMP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Signed 8-bit value, directly modifying trimmed XOSC cap-array step value. Enabled by XOSC_CAP_MOD."]
    #[inline(always)]
    pub fn xosc_caparray_delta(&self) -> XOSC_CAPARRAY_DELTA_R {
        XOSC_CAPARRAY_DELTA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Unsigned 8-bit integer, representing the minimum decoupling capacitance (worst case) on VDDR, in units of 100nF. This should take into account capacitor tolerance and voltage dependent capacitance variation. This bit affects the recharge period calculation when going into powerdown or standby. NOTE! If using the following functions this field must be configured (used by TI RTOS): SysCtrlSetRechargeBeforePowerDown() SysCtrlAdjustRechargeAfterPowerDown()"]
    #[inline(always)]
    pub fn vddr_cap(&self) -> VDDR_CAP_R {
        VDDR_CAP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - 31:28\\]
Signed delta value to apply to the VDDR_TRIM_SLEEP target, minus one. See FCFG1:VOLT_TRIM.VDDR_TRIM_SLEEP_H. 0x8 (-8) : Delta = -7 ... 0xF (-1) : Delta = 0 0x0 (0) : Delta = +1 ... 0x7 (7) : Delta = +8"]
    #[inline(always)]
    pub fn vddr_trim_sleep_delta(&mut self) -> VDDR_TRIM_SLEEP_DELTA_W {
        VDDR_TRIM_SLEEP_DELTA_W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\]
DC/DC during recharge in powerdown. 0: Use the DC/DC during recharge in powerdown. 1: Do not use the DC/DC during recharge in powerdown (default). NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline(always)]
    pub fn dcdc_recharge(&mut self) -> DCDC_RECHARGE_W {
        DCDC_RECHARGE_W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\]
DC/DC in active mode. 0: Use the DC/DC during active mode. 1: Do not use the DC/DC during active mode (default). NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline(always)]
    pub fn dcdc_active(&mut self) -> DCDC_ACTIVE_W {
        DCDC_ACTIVE_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_load(&mut self) -> VDDR_EXT_LOAD_W {
        VDDR_EXT_LOAD_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
VDDS BOD level. 0: VDDS BOD level is 2.0V (necessary for external load mode, or for maximum PA output power on CC13xx). 1: VDDS BOD level is 1.8V (or 1.65V for external regulator mode) (default)."]
    #[inline(always)]
    pub fn vdds_bod_level(&mut self) -> VDDS_BOD_LEVEL_W {
        VDDS_BOD_LEVEL_W { w: self }
    }
    #[doc = "Bits 22:23 - 23:22\\]
Select source for SCLK_LF."]
    #[inline(always)]
    pub fn sclk_lf_option(&mut self) -> SCLK_LF_OPTION_W {
        SCLK_LF_OPTION_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\]
0x1: VDDR_TRIM_SLEEP_DELTA is not temperature compensated 0x0: RTOS/driver temperature compensates VDDR_TRIM_SLEEP_DELTA every time standby mode is entered. This improves low-temperature RCOSC_LF frequency stability in standby mode. When temperature compensation is performed, the delta is calculates this way: Delta = max (delta, min(8, floor(62-temp)/8)) Here, delta is given by VDDR_TRIM_SLEEP_DELTA, and temp is the current temperature in degrees C."]
    #[inline(always)]
    pub fn vddr_trim_sleep_tc(&mut self) -> VDDR_TRIM_SLEEP_TC_W {
        VDDR_TRIM_SLEEP_TC_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn rtc_comp(&mut self) -> RTC_COMP_W {
        RTC_COMP_W { w: self }
    }
    #[doc = "Bits 18:19 - 19:18\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn xosc_freq(&mut self) -> XOSC_FREQ_W {
        XOSC_FREQ_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Enable modification (delta) to XOSC cap-array. Value specified in XOSC_CAPARRAY_DELTA. 0: Apply cap-array delta 1: Do not apply cap-array delta (default)"]
    #[inline(always)]
    pub fn xosc_cap_mod(&mut self) -> XOSC_CAP_MOD_W {
        XOSC_CAP_MOD_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn hf_comp(&mut self) -> HF_COMP_W {
        HF_COMP_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Signed 8-bit value, directly modifying trimmed XOSC cap-array step value. Enabled by XOSC_CAP_MOD."]
    #[inline(always)]
    pub fn xosc_caparray_delta(&mut self) -> XOSC_CAPARRAY_DELTA_W {
        XOSC_CAPARRAY_DELTA_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Unsigned 8-bit integer, representing the minimum decoupling capacitance (worst case) on VDDR, in units of 100nF. This should take into account capacitor tolerance and voltage dependent capacitance variation. This bit affects the recharge period calculation when going into powerdown or standby. NOTE! If using the following functions this field must be configured (used by TI RTOS): SysCtrlSetRechargeBeforePowerDown() SysCtrlAdjustRechargeAfterPowerDown()"]
    #[inline(always)]
    pub fn vddr_cap(&mut self) -> VDDR_CAP_W {
        VDDR_CAP_W { w: self }
    }
}
