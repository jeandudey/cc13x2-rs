#[doc = "Reader of register CTL1"]
pub type R = crate::R<u32, super::CTL1>;
#[doc = "Writer for register CTL1"]
pub type W = crate::W<u32, super::CTL1>;
#[doc = "Register CTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED23`"]
pub type RESERVED23_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED23`"]
pub struct RESERVED23_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 23)) | (((value as u32) & 0x01ff) << 23);
        self.w
    }
}
#[doc = "Reader of field `RCOSCHFCTRIMFRACT`"]
pub type RCOSCHFCTRIMFRACT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RCOSCHFCTRIMFRACT`"]
pub struct RCOSCHFCTRIMFRACT_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCHFCTRIMFRACT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 18)) | (((value as u32) & 0x1f) << 18);
        self.w
    }
}
#[doc = "Reader of field `RCOSCHFCTRIMFRACT_EN`"]
pub type RCOSCHFCTRIMFRACT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCOSCHFCTRIMFRACT_EN`"]
pub struct RCOSCHFCTRIMFRACT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCHFCTRIMFRACT_EN_W<'a> {
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
#[doc = "Reader of field `SPARE2`"]
pub type SPARE2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SPARE2`"]
pub struct SPARE2_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 2)) | (((value as u32) & 0x7fff) << 2);
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
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 23:31 - 31:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved23(&self) -> RESERVED23_R {
        RESERVED23_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
    #[doc = "Bits 18:22 - 22:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschfctrimfract(&self) -> RCOSCHFCTRIMFRACT_R {
        RCOSCHFCTRIMFRACT_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschfctrimfract_en(&self) -> RCOSCHFCTRIMFRACT_EN_R {
        RCOSCHFCTRIMFRACT_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 2:16 - 16:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare2(&self) -> SPARE2_R {
        SPARE2_R::new(((self.bits >> 2) & 0x7fff) as u16)
    }
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_fast_start(&self) -> XOSC_HF_FAST_START_R {
        XOSC_HF_FAST_START_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 23:31 - 31:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved23(&mut self) -> RESERVED23_W {
        RESERVED23_W { w: self }
    }
    #[doc = "Bits 18:22 - 22:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschfctrimfract(&mut self) -> RCOSCHFCTRIMFRACT_W {
        RCOSCHFCTRIMFRACT_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschfctrimfract_en(&mut self) -> RCOSCHFCTRIMFRACT_EN_W {
        RCOSCHFCTRIMFRACT_EN_W { w: self }
    }
    #[doc = "Bits 2:16 - 16:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare2(&mut self) -> SPARE2_W {
        SPARE2_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_fast_start(&mut self) -> XOSC_HF_FAST_START_W {
        XOSC_HF_FAST_START_W { w: self }
    }
}
