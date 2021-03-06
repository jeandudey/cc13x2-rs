#[doc = "Reader of register ANABYPASS_VALUE2"]
pub type R = crate::R<u32, super::ANABYPASS_VALUE2>;
#[doc = "Writer for register ANABYPASS_VALUE2"]
pub type W = crate::W<u32, super::ANABYPASS_VALUE2>;
#[doc = "Register ANABYPASS_VALUE2 `reset()`'s with value 0xffff_c3ff"]
impl crate::ResetValue for super::ANABYPASS_VALUE2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_c3ff
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
    #[doc = "Bits 0:13 - 13:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_ibiastherm(&self) -> XOSC_HF_IBIASTHERM_R {
        XOSC_HF_IBIASTHERM_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - 13:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_ibiastherm(&mut self) -> XOSC_HF_IBIASTHERM_W {
        XOSC_HF_IBIASTHERM_W { w: self }
    }
}
