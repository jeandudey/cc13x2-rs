#[doc = "Reader of register MVFR1"]
pub type R = crate::R<u32, super::MVFR1>;
#[doc = "Writer for register MVFR1"]
pub type W = crate::W<u32, super::MVFR1>;
#[doc = "Register MVFR1 `reset()`'s with value 0x1100_0011"]
impl crate::ResetValue for super::MVFR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1100_0011
    }
}
#[doc = "Reader of field `FP_FUSED_MAC`"]
pub type FP_FUSED_MAC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FP_FUSED_MAC`"]
pub struct FP_FUSED_MAC_W<'a> {
    w: &'a mut W,
}
impl<'a> FP_FUSED_MAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `FP_HPFP`"]
pub type FP_HPFP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FP_HPFP`"]
pub struct FP_HPFP_W<'a> {
    w: &'a mut W,
}
impl<'a> FP_HPFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `RESERVED8`"]
pub type RESERVED8_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED8`"]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 8)) | (((value as u32) & 0xffff) << 8);
        self.w
    }
}
#[doc = "Reader of field `D_NAN_MODE`"]
pub type D_NAN_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `D_NAN_MODE`"]
pub struct D_NAN_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> D_NAN_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `FTZ_MODE`"]
pub type FTZ_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FTZ_MODE`"]
pub struct FTZ_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FTZ_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - 31:28\\]
Indicates whether the FP supports fused multiply accumulate operations. The value of this field is: 0b0001 - supported."]
    #[inline(always)]
    pub fn fp_fused_mac(&self) -> FP_FUSED_MAC_R {
        FP_FUSED_MAC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Indicates whether the FP supports half-precision floating-point conversion operations. The value of this field is: 0b0001 - supported."]
    #[inline(always)]
    pub fn fp_hpfp(&self) -> FP_HPFP_R {
        FP_HPFP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 8:23 - 23:8\\]
Software should not rely on the value of a reserved."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates whether the FP hardware implementation supports only the Default NaN mode. The value of this field is: 0b0001 - hardware supports propagation of NaN values."]
    #[inline(always)]
    pub fn d_nan_mode(&self) -> D_NAN_MODE_R {
        D_NAN_MODE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Indicates whether the FP hardware implementation supports only the Flush-to-Zero mode of operation. The value of this field is: 0b0001 - hardware supports full denormalized number arithmetic."]
    #[inline(always)]
    pub fn ftz_mode(&self) -> FTZ_MODE_R {
        FTZ_MODE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - 31:28\\]
Indicates whether the FP supports fused multiply accumulate operations. The value of this field is: 0b0001 - supported."]
    #[inline(always)]
    pub fn fp_fused_mac(&mut self) -> FP_FUSED_MAC_W {
        FP_FUSED_MAC_W { w: self }
    }
    #[doc = "Bits 24:27 - 27:24\\]
Indicates whether the FP supports half-precision floating-point conversion operations. The value of this field is: 0b0001 - supported."]
    #[inline(always)]
    pub fn fp_hpfp(&mut self) -> FP_HPFP_W {
        FP_HPFP_W { w: self }
    }
    #[doc = "Bits 8:23 - 23:8\\]
Software should not rely on the value of a reserved."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates whether the FP hardware implementation supports only the Default NaN mode. The value of this field is: 0b0001 - hardware supports propagation of NaN values."]
    #[inline(always)]
    pub fn d_nan_mode(&mut self) -> D_NAN_MODE_W {
        D_NAN_MODE_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Indicates whether the FP hardware implementation supports only the Flush-to-Zero mode of operation. The value of this field is: 0b0001 - hardware supports full denormalized number arithmetic."]
    #[inline(always)]
    pub fn ftz_mode(&mut self) -> FTZ_MODE_W {
        FTZ_MODE_W { w: self }
    }
}
