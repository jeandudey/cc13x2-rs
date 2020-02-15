#[doc = "Reader of register CLAIMTAG"]
pub type R = crate::R<u32, super::CLAIMTAG>;
#[doc = "Writer for register CLAIMTAG"]
pub type W = crate::W<u32, super::CLAIMTAG>;
#[doc = "Register CLAIMTAG `reset()`'s with value 0"]
impl crate::ResetValue for super::CLAIMTAG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLAIMTAG`"]
pub type CLAIMTAG_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CLAIMTAG`"]
pub struct CLAIMTAG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLAIMTAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register forms one half of the Claim Tag value. Reading this register returns the current Claim Tag value. Reading CLAIMMASK determines how many bits from this register must be used. The behavior when writing to this register is described in CLAIMCLR."]
    #[inline(always)]
    pub fn claimtag(&self) -> CLAIMTAG_R {
        CLAIMTAG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register forms one half of the Claim Tag value. Reading this register returns the current Claim Tag value. Reading CLAIMMASK determines how many bits from this register must be used. The behavior when writing to this register is described in CLAIMCLR."]
    #[inline(always)]
    pub fn claimtag(&mut self) -> CLAIMTAG_W {
        CLAIMTAG_W { w: self }
    }
}
