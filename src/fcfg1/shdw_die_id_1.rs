#[doc = "Reader of register SHDW_DIE_ID_1"]
pub type R = crate::R<u32, super::SHDW_DIE_ID_1>;
#[doc = "Writer for register SHDW_DIE_ID_1"]
pub type W = crate::W<u32, super::SHDW_DIE_ID_1>;
#[doc = "Register SHDW_DIE_ID_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SHDW_DIE_ID_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ID_63_32`"]
pub type ID_63_32_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ID_63_32`"]
pub struct ID_63_32_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_63_32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Shadow of DIE_ID_1 register in eFuse row number 6"]
    #[inline(always)]
    pub fn id_63_32(&self) -> ID_63_32_R {
        ID_63_32_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Shadow of DIE_ID_1 register in eFuse row number 6"]
    #[inline(always)]
    pub fn id_63_32(&mut self) -> ID_63_32_W {
        ID_63_32_W { w: self }
    }
}
