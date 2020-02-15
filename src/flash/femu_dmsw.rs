#[doc = "Reader of register FEMU_DMSW"]
pub type R = crate::R<u32, super::FEMU_DMSW>;
#[doc = "Writer for register FEMU_DMSW"]
pub type W = crate::W<u32, super::FEMU_DMSW>;
#[doc = "Register FEMU_DMSW `reset()`'s with value 0"]
impl crate::ResetValue for super::FEMU_DMSW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FEMU_DMSW`"]
pub type FEMU_DMSW_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FEMU_DMSW`"]
pub struct FEMU_DMSW_W<'a> {
    w: &'a mut W,
}
impl<'a> FEMU_DMSW_W<'a> {
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
    pub fn femu_dmsw(&self) -> FEMU_DMSW_R {
        FEMU_DMSW_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn femu_dmsw(&mut self) -> FEMU_DMSW_W {
        FEMU_DMSW_W { w: self }
    }
}
