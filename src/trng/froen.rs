#[doc = "Reader of register FROEN"]
pub type R = crate::R<u32, super::FROEN>;
#[doc = "Writer for register FROEN"]
pub type W = crate::W<u32, super::FROEN>;
#[doc = "Register FROEN `reset()`'s with value 0x00ff_ffff"]
impl crate::ResetValue for super::FROEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x00ff_ffff
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
#[doc = "Reader of field `FRO_MASK`"]
pub type FRO_MASK_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FRO_MASK`"]
pub struct FRO_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> FRO_MASK_W<'a> {
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
Enable bits for the individual FROs. A '1' in bit \\[n\\]
enables FRO 'n'. Default state is all '1's to enable all FROs after power-up. Note that they are not actually started up before the CTL.TRNG_EN bit is set to '1'. Bits are automatically forced to '0' here (and cannot be written to '1') while the corresponding bit in ALARMSTOP.FRO_FLAGS has value '1'."]
    #[inline(always)]
    pub fn fro_mask(&self) -> FRO_MASK_R {
        FRO_MASK_R::new((self.bits & 0x00ff_ffff) as u32)
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
Enable bits for the individual FROs. A '1' in bit \\[n\\]
enables FRO 'n'. Default state is all '1's to enable all FROs after power-up. Note that they are not actually started up before the CTL.TRNG_EN bit is set to '1'. Bits are automatically forced to '0' here (and cannot be written to '1') while the corresponding bit in ALARMSTOP.FRO_FLAGS has value '1'."]
    #[inline(always)]
    pub fn fro_mask(&mut self) -> FRO_MASK_W {
        FRO_MASK_W { w: self }
    }
}
