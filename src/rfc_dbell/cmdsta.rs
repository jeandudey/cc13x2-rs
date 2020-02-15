#[doc = "Reader of register CMDSTA"]
pub type R = crate::R<u32, super::CMDSTA>;
#[doc = "Writer for register CMDSTA"]
pub type W = crate::W<u32, super::CMDSTA>;
#[doc = "Register CMDSTA `reset()`'s with value 0"]
impl crate::ResetValue for super::CMDSTA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STAT`"]
pub type STAT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `STAT`"]
pub struct STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> STAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Status of the last command used"]
    #[inline(always)]
    pub fn stat(&self) -> STAT_R {
        STAT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Status of the last command used"]
    #[inline(always)]
    pub fn stat(&mut self) -> STAT_W {
        STAT_W { w: self }
    }
}
