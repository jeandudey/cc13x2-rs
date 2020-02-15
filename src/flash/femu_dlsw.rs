#[doc = "Reader of register FEMU_DLSW"]
pub type R = crate::R<u32, super::FEMU_DLSW>;
#[doc = "Writer for register FEMU_DLSW"]
pub type W = crate::W<u32, super::FEMU_DLSW>;
#[doc = "Register FEMU_DLSW `reset()`'s with value 0"]
impl crate::ResetValue for super::FEMU_DLSW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FEMU_DLSW`"]
pub type FEMU_DLSW_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FEMU_DLSW`"]
pub struct FEMU_DLSW_W<'a> {
    w: &'a mut W,
}
impl<'a> FEMU_DLSW_W<'a> {
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
    pub fn femu_dlsw(&self) -> FEMU_DLSW_R {
        FEMU_DLSW_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn femu_dlsw(&mut self) -> FEMU_DLSW_W {
        FEMU_DLSW_W { w: self }
    }
}
