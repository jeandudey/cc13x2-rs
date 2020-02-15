#[doc = "Reader of register FEMU_ECC"]
pub type R = crate::R<u32, super::FEMU_ECC>;
#[doc = "Writer for register FEMU_ECC"]
pub type W = crate::W<u32, super::FEMU_ECC>;
#[doc = "Register FEMU_ECC `reset()`'s with value 0"]
impl crate::ResetValue for super::FEMU_ECC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EMU_ECC`"]
pub type EMU_ECC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EMU_ECC`"]
pub struct EMU_ECC_W<'a> {
    w: &'a mut W,
}
impl<'a> EMU_ECC_W<'a> {
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
    pub fn emu_ecc(&self) -> EMU_ECC_R {
        EMU_ECC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn emu_ecc(&mut self) -> EMU_ECC_W {
        EMU_ECC_W { w: self }
    }
}
