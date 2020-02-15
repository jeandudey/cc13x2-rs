#[doc = "Reader of register FCOR_ERR_ADD"]
pub type R = crate::R<u32, super::FCOR_ERR_ADD>;
#[doc = "Writer for register FCOR_ERR_ADD"]
pub type W = crate::W<u32, super::FCOR_ERR_ADD>;
#[doc = "Register FCOR_ERR_ADD `reset()`'s with value 0"]
impl crate::ResetValue for super::FCOR_ERR_ADD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FCOR_ERR_ADD`"]
pub type FCOR_ERR_ADD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FCOR_ERR_ADD`"]
pub struct FCOR_ERR_ADD_W<'a> {
    w: &'a mut W,
}
impl<'a> FCOR_ERR_ADD_W<'a> {
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
    pub fn fcor_err_add(&self) -> FCOR_ERR_ADD_R {
        FCOR_ERR_ADD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fcor_err_add(&mut self) -> FCOR_ERR_ADD_W {
        FCOR_ERR_ADD_W { w: self }
    }
}
