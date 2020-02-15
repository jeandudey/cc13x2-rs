#[doc = "Reader of register ADCREF1"]
pub type R = crate::R<u8, super::ADCREF1>;
#[doc = "Writer for register ADCREF1"]
pub type W = crate::W<u8, super::ADCREF1>;
#[doc = "Register ADCREF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCREF1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED6`"]
pub type RESERVED6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED6`"]
pub struct RESERVED6_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u8) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `VTRIM`"]
pub type VTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VTRIM`"]
pub struct VTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u8) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 0:5 - 5:0\\]
Trim output voltage of ADC fixed reference (64 steps, 2's complement). Applies only for ADCREF0.SRC = 0. Examples: 0x00 - nominal voltage 1.43V 0x01 - nominal + 0.4% 1.435V 0x3F - nominal - 0.4% 1.425V 0x1F - maximum voltage 1.6V 0x20 - minimum voltage 1.3V"]
    #[inline(always)]
    pub fn vtrim(&self) -> VTRIM_R {
        VTRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&mut self) -> RESERVED6_W {
        RESERVED6_W { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\]
Trim output voltage of ADC fixed reference (64 steps, 2's complement). Applies only for ADCREF0.SRC = 0. Examples: 0x00 - nominal voltage 1.43V 0x01 - nominal + 0.4% 1.435V 0x3F - nominal - 0.4% 1.425V 0x1F - maximum voltage 1.6V 0x20 - minimum voltage 1.3V"]
    #[inline(always)]
    pub fn vtrim(&mut self) -> VTRIM_W {
        VTRIM_W { w: self }
    }
}
