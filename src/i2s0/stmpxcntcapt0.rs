#[doc = "Reader of register STMPXCNTCAPT0"]
pub type R = crate::R<u32, super::STMPXCNTCAPT0>;
#[doc = "Writer for register STMPXCNTCAPT0"]
pub type W = crate::W<u32, super::STMPXCNTCAPT0>;
#[doc = "Register STMPXCNTCAPT0 `reset()`'s with value 0"]
impl crate::ResetValue for super::STMPXCNTCAPT0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAPT_VALUE`"]
pub type CAPT_VALUE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CAPT_VALUE`"]
pub struct CAPT_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPT_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
The value of the samplestamp XOSC counter (STMPXCNT.CURR_VALUE) last time an event was pulsed (event source selected in \\[EVENT.I2SSTMPSEL0.EV\\]
for channel 0). This number corresponds to the number of 24 MHz clock cycles since the last positive edge of the selected WCLK. The value is cleared when STMPCTL.STMP_EN = 0. Note: Due to buffering and synchronization, WCLK is delayed by a small number of BCLK periods and clk periods. Note: When calculating the fractional part of the sample stamp, STMPXPER may be less than this bit field."]
    #[inline(always)]
    pub fn capt_value(&self) -> CAPT_VALUE_R {
        CAPT_VALUE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
The value of the samplestamp XOSC counter (STMPXCNT.CURR_VALUE) last time an event was pulsed (event source selected in \\[EVENT.I2SSTMPSEL0.EV\\]
for channel 0). This number corresponds to the number of 24 MHz clock cycles since the last positive edge of the selected WCLK. The value is cleared when STMPCTL.STMP_EN = 0. Note: Due to buffering and synchronization, WCLK is delayed by a small number of BCLK periods and clk periods. Note: When calculating the fractional part of the sample stamp, STMPXPER may be less than this bit field."]
    #[inline(always)]
    pub fn capt_value(&mut self) -> CAPT_VALUE_W {
        CAPT_VALUE_W { w: self }
    }
}
