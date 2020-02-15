#[doc = "Reader of register STAT2"]
pub type R = crate::R<u32, super::STAT2>;
#[doc = "Writer for register STAT2"]
pub type W = crate::W<u32, super::STAT2>;
#[doc = "Register STAT2 `reset()`'s with value 0"]
impl crate::ResetValue for super::STAT2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC_DCBIAS`"]
pub type ADC_DCBIAS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_DCBIAS`"]
pub struct ADC_DCBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_DCBIAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | (((value as u32) & 0x3f) << 26);
        self.w
    }
}
#[doc = "Reader of field `HPM_RAMP1_THMET`"]
pub type HPM_RAMP1_THMET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HPM_RAMP1_THMET`"]
pub struct HPM_RAMP1_THMET_W<'a> {
    w: &'a mut W,
}
impl<'a> HPM_RAMP1_THMET_W<'a> {
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
#[doc = "Reader of field `HPM_RAMP2_THMET`"]
pub type HPM_RAMP2_THMET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HPM_RAMP2_THMET`"]
pub struct HPM_RAMP2_THMET_W<'a> {
    w: &'a mut W,
}
impl<'a> HPM_RAMP2_THMET_W<'a> {
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
#[doc = "Reader of field `HPM_RAMP3_THMET`"]
pub type HPM_RAMP3_THMET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HPM_RAMP3_THMET`"]
pub struct HPM_RAMP3_THMET_W<'a> {
    w: &'a mut W,
}
impl<'a> HPM_RAMP3_THMET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `RESERVED16`"]
pub type RESERVED16_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED16`"]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `RAMPSTATE`"]
pub type RAMPSTATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RAMPSTATE`"]
pub struct RAMPSTATE_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMPSTATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `RESERVED4`"]
pub type RESERVED4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED4`"]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 4)) | (((value as u32) & 0xff) << 4);
        self.w
    }
}
#[doc = "Reader of field `AMPCOMP_REQ`"]
pub type AMPCOMP_REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AMPCOMP_REQ`"]
pub struct AMPCOMP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> AMPCOMP_REQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `XOSC_HF_AMPGOOD`"]
pub type XOSC_HF_AMPGOOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSC_HF_AMPGOOD`"]
pub struct XOSC_HF_AMPGOOD_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_HF_AMPGOOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `XOSC_HF_FREQGOOD`"]
pub type XOSC_HF_FREQGOOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSC_HF_FREQGOOD`"]
pub struct XOSC_HF_FREQGOOD_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_HF_FREQGOOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `XOSC_HF_RF_FREQGOOD`"]
pub type XOSC_HF_RF_FREQGOOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSC_HF_RF_FREQGOOD`"]
pub struct XOSC_HF_RF_FREQGOOD_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_HF_RF_FREQGOOD_W<'a> {
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
    #[doc = "Bits 26:31 - 31:26\\]
DC Bias read by RADC during SAR mode The value is an unsigned integer. It is used for debug only."]
    #[inline(always)]
    pub fn adc_dcbias(&self) -> ADC_DCBIAS_R {
        ADC_DCBIAS_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
    #[doc = "Bit 25 - 25:25\\]
Indication of threshold is met for hpm_ramp1"]
    #[inline(always)]
    pub fn hpm_ramp1_thmet(&self) -> HPM_RAMP1_THMET_R {
        HPM_RAMP1_THMET_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Indication of threshold is met for hpm_ramp2"]
    #[inline(always)]
    pub fn hpm_ramp2_thmet(&self) -> HPM_RAMP2_THMET_R {
        HPM_RAMP2_THMET_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Indication of threshold is met for hpm_ramp3"]
    #[inline(always)]
    pub fn hpm_ramp3_thmet(&self) -> HPM_RAMP3_THMET_R {
        HPM_RAMP3_THMET_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
xosc_hf amplitude compensation FSM This is identical to STAT1.RAMPSTATE. See that description for encoding."]
    #[inline(always)]
    pub fn rampstate(&self) -> RAMPSTATE_R {
        RAMPSTATE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 4:11 - 11:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
ampcomp_req"]
    #[inline(always)]
    pub fn ampcomp_req(&self) -> AMPCOMP_REQ_R {
        AMPCOMP_REQ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
amplitude of xosc_hf is within the required threshold (set by DDI). Not used for anything just for debug/status"]
    #[inline(always)]
    pub fn xosc_hf_ampgood(&self) -> XOSC_HF_AMPGOOD_R {
        XOSC_HF_AMPGOOD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
frequency of xosc_hf is good to use for the digital clocks"]
    #[inline(always)]
    pub fn xosc_hf_freqgood(&self) -> XOSC_HF_FREQGOOD_R {
        XOSC_HF_FREQGOOD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
frequency of xosc_hf is within +/- 20 ppm and xosc_hf is good for radio operations. Used for SW to start synthesizer."]
    #[inline(always)]
    pub fn xosc_hf_rf_freqgood(&self) -> XOSC_HF_RF_FREQGOOD_R {
        XOSC_HF_RF_FREQGOOD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 26:31 - 31:26\\]
DC Bias read by RADC during SAR mode The value is an unsigned integer. It is used for debug only."]
    #[inline(always)]
    pub fn adc_dcbias(&mut self) -> ADC_DCBIAS_W {
        ADC_DCBIAS_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
Indication of threshold is met for hpm_ramp1"]
    #[inline(always)]
    pub fn hpm_ramp1_thmet(&mut self) -> HPM_RAMP1_THMET_W {
        HPM_RAMP1_THMET_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
Indication of threshold is met for hpm_ramp2"]
    #[inline(always)]
    pub fn hpm_ramp2_thmet(&mut self) -> HPM_RAMP2_THMET_W {
        HPM_RAMP2_THMET_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
Indication of threshold is met for hpm_ramp3"]
    #[inline(always)]
    pub fn hpm_ramp3_thmet(&mut self) -> HPM_RAMP3_THMET_W {
        HPM_RAMP3_THMET_W { w: self }
    }
    #[doc = "Bits 16:22 - 22:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\]
xosc_hf amplitude compensation FSM This is identical to STAT1.RAMPSTATE. See that description for encoding."]
    #[inline(always)]
    pub fn rampstate(&mut self) -> RAMPSTATE_W {
        RAMPSTATE_W { w: self }
    }
    #[doc = "Bits 4:11 - 11:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
ampcomp_req"]
    #[inline(always)]
    pub fn ampcomp_req(&mut self) -> AMPCOMP_REQ_W {
        AMPCOMP_REQ_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
amplitude of xosc_hf is within the required threshold (set by DDI). Not used for anything just for debug/status"]
    #[inline(always)]
    pub fn xosc_hf_ampgood(&mut self) -> XOSC_HF_AMPGOOD_W {
        XOSC_HF_AMPGOOD_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
frequency of xosc_hf is good to use for the digital clocks"]
    #[inline(always)]
    pub fn xosc_hf_freqgood(&mut self) -> XOSC_HF_FREQGOOD_W {
        XOSC_HF_FREQGOOD_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
frequency of xosc_hf is within +/- 20 ppm and xosc_hf is good for radio operations. Used for SW to start synthesizer."]
    #[inline(always)]
    pub fn xosc_hf_rf_freqgood(&mut self) -> XOSC_HF_RF_FREQGOOD_W {
        XOSC_HF_RF_FREQGOOD_W { w: self }
    }
}
