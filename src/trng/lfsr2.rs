#[doc = "Reader of register LFSR2"]
pub type R = crate::R<u32, super::LFSR2>;
#[doc = "Writer for register LFSR2"]
pub type W = crate::W<u32, super::LFSR2>;
#[doc = "Register LFSR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::LFSR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED17`"]
pub type RESERVED17_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED17`"]
pub struct RESERVED17_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 17)) | (((value as u32) & 0x7fff) << 17);
        self.w
    }
}
#[doc = "Reader of field `LFSR_80_64`"]
pub type LFSR_80_64_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LFSR_80_64`"]
pub struct LFSR_80_64_W<'a> {
    w: &'a mut W,
}
impl<'a> LFSR_80_64_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | ((value as u32) & 0x0001_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> RESERVED17_R {
        RESERVED17_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
    #[doc = "Bits 0:16 - 16:0\\]
Bits \\[80:64\\]
of the main entropy accumulation LFSR. Register can only be accessed when CTL.TEST_MODE = 1. Register contents will be cleared to zero before access is enabled."]
    #[inline(always)]
    pub fn lfsr_80_64(&self) -> LFSR_80_64_R {
        LFSR_80_64_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&mut self) -> RESERVED17_W {
        RESERVED17_W { w: self }
    }
    #[doc = "Bits 0:16 - 16:0\\]
Bits \\[80:64\\]
of the main entropy accumulation LFSR. Register can only be accessed when CTL.TEST_MODE = 1. Register contents will be cleared to zero before access is enabled."]
    #[inline(always)]
    pub fn lfsr_80_64(&mut self) -> LFSR_80_64_W {
        LFSR_80_64_W { w: self }
    }
}
