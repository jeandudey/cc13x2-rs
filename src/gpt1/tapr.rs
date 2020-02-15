#[doc = "Reader of register TAPR"]
pub type R = crate::R<u32, super::TAPR>;
#[doc = "Writer for register TAPR"]
pub type W = crate::W<u32, super::TAPR>;
#[doc = "Register TAPR `reset()`'s with value 0"]
impl crate::ResetValue for super::TAPR {
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
#[doc = "Reader of field `TAPSR`"]
pub type TAPSR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TAPSR`"]
pub struct TAPSR_W<'a> {
    w: &'a mut W,
}
impl<'a> TAPSR_W<'a> {
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
Timer A Pre-scale. Prescaler ratio in one-shot and periodic count mode is TAPSR + 1, that is: 0: Prescaler ratio = 1 1: Prescaler ratio = 2 2: Prescaler ratio = 3 ... 255: Prescaler ratio = 256"]
    #[inline(always)]
    pub fn tapsr(&self) -> TAPSR_R {
        TAPSR_R::new((self.bits & 0xff) as u8)
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
Timer A Pre-scale. Prescaler ratio in one-shot and periodic count mode is TAPSR + 1, that is: 0: Prescaler ratio = 1 1: Prescaler ratio = 2 2: Prescaler ratio = 3 ... 255: Prescaler ratio = 256"]
    #[inline(always)]
    pub fn tapsr(&mut self) -> TAPSR_W {
        TAPSR_W { w: self }
    }
}
