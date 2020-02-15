#[doc = "Reader of register SHDW_ANA_TRIM"]
pub type R = crate::R<u32, super::SHDW_ANA_TRIM>;
#[doc = "Writer for register SHDW_ANA_TRIM"]
pub type W = crate::W<u32, super::SHDW_ANA_TRIM>;
#[doc = "Register SHDW_ANA_TRIM `reset()`'s with value 0"]
impl crate::ResetValue for super::SHDW_ANA_TRIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BOD_BANDGAP_TRIM_CNF`"]
pub type BOD_BANDGAP_TRIM_CNF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BOD_BANDGAP_TRIM_CNF`"]
pub struct BOD_BANDGAP_TRIM_CNF_W<'a> {
    w: &'a mut W,
}
impl<'a> BOD_BANDGAP_TRIM_CNF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
#[doc = "Reader of field `VDDR_ENABLE_PG1`"]
pub type VDDR_ENABLE_PG1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDDR_ENABLE_PG1`"]
pub struct VDDR_ENABLE_PG1_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_ENABLE_PG1_W<'a> {
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
#[doc = "Reader of field `VDDR_OK_HYS`"]
pub type VDDR_OK_HYS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDDR_OK_HYS`"]
pub struct VDDR_OK_HYS_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_OK_HYS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `IPTAT_TRIM`"]
pub type IPTAT_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IPTAT_TRIM`"]
pub struct IPTAT_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> IPTAT_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "Reader of field `VDDR_TRIM`"]
pub type VDDR_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VDDR_TRIM`"]
pub struct VDDR_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `TRIMBOD_INTMODE`"]
pub type TRIMBOD_INTMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIMBOD_INTMODE`"]
pub struct TRIMBOD_INTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMBOD_INTMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
        self.w
    }
}
#[doc = "Reader of field `TRIMBOD_EXTMODE`"]
pub type TRIMBOD_EXTMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIMBOD_EXTMODE`"]
pub struct TRIMBOD_EXTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMBOD_EXTMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | (((value as u32) & 0x1f) << 6);
        self.w
    }
}
#[doc = "Reader of field `TRIMTEMP`"]
pub type TRIMTEMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIMTEMP`"]
pub struct TRIMTEMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMTEMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:26 - 26:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bod_bandgap_trim_cnf(&self) -> BOD_BANDGAP_TRIM_CNF_R {
        BOD_BANDGAP_TRIM_CNF_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_enable_pg1(&self) -> VDDR_ENABLE_PG1_R {
        VDDR_ENABLE_PG1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_ok_hys(&self) -> VDDR_OK_HYS_R {
        VDDR_OK_HYS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - 22:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn iptat_trim(&self) -> IPTAT_TRIM_R {
        IPTAT_TRIM_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_trim(&self) -> VDDR_TRIM_R {
        VDDR_TRIM_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimbod_intmode(&self) -> TRIMBOD_INTMODE_R {
        TRIMBOD_INTMODE_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - 10:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimbod_extmode(&self) -> TRIMBOD_EXTMODE_R {
        TRIMBOD_EXTMODE_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimtemp(&self) -> TRIMTEMP_R {
        TRIMTEMP_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:26 - 26:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bod_bandgap_trim_cnf(&mut self) -> BOD_BANDGAP_TRIM_CNF_W {
        BOD_BANDGAP_TRIM_CNF_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_enable_pg1(&mut self) -> VDDR_ENABLE_PG1_W {
        VDDR_ENABLE_PG1_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_ok_hys(&mut self) -> VDDR_OK_HYS_W {
        VDDR_OK_HYS_W { w: self }
    }
    #[doc = "Bits 21:22 - 22:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn iptat_trim(&mut self) -> IPTAT_TRIM_W {
        IPTAT_TRIM_W { w: self }
    }
    #[doc = "Bits 16:20 - 20:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_trim(&mut self) -> VDDR_TRIM_W {
        VDDR_TRIM_W { w: self }
    }
    #[doc = "Bits 11:15 - 15:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimbod_intmode(&mut self) -> TRIMBOD_INTMODE_W {
        TRIMBOD_INTMODE_W { w: self }
    }
    #[doc = "Bits 6:10 - 10:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimbod_extmode(&mut self) -> TRIMBOD_EXTMODE_W {
        TRIMBOD_EXTMODE_W { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimtemp(&mut self) -> TRIMTEMP_W {
        TRIMTEMP_W { w: self }
    }
}
