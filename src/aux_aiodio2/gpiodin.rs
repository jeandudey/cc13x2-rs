#[doc = "Reader of register GPIODIN"]
pub type R = crate::R<u32, super::GPIODIN>;
#[doc = "Writer for register GPIODIN"]
pub type W = crate::W<u32, super::GPIODIN>;
#[doc = "Register GPIODIN `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIODIN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED8`"]
pub type RESERVED8_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED8`"]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Reader of field `IO7_0`"]
pub type IO7_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IO7_0`"]
pub struct IO7_0_W<'a> {
    w: &'a mut W,
}
impl<'a> IO7_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Bit n in this bit vector contains the value for AUXIO\\[8i+n\\]
when GPIODIE bit n is set. Otherwise, bit n is read as 0."]
    #[inline(always)]
    pub fn io7_0(&self) -> IO7_0_R {
        IO7_0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Bit n in this bit vector contains the value for AUXIO\\[8i+n\\]
when GPIODIE bit n is set. Otherwise, bit n is read as 0."]
    #[inline(always)]
    pub fn io7_0(&mut self) -> IO7_0_W {
        IO7_0_W { w: self }
    }
}
