#[doc = "Reader of register ANA2_TRIM"]
pub type R = crate::R<u32, super::ANA2_TRIM>;
#[doc = "Writer for register ANA2_TRIM"]
pub type W = crate::W<u32, super::ANA2_TRIM>;
#[doc = "Register ANA2_TRIM `reset()`'s with value 0x8240_787f"]
impl crate::ResetValue for super::ANA2_TRIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8240_787f
    }
}
#[doc = "Reader of field `RCOSCHFCTRIMFRACT_EN`"]
pub type RCOSCHFCTRIMFRACT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCOSCHFCTRIMFRACT_EN`"]
pub struct RCOSCHFCTRIMFRACT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCHFCTRIMFRACT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `RCOSCHFCTRIMFRACT`"]
pub type RCOSCHFCTRIMFRACT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RCOSCHFCTRIMFRACT`"]
pub struct RCOSCHFCTRIMFRACT_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCHFCTRIMFRACT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 26)) | (((value as u32) & 0x1f) << 26);
        self.w
    }
}
#[doc = "Reader of field `RESERVED0`"]
pub type RESERVED0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED0`"]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `SET_RCOSC_HF_FINE_RESISTOR`"]
pub type SET_RCOSC_HF_FINE_RESISTOR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SET_RCOSC_HF_FINE_RESISTOR`"]
pub struct SET_RCOSC_HF_FINE_RESISTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_RCOSC_HF_FINE_RESISTOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 23)) | (((value as u32) & 0x03) << 23);
        self.w
    }
}
#[doc = "Reader of field `ATESTLF_UDIGLDO_IBIAS_TRIM`"]
pub type ATESTLF_UDIGLDO_IBIAS_TRIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ATESTLF_UDIGLDO_IBIAS_TRIM`"]
pub struct ATESTLF_UDIGLDO_IBIAS_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ATESTLF_UDIGLDO_IBIAS_TRIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `NANOAMP_RES_TRIM`"]
pub type NANOAMP_RES_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NANOAMP_RES_TRIM`"]
pub struct NANOAMP_RES_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> NANOAMP_RES_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 15)) | (((value as u32) & 0x7f) << 15);
        self.w
    }
}
#[doc = "Reader of field `RESERVED1`"]
pub type RESERVED1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `DITHER_EN`"]
pub type DITHER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DITHER_EN`"]
pub struct DITHER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DITHER_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `DCDC_IPEAK`"]
pub type DCDC_IPEAK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DCDC_IPEAK`"]
pub struct DCDC_IPEAK_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC_IPEAK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `DEAD_TIME_TRIM`"]
pub type DEAD_TIME_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DEAD_TIME_TRIM`"]
pub struct DEAD_TIME_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DEAD_TIME_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `DCDC_LOW_EN_SEL`"]
pub type DCDC_LOW_EN_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DCDC_LOW_EN_SEL`"]
pub struct DCDC_LOW_EN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC_LOW_EN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `DCDC_HIGH_EN_SEL`"]
pub type DCDC_HIGH_EN_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DCDC_HIGH_EN_SEL`"]
pub struct DCDC_HIGH_EN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC_HIGH_EN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschfctrimfract_en(&self) -> RCOSCHFCTRIMFRACT_EN_R {
        RCOSCHFCTRIMFRACT_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 26:30 - 30:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschfctrimfract(&self) -> RCOSCHFCTRIMFRACT_R {
        RCOSCHFCTRIMFRACT_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 25 - 25:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 23:24 - 24:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn set_rcosc_hf_fine_resistor(&self) -> SET_RCOSC_HF_FINE_RESISTOR_R {
        SET_RCOSC_HF_FINE_RESISTOR_R::new(((self.bits >> 23) & 0x03) as u8)
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn atestlf_udigldo_ibias_trim(&self) -> ATESTLF_UDIGLDO_IBIAS_TRIM_R {
        ATESTLF_UDIGLDO_IBIAS_TRIM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 15:21 - 21:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nanoamp_res_trim(&self) -> NANOAMP_RES_TRIM_R {
        NANOAMP_RES_TRIM_R::new(((self.bits >> 15) & 0x7f) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dither_en(&self) -> DITHER_EN_R {
        DITHER_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dcdc_ipeak(&self) -> DCDC_IPEAK_R {
        DCDC_IPEAK_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dead_time_trim(&self) -> DEAD_TIME_TRIM_R {
        DEAD_TIME_TRIM_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dcdc_low_en_sel(&self) -> DCDC_LOW_EN_SEL_R {
        DCDC_LOW_EN_SEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dcdc_high_en_sel(&self) -> DCDC_HIGH_EN_SEL_R {
        DCDC_HIGH_EN_SEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschfctrimfract_en(&mut self) -> RCOSCHFCTRIMFRACT_EN_W {
        RCOSCHFCTRIMFRACT_EN_W { w: self }
    }
    #[doc = "Bits 26:30 - 30:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschfctrimfract(&mut self) -> RCOSCHFCTRIMFRACT_W {
        RCOSCHFCTRIMFRACT_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
    #[doc = "Bits 23:24 - 24:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn set_rcosc_hf_fine_resistor(&mut self) -> SET_RCOSC_HF_FINE_RESISTOR_W {
        SET_RCOSC_HF_FINE_RESISTOR_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn atestlf_udigldo_ibias_trim(&mut self) -> ATESTLF_UDIGLDO_IBIAS_TRIM_W {
        ATESTLF_UDIGLDO_IBIAS_TRIM_W { w: self }
    }
    #[doc = "Bits 15:21 - 21:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nanoamp_res_trim(&mut self) -> NANOAMP_RES_TRIM_W {
        NANOAMP_RES_TRIM_W { w: self }
    }
    #[doc = "Bits 12:14 - 14:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dither_en(&mut self) -> DITHER_EN_W {
        DITHER_EN_W { w: self }
    }
    #[doc = "Bits 8:10 - 10:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dcdc_ipeak(&mut self) -> DCDC_IPEAK_W {
        DCDC_IPEAK_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dead_time_trim(&mut self) -> DEAD_TIME_TRIM_W {
        DEAD_TIME_TRIM_W { w: self }
    }
    #[doc = "Bits 3:5 - 5:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dcdc_low_en_sel(&mut self) -> DCDC_LOW_EN_SEL_W {
        DCDC_LOW_EN_SEL_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dcdc_high_en_sel(&mut self) -> DCDC_HIGH_EN_SEL_W {
        DCDC_HIGH_EN_SEL_W { w: self }
    }
}
