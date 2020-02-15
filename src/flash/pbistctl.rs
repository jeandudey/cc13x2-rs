#[doc = "Reader of register PBISTCTL"]
pub type R = crate::R<u32, super::PBISTCTL>;
#[doc = "Writer for register PBISTCTL"]
pub type W = crate::W<u32, super::PBISTCTL>;
#[doc = "Register PBISTCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::PBISTCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PBIST_KEY`"]
pub type PBIST_KEY_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PBIST_KEY`"]
pub struct PBIST_KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIST_KEY_W<'a> {
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
    pub fn pbist_key(&self) -> PBIST_KEY_R {
        PBIST_KEY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pbist_key(&mut self) -> PBIST_KEY_W {
        PBIST_KEY_W { w: self }
    }
}
