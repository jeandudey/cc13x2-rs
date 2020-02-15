#[doc = "Reader of register SLEEPCTL"]
pub type R = crate::R<u32, super::SLEEPCTL>;
#[doc = "Writer for register SLEEPCTL"]
pub type W = crate::W<u32, super::SLEEPCTL>;
#[doc = "Register SLEEPCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::SLEEPCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED1`"]
pub type RESERVED1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff_ffff << 1)) | (((value as u32) & 0x7fff_ffff) << 1);
        self.w
    }
}
#[doc = "Reader of field `IO_PAD_SLEEP_DIS`"]
pub type IO_PAD_SLEEP_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IO_PAD_SLEEP_DIS`"]
pub struct IO_PAD_SLEEP_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_PAD_SLEEP_DIS_W<'a> {
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
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 0 - 0:0\\]
Controls the I/O pad sleep mode. The boot code will set this bitfield automatically unless waking up from a SHUTDOWN ( RESETCTL.WU_FROM_SD is set). 0: I/O pad sleep mode is enabled, meaning all outputs and pad configurations are latched. Inputs are transparent if pad is configured as input before IO_PAD_SLEEP_DIS is set to 1 1: I/O pad sleep mode is disabled Application software must reconfigure the state for all IO's before setting this bitfield upon waking up from a SHUTDOWN to avoid glitches on pins."]
    #[inline(always)]
    pub fn io_pad_sleep_dis(&self) -> IO_PAD_SLEEP_DIS_R {
        IO_PAD_SLEEP_DIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Controls the I/O pad sleep mode. The boot code will set this bitfield automatically unless waking up from a SHUTDOWN ( RESETCTL.WU_FROM_SD is set). 0: I/O pad sleep mode is enabled, meaning all outputs and pad configurations are latched. Inputs are transparent if pad is configured as input before IO_PAD_SLEEP_DIS is set to 1 1: I/O pad sleep mode is disabled Application software must reconfigure the state for all IO's before setting this bitfield upon waking up from a SHUTDOWN to avoid glitches on pins."]
    #[inline(always)]
    pub fn io_pad_sleep_dis(&mut self) -> IO_PAD_SLEEP_DIS_W {
        IO_PAD_SLEEP_DIS_W { w: self }
    }
}
