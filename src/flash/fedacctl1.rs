#[doc = "Reader of register FEDACCTL1"]
pub type R = crate::R<u32, super::FEDACCTL1>;
#[doc = "Writer for register FEDACCTL1"]
pub type W = crate::W<u32, super::FEDACCTL1>;
#[doc = "Register FEDACCTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::FEDACCTL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED25`"]
pub type RESERVED25_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED25`"]
pub struct RESERVED25_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED25_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | (((value as u32) & 0x7f) << 25);
        self.w
    }
}
#[doc = "Reader of field `SUSP_IGNR`"]
pub type SUSP_IGNR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUSP_IGNR`"]
pub struct SUSP_IGNR_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSP_IGNR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `EDACEN`"]
pub type EDACEN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EDACEN`"]
pub struct EDACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EDACEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:31 - 31:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved25(&self) -> RESERVED25_R {
        RESERVED25_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn susp_ignr(&self) -> SUSP_IGNR_R {
        SUSP_IGNR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn edacen(&self) -> EDACEN_R {
        EDACEN_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 25:31 - 31:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved25(&mut self) -> RESERVED25_W {
        RESERVED25_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn susp_ignr(&mut self) -> SUSP_IGNR_W {
        SUSP_IGNR_W { w: self }
    }
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn edacen(&mut self) -> EDACEN_W {
        EDACEN_W { w: self }
    }
}
