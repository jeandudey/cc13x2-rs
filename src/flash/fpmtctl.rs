#[doc = "Reader of register FPMTCTL"]
pub type R = crate::R<u32, super::FPMTCTL>;
#[doc = "Writer for register FPMTCTL"]
pub type W = crate::W<u32, super::FPMTCTL>;
#[doc = "Register FPMTCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::FPMTCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDR_INCR`"]
pub type ADDR_INCR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADDR_INCR`"]
pub struct ADDR_INCR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_INCR_W<'a> {
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
    pub fn addr_incr(&self) -> ADDR_INCR_R {
        ADDR_INCR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn addr_incr(&mut self) -> ADDR_INCR_W {
        ADDR_INCR_W { w: self }
    }
}
