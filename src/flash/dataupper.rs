#[doc = "Reader of register DATAUPPER"]
pub type R = crate::R<u32, super::DATAUPPER>;
#[doc = "Writer for register DATAUPPER"]
pub type W = crate::W<u32, super::DATAUPPER>;
#[doc = "Register DATAUPPER `reset()`'s with value 0"]
impl crate::ResetValue for super::DATAUPPER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED8`"]
pub type RESERVED8_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED8`"]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SPARE`"]
pub type SPARE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPARE`"]
pub struct SPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Reader of field `P`"]
pub type P_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P`"]
pub struct P_W<'a> {
    w: &'a mut W,
}
impl<'a> P_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `R`"]
pub type R_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R`"]
pub struct R_W<'a> {
    w: &'a mut W,
}
impl<'a> R_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `EEN`"]
pub type EEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EEN`"]
pub struct EEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn r(&self) -> R_R {
        R_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn een(&self) -> EEN_R {
        EEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 3:7 - 7:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn spare(&mut self) -> SPARE_W {
        SPARE_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn p(&mut self) -> P_W {
        P_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn r(&mut self) -> R_W {
        R_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn een(&mut self) -> EEN_W {
        EEN_W { w: self }
    }
}
