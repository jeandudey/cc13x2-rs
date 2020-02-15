#[doc = "Reader of register FUNC_ERR_ADD"]
pub type R = crate::R<u32, super::FUNC_ERR_ADD>;
#[doc = "Writer for register FUNC_ERR_ADD"]
pub type W = crate::W<u32, super::FUNC_ERR_ADD>;
#[doc = "Register FUNC_ERR_ADD `reset()`'s with value 0"]
impl crate::ResetValue for super::FUNC_ERR_ADD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FUNC_ERR_ADD`"]
pub type FUNC_ERR_ADD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FUNC_ERR_ADD`"]
pub struct FUNC_ERR_ADD_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNC_ERR_ADD_W<'a> {
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
    pub fn func_err_add(&self) -> FUNC_ERR_ADD_R {
        FUNC_ERR_ADD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn func_err_add(&mut self) -> FUNC_ERR_ADD_W {
        FUNC_ERR_ADD_W { w: self }
    }
}
