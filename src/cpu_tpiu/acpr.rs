#[doc = "Reader of register ACPR"]
pub type R = crate::R<u32, super::ACPR>;
#[doc = "Writer for register ACPR"]
pub type W = crate::W<u32, super::ACPR>;
#[doc = "Register ACPR `reset()`'s with value 0"]
impl crate::ResetValue for super::ACPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED13`"]
pub type RESERVED13_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED13`"]
pub struct RESERVED13_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0007_ffff << 13)) | (((value as u32) & 0x0007_ffff) << 13);
        self.w
    }
}
#[doc = "Reader of field `PRESCALER`"]
pub type PRESCALER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PRESCALER`"]
pub struct PRESCALER_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | ((value as u32) & 0x1fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 13:31 - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&self) -> RESERVED13_R {
        RESERVED13_R::new(((self.bits >> 13) & 0x0007_ffff) as u32)
    }
    #[doc = "Bits 0:12 - 12:0\\]
Divisor for input trace clock is (PRESCALER + 1)."]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 13:31 - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&mut self) -> RESERVED13_W {
        RESERVED13_W { w: self }
    }
    #[doc = "Bits 0:12 - 12:0\\]
Divisor for input trace clock is (PRESCALER + 1)."]
    #[inline(always)]
    pub fn prescaler(&mut self) -> PRESCALER_W {
        PRESCALER_W { w: self }
    }
}
