#[doc = "Reader of register LFSR1"]
pub type R = crate::R<u32, super::LFSR1>;
#[doc = "Writer for register LFSR1"]
pub type W = crate::W<u32, super::LFSR1>;
#[doc = "Register LFSR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::LFSR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LFSR_63_32`"]
pub type LFSR_63_32_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LFSR_63_32`"]
pub struct LFSR_63_32_W<'a> {
    w: &'a mut W,
}
impl<'a> LFSR_63_32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Bits \\[63:32\\]
of the main entropy accumulation LFSR. Register can only be accessed when CTL.TEST_MODE = 1. Register contents will be cleared to zero before access is enabled."]
    #[inline(always)]
    pub fn lfsr_63_32(&self) -> LFSR_63_32_R {
        LFSR_63_32_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Bits \\[63:32\\]
of the main entropy accumulation LFSR. Register can only be accessed when CTL.TEST_MODE = 1. Register contents will be cleared to zero before access is enabled."]
    #[inline(always)]
    pub fn lfsr_63_32(&mut self) -> LFSR_63_32_W {
        LFSR_63_32_W { w: self }
    }
}
