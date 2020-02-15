#[doc = "Reader of register CLAIMCLR"]
pub type R = crate::R<u32, super::CLAIMCLR>;
#[doc = "Writer for register CLAIMCLR"]
pub type W = crate::W<u32, super::CLAIMCLR>;
#[doc = "Register CLAIMCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::CLAIMCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLAIMCLR`"]
pub type CLAIMCLR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CLAIMCLR`"]
pub struct CLAIMCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLAIMCLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register forms one half of the Claim Tag value. Writing to this location enables individual bits to be cleared (each bit is considered separately): 0: No effect 1: Clear this bit in the claim tag. The behavior when reading from this location is described in CLAIMTAG."]
    #[inline(always)]
    pub fn claimclr(&self) -> CLAIMCLR_R {
        CLAIMCLR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register forms one half of the Claim Tag value. Writing to this location enables individual bits to be cleared (each bit is considered separately): 0: No effect 1: Clear this bit in the claim tag. The behavior when reading from this location is described in CLAIMTAG."]
    #[inline(always)]
    pub fn claimclr(&mut self) -> CLAIMCLR_W {
        CLAIMCLR_W { w: self }
    }
}
