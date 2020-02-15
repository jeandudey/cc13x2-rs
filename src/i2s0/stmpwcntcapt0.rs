#[doc = "Reader of register STMPWCNTCAPT0"]
pub type R = crate::R<u32, super::STMPWCNTCAPT0>;
#[doc = "Writer for register STMPWCNTCAPT0"]
pub type W = crate::W<u32, super::STMPWCNTCAPT0>;
#[doc = "Register STMPWCNTCAPT0 `reset()`'s with value 0"]
impl crate::ResetValue for super::STMPWCNTCAPT0 {
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
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - 15:0\\]
The value of the samplestamp WCLK counter (STMPWCNT.CURR_VALUE) last time an event was pulsed (event source selected in EVENT:I2SSTMPSEL0.EV for channel 0). This number corresponds to the number of positive WCLK edges since the samplestamp generator was enabled (not taking modification through STMPWADD/STMPWSET into account). The value is cleared when STMPCTL.STMP_EN = 0."]
    #[inline(always)]
    pub fn capt_value(&self) -> CAPT_VALUE_R {
        CAPT_VALUE_R::new((self.bits & 0xffff) as u16)
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
The value of the samplestamp WCLK counter (STMPWCNT.CURR_VALUE) last time an event was pulsed (event source selected in EVENT:I2SSTMPSEL0.EV for channel 0). This number corresponds to the number of positive WCLK edges since the samplestamp generator was enabled (not taking modification through STMPWADD/STMPWSET into account). The value is cleared when STMPCTL.STMP_EN = 0."]
    #[inline(always)]
    pub fn capt_value(&mut self) -> CAPT_VALUE_W {
        CAPT_VALUE_W { w: self }
    }
}
