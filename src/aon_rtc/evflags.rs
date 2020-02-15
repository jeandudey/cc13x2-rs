#[doc = "Reader of register EVFLAGS"]
pub type R = crate::R<u32, super::EVFLAGS>;
#[doc = "Writer for register EVFLAGS"]
pub type W = crate::W<u32, super::EVFLAGS>;
#[doc = "Register EVFLAGS `reset()`'s with value 0"]
impl crate::ResetValue for super::EVFLAGS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED17`"]
pub type RESERVED17_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED17`"]
pub struct RESERVED17_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 17)) | (((value as u32) & 0x7fff) << 17);
        self.w
    }
}
#[doc = "Reader of field `CH2`"]
pub type CH2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2`"]
pub struct CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `RESERVED9`"]
pub type RESERVED9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED9`"]
pub struct RESERVED9_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 9)) | (((value as u32) & 0x7f) << 9);
        self.w
    }
}
#[doc = "Reader of field `CH1`"]
pub type CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1`"]
pub struct CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `RESERVED1`"]
pub type RESERVED1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u32) & 0x7f) << 1);
        self.w
    }
}
#[doc = "Reader of field `CH0`"]
pub type CH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0`"]
pub struct CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> RESERVED17_R {
        RESERVED17_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Channel 2 event flag, set when CHCTL.CH2_EN = 1 and the RTC value matches or passes the CH2CMP value. An event will be scheduled to occur as soon as possible when writing to CH2CMP provided that the channel is enabled and the new value matches any time between next RTC value and 1 second in the past Writing 1 clears this flag. AUX_SCE can read the flag through AUX_EVCTL:EVSTAT2.AON_RTC_CH2 and clear it using AUX_SYSIF:RTCEVCLR.RTC_CH2_EV_CLR."]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Channel 1 event flag, set when CHCTL.CH1_EN = 1 and one of the following: - CHCTL.CH1_CAPT_EN = 0 and the RTC value matches or passes the CH1CMP value. - CHCTL.CH1_CAPT_EN = 1 and capture occurs. An event will be scheduled to occur as soon as possible when writing to CH1CMP provided that the channel is enabled, in compare mode and the new value matches any time between next RTC value and 1 second in the past. Writing 1 clears this flag."]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 0 - 0:0\\]
Channel 0 event flag, set when CHCTL.CH0_EN = 1 and the RTC value matches or passes the CH0CMP value. An event will be scheduled to occur as soon as possible when writing to CH0CMP provided that the channels is enabled and the new value matches any time between next RTC value and 1 second in the past. Writing 1 clears this flag."]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&mut self) -> RESERVED17_W {
        RESERVED17_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Channel 2 event flag, set when CHCTL.CH2_EN = 1 and the RTC value matches or passes the CH2CMP value. An event will be scheduled to occur as soon as possible when writing to CH2CMP provided that the channel is enabled and the new value matches any time between next RTC value and 1 second in the past Writing 1 clears this flag. AUX_SCE can read the flag through AUX_EVCTL:EVSTAT2.AON_RTC_CH2 and clear it using AUX_SYSIF:RTCEVCLR.RTC_CH2_EV_CLR."]
    #[inline(always)]
    pub fn ch2(&mut self) -> CH2_W {
        CH2_W { w: self }
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&mut self) -> RESERVED9_W {
        RESERVED9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Channel 1 event flag, set when CHCTL.CH1_EN = 1 and one of the following: - CHCTL.CH1_CAPT_EN = 0 and the RTC value matches or passes the CH1CMP value. - CHCTL.CH1_CAPT_EN = 1 and capture occurs. An event will be scheduled to occur as soon as possible when writing to CH1CMP provided that the channel is enabled, in compare mode and the new value matches any time between next RTC value and 1 second in the past. Writing 1 clears this flag."]
    #[inline(always)]
    pub fn ch1(&mut self) -> CH1_W {
        CH1_W { w: self }
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Channel 0 event flag, set when CHCTL.CH0_EN = 1 and the RTC value matches or passes the CH0CMP value. An event will be scheduled to occur as soon as possible when writing to CH0CMP provided that the channels is enabled and the new value matches any time between next RTC value and 1 second in the past. Writing 1 clears this flag."]
    #[inline(always)]
    pub fn ch0(&mut self) -> CH0_W {
        CH0_W { w: self }
    }
}
