#[doc = "Reader of register PER_CHK"]
pub type R = crate::R<u32, super::PER_CHK>;
#[doc = "Writer for register PER_CHK"]
pub type W = crate::W<u32, super::PER_CHK>;
#[doc = "Register PER_CHK `reset()`'s with value 0"]
impl crate::ResetValue for super::PER_CHK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED24`"]
pub type RESERVED24_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED24`"]
pub struct RESERVED24_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `PER_ADDR`"]
pub type PER_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PER_ADDR`"]
pub struct PER_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 0:23 - 23:0\\]
Parity Error Address Offset Returns the last address offset which resulted in a parity error during an SRAM read. The address offset returned is always the word-aligned address that contains the location with the parity error. For parity faults on non word-aligned accesses, CPU_SCS:BFAR.ADDRESS will hold the address of the location that resulted in parity error."]
    #[inline(always)]
    pub fn per_addr(&self) -> PER_ADDR_R {
        PER_ADDR_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&mut self) -> RESERVED24_W {
        RESERVED24_W { w: self }
    }
    #[doc = "Bits 0:23 - 23:0\\]
Parity Error Address Offset Returns the last address offset which resulted in a parity error during an SRAM read. The address offset returned is always the word-aligned address that contains the location with the parity error. For parity faults on non word-aligned accesses, CPU_SCS:BFAR.ADDRESS will hold the address of the location that resulted in parity error."]
    #[inline(always)]
    pub fn per_addr(&mut self) -> PER_ADDR_W {
        PER_ADDR_W { w: self }
    }
}
