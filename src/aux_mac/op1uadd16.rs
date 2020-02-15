#[doc = "Reader of register OP1UADD16"]
pub type R = crate::R<u32, super::OP1UADD16>;
#[doc = "Writer for register OP1UADD16"]
pub type W = crate::W<u32, super::OP1UADD16>;
#[doc = "Register OP1UADD16 `reset()`'s with value 0"]
impl crate::ResetValue for super::OP1UADD16 {
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
#[doc = "Reader of field `OP1_VALUE`"]
pub type OP1_VALUE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OP1_VALUE`"]
pub struct OP1_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> OP1_VALUE_W<'a> {
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
Unsigned operand 1 and 16-bit addition trigger. Write OP1_VALUE to set unsigned operand 1 and trigger the following operation: ACC = ACC + OP1_VALUE."]
    #[inline(always)]
    pub fn op1_value(&self) -> OP1_VALUE_R {
        OP1_VALUE_R::new((self.bits & 0xffff) as u16)
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
Unsigned operand 1 and 16-bit addition trigger. Write OP1_VALUE to set unsigned operand 1 and trigger the following operation: ACC = ACC + OP1_VALUE."]
    #[inline(always)]
    pub fn op1_value(&mut self) -> OP1_VALUE_W {
        OP1_VALUE_W { w: self }
    }
}
