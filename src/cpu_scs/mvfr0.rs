#[doc = "Reader of register MVFR0"]
pub type R = crate::R<u32, super::MVFR0>;
#[doc = "Writer for register MVFR0"]
pub type W = crate::W<u32, super::MVFR0>;
#[doc = "Register MVFR0 `reset()`'s with value 0x1011_0021"]
impl crate::ResetValue for super::MVFR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1011_0021
    }
}
#[doc = "Reader of field `FP_ROUNDING_MODES`"]
pub type FP_ROUNDING_MODES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FP_ROUNDING_MODES`"]
pub struct FP_ROUNDING_MODES_W<'a> {
    w: &'a mut W,
}
impl<'a> FP_ROUNDING_MODES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `SHORT_VECTORS`"]
pub type SHORT_VECTORS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SHORT_VECTORS`"]
pub struct SHORT_VECTORS_W<'a> {
    w: &'a mut W,
}
impl<'a> SHORT_VECTORS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `SQUARE_ROOT`"]
pub type SQUARE_ROOT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SQUARE_ROOT`"]
pub struct SQUARE_ROOT_W<'a> {
    w: &'a mut W,
}
impl<'a> SQUARE_ROOT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `DIVIDE`"]
pub type DIVIDE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIVIDE`"]
pub struct DIVIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVIDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `FP_EXCEPTION_TRAPPING`"]
pub type FP_EXCEPTION_TRAPPING_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FP_EXCEPTION_TRAPPING`"]
pub struct FP_EXCEPTION_TRAPPING_W<'a> {
    w: &'a mut W,
}
impl<'a> FP_EXCEPTION_TRAPPING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `DOUBLE_PRECISION`"]
pub type DOUBLE_PRECISION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DOUBLE_PRECISION`"]
pub struct DOUBLE_PRECISION_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUBLE_PRECISION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `SINGLE_PRECISION`"]
pub type SINGLE_PRECISION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SINGLE_PRECISION`"]
pub struct SINGLE_PRECISION_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGLE_PRECISION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `A_SIMD`"]
pub type A_SIMD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `A_SIMD`"]
pub struct A_SIMD_W<'a> {
    w: &'a mut W,
}
impl<'a> A_SIMD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - 31:28\\]
Indicates the rounding modes supported by the FP floating-point hardware. The value of this field is: 0b0001 - all rounding modes supported."]
    #[inline(always)]
    pub fn fp_rounding_modes(&self) -> FP_ROUNDING_MODES_R {
        FP_ROUNDING_MODES_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Indicates the hardware support for FP short vectors. The value of this field is: 0b0000 - not supported."]
    #[inline(always)]
    pub fn short_vectors(&self) -> SHORT_VECTORS_R {
        SHORT_VECTORS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Indicates the hardware support for FP square root operations. The value of this field is: 0b0001 - supported."]
    #[inline(always)]
    pub fn square_root(&self) -> SQUARE_ROOT_R {
        SQUARE_ROOT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Indicates the hardware support for FP divide operations. The value of this field is: 0b0001 - supported."]
    #[inline(always)]
    pub fn divide(&self) -> DIVIDE_R {
        DIVIDE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Indicates whether the FP hardware implementation supports exception trapping. The value of this field is: 0b0000 - not supported."]
    #[inline(always)]
    pub fn fp_exception_trapping(&self) -> FP_EXCEPTION_TRAPPING_R {
        FP_EXCEPTION_TRAPPING_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Indicates the hardware support for FP double-precision operations. The value of this field is: 0b0000 - not supported."]
    #[inline(always)]
    pub fn double_precision(&self) -> DOUBLE_PRECISION_R {
        DOUBLE_PRECISION_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates the hardware support for FP single-precision operations. The value of this field is: 0b0010 - supported."]
    #[inline(always)]
    pub fn single_precision(&self) -> SINGLE_PRECISION_R {
        SINGLE_PRECISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Indicates the size of the FP register bank. The value of this field is: 0b0001 - supported, 16 x 64-bit registers."]
    #[inline(always)]
    pub fn a_simd(&self) -> A_SIMD_R {
        A_SIMD_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - 31:28\\]
Indicates the rounding modes supported by the FP floating-point hardware. The value of this field is: 0b0001 - all rounding modes supported."]
    #[inline(always)]
    pub fn fp_rounding_modes(&mut self) -> FP_ROUNDING_MODES_W {
        FP_ROUNDING_MODES_W { w: self }
    }
    #[doc = "Bits 24:27 - 27:24\\]
Indicates the hardware support for FP short vectors. The value of this field is: 0b0000 - not supported."]
    #[inline(always)]
    pub fn short_vectors(&mut self) -> SHORT_VECTORS_W {
        SHORT_VECTORS_W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\]
Indicates the hardware support for FP square root operations. The value of this field is: 0b0001 - supported."]
    #[inline(always)]
    pub fn square_root(&mut self) -> SQUARE_ROOT_W {
        SQUARE_ROOT_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\]
Indicates the hardware support for FP divide operations. The value of this field is: 0b0001 - supported."]
    #[inline(always)]
    pub fn divide(&mut self) -> DIVIDE_W {
        DIVIDE_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\]
Indicates whether the FP hardware implementation supports exception trapping. The value of this field is: 0b0000 - not supported."]
    #[inline(always)]
    pub fn fp_exception_trapping(&mut self) -> FP_EXCEPTION_TRAPPING_W {
        FP_EXCEPTION_TRAPPING_W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\]
Indicates the hardware support for FP double-precision operations. The value of this field is: 0b0000 - not supported."]
    #[inline(always)]
    pub fn double_precision(&mut self) -> DOUBLE_PRECISION_W {
        DOUBLE_PRECISION_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates the hardware support for FP single-precision operations. The value of this field is: 0b0010 - supported."]
    #[inline(always)]
    pub fn single_precision(&mut self) -> SINGLE_PRECISION_W {
        SINGLE_PRECISION_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Indicates the size of the FP register bank. The value of this field is: 0b0001 - supported, 16 x 64-bit registers."]
    #[inline(always)]
    pub fn a_simd(&mut self) -> A_SIMD_W {
        A_SIMD_W { w: self }
    }
}
