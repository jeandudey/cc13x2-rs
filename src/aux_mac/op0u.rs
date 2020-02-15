#[doc = "Reader of register OP0U"]
pub type R = crate::R<u32, super::OP0U>;
#[doc = "Writer for register OP0U"]
pub type W = crate::W<u32, super::OP0U>;
#[doc = "Register OP0U `reset()`'s with value 0"]
impl crate::ResetValue for super::OP0U {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED16`"]
pub type RESERVED16_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED16`"]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `OP0_VALUE`"]
pub type OP0_VALUE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OP0_VALUE`"]
pub struct OP0_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> OP0_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - 15:0\\]
Unsigned operand 0. Operand for multiply, multiply-and-accumulate, or 32-bit add operations."]
    #[inline(always)]
    pub fn op0_value(&self) -> OP0_VALUE_R {
        OP0_VALUE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
Unsigned operand 0. Operand for multiply, multiply-and-accumulate, or 32-bit add operations."]
    #[inline(always)]
    pub fn op0_value(&mut self) -> OP0_VALUE_W {
        OP0_VALUE_W { w: self }
    }
}
