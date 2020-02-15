#[doc = "Reader of register COMP0"]
pub type R = crate::R<u32, super::COMP0>;
#[doc = "Writer for register COMP0"]
pub type W = crate::W<u32, super::COMP0>;
#[doc = "Register COMP0 `reset()`'s with value 0"]
impl crate::ResetValue for super::COMP0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMP`"]
pub type COMP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `COMP`"]
pub struct COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reference value to compare against PC or the data address as given by FUNCTION0. Comparator 0 can also compare against the value of the PC Sampler Counter (CYCCNT)."]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reference value to compare against PC or the data address as given by FUNCTION0. Comparator 0 can also compare against the value of the PC Sampler Counter (CYCCNT)."]
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W {
        COMP_W { w: self }
    }
}
