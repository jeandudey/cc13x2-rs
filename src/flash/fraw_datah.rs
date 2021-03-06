#[doc = "Reader of register FRAW_DATAH"]
pub type R = crate::R<u32, super::FRAW_DATAH>;
#[doc = "Writer for register FRAW_DATAH"]
pub type W = crate::W<u32, super::FRAW_DATAH>;
#[doc = "Register FRAW_DATAH `reset()`'s with value 0"]
impl crate::ResetValue for super::FRAW_DATAH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FRAW_DATAH`"]
pub type FRAW_DATAH_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FRAW_DATAH`"]
pub struct FRAW_DATAH_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAW_DATAH_W<'a> {
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
    pub fn fraw_datah(&self) -> FRAW_DATAH_R {
        FRAW_DATAH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fraw_datah(&mut self) -> FRAW_DATAH_W {
        FRAW_DATAH_W { w: self }
    }
}
