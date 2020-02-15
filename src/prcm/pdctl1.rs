#[doc = "Reader of register PDCTL1"]
pub type R = crate::R<u32, super::PDCTL1>;
#[doc = "Writer for register PDCTL1"]
pub type W = crate::W<u32, super::PDCTL1>;
#[doc = "Register PDCTL1 `reset()`'s with value 0x0a"]
impl crate::ResetValue for super::PDCTL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0a
    }
}
#[doc = "Reader of field `RESERVED5`"]
pub type RESERVED5_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED5`"]
pub struct RESERVED5_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff_ffff << 5)) | (((value as u32) & 0x07ff_ffff) << 5);
        self.w
    }
}
#[doc = "Reader of field `VIMS_MODE`"]
pub type VIMS_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VIMS_MODE`"]
pub struct VIMS_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> VIMS_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `RFC_ON`"]
pub type RFC_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFC_ON`"]
pub struct RFC_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> RFC_ON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `CPU_ON`"]
pub type CPU_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPU_ON`"]
pub struct CPU_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_ON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `RESERVED0`"]
pub type RESERVED0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED0`"]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
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
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 5) & 0x07ff_ffff) as u32)
    }
    #[doc = "Bits 3:4 - 4:3\\]
00: VIMS power domain is only powered when CPU power domain is powered. 01: VIMS power domain is powered whenever the BUS power domain is powered. 1X: Block power up of VIMS power domain at next wake up. This mode only has effect when VIMS power domain is not powered. Used for Autonomous RF Core."]
    #[inline(always)]
    pub fn vims_mode(&self) -> VIMS_MODE_R {
        VIMS_MODE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
0: RFC power domain powered off if also PDCTL0.RFC_ON = 0 1: RFC power domain powered on Bit shall be used by RFC in autonomous mode but there is no HW restrictions fom system CPU to access the bit."]
    #[inline(always)]
    pub fn rfc_on(&self) -> RFC_ON_R {
        RFC_ON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Causes a power down of the CPU power domain when system CPU indicates it is idle. 1: Initiates power-on of the CPU power domain. This bit is automatically set by a WIC power-on event."]
    #[inline(always)]
    pub fn cpu_on(&self) -> CPU_ON_R {
        CPU_ON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&mut self) -> RESERVED5_W {
        RESERVED5_W { w: self }
    }
    #[doc = "Bits 3:4 - 4:3\\]
00: VIMS power domain is only powered when CPU power domain is powered. 01: VIMS power domain is powered whenever the BUS power domain is powered. 1X: Block power up of VIMS power domain at next wake up. This mode only has effect when VIMS power domain is not powered. Used for Autonomous RF Core."]
    #[inline(always)]
    pub fn vims_mode(&mut self) -> VIMS_MODE_W {
        VIMS_MODE_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
0: RFC power domain powered off if also PDCTL0.RFC_ON = 0 1: RFC power domain powered on Bit shall be used by RFC in autonomous mode but there is no HW restrictions fom system CPU to access the bit."]
    #[inline(always)]
    pub fn rfc_on(&mut self) -> RFC_ON_W {
        RFC_ON_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
0: Causes a power down of the CPU power domain when system CPU indicates it is idle. 1: Initiates power-on of the CPU power domain. This bit is automatically set by a WIC power-on event."]
    #[inline(always)]
    pub fn cpu_on(&mut self) -> CPU_ON_W {
        CPU_ON_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
}
