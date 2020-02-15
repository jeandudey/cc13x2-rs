#[doc = "Reader of register FTCR"]
pub type R = crate::R<u32, super::FTCR>;
#[doc = "Writer for register FTCR"]
pub type W = crate::W<u32, super::FTCR>;
#[doc = "Register FTCR `reset()`'s with value 0"]
impl crate::ResetValue for super::FTCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED7`"]
pub type RESERVED7_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED7`"]
pub struct RESERVED7_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff_ffff << 7)) | (((value as u32) & 0x01ff_ffff) << 7);
        self.w
    }
}
#[doc = "Reader of field `TCR`"]
pub type TCR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TCR`"]
pub struct TCR_W<'a> {
    w: &'a mut W,
}
impl<'a> TCR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 7:31 - 31:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 7) & 0x01ff_ffff) as u32)
    }
    #[doc = "Bits 0:6 - 6:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn tcr(&self) -> TCR_R {
        TCR_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 7:31 - 31:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved7(&mut self) -> RESERVED7_W {
        RESERVED7_W { w: self }
    }
    #[doc = "Bits 0:6 - 6:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn tcr(&mut self) -> TCR_W {
        TCR_W { w: self }
    }
}
