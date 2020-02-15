#[doc = "Reader of register LAR"]
pub type R = crate::R<u32, super::LAR>;
#[doc = "Writer for register LAR"]
pub type W = crate::W<u32, super::LAR>;
#[doc = "Register LAR `reset()`'s with value 0"]
impl crate::ResetValue for super::LAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LOCK_ACCESS`"]
pub type LOCK_ACCESS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LOCK_ACCESS`"]
pub struct LOCK_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_ACCESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
A privileged write of 0xC5ACCE55 enables more write access to Control Registers TER, TPR and TCR. An invalid write removes write access."]
    #[inline(always)]
    pub fn lock_access(&self) -> LOCK_ACCESS_R {
        LOCK_ACCESS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
A privileged write of 0xC5ACCE55 enables more write access to Control Registers TER, TPR and TCR. An invalid write removes write access."]
    #[inline(always)]
    pub fn lock_access(&mut self) -> LOCK_ACCESS_W {
        LOCK_ACCESS_W { w: self }
    }
}
