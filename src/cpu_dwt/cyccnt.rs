#[doc = "Reader of register CYCCNT"]
pub type R = crate::R<u32, super::CYCCNT>;
#[doc = "Writer for register CYCCNT"]
pub type W = crate::W<u32, super::CYCCNT>;
#[doc = "Register CYCCNT `reset()`'s with value 0"]
impl crate::ResetValue for super::CYCCNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CYCCNT`"]
pub type CYCCNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CYCCNT`"]
pub struct CYCCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CYCCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Current PC Sampler Cycle Counter count value. When enabled, this counter counts the number of core cycles, except when the core is halted. The cycle counter is a free running counter, counting upwards (this counter will not advance in power modes where free-running clock to CPU stops). It wraps around to 0 on overflow. The debugger must initialize this to 0 when first enabling."]
    #[inline(always)]
    pub fn cyccnt(&self) -> CYCCNT_R {
        CYCCNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Current PC Sampler Cycle Counter count value. When enabled, this counter counts the number of core cycles, except when the core is halted. The cycle counter is a free running counter, counting upwards (this counter will not advance in power modes where free-running clock to CPU stops). It wraps around to 0 on overflow. The debugger must initialize this to 0 when first enabling."]
    #[inline(always)]
    pub fn cyccnt(&mut self) -> CYCCNT_W {
        CYCCNT_W { w: self }
    }
}
