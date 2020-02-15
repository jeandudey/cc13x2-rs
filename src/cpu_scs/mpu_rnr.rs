#[doc = "Reader of register MPU_RNR"]
pub type R = crate::R<u32, super::MPU_RNR>;
#[doc = "Writer for register MPU_RNR"]
pub type W = crate::W<u32, super::MPU_RNR>;
#[doc = "Register MPU_RNR `reset()`'s with value 0"]
impl crate::ResetValue for super::MPU_RNR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED8`"]
pub type RESERVED8_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED8`"]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Reader of field `REGION`"]
pub type REGION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REGION`"]
pub struct REGION_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Region select field. This field selects the region to operate on when using the MPU_RASR and MPU_RBAR. It must be written first except when the address MPU_RBAR.VALID and MPU_RBAR.REGION fields are written, which overwrites this."]
    #[inline(always)]
    pub fn region(&self) -> REGION_R {
        REGION_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Region select field. This field selects the region to operate on when using the MPU_RASR and MPU_RBAR. It must be written first except when the address MPU_RBAR.VALID and MPU_RBAR.REGION fields are written, which overwrites this."]
    #[inline(always)]
    pub fn region(&mut self) -> REGION_W {
        REGION_W { w: self }
    }
}
