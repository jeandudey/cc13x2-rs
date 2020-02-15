#[doc = "Reader of register IRQSET"]
pub type R = crate::R<u32, super::IRQSET>;
#[doc = "Writer for register IRQSET"]
pub type W = crate::W<u32, super::IRQSET>;
#[doc = "Register IRQSET `reset()`'s with value 0"]
impl crate::ResetValue for super::IRQSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RDY`"]
pub type RDY_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RDY`"]
pub struct RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> RDY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn rdy(&mut self) -> RDY_W {
        RDY_W { w: self }
    }
}
