#[doc = "Reader of register VALUE"]
pub type R = crate::R<u32, super::VALUE>;
#[doc = "Writer for register VALUE"]
pub type W = crate::W<u32, super::VALUE>;
#[doc = "Register VALUE `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::VALUE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `WDTVALUE`"]
pub type WDTVALUE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WDTVALUE`"]
pub struct WDTVALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTVALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register contains the current count value of the timer."]
    #[inline(always)]
    pub fn wdtvalue(&self) -> WDTVALUE_R {
        WDTVALUE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register contains the current count value of the timer."]
    #[inline(always)]
    pub fn wdtvalue(&mut self) -> WDTVALUE_W {
        WDTVALUE_W { w: self }
    }
}
