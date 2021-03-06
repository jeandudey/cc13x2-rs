#[doc = "Reader of register TBV"]
pub type R = crate::R<u32, super::TBV>;
#[doc = "Writer for register TBV"]
pub type W = crate::W<u32, super::TBV>;
#[doc = "Register TBV `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::TBV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `TBV`"]
pub type TBV_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TBV`"]
pub struct TBV_W<'a> {
    w: &'a mut W,
}
impl<'a> TBV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
GPT Timer B Register A read returns the current, free-running value of Timer B in all modes. When written, the value written into this register is loaded into the TBR register on the next clock cycle. Note: In 16-bit mode, only the lower 16-bits of this register can be written with a new value. Writes to the prescaler bits have no effect"]
    #[inline(always)]
    pub fn tbv(&self) -> TBV_R {
        TBV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
GPT Timer B Register A read returns the current, free-running value of Timer B in all modes. When written, the value written into this register is loaded into the TBR register on the next clock cycle. Note: In 16-bit mode, only the lower 16-bits of this register can be written with a new value. Writes to the prescaler bits have no effect"]
    #[inline(always)]
    pub fn tbv(&mut self) -> TBV_W {
        TBV_W { w: self }
    }
}
