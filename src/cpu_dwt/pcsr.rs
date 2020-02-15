#[doc = "Reader of register PCSR"]
pub type R = crate::R<u32, super::PCSR>;
#[doc = "Writer for register PCSR"]
pub type W = crate::W<u32, super::PCSR>;
#[doc = "Register PCSR `reset()`'s with value 0"]
impl crate::ResetValue for super::PCSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EIASAMPLE`"]
pub type EIASAMPLE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EIASAMPLE`"]
pub struct EIASAMPLE_W<'a> {
    w: &'a mut W,
}
impl<'a> EIASAMPLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Execution instruction address sample, or 0xFFFFFFFF if the core is halted."]
    #[inline(always)]
    pub fn eiasample(&self) -> EIASAMPLE_R {
        EIASAMPLE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Execution instruction address sample, or 0xFFFFFFFF if the core is halted."]
    #[inline(always)]
    pub fn eiasample(&mut self) -> EIASAMPLE_W {
        EIASAMPLE_W { w: self }
    }
}
