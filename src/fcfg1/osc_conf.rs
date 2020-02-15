#[doc = "Reader of register OSC_CONF"]
pub type R = crate::R<u32, super::OSC_CONF>;
#[doc = "Writer for register OSC_CONF"]
pub type W = crate::W<u32, super::OSC_CONF>;
#[doc = "Register OSC_CONF `reset()`'s with value 0xf009_00e6"]
impl crate::ResetValue for super::OSC_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xf009_00e6
    }
}
#[doc = "Reader of field `RESERVED1`"]
pub type RESERVED1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `ADC_SH_VBUF_EN`"]
pub type ADC_SH_VBUF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SH_VBUF_EN`"]
pub struct ADC_SH_VBUF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SH_VBUF_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `ADC_SH_MODE_EN`"]
pub type ADC_SH_MODE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SH_MODE_EN`"]
pub struct ADC_SH_MODE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SH_MODE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `ATESTLF_RCOSCLF_IBIAS_TRIM`"]
pub type ATESTLF_RCOSCLF_IBIAS_TRIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ATESTLF_RCOSCLF_IBIAS_TRIM`"]
pub struct ATESTLF_RCOSCLF_IBIAS_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ATESTLF_RCOSCLF_IBIAS_TRIM_W<'a> {
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
#[doc = "Reader of field `XOSCLF_REGULATOR_TRIM`"]
pub type XOSCLF_REGULATOR_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XOSCLF_REGULATOR_TRIM`"]
pub struct XOSCLF_REGULATOR_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCLF_REGULATOR_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
#[doc = "Reader of field `XOSCLF_CMIRRWR_RATIO`"]
pub type XOSCLF_CMIRRWR_RATIO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XOSCLF_CMIRRWR_RATIO`"]
pub struct XOSCLF_CMIRRWR_RATIO_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCLF_CMIRRWR_RATIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 21)) | (((value as u32) & 0x0f) << 21);
        self.w
    }
}
#[doc = "Reader of field `XOSC_HF_FAST_START`"]
pub type XOSC_HF_FAST_START_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XOSC_HF_FAST_START`"]
pub struct XOSC_HF_FAST_START_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_HF_FAST_START_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | (((value as u32) & 0x03) << 19);
        self.w
    }
}
#[doc = "Reader of field `XOSC_OPTION`"]
pub type XOSC_OPTION_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSC_OPTION`"]
pub struct XOSC_OPTION_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_OPTION_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `HPOSC_OPTION`"]
pub type HPOSC_OPTION_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HPOSC_OPTION`"]
pub struct HPOSC_OPTION_W<'a> {
    w: &'a mut W,
}
impl<'a> HPOSC_OPTION_W<'a> {
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
#[doc = "Reader of field `HPOSC_BIAS_HOLD_MODE_EN`"]
pub type HPOSC_BIAS_HOLD_MODE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HPOSC_BIAS_HOLD_MODE_EN`"]
pub struct HPOSC_BIAS_HOLD_MODE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HPOSC_BIAS_HOLD_MODE_EN_W<'a> {
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
#[doc = "Reader of field `HPOSC_CURRMIRR_RATIO`"]
pub type HPOSC_CURRMIRR_RATIO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HPOSC_CURRMIRR_RATIO`"]
pub struct HPOSC_CURRMIRR_RATIO_W<'a> {
    w: &'a mut W,
}
impl<'a> HPOSC_CURRMIRR_RATIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `HPOSC_BIAS_RES_SET`"]
pub type HPOSC_BIAS_RES_SET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HPOSC_BIAS_RES_SET`"]
pub struct HPOSC_BIAS_RES_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> HPOSC_BIAS_RES_SET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `HPOSC_FILTER_EN`"]
pub type HPOSC_FILTER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HPOSC_FILTER_EN`"]
pub struct HPOSC_FILTER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HPOSC_FILTER_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `HPOSC_BIAS_RECHARGE_DELAY`"]
pub type HPOSC_BIAS_RECHARGE_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HPOSC_BIAS_RECHARGE_DELAY`"]
pub struct HPOSC_BIAS_RECHARGE_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> HPOSC_BIAS_RECHARGE_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `RESERVED2`"]
pub type RESERVED2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED2`"]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `HPOSC_SERIES_CAP`"]
pub type HPOSC_SERIES_CAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HPOSC_SERIES_CAP`"]
pub struct HPOSC_SERIES_CAP_W<'a> {
    w: &'a mut W,
}
impl<'a> HPOSC_SERIES_CAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `HPOSC_DIV3_BYPASS`"]
pub type HPOSC_DIV3_BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HPOSC_DIV3_BYPASS`"]
pub struct HPOSC_DIV3_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> HPOSC_DIV3_BYPASS_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bit 29 - 29:29\\]
Trim value for DDI_0_OSC:ADCDOUBLERNANOAMPCTL.ADC_SH_VBUF_EN."]
    #[inline(always)]
    pub fn adc_sh_vbuf_en(&self) -> ADC_SH_VBUF_EN_R {
        ADC_SH_VBUF_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Trim value for DDI_0_OSC:ADCDOUBLERNANOAMPCTL.ADC_SH_MODE_EN."]
    #[inline(always)]
    pub fn adc_sh_mode_en(&self) -> ADC_SH_MODE_EN_R {
        ADC_SH_MODE_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Trim value for DDI_0_OSC:ATESTCTL.ATESTLF_RCOSCLF_IBIAS_TRIM."]
    #[inline(always)]
    pub fn atestlf_rcosclf_ibias_trim(&self) -> ATESTLF_RCOSCLF_IBIAS_TRIM_R {
        ATESTLF_RCOSCLF_IBIAS_TRIM_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - 26:25\\]
Trim value for DDI_0_OSC:LFOSCCTL.XOSCLF_REGULATOR_TRIM."]
    #[inline(always)]
    pub fn xosclf_regulator_trim(&self) -> XOSCLF_REGULATOR_TRIM_R {
        XOSCLF_REGULATOR_TRIM_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bits 21:24 - 24:21\\]
Trim value for DDI_0_OSC:LFOSCCTL.XOSCLF_CMIRRWR_RATIO."]
    #[inline(always)]
    pub fn xosclf_cmirrwr_ratio(&self) -> XOSCLF_CMIRRWR_RATIO_R {
        XOSCLF_CMIRRWR_RATIO_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Trim value for DDI_0_OSC:CTL1.XOSC_HF_FAST_START."]
    #[inline(always)]
    pub fn xosc_hf_fast_start(&self) -> XOSC_HF_FAST_START_R {
        XOSC_HF_FAST_START_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bit 18 - 18:18\\]
0: XOSC_HF unavailable (may not be bonded out) 1: XOSC_HF available (default)"]
    #[inline(always)]
    pub fn xosc_option(&self) -> XOSC_OPTION_R {
        XOSC_OPTION_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_option(&self) -> HPOSC_OPTION_R {
        HPOSC_OPTION_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_bias_hold_mode_en(&self) -> HPOSC_BIAS_HOLD_MODE_EN_R {
        HPOSC_BIAS_HOLD_MODE_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_currmirr_ratio(&self) -> HPOSC_CURRMIRR_RATIO_R {
        HPOSC_CURRMIRR_RATIO_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_bias_res_set(&self) -> HPOSC_BIAS_RES_SET_R {
        HPOSC_BIAS_RES_SET_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_filter_en(&self) -> HPOSC_FILTER_EN_R {
        HPOSC_FILTER_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_bias_recharge_delay(&self) -> HPOSC_BIAS_RECHARGE_DELAY_R {
        HPOSC_BIAS_RECHARGE_DELAY_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_series_cap(&self) -> HPOSC_SERIES_CAP_R {
        HPOSC_SERIES_CAP_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_div3_bypass(&self) -> HPOSC_DIV3_BYPASS_R {
        HPOSC_DIV3_BYPASS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\]
Trim value for DDI_0_OSC:ADCDOUBLERNANOAMPCTL.ADC_SH_VBUF_EN."]
    #[inline(always)]
    pub fn adc_sh_vbuf_en(&mut self) -> ADC_SH_VBUF_EN_W {
        ADC_SH_VBUF_EN_W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\]
Trim value for DDI_0_OSC:ADCDOUBLERNANOAMPCTL.ADC_SH_MODE_EN."]
    #[inline(always)]
    pub fn adc_sh_mode_en(&mut self) -> ADC_SH_MODE_EN_W {
        ADC_SH_MODE_EN_W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\]
Trim value for DDI_0_OSC:ATESTCTL.ATESTLF_RCOSCLF_IBIAS_TRIM."]
    #[inline(always)]
    pub fn atestlf_rcosclf_ibias_trim(&mut self) -> ATESTLF_RCOSCLF_IBIAS_TRIM_W {
        ATESTLF_RCOSCLF_IBIAS_TRIM_W { w: self }
    }
    #[doc = "Bits 25:26 - 26:25\\]
Trim value for DDI_0_OSC:LFOSCCTL.XOSCLF_REGULATOR_TRIM."]
    #[inline(always)]
    pub fn xosclf_regulator_trim(&mut self) -> XOSCLF_REGULATOR_TRIM_W {
        XOSCLF_REGULATOR_TRIM_W { w: self }
    }
    #[doc = "Bits 21:24 - 24:21\\]
Trim value for DDI_0_OSC:LFOSCCTL.XOSCLF_CMIRRWR_RATIO."]
    #[inline(always)]
    pub fn xosclf_cmirrwr_ratio(&mut self) -> XOSCLF_CMIRRWR_RATIO_W {
        XOSCLF_CMIRRWR_RATIO_W { w: self }
    }
    #[doc = "Bits 19:20 - 20:19\\]
Trim value for DDI_0_OSC:CTL1.XOSC_HF_FAST_START."]
    #[inline(always)]
    pub fn xosc_hf_fast_start(&mut self) -> XOSC_HF_FAST_START_W {
        XOSC_HF_FAST_START_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
0: XOSC_HF unavailable (may not be bonded out) 1: XOSC_HF available (default)"]
    #[inline(always)]
    pub fn xosc_option(&mut self) -> XOSC_OPTION_W {
        XOSC_OPTION_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_option(&mut self) -> HPOSC_OPTION_W {
        HPOSC_OPTION_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_bias_hold_mode_en(&mut self) -> HPOSC_BIAS_HOLD_MODE_EN_W {
        HPOSC_BIAS_HOLD_MODE_EN_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_currmirr_ratio(&mut self) -> HPOSC_CURRMIRR_RATIO_W {
        HPOSC_CURRMIRR_RATIO_W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_bias_res_set(&mut self) -> HPOSC_BIAS_RES_SET_W {
        HPOSC_BIAS_RES_SET_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_filter_en(&mut self) -> HPOSC_FILTER_EN_W {
        HPOSC_FILTER_EN_W { w: self }
    }
    #[doc = "Bits 5:6 - 6:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_bias_recharge_delay(&mut self) -> HPOSC_BIAS_RECHARGE_DELAY_W {
        HPOSC_BIAS_RECHARGE_DELAY_W { w: self }
    }
    #[doc = "Bits 3:4 - 4:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bits 1:2 - 2:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_series_cap(&mut self) -> HPOSC_SERIES_CAP_W {
        HPOSC_SERIES_CAP_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_div3_bypass(&mut self) -> HPOSC_DIV3_BYPASS_W {
        HPOSC_DIV3_BYPASS_W { w: self }
    }
}
