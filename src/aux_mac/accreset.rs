#[doc = "Reader of register ACCRESET"]
pub type R = crate::R<u32, super::ACCRESET>;
#[doc = "Writer for register ACCRESET"]
pub type W = crate::W<u32, super::ACCRESET>;
#[doc = "Register ACCRESET `reset()`'s with value 0"]
impl crate::ResetValue for super::ACCRESET {
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
#[doc = "Reader of field `TRG`"]
pub type TRG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TRG`"]
pub struct TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> TRG_W<'a> {
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
Write any value to this register to trigger a reset of all bits in the accumulator."]
    #[inline(always)]
    pub fn trg(&self) -> TRG_R {
        TRG_R::new((self.bits & 0xffff) as u16)
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
Write any value to this register to trigger a reset of all bits in the accumulator."]
    #[inline(always)]
    pub fn trg(&mut self) -> TRG_W {
        TRG_W { w: self }
    }
}
