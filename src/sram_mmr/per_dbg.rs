#[doc = "Reader of register PER_DBG"]
pub type R = crate::R<u32, super::PER_DBG>;
#[doc = "Writer for register PER_DBG"]
pub type W = crate::W<u32, super::PER_DBG>;
#[doc = "Register PER_DBG `reset()`'s with value 0"]
impl crate::ResetValue for super::PER_DBG {
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
#[doc = "Reader of field `PER_DEBUG_ADDR`"]
pub type PER_DEBUG_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PER_DEBUG_ADDR`"]
pub struct PER_DEBUG_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_DEBUG_ADDR_W<'a> {
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
Debug Parity Error Address Offset When PER_CTL.PER_DEBUG is 1, this field is used to set a parity debug address offset. The address offset must be a word-aligned address. Writes within this address offset will force incorrect parity bits to be stored together with the data written. The following reads within this same address offset will thus result in parity errors to be generated."]
    #[inline(always)]
    pub fn per_debug_addr(&self) -> PER_DEBUG_ADDR_R {
        PER_DEBUG_ADDR_R::new((self.bits & 0x00ff_ffff) as u32)
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
Debug Parity Error Address Offset When PER_CTL.PER_DEBUG is 1, this field is used to set a parity debug address offset. The address offset must be a word-aligned address. Writes within this address offset will force incorrect parity bits to be stored together with the data written. The following reads within this same address offset will thus result in parity errors to be generated."]
    #[inline(always)]
    pub fn per_debug_addr(&mut self) -> PER_DEBUG_ADDR_W {
        PER_DEBUG_ADDR_W { w: self }
    }
}
