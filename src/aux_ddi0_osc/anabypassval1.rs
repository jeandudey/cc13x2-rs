#[doc = "Reader of register ANABYPASSVAL1"]
pub type R = crate::R<u32, super::ANABYPASSVAL1>;
#[doc = "Writer for register ANABYPASSVAL1"]
pub type W = crate::W<u32, super::ANABYPASSVAL1>;
#[doc = "Register ANABYPASSVAL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ANABYPASSVAL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED20`"]
pub type RESERVED20_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED20`"]
pub struct RESERVED20_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 20)) | (((value as u32) & 0x0fff) << 20);
        self.w
    }
}
#[doc = "Reader of field `XOSC_HF_ROW_Q12`"]
pub type XOSC_HF_ROW_Q12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XOSC_HF_ROW_Q12`"]
pub struct XOSC_HF_ROW_Q12_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_HF_ROW_Q12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `XOSC_HF_COLUMN_Q12`"]
pub type XOSC_HF_COLUMN_Q12_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `XOSC_HF_COLUMN_Q12`"]
pub struct XOSC_HF_COLUMN_Q12_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_HF_COLUMN_Q12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:31 - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&self) -> RESERVED20_R {
        RESERVED20_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_row_q12(&self) -> XOSC_HF_ROW_Q12_R {
        XOSC_HF_ROW_Q12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_column_q12(&self) -> XOSC_HF_COLUMN_Q12_R {
        XOSC_HF_COLUMN_Q12_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 20:31 - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&mut self) -> RESERVED20_W {
        RESERVED20_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_row_q12(&mut self) -> XOSC_HF_ROW_Q12_W {
        XOSC_HF_ROW_Q12_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_column_q12(&mut self) -> XOSC_HF_COLUMN_Q12_W {
        XOSC_HF_COLUMN_Q12_W { w: self }
    }
}
