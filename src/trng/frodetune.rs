#[doc = "Reader of register FRODETUNE"]
pub type R = crate::R<u32, super::FRODETUNE>;
#[doc = "Writer for register FRODETUNE"]
pub type W = crate::W<u32, super::FRODETUNE>;
#[doc = "Register FRODETUNE `reset()`'s with value 0"]
impl crate::ResetValue for super::FRODETUNE {
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
De-tune bits for the individual FROs. A '1' in bit \\[n\\]
lets FRO 'n' run approximately 5% faster. The value of one of these bits may only be changed while the corresponding FRO is turned off (by temporarily writing a '0' in the corresponding bit of the FROEN.FRO_MASK register)."]
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
De-tune bits for the individual FROs. A '1' in bit \\[n\\]
lets FRO 'n' run approximately 5% faster. The value of one of these bits may only be changed while the corresponding FRO is turned off (by temporarily writing a '0' in the corresponding bit of the FROEN.FRO_MASK register)."]
    #[inline(always)]
    pub fn fro_mask(&mut self) -> FRO_MASK_W {
        FRO_MASK_W { w: self }
    }
}
