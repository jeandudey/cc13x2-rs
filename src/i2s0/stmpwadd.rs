#[doc = "Reader of register STMPWADD"]
pub type R = crate::R<u32, super::STMPWADD>;
#[doc = "Writer for register STMPWADD"]
pub type W = crate::W<u32, super::STMPWADD>;
#[doc = "Register STMPWADD `reset()`'s with value 0"]
impl crate::ResetValue for super::STMPWADD {
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
#[doc = "Reader of field `VALUE_INC`"]
pub type VALUE_INC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VALUE_INC`"]
pub struct VALUE_INC_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_INC_W<'a> {
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
WCLK counter modification: Adds the written value to the running WCLK counter. If a positive edge of WCLK occurs at the same time as the operation, this will be taken into account. To add a negative value, write \"STMPWPER.VALUE - value\"."]
    #[inline(always)]
    pub fn value_inc(&self) -> VALUE_INC_R {
        VALUE_INC_R::new((self.bits & 0xffff) as u16)
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
WCLK counter modification: Adds the written value to the running WCLK counter. If a positive edge of WCLK occurs at the same time as the operation, this will be taken into account. To add a negative value, write \"STMPWPER.VALUE - value\"."]
    #[inline(always)]
    pub fn value_inc(&mut self) -> VALUE_INC_W {
        VALUE_INC_W { w: self }
    }
}
