#[doc = "Reader of register MPU_RASR_A1"]
pub type R = crate::R<u32, super::MPU_RASR_A1>;
#[doc = "Writer for register MPU_RASR_A1"]
pub type W = crate::W<u32, super::MPU_RASR_A1>;
#[doc = "Register MPU_RASR_A1 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPU_RASR_A1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MPU_RASR_A1`"]
pub type MPU_RASR_A1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MPU_RASR_A1`"]
pub struct MPU_RASR_A1_W<'a> {
    w: &'a mut W,
}
impl<'a> MPU_RASR_A1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Alias for MPU_RASR"]
    #[inline(always)]
    pub fn mpu_rasr_a1(&self) -> MPU_RASR_A1_R {
        MPU_RASR_A1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Alias for MPU_RASR"]
    #[inline(always)]
    pub fn mpu_rasr_a1(&mut self) -> MPU_RASR_A1_W {
        MPU_RASR_A1_W { w: self }
    }
}
