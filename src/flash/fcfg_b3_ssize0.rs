#[doc = "Reader of register FCFG_B3_SSIZE0"]
pub type R = crate::R<u32, super::FCFG_B3_SSIZE0>;
#[doc = "Writer for register FCFG_B3_SSIZE0"]
pub type W = crate::W<u32, super::FCFG_B3_SSIZE0>;
#[doc = "Register FCFG_B3_SSIZE0 `reset()`'s with value 0"]
impl crate::ResetValue for super::FCFG_B3_SSIZE0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B3_SECT_SIZE`"]
pub type B3_SECT_SIZE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `B3_SECT_SIZE`"]
pub struct B3_SECT_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> B3_SECT_SIZE_W<'a> {
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
    pub fn b3_sect_size(&self) -> B3_SECT_SIZE_R {
        B3_SECT_SIZE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b3_sect_size(&mut self) -> B3_SECT_SIZE_W {
        B3_SECT_SIZE_W { w: self }
    }
}
