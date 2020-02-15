#[doc = "Reader of register FCFG_WRAPPER"]
pub type R = crate::R<u32, super::FCFG_WRAPPER>;
#[doc = "Writer for register FCFG_WRAPPER"]
pub type W = crate::W<u32, super::FCFG_WRAPPER>;
#[doc = "Register FCFG_WRAPPER `reset()`'s with value 0x5000_9007"]
impl crate::ResetValue for super::FCFG_WRAPPER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x5000_9007
    }
}
#[doc = "Reader of field `FAMILY_TYPE`"]
pub type FAMILY_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FAMILY_TYPE`"]
pub struct FAMILY_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> FAMILY_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `RESERVED21`"]
pub type RESERVED21_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED21`"]
pub struct RESERVED21_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | (((value as u32) & 0x07) << 21);
        self.w
    }
}
#[doc = "Reader of field `MEM_MAP`"]
pub type MEM_MAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MEM_MAP`"]
pub struct MEM_MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_MAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `CPU2`"]
pub type CPU2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CPU2`"]
pub struct CPU2_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `EE_IN_MAIN`"]
pub type EE_IN_MAIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EE_IN_MAIN`"]
pub struct EE_IN_MAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> EE_IN_MAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `ROM`"]
pub type ROM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ROM`"]
pub struct ROM_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_W<'a> {
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
#[doc = "Reader of field `IFLUSH`"]
pub type IFLUSH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IFLUSH`"]
pub struct IFLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> IFLUSH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `SIL3`"]
pub type SIL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIL3`"]
pub struct SIL3_W<'a> {
    w: &'a mut W,
}
impl<'a> SIL3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `ECCA`"]
pub type ECCA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECCA`"]
pub struct ECCA_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `AUTO_SUSP`"]
pub type AUTO_SUSP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AUTO_SUSP`"]
pub struct AUTO_SUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_SUSP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `UERR`"]
pub type UERR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UERR`"]
pub struct UERR_W<'a> {
    w: &'a mut W,
}
impl<'a> UERR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `CPU_TYPE1`"]
pub type CPU_TYPE1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CPU_TYPE1`"]
pub struct CPU_TYPE1_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_TYPE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn family_type(&self) -> FAMILY_TYPE_R {
        FAMILY_TYPE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved21(&self) -> RESERVED21_R {
        RESERVED21_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn mem_map(&self) -> MEM_MAP_R {
        MEM_MAP_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cpu2(&self) -> CPU2_R {
        CPU2_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ee_in_main(&self) -> EE_IN_MAIN_R {
        EE_IN_MAIN_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn iflush(&self) -> IFLUSH_R {
        IFLUSH_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sil3(&self) -> SIL3_R {
        SIL3_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ecca(&self) -> ECCA_R {
        ECCA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auto_susp(&self) -> AUTO_SUSP_R {
        AUTO_SUSP_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn uerr(&self) -> UERR_R {
        UERR_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cpu_type1(&self) -> CPU_TYPE1_R {
        CPU_TYPE1_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn family_type(&mut self) -> FAMILY_TYPE_W {
        FAMILY_TYPE_W { w: self }
    }
    #[doc = "Bits 21:23 - 23:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved21(&mut self) -> RESERVED21_W {
        RESERVED21_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn mem_map(&mut self) -> MEM_MAP_W {
        MEM_MAP_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cpu2(&mut self) -> CPU2_W {
        CPU2_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ee_in_main(&mut self) -> EE_IN_MAIN_W {
        EE_IN_MAIN_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rom(&mut self) -> ROM_W {
        ROM_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn iflush(&mut self) -> IFLUSH_W {
        IFLUSH_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sil3(&mut self) -> SIL3_W {
        SIL3_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ecca(&mut self) -> ECCA_W {
        ECCA_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auto_susp(&mut self) -> AUTO_SUSP_W {
        AUTO_SUSP_W { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn uerr(&mut self) -> UERR_W {
        UERR_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cpu_type1(&mut self) -> CPU_TYPE1_W {
        CPU_TYPE1_W { w: self }
    }
}
