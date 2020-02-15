#[doc = "Reader of register OUT0"]
pub type R = crate::R<u32, super::OUT0>;
#[doc = "Writer for register OUT0"]
pub type W = crate::W<u32, super::OUT0>;
#[doc = "Register OUT0 `reset()`'s with value 0"]
impl crate::ResetValue for super::OUT0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VALUE_31_0`"]
pub type VALUE_31_0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `VALUE_31_0`"]
pub struct VALUE_31_0_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_31_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
LSW of 64- bit random value. New value ready when IRQFLAGSTAT.RDY = 1."]
    #[inline(always)]
    pub fn value_31_0(&self) -> VALUE_31_0_R {
        VALUE_31_0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
LSW of 64- bit random value. New value ready when IRQFLAGSTAT.RDY = 1."]
    #[inline(always)]
    pub fn value_31_0(&mut self) -> VALUE_31_0_W {
        VALUE_31_0_W { w: self }
    }
}
