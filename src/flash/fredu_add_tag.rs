#[doc = "Reader of register FREDU_ADD_TAG"]
pub type R = crate::R<u32, super::FREDU_ADD_TAG>;
#[doc = "Writer for register FREDU_ADD_TAG"]
pub type W = crate::W<u32, super::FREDU_ADD_TAG>;
#[doc = "Register FREDU_ADD_TAG `reset()`'s with value 0"]
impl crate::ResetValue for super::FREDU_ADD_TAG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REDU_ADD_TAG`"]
pub type REDU_ADD_TAG_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `REDU_ADD_TAG`"]
pub struct REDU_ADD_TAG_W<'a> {
    w: &'a mut W,
}
impl<'a> REDU_ADD_TAG_W<'a> {
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
    pub fn redu_add_tag(&self) -> REDU_ADD_TAG_R {
        REDU_ADD_TAG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn redu_add_tag(&mut self) -> REDU_ADD_TAG_W {
        REDU_ADD_TAG_W { w: self }
    }
}
