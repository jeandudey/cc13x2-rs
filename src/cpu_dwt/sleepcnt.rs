#[doc = "Reader of register SLEEPCNT"]
pub type R = crate::R<u32, super::SLEEPCNT>;
#[doc = "Writer for register SLEEPCNT"]
pub type W = crate::W<u32, super::SLEEPCNT>;
#[doc = "Register SLEEPCNT `reset()`'s with value 0"]
impl crate::ResetValue for super::SLEEPCNT {
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
#[doc = "Reader of field `SLEEPCNT`"]
pub type SLEEPCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLEEPCNT`"]
pub struct SLEEPCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEPCNT_W<'a> {
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
Sleep counter. Counts the number of cycles during which the processor is sleeping. An event is emitted on counter overflow (every 256 cycles). This counter initializes to 0 when it is enabled using CTRL.SLEEPEVTENA. Note that the sleep counter is clocked using CPU's free-running clock. In some power modes the free-running clock to CPU is gated to minimize power consumption. This means that the sleep counter will be invalid in these power modes."]
    #[inline(always)]
    pub fn sleepcnt(&self) -> SLEEPCNT_R {
        SLEEPCNT_R::new((self.bits & 0xff) as u8)
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
Sleep counter. Counts the number of cycles during which the processor is sleeping. An event is emitted on counter overflow (every 256 cycles). This counter initializes to 0 when it is enabled using CTRL.SLEEPEVTENA. Note that the sleep counter is clocked using CPU's free-running clock. In some power modes the free-running clock to CPU is gated to minimize power consumption. This means that the sleep counter will be invalid in these power modes."]
    #[inline(always)]
    pub fn sleepcnt(&mut self) -> SLEEPCNT_W {
        SLEEPCNT_W { w: self }
    }
}
