#[doc = "Reader of register AMPCOMP_TH2"]
pub type R = crate::R<u32, super::AMPCOMP_TH2>;
#[doc = "Writer for register AMPCOMP_TH2"]
pub type W = crate::W<u32, super::AMPCOMP_TH2>;
#[doc = "Register AMPCOMP_TH2 `reset()`'s with value 0x6b8b_0303"]
impl crate::ResetValue for super::AMPCOMP_TH2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x6b8b_0303
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
#[doc = "Reader of field `RESERVED3`"]
pub type RESERVED3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED3`"]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `LPMUPDATE_HTM`"]
pub type LPMUPDATE_HTM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPMUPDATE_HTM`"]
pub struct LPMUPDATE_HTM_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMUPDATE_HTM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
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
#[doc = "Reader of field `RESERVED0`"]
pub type RESERVED0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED0`"]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
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
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 18:23 - 23:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpmupdate_htm(&self) -> LPMUPDATE_HTM_R {
        LPMUPDATE_HTM_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_comp_ampth_lpm(&self) -> ADC_COMP_AMPTH_LPM_R {
        ADC_COMP_AMPTH_LPM_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_comp_ampth_hpm(&self) -> ADC_COMP_AMPTH_HPM_R {
        ADC_COMP_AMPTH_HPM_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x03) as u8)
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
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bits 18:23 - 23:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpmupdate_htm(&mut self) -> LPMUPDATE_HTM_W {
        LPMUPDATE_HTM_W { w: self }
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bits 10:15 - 15:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_comp_ampth_lpm(&mut self) -> ADC_COMP_AMPTH_LPM_W {
        ADC_COMP_AMPTH_LPM_W { w: self }
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bits 2:7 - 7:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_comp_ampth_hpm(&mut self) -> ADC_COMP_AMPTH_HPM_W {
        ADC_COMP_AMPTH_HPM_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
}
