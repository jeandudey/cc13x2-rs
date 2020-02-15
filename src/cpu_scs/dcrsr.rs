#[doc = "Reader of register DCRSR"]
pub type R = crate::R<u32, super::DCRSR>;
#[doc = "Writer for register DCRSR"]
pub type W = crate::W<u32, super::DCRSR>;
#[doc = "Register DCRSR `reset()`'s with value 0"]
impl crate::ResetValue for super::DCRSR {
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
#[doc = "Reader of field `REGWNR`"]
pub type REGWNR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REGWNR`"]
pub struct REGWNR_W<'a> {
    w: &'a mut W,
}
impl<'a> REGWNR_W<'a> {
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
#[doc = "Reader of field `RESERVED5`"]
pub type RESERVED5_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED5`"]
pub struct RESERVED5_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 5)) | (((value as u32) & 0x07ff) << 5);
        self.w
    }
}
#[doc = "Reader of field `REGSEL`"]
pub type REGSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REGSEL`"]
pub struct REGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REGSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Write 0."]
    #[inline(always)]
    pub fn reserved17(&self) -> RESERVED17_R {
        RESERVED17_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
1: Write 0: Read"]
    #[inline(always)]
    pub fn regwnr(&self) -> REGWNR_R {
        REGWNR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 5:15 - 15:5\\]
Software should not rely on the value of a reserved. Write 0."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 5) & 0x07ff) as u16)
    }
    #[doc = "Bits 0:4 - 4:0\\]
Register select 0x00: R0 0x01: R1 0x02: R2 0x03: R3 0x04: R4 0x05: R5 0x06: R6 0x07: R7 0x08: R8 0x09: R9 0x0A: R10 0x0B: R11 0x0C: R12 0x0D: Current SP 0x0E: LR 0x0F: DebugReturnAddress 0x10: XPSR/flags, execution state information, and exception number 0x11: MSP (Main SP) 0x12: PSP (Process SP) 0x14: CONTROL<<24 | FAULTMASK<<16 | BASEPRI<<8 | PRIMASK"]
    #[inline(always)]
    pub fn regsel(&self) -> REGSEL_R {
        REGSEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Write 0."]
    #[inline(always)]
    pub fn reserved17(&mut self) -> RESERVED17_W {
        RESERVED17_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
1: Write 0: Read"]
    #[inline(always)]
    pub fn regwnr(&mut self) -> REGWNR_W {
        REGWNR_W { w: self }
    }
    #[doc = "Bits 5:15 - 15:5\\]
Software should not rely on the value of a reserved. Write 0."]
    #[inline(always)]
    pub fn reserved5(&mut self) -> RESERVED5_W {
        RESERVED5_W { w: self }
    }
    #[doc = "Bits 0:4 - 4:0\\]
Register select 0x00: R0 0x01: R1 0x02: R2 0x03: R3 0x04: R4 0x05: R5 0x06: R6 0x07: R7 0x08: R8 0x09: R9 0x0A: R10 0x0B: R11 0x0C: R12 0x0D: Current SP 0x0E: LR 0x0F: DebugReturnAddress 0x10: XPSR/flags, execution state information, and exception number 0x11: MSP (Main SP) 0x12: PSP (Process SP) 0x14: CONTROL<<24 | FAULTMASK<<16 | BASEPRI<<8 | PRIMASK"]
    #[inline(always)]
    pub fn regsel(&mut self) -> REGSEL_W {
        REGSEL_W { w: self }
    }
}
