#[doc = "Reader of register ANABYPASSVAL2"]
pub type R = crate::R<u32, super::ANABYPASSVAL2>;
#[doc = "Writer for register ANABYPASSVAL2"]
pub type W = crate::W<u32, super::ANABYPASSVAL2>;
#[doc = "Register ANABYPASSVAL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::ANABYPASSVAL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED14`"]
pub type RESERVED14_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED14`"]
pub struct RESERVED14_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 14)) | (((value as u32) & 0x0003_ffff) << 14);
        self.w
    }
}
#[doc = "Reader of field `XOSC_HF_IBIASTHERM`"]
pub type XOSC_HF_IBIASTHERM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `XOSC_HF_IBIASTHERM`"]
pub struct XOSC_HF_IBIASTHERM_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_HF_IBIASTHERM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> RESERVED14_R {
        RESERVED14_R::new(((self.bits >> 14) & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 0:13 - 13:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_ibiastherm(&self) -> XOSC_HF_IBIASTHERM_R {
        XOSC_HF_IBIASTHERM_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&mut self) -> RESERVED14_W {
        RESERVED14_W { w: self }
    }
    #[doc = "Bits 0:13 - 13:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_ibiastherm(&mut self) -> XOSC_HF_IBIASTHERM_W {
        XOSC_HF_IBIASTHERM_W { w: self }
    }
}
