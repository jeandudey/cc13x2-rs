#[doc = "Reader of register JTAGCFG"]
pub type R = crate::R<u32, super::JTAGCFG>;
#[doc = "Writer for register JTAGCFG"]
pub type W = crate::W<u32, super::JTAGCFG>;
#[doc = "Register JTAGCFG `reset()`'s with value 0x0100"]
impl crate::ResetValue for super::JTAGCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100
    }
}
#[doc = "Reader of field `RESERVED9`"]
pub type RESERVED9_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED9`"]
pub struct RESERVED9_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x007f_ffff << 9)) | (((value as u32) & 0x007f_ffff) << 9);
        self.w
    }
}
#[doc = "Reader of field `JTAG_PD_FORCE_ON`"]
pub type JTAG_PD_FORCE_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JTAG_PD_FORCE_ON`"]
pub struct JTAG_PD_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> JTAG_PD_FORCE_ON_W<'a> {
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
#[doc = "Reader of field `RESERVED0`"]
pub type RESERVED0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED0`"]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x007f_ffff) as u32)
    }
    #[doc = "Bit 8 - 8:8\\]
Controls JTAG Power domain power state: 0: Controlled exclusively by debug subsystem. (JTAG Power domain will be powered off unless a debugger is attached) 1: JTAG Power Domain is forced on, independent of debug subsystem. Note: The reset value causes JTAG Power domain to be powered on by default. Software must clear this bit to turn off the JTAG Power domain"]
    #[inline(always)]
    pub fn jtag_pd_force_on(&self) -> JTAG_PD_FORCE_ON_R {
        JTAG_PD_FORCE_ON_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&mut self) -> RESERVED9_W {
        RESERVED9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Controls JTAG Power domain power state: 0: Controlled exclusively by debug subsystem. (JTAG Power domain will be powered off unless a debugger is attached) 1: JTAG Power Domain is forced on, independent of debug subsystem. Note: The reset value causes JTAG Power domain to be powered on by default. Software must clear this bit to turn off the JTAG Power domain"]
    #[inline(always)]
    pub fn jtag_pd_force_on(&mut self) -> JTAG_PD_FORCE_ON_W {
        JTAG_PD_FORCE_ON_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
}
