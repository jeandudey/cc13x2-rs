#[doc = "Reader of register OUT1"]
pub type R = crate::R<u32, super::OUT1>;
#[doc = "Writer for register OUT1"]
pub type W = crate::W<u32, super::OUT1>;
#[doc = "Register OUT1 `reset()`'s with value 0"]
impl crate::ResetValue for super::OUT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VALUE_63_32`"]
pub type VALUE_63_32_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `VALUE_63_32`"]
pub struct VALUE_63_32_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_63_32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
MSW of 64-bit random value. New value ready when IRQFLAGSTAT.RDY = 1."]
    #[inline(always)]
    pub fn value_63_32(&self) -> VALUE_63_32_R {
        VALUE_63_32_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
MSW of 64-bit random value. New value ready when IRQFLAGSTAT.RDY = 1."]
    #[inline(always)]
    pub fn value_63_32(&mut self) -> VALUE_63_32_W {
        VALUE_63_32_W { w: self }
    }
}
