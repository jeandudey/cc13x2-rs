#[doc = "Reader of register FVWLCT"]
pub type R = crate::R<u32, super::FVWLCT>;
#[doc = "Writer for register FVWLCT"]
pub type W = crate::W<u32, super::FVWLCT>;
#[doc = "Register FVWLCT `reset()`'s with value 0x08"]
impl crate::ResetValue for super::FVWLCT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x08
    }
}
#[doc = "Reader of field `RESERVED5`"]
pub type RESERVED5_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED5`"]
pub struct RESERVED5_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff_ffff << 5)) | (((value as u32) & 0x07ff_ffff) << 5);
        self.w
    }
}
#[doc = "Reader of field `VWLCT_P`"]
pub type VWLCT_P_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VWLCT_P`"]
pub struct VWLCT_P_W<'a> {
    w: &'a mut W,
}
impl<'a> VWLCT_P_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 5:31 - 31:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 5) & 0x07ff_ffff) as u32)
    }
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vwlct_p(&self) -> VWLCT_P_R {
        VWLCT_P_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 5:31 - 31:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved5(&mut self) -> RESERVED5_W {
        RESERVED5_W { w: self }
    }
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vwlct_p(&mut self) -> VWLCT_P_W {
        VWLCT_P_W { w: self }
    }
}
