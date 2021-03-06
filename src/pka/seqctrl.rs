#[doc = "Reader of register SEQCTRL"]
pub type R = crate::R<u32, super::SEQCTRL>;
#[doc = "Writer for register SEQCTRL"]
pub type W = crate::W<u32, super::SEQCTRL>;
#[doc = "Register SEQCTRL `reset()`'s with value 0x0100"]
impl crate::ResetValue for super::SEQCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100
    }
}
#[doc = "Reader of field `RESET`"]
pub type RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESET`"]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
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
        self.w.bits = (self.w.bits & !(0x7fff << 16)) | (((value as u32) & 0x7fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SEQUENCER_STAT`"]
pub type SEQUENCER_STAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEQUENCER_STAT`"]
pub struct SEQUENCER_STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQUENCER_STAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SW_CONTROL_STAT`"]
pub type SW_CONTROL_STAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SW_CONTROL_STAT`"]
pub struct SW_CONTROL_STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_CONTROL_STAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - 31:31\\]
Option program ROM: Reset value = 0. Read/Write, reset value 0b (ZERO). Writing 1b resets the sequencer, write to 0b to restart operations again. As the reset value is 0b, the sequencer will automatically start operations executing from program ROM. This bit should always be written with zero and ignored when reading this register. Option Program RAM: Reset value =1. Read/Write, reset value 1b (ONE). When 1b, the sequencer is held in a reset state and the PKA_PROGRAM area is accessible for loading the sequencer program (while the PKA_DATA_RAM is inaccessible), write to 0b to (re)start sequencer operations and disable PKA_PROGRAM area accessibility (also enables the PKA_DATA_RAM accesses). Resetting the sequencer (in order to load other firmware) should only be done when the PKA Engine is not performing any operations (i.e. the FUNCTION.RUN bit should be zero)."]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 16:30 - 30:16\\]
Set to zero on write, ignore on read"]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bits 8:15 - 15:8\\]
These read-only bits can be used by the sequencer to communicate status to the outside world. Bit \\[8\\]
is also used as sequencer interrupt, with the complement of this bit ORed into the FUNCTION.RUN bit. This field should always be written with zeroes and ignored when reading this register."]
    #[inline(always)]
    pub fn sequencer_stat(&self) -> SEQUENCER_STAT_R {
        SEQUENCER_STAT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
These bits can be used by software to trigger sequencer operations. External logic can set these bits by writing 1b, cannot reset them by writing 0b. The sequencer can reset these bits by writing 0b, cannot set them by writing 1b. Setting the FUNCTION.RUN bit together with a nonzero sequencer operations field automatically sets bit \\[0\\]
here. This field should always be written with zeroes and ignored when reading this register."]
    #[inline(always)]
    pub fn sw_control_stat(&self) -> SW_CONTROL_STAT_R {
        SW_CONTROL_STAT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Option program ROM: Reset value = 0. Read/Write, reset value 0b (ZERO). Writing 1b resets the sequencer, write to 0b to restart operations again. As the reset value is 0b, the sequencer will automatically start operations executing from program ROM. This bit should always be written with zero and ignored when reading this register. Option Program RAM: Reset value =1. Read/Write, reset value 1b (ONE). When 1b, the sequencer is held in a reset state and the PKA_PROGRAM area is accessible for loading the sequencer program (while the PKA_DATA_RAM is inaccessible), write to 0b to (re)start sequencer operations and disable PKA_PROGRAM area accessibility (also enables the PKA_DATA_RAM accesses). Resetting the sequencer (in order to load other firmware) should only be done when the PKA Engine is not performing any operations (i.e. the FUNCTION.RUN bit should be zero)."]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Bits 16:30 - 30:16\\]
Set to zero on write, ignore on read"]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
These read-only bits can be used by the sequencer to communicate status to the outside world. Bit \\[8\\]
is also used as sequencer interrupt, with the complement of this bit ORed into the FUNCTION.RUN bit. This field should always be written with zeroes and ignored when reading this register."]
    #[inline(always)]
    pub fn sequencer_stat(&mut self) -> SEQUENCER_STAT_W {
        SEQUENCER_STAT_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
These bits can be used by software to trigger sequencer operations. External logic can set these bits by writing 1b, cannot reset them by writing 0b. The sequencer can reset these bits by writing 0b, cannot set them by writing 1b. Setting the FUNCTION.RUN bit together with a nonzero sequencer operations field automatically sets bit \\[0\\]
here. This field should always be written with zeroes and ignored when reading this register."]
    #[inline(always)]
    pub fn sw_control_stat(&mut self) -> SW_CONTROL_STAT_W {
        SW_CONTROL_STAT_W { w: self }
    }
}
