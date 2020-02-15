#[doc = "Reader of register FCOR_ERR_POS"]
pub type R = crate::R<u32, super::FCOR_ERR_POS>;
#[doc = "Writer for register FCOR_ERR_POS"]
pub type W = crate::W<u32, super::FCOR_ERR_POS>;
#[doc = "Register FCOR_ERR_POS `reset()`'s with value 0"]
impl crate::ResetValue for super::FCOR_ERR_POS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SERR_POS`"]
pub type SERR_POS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SERR_POS`"]
pub struct SERR_POS_W<'a> {
    w: &'a mut W,
}
impl<'a> SERR_POS_W<'a> {
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
    pub fn serr_pos(&self) -> SERR_POS_R {
        SERR_POS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn serr_pos(&mut self) -> SERR_POS_W {
        SERR_POS_W { w: self }
    }
}
