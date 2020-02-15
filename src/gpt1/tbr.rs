#[doc = "Reader of register TBR"]
pub type R = crate::R<u32, super::TBR>;
#[doc = "Writer for register TBR"]
pub type W = crate::W<u32, super::TBR>;
#[doc = "Register TBR `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::TBR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `TBR`"]
pub type TBR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TBR`"]
pub struct TBR_W<'a> {
    w: &'a mut W,
}
impl<'a> TBR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
GPT Timer B Register Based on the value in the register field TBMR.TBILD, this register is updated with the value from TBILR register either on the next cycle or on the next timeout. A read returns the current value of the Timer B Count Register, in all cases except for Input Edge count and Timer modes. In the Input Edge Count Mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place."]
    #[inline(always)]
    pub fn tbr(&self) -> TBR_R {
        TBR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
GPT Timer B Register Based on the value in the register field TBMR.TBILD, this register is updated with the value from TBILR register either on the next cycle or on the next timeout. A read returns the current value of the Timer B Count Register, in all cases except for Input Edge count and Timer modes. In the Input Edge Count Mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place."]
    #[inline(always)]
    pub fn tbr(&mut self) -> TBR_W {
        TBR_W { w: self }
    }
}
