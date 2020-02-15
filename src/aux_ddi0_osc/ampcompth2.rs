#[doc = "Reader of register AMPCOMPTH2"]
pub type R = crate::R<u32, super::AMPCOMPTH2>;
#[doc = "Writer for register AMPCOMPTH2"]
pub type W = crate::W<u32, super::AMPCOMPTH2>;
#[doc = "Register AMPCOMPTH2 `reset()`'s with value 0"]
impl crate::ResetValue for super::AMPCOMPTH2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LPMUPDATE_LTH`"]
pub type LPMUPDATE_LTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPMUPDATE_LTH`"]
pub struct LPMUPDATE_LTH_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMUPDATE_LTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | (((value as u32) & 0x3f) << 26);
        self.w
    }
}
#[doc = "Reader of field `SPARE24`"]
pub type SPARE24_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPARE24`"]
pub struct SPARE24_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `LPMUPDATE_HTH`"]
pub type LPMUPDATE_HTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPMUPDATE_HTH`"]
pub struct LPMUPDATE_HTH_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMUPDATE_HTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `SPARE16`"]
pub type SPARE16_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPARE16`"]
pub struct SPARE16_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `ADC_COMP_AMPTH_LPM`"]
pub type ADC_COMP_AMPTH_LPM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_COMP_AMPTH_LPM`"]
pub struct ADC_COMP_AMPTH_LPM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_COMP_AMPTH_LPM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | (((value as u32) & 0x3f) << 10);
        self.w
    }
}
#[doc = "Reader of field `SPARE8`"]
pub type SPARE8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPARE8`"]
pub struct SPARE8_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `ADC_COMP_AMPTH_HPM`"]
pub type ADC_COMP_AMPTH_HPM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_COMP_AMPTH_HPM`"]
pub struct ADC_COMP_AMPTH_HPM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_COMP_AMPTH_HPM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | (((value as u32) & 0x3f) << 2);
        self.w
    }
}
#[doc = "Reader of field `SPARE0`"]
pub type SPARE0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPARE0`"]
pub struct SPARE0_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 26:31 - 31:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpmupdate_lth(&self) -> LPMUPDATE_LTH_R {
        LPMUPDATE_LTH_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare24(&self) -> SPARE24_R {
        SPARE24_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 18:23 - 23:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpmupdate_hth(&self) -> LPMUPDATE_HTH_R {
        LPMUPDATE_HTH_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare16(&self) -> SPARE16_R {
        SPARE16_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_comp_ampth_lpm(&self) -> ADC_COMP_AMPTH_LPM_R {
        ADC_COMP_AMPTH_LPM_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare8(&self) -> SPARE8_R {
        SPARE8_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_comp_ampth_hpm(&self) -> ADC_COMP_AMPTH_HPM_R {
        ADC_COMP_AMPTH_HPM_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 0:1 - 1:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare0(&self) -> SPARE0_R {
        SPARE0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 26:31 - 31:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpmupdate_lth(&mut self) -> LPMUPDATE_LTH_W {
        LPMUPDATE_LTH_W { w: self }
    }
    #[doc = "Bits 24:25 - 25:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare24(&mut self) -> SPARE24_W {
        SPARE24_W { w: self }
    }
    #[doc = "Bits 18:23 - 23:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpmupdate_hth(&mut self) -> LPMUPDATE_HTH_W {
        LPMUPDATE_HTH_W { w: self }
    }
    #[doc = "Bits 16:17 - 17:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare16(&mut self) -> SPARE16_W {
        SPARE16_W { w: self }
    }
    #[doc = "Bits 10:15 - 15:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_comp_ampth_lpm(&mut self) -> ADC_COMP_AMPTH_LPM_W {
        ADC_COMP_AMPTH_LPM_W { w: self }
    }
    #[doc = "Bits 8:9 - 9:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare8(&mut self) -> SPARE8_W {
        SPARE8_W { w: self }
    }
    #[doc = "Bits 2:7 - 7:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_comp_ampth_hpm(&mut self) -> ADC_COMP_AMPTH_HPM_W {
        ADC_COMP_AMPTH_HPM_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare0(&mut self) -> SPARE0_W {
        SPARE0_W { w: self }
    }
}
