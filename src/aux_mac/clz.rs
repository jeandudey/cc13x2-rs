#[doc = "Reader of register CLZ"]
pub type R = crate::R<u32, super::CLZ>;
#[doc = "Writer for register CLZ"]
pub type W = crate::W<u32, super::CLZ>;
#[doc = "Register CLZ `reset()`'s with value 0x28"]
impl crate::ResetValue for super::CLZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x28
    }
}
#[doc = "Reader of field `RESERVED6`"]
pub type RESERVED6_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED6`"]
pub struct RESERVED6_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff_ffff << 6)) | (((value as u32) & 0x03ff_ffff) << 6);
        self.w
    }
}
#[doc = "Reader of field `VALUE`"]
pub type VALUE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VALUE`"]
pub struct VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 0x03ff_ffff) as u32)
    }
    #[doc = "Bits 0:5 - 5:0\\]
Number of leading zero bits in the accumulator: 0x00: 0 leading zeros. 0x01: 1 leading zero. ... 0x28: 40 leading zeros (accumulator value is 0)."]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&mut self) -> RESERVED6_W {
        RESERVED6_W { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\]
Number of leading zero bits in the accumulator: 0x00: 0 leading zeros. 0x01: 1 leading zero. ... 0x28: 40 leading zeros (accumulator value is 0)."]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
}
