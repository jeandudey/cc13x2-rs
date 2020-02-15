#[doc = "Reader of register FCLKTRIM"]
pub type R = crate::R<u32, super::FCLKTRIM>;
#[doc = "Writer for register FCLKTRIM"]
pub type W = crate::W<u32, super::FCLKTRIM>;
#[doc = "Register FCLKTRIM `reset()`'s with value 0"]
impl crate::ResetValue for super::FCLKTRIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRIM_EN`"]
pub type TRIM_EN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TRIM_EN`"]
pub struct TRIM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_EN_W<'a> {
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
    pub fn trim_en(&self) -> TRIM_EN_R {
        TRIM_EN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim_en(&mut self) -> TRIM_EN_W {
        TRIM_EN_W { w: self }
    }
}
