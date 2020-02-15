#[doc = "Reader of register SELFTESTCYC"]
pub type R = crate::R<u32, super::SELFTESTCYC>;
#[doc = "Writer for register SELFTESTCYC"]
pub type W = crate::W<u32, super::SELFTESTCYC>;
#[doc = "Register SELFTESTCYC `reset()`'s with value 0"]
impl crate::ResetValue for super::SELFTESTCYC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CYCLES`"]
pub type CYCLES_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CYCLES`"]
pub struct CYCLES_W<'a> {
    w: &'a mut W,
}
impl<'a> CYCLES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cycles(&self) -> CYCLES_R {
        CYCLES_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cycles(&mut self) -> CYCLES_W {
        CYCLES_W { w: self }
    }
}
