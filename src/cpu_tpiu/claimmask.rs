#[doc = "Reader of register CLAIMMASK"]
pub type R = crate::R<u32, super::CLAIMMASK>;
#[doc = "Writer for register CLAIMMASK"]
pub type W = crate::W<u32, super::CLAIMMASK>;
#[doc = "Register CLAIMMASK `reset()`'s with value 0x0f"]
impl crate::ResetValue for super::CLAIMMASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f
    }
}
#[doc = "Reader of field `CLAIMMASK`"]
pub type CLAIMMASK_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CLAIMMASK`"]
pub struct CLAIMMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CLAIMMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register forms one half of the Claim Tag value. When reading this register returns the number of bits that can be set (each bit is considered separately): 0: This claim tag bit is not implemented 1: This claim tag bit is not implemented The behavior when writing to this register is described in CLAIMSET."]
    #[inline(always)]
    pub fn claimmask(&self) -> CLAIMMASK_R {
        CLAIMMASK_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register forms one half of the Claim Tag value. When reading this register returns the number of bits that can be set (each bit is considered separately): 0: This claim tag bit is not implemented 1: This claim tag bit is not implemented The behavior when writing to this register is described in CLAIMSET."]
    #[inline(always)]
    pub fn claimmask(&mut self) -> CLAIMMASK_W {
        CLAIMMASK_W { w: self }
    }
}
