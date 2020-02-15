#[doc = "Reader of register FEDACSDIS"]
pub type R = crate::R<u32, super::FEDACSDIS>;
#[doc = "Writer for register FEDACSDIS"]
pub type W = crate::W<u32, super::FEDACSDIS>;
#[doc = "Register FEDACSDIS `reset()`'s with value 0"]
impl crate::ResetValue for super::FEDACSDIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SECTORID0`"]
pub type SECTORID0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SECTORID0`"]
pub struct SECTORID0_W<'a> {
    w: &'a mut W,
}
impl<'a> SECTORID0_W<'a> {
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
    pub fn sectorid0(&self) -> SECTORID0_R {
        SECTORID0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sectorid0(&mut self) -> SECTORID0_W {
        SECTORID0_W { w: self }
    }
}
