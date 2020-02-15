#[doc = "Reader of register CLAIMSET"]
pub type R = crate::R<u32, super::CLAIMSET>;
#[doc = "Writer for register CLAIMSET"]
pub type W = crate::W<u32, super::CLAIMSET>;
#[doc = "Register CLAIMSET `reset()`'s with value 0x0f"]
impl crate::ResetValue for super::CLAIMSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f
    }
}
#[doc = "Reader of field `CLAIMSET`"]
pub type CLAIMSET_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CLAIMSET`"]
pub struct CLAIMSET_W<'a> {
    w: &'a mut W,
}
impl<'a> CLAIMSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register forms one half of the Claim Tag value. Writing to this location allows individual bits to be set (each bit is considered separately): 0: No effect 1: Set this bit in the claim tag The behavior when reading from this location is described in CLAIMMASK."]
    #[inline(always)]
    pub fn claimset(&self) -> CLAIMSET_R {
        CLAIMSET_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register forms one half of the Claim Tag value. Writing to this location allows individual bits to be set (each bit is considered separately): 0: No effect 1: Set this bit in the claim tag The behavior when reading from this location is described in CLAIMMASK."]
    #[inline(always)]
    pub fn claimset(&mut self) -> CLAIMSET_W {
        CLAIMSET_W { w: self }
    }
}
