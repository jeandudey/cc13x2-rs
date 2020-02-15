#[doc = "Reader of register RAMRETEN"]
pub type R = crate::R<u32, super::RAMRETEN>;
#[doc = "Writer for register RAMRETEN"]
pub type W = crate::W<u32, super::RAMRETEN>;
#[doc = "Register RAMRETEN `reset()`'s with value 0x0b"]
impl crate::ResetValue for super::RAMRETEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0b
    }
}
#[doc = "Reader of field `RESERVED4`"]
pub type RESERVED4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED4`"]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff_ffff << 4)) | (((value as u32) & 0x0fff_ffff) << 4);
        self.w
    }
}
#[doc = "Reader of field `RFCULL`"]
pub type RFCULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFCULL`"]
pub struct RFCULL_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCULL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `RFC`"]
pub type RFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFC`"]
pub struct RFC_W<'a> {
    w: &'a mut W,
}
impl<'a> RFC_W<'a> {
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
#[doc = "Reader of field `VIMS`"]
pub type VIMS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VIMS`"]
pub struct VIMS_W<'a> {
    w: &'a mut W,
}
impl<'a> VIMS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
    #[doc = "Bit 3 - 3:3\\]
0: Retention for RFC ULL SRAM disabled 1: Retention for RFC ULL SRAM enabled Memories controlled: CPEULLRAM"]
    #[inline(always)]
    pub fn rfcull(&self) -> RFCULL_R {
        RFCULL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Retention for RFC SRAM disabled 1: Retention for RFC SRAM enabled Memories controlled: CPERAM MCERAM RFERAM DSBRAM"]
    #[inline(always)]
    pub fn rfc(&self) -> RFC_R {
        RFC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - 1:0\\]
0: Memory retention disabled 1: Memory retention enabled Bit 0: VIMS_TRAM Bit 1: VIMS_CRAM Legal modes depend on settings in VIMS:CTL.MODE 00: VIMS:CTL.MODE must be OFF before DEEPSLEEP is asserted - must be set to CACHE or SPLIT mode after waking up again 01: VIMS:CTL.MODE must be GPRAM before DEEPSLEEP is asserted. Must remain in GPRAM mode after wake up, alternatively select OFF mode first and then CACHE or SPILT mode. 10: Illegal mode 11: No restrictions"]
    #[inline(always)]
    pub fn vims(&self) -> VIMS_R {
        VIMS_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
0: Retention for RFC ULL SRAM disabled 1: Retention for RFC ULL SRAM enabled Memories controlled: CPEULLRAM"]
    #[inline(always)]
    pub fn rfcull(&mut self) -> RFCULL_W {
        RFCULL_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
0: Retention for RFC SRAM disabled 1: Retention for RFC SRAM enabled Memories controlled: CPERAM MCERAM RFERAM DSBRAM"]
    #[inline(always)]
    pub fn rfc(&mut self) -> RFC_W {
        RFC_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
0: Memory retention disabled 1: Memory retention enabled Bit 0: VIMS_TRAM Bit 1: VIMS_CRAM Legal modes depend on settings in VIMS:CTL.MODE 00: VIMS:CTL.MODE must be OFF before DEEPSLEEP is asserted - must be set to CACHE or SPLIT mode after waking up again 01: VIMS:CTL.MODE must be GPRAM before DEEPSLEEP is asserted. Must remain in GPRAM mode after wake up, alternatively select OFF mode first and then CACHE or SPILT mode. 10: Illegal mode 11: No restrictions"]
    #[inline(always)]
    pub fn vims(&mut self) -> VIMS_W {
        VIMS_W { w: self }
    }
}
