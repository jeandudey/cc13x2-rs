#[doc = "Reader of register RCOSC_HF_TEMPCOMP"]
pub type R = crate::R<u32, super::RCOSC_HF_TEMPCOMP>;
#[doc = "Writer for register RCOSC_HF_TEMPCOMP"]
pub type W = crate::W<u32, super::RCOSC_HF_TEMPCOMP>;
#[doc = "Register RCOSC_HF_TEMPCOMP `reset()`'s with value 0x03"]
impl crate::ResetValue for super::RCOSC_HF_TEMPCOMP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "Reader of field `FINE_RESISTOR`"]
pub type FINE_RESISTOR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FINE_RESISTOR`"]
pub struct FINE_RESISTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> FINE_RESISTOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `CTRIM`"]
pub type CTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTRIM`"]
pub struct CTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CTRIMFRACT_QUAD`"]
pub type CTRIMFRACT_QUAD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTRIMFRACT_QUAD`"]
pub struct CTRIMFRACT_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRIMFRACT_QUAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `CTRIMFRACT_SLOPE`"]
pub type CTRIMFRACT_SLOPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTRIMFRACT_SLOPE`"]
pub struct CTRIMFRACT_SLOPE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRIMFRACT_SLOPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fine_resistor(&self) -> FINE_RESISTOR_R {
        FINE_RESISTOR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctrim(&self) -> CTRIM_R {
        CTRIM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctrimfract_quad(&self) -> CTRIMFRACT_QUAD_R {
        CTRIMFRACT_QUAD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctrimfract_slope(&self) -> CTRIMFRACT_SLOPE_R {
        CTRIMFRACT_SLOPE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fine_resistor(&mut self) -> FINE_RESISTOR_W {
        FINE_RESISTOR_W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctrim(&mut self) -> CTRIM_W {
        CTRIM_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctrimfract_quad(&mut self) -> CTRIMFRACT_QUAD_W {
        CTRIMFRACT_QUAD_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctrimfract_slope(&mut self) -> CTRIMFRACT_SLOPE_W {
        CTRIMFRACT_SLOPE_W { w: self }
    }
}
