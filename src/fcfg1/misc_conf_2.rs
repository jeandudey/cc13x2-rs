#[doc = "Reader of register MISC_CONF_2"]
pub type R = crate::R<u32, super::MISC_CONF_2>;
#[doc = "Writer for register MISC_CONF_2"]
pub type W = crate::W<u32, super::MISC_CONF_2>;
#[doc = "Register MISC_CONF_2 `reset()`'s with value 0xffff_ff00"]
impl crate::ResetValue for super::MISC_CONF_2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ff00
    }
}
#[doc = "Reader of field `HPOSC_COMP_P3`"]
pub type HPOSC_COMP_P3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HPOSC_COMP_P3`"]
pub struct HPOSC_COMP_P3_W<'a> {
    w: &'a mut W,
}
impl<'a> HPOSC_COMP_P3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_comp_p3(&self) -> HPOSC_COMP_P3_R {
        HPOSC_COMP_P3_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_comp_p3(&mut self) -> HPOSC_COMP_P3_W {
        HPOSC_COMP_P3_W { w: self }
    }
}
