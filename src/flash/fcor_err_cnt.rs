#[doc = "Reader of register FCOR_ERR_CNT"]
pub type R = crate::R<u32, super::FCOR_ERR_CNT>;
#[doc = "Writer for register FCOR_ERR_CNT"]
pub type W = crate::W<u32, super::FCOR_ERR_CNT>;
#[doc = "Register FCOR_ERR_CNT `reset()`'s with value 0"]
impl crate::ResetValue for super::FCOR_ERR_CNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COR_ERR_CNT`"]
pub type COR_ERR_CNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `COR_ERR_CNT`"]
pub struct COR_ERR_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> COR_ERR_CNT_W<'a> {
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
    pub fn cor_err_cnt(&self) -> COR_ERR_CNT_R {
        COR_ERR_CNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cor_err_cnt(&mut self) -> COR_ERR_CNT_W {
        COR_ERR_CNT_W { w: self }
    }
}
