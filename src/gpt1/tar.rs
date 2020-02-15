#[doc = "Reader of register TAR"]
pub type R = crate::R<u32, super::TAR>;
#[doc = "Writer for register TAR"]
pub type W = crate::W<u32, super::TAR>;
#[doc = "Register TAR `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::TAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `TAR`"]
pub type TAR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TAR`"]
pub struct TAR_W<'a> {
    w: &'a mut W,
}
impl<'a> TAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
GPT Timer A Register Based on the value in the register field TAMR.TAILD, this register is updated with the value from TAILR register either on the next cycle or on the next timeout. A read returns the current value of the Timer A Count Register, in all cases except for Input Edge count and Timer modes. In the Input Edge Count Mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place."]
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
GPT Timer A Register Based on the value in the register field TAMR.TAILD, this register is updated with the value from TAILR register either on the next cycle or on the next timeout. A read returns the current value of the Timer A Count Register, in all cases except for Input Edge count and Timer modes. In the Input Edge Count Mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place."]
    #[inline(always)]
    pub fn tar(&mut self) -> TAR_W {
        TAR_W { w: self }
    }
}
