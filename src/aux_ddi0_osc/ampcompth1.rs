#[doc = "Reader of register AMPCOMPTH1"]
pub type R = crate::R<u32, super::AMPCOMPTH1>;
#[doc = "Writer for register AMPCOMPTH1"]
pub type W = crate::W<u32, super::AMPCOMPTH1>;
#[doc = "Register AMPCOMPTH1 `reset()`'s with value 0"]
impl crate::ResetValue for super::AMPCOMPTH1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `HPMRAMP3_LTH`"]
pub type HPMRAMP3_LTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HPMRAMP3_LTH`"]
pub struct HPMRAMP3_LTH_W<'a> {
    w: &'a mut W,
}
impl<'a> HPMRAMP3_LTH_W<'a> {
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
#[doc = "Reader of field `HPMRAMP3_HTH`"]
pub type HPMRAMP3_HTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HPMRAMP3_HTH`"]
pub struct HPMRAMP3_HTH_W<'a> {
    w: &'a mut W,
}
impl<'a> HPMRAMP3_HTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | (((value as u32) & 0x3f) << 10);
        self.w
    }
}
#[doc = "Reader of field `IBIASCAP_LPTOHP_OL_CNT`"]
pub type IBIASCAP_LPTOHP_OL_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IBIASCAP_LPTOHP_OL_CNT`"]
pub struct IBIASCAP_LPTOHP_OL_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> IBIASCAP_LPTOHP_OL_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 6)) | (((value as u32) & 0x0f) << 6);
        self.w
    }
}
#[doc = "Reader of field `HPMRAMP1_TH`"]
pub type HPMRAMP1_TH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HPMRAMP1_TH`"]
pub struct HPMRAMP1_TH_W<'a> {
    w: &'a mut W,
}
impl<'a> HPMRAMP1_TH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare24(&self) -> SPARE24_R {
        SPARE24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 18:23 - 23:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hpmramp3_lth(&self) -> HPMRAMP3_LTH_R {
        HPMRAMP3_LTH_R::new(((self.bits >> 18) & 0x3f) as u8)
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
    pub fn hpmramp3_hth(&self) -> HPMRAMP3_HTH_R {
        HPMRAMP3_HTH_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 6:9 - 9:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibiascap_lptohp_ol_cnt(&self) -> IBIASCAP_LPTOHP_OL_CNT_R {
        IBIASCAP_LPTOHP_OL_CNT_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hpmramp1_th(&self) -> HPMRAMP1_TH_R {
        HPMRAMP1_TH_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare24(&mut self) -> SPARE24_W {
        SPARE24_W { w: self }
    }
    #[doc = "Bits 18:23 - 23:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hpmramp3_lth(&mut self) -> HPMRAMP3_LTH_W {
        HPMRAMP3_LTH_W { w: self }
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
    pub fn hpmramp3_hth(&mut self) -> HPMRAMP3_HTH_W {
        HPMRAMP3_HTH_W { w: self }
    }
    #[doc = "Bits 6:9 - 9:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibiascap_lptohp_ol_cnt(&mut self) -> IBIASCAP_LPTOHP_OL_CNT_W {
        IBIASCAP_LPTOHP_OL_CNT_W { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hpmramp1_th(&mut self) -> HPMRAMP1_TH_W {
        HPMRAMP1_TH_W { w: self }
    }
}
