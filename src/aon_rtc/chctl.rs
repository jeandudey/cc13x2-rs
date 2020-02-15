#[doc = "Reader of register CHCTL"]
pub type R = crate::R<u32, super::CHCTL>;
#[doc = "Writer for register CHCTL"]
pub type W = crate::W<u32, super::CHCTL>;
#[doc = "Register CHCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CHCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED19`"]
pub type RESERVED19_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED19`"]
pub struct RESERVED19_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED19_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 19)) | (((value as u32) & 0x1fff) << 19);
        self.w
    }
}
#[doc = "Reader of field `CH2_CONT_EN`"]
pub type CH2_CONT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2_CONT_EN`"]
pub struct CH2_CONT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_CONT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `RESERVED17`"]
pub type RESERVED17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED17`"]
pub struct RESERVED17_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED17_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `CH2_EN`"]
pub type CH2_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2_EN`"]
pub struct CH2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_EN_W<'a> {
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
#[doc = "Reader of field `RESERVED10`"]
pub type RESERVED10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED10`"]
pub struct RESERVED10_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | (((value as u32) & 0x3f) << 10);
        self.w
    }
}
#[doc = "Reader of field `CH1_CAPT_EN`"]
pub type CH1_CAPT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1_CAPT_EN`"]
pub struct CH1_CAPT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_CAPT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `CH1_EN`"]
pub type CH1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1_EN`"]
pub struct CH1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_EN_W<'a> {
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
#[doc = "Reader of field `CH0_EN`"]
pub type CH0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0_EN`"]
pub struct CH0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_EN_W<'a> {
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
    #[doc = "Bits 19:31 - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved19(&self) -> RESERVED19_R {
        RESERVED19_R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
    #[doc = "Bit 18 - 18:18\\]
Set to enable continuous operation of Channel 2"]
    #[inline(always)]
    pub fn ch2_cont_en(&self) -> CH2_CONT_EN_R {
        CH2_CONT_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> RESERVED17_R {
        RESERVED17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
RTC Channel 2 Enable 0: Disable RTC Channel 2 1: Enable RTC Channel 2"]
    #[inline(always)]
    pub fn ch2_en(&self) -> CH2_EN_R {
        CH2_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> RESERVED10_R {
        RESERVED10_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 9 - 9:9\\]
Set Channel 1 mode 0: Compare mode (default) 1: Capture mode"]
    #[inline(always)]
    pub fn ch1_capt_en(&self) -> CH1_CAPT_EN_R {
        CH1_CAPT_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
RTC Channel 1 Enable 0: Disable RTC Channel 1 1: Enable RTC Channel 1"]
    #[inline(always)]
    pub fn ch1_en(&self) -> CH1_EN_R {
        CH1_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 0 - 0:0\\]
RTC Channel 0 Enable 0: Disable RTC Channel 0 1: Enable RTC Channel 0"]
    #[inline(always)]
    pub fn ch0_en(&self) -> CH0_EN_R {
        CH0_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 19:31 - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved19(&mut self) -> RESERVED19_W {
        RESERVED19_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
Set to enable continuous operation of Channel 2"]
    #[inline(always)]
    pub fn ch2_cont_en(&mut self) -> CH2_CONT_EN_W {
        CH2_CONT_EN_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&mut self) -> RESERVED17_W {
        RESERVED17_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
RTC Channel 2 Enable 0: Disable RTC Channel 2 1: Enable RTC Channel 2"]
    #[inline(always)]
    pub fn ch2_en(&mut self) -> CH2_EN_W {
        CH2_EN_W { w: self }
    }
    #[doc = "Bits 10:15 - 15:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&mut self) -> RESERVED10_W {
        RESERVED10_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Set Channel 1 mode 0: Compare mode (default) 1: Capture mode"]
    #[inline(always)]
    pub fn ch1_capt_en(&mut self) -> CH1_CAPT_EN_W {
        CH1_CAPT_EN_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
RTC Channel 1 Enable 0: Disable RTC Channel 1 1: Enable RTC Channel 1"]
    #[inline(always)]
    pub fn ch1_en(&mut self) -> CH1_EN_W {
        CH1_EN_W { w: self }
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
RTC Channel 0 Enable 0: Disable RTC Channel 0 1: Enable RTC Channel 0"]
    #[inline(always)]
    pub fn ch0_en(&mut self) -> CH0_EN_W {
        CH0_EN_W { w: self }
    }
}
