#[doc = "Reader of register FDIAGCTL"]
pub type R = crate::R<u32, super::FDIAGCTL>;
#[doc = "Writer for register FDIAGCTL"]
pub type W = crate::W<u32, super::FDIAGCTL>;
#[doc = "Register FDIAGCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::FDIAGCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIAGMODE`"]
pub type DIAGMODE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DIAGMODE`"]
pub struct DIAGMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAGMODE_W<'a> {
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
    pub fn diagmode(&self) -> DIAGMODE_R {
        DIAGMODE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn diagmode(&mut self) -> DIAGMODE_W {
        DIAGMODE_W { w: self }
    }
}
