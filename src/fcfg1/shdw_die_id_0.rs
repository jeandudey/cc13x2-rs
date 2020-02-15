#[doc = "Reader of register SHDW_DIE_ID_0"]
pub type R = crate::R<u32, super::SHDW_DIE_ID_0>;
#[doc = "Writer for register SHDW_DIE_ID_0"]
pub type W = crate::W<u32, super::SHDW_DIE_ID_0>;
#[doc = "Register SHDW_DIE_ID_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SHDW_DIE_ID_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ID_31_0`"]
pub type ID_31_0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ID_31_0`"]
pub struct ID_31_0_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_31_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Shadow of DIE_ID_0 register in eFuse row number 5"]
    #[inline(always)]
    pub fn id_31_0(&self) -> ID_31_0_R {
        ID_31_0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Shadow of DIE_ID_0 register in eFuse row number 5"]
    #[inline(always)]
    pub fn id_31_0(&mut self) -> ID_31_0_W {
        ID_31_0_W { w: self }
    }
}
