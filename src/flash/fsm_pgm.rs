#[doc = "Reader of register FSM_PGM"]
pub type R = crate::R<u32, super::FSM_PGM>;
#[doc = "Writer for register FSM_PGM"]
pub type W = crate::W<u32, super::FSM_PGM>;
#[doc = "Register FSM_PGM `reset()`'s with value 0"]
impl crate::ResetValue for super::FSM_PGM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED26`"]
pub type RESERVED26_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED26`"]
pub struct RESERVED26_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED26_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | (((value as u32) & 0x3f) << 26);
        self.w
    }
}
#[doc = "Reader of field `PGM_BANK`"]
pub type PGM_BANK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PGM_BANK`"]
pub struct PGM_BANK_W<'a> {
    w: &'a mut W,
}
impl<'a> PGM_BANK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 23)) | (((value as u32) & 0x07) << 23);
        self.w
    }
}
#[doc = "Reader of field `PGM_ADDR`"]
pub type PGM_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PGM_ADDR`"]
pub struct PGM_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> PGM_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x007f_ffff) | ((value as u32) & 0x007f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 26:31 - 31:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved26(&self) -> RESERVED26_R {
        RESERVED26_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
    #[doc = "Bits 23:25 - 25:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgm_bank(&self) -> PGM_BANK_R {
        PGM_BANK_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bits 0:22 - 22:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgm_addr(&self) -> PGM_ADDR_R {
        PGM_ADDR_R::new((self.bits & 0x007f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 26:31 - 31:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved26(&mut self) -> RESERVED26_W {
        RESERVED26_W { w: self }
    }
    #[doc = "Bits 23:25 - 25:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgm_bank(&mut self) -> PGM_BANK_W {
        PGM_BANK_W { w: self }
    }
    #[doc = "Bits 0:22 - 22:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgm_addr(&mut self) -> PGM_ADDR_W {
        PGM_ADDR_W { w: self }
    }
}
