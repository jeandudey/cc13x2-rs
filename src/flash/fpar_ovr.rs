#[doc = "Reader of register FPAR_OVR"]
pub type R = crate::R<u32, super::FPAR_OVR>;
#[doc = "Writer for register FPAR_OVR"]
pub type W = crate::W<u32, super::FPAR_OVR>;
#[doc = "Register FPAR_OVR `reset()`'s with value 0"]
impl crate::ResetValue for super::FPAR_OVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DAT_INV_PAR`"]
pub type DAT_INV_PAR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DAT_INV_PAR`"]
pub struct DAT_INV_PAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_INV_PAR_W<'a> {
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
    pub fn dat_inv_par(&self) -> DAT_INV_PAR_R {
        DAT_INV_PAR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dat_inv_par(&mut self) -> DAT_INV_PAR_W {
        DAT_INV_PAR_W { w: self }
    }
}
