#[doc = "Reader of register MPU_RBAR_A3"]
pub type R = crate::R<u32, super::MPU_RBAR_A3>;
#[doc = "Writer for register MPU_RBAR_A3"]
pub type W = crate::W<u32, super::MPU_RBAR_A3>;
#[doc = "Register MPU_RBAR_A3 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPU_RBAR_A3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MPU_RBAR_A3`"]
pub type MPU_RBAR_A3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MPU_RBAR_A3`"]
pub struct MPU_RBAR_A3_W<'a> {
    w: &'a mut W,
}
impl<'a> MPU_RBAR_A3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Alias for MPU_RBAR"]
    #[inline(always)]
    pub fn mpu_rbar_a3(&self) -> MPU_RBAR_A3_R {
        MPU_RBAR_A3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Alias for MPU_RBAR"]
    #[inline(always)]
    pub fn mpu_rbar_a3(&mut self) -> MPU_RBAR_A3_W {
        MPU_RBAR_A3_W { w: self }
    }
}
