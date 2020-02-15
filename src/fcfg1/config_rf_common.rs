#[doc = "Reader of register CONFIG_RF_COMMON"]
pub type R = crate::R<u32, super::CONFIG_RF_COMMON>;
#[doc = "Writer for register CONFIG_RF_COMMON"]
pub type W = crate::W<u32, super::CONFIG_RF_COMMON>;
#[doc = "Register CONFIG_RF_COMMON `reset()`'s with value 0x01c0_014d"]
impl crate::ResetValue for super::CONFIG_RF_COMMON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01c0_014d
    }
}
#[doc = "Reader of field `DISABLE_CORNER_CAP`"]
pub type DISABLE_CORNER_CAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLE_CORNER_CAP`"]
pub struct DISABLE_CORNER_CAP_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_CORNER_CAP_W<'a> {
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
#[doc = "Reader of field `SLDO_TRIM_OUTPUT`"]
pub type SLDO_TRIM_OUTPUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLDO_TRIM_OUTPUT`"]
pub struct SLDO_TRIM_OUTPUT_W<'a> {
    w: &'a mut W,
}
impl<'a> SLDO_TRIM_OUTPUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 25)) | (((value as u32) & 0x3f) << 25);
        self.w
    }
}
#[doc = "Reader of field `PA20DBMTRIMCOMPLETE_N`"]
pub type PA20DBMTRIMCOMPLETE_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PA20DBMTRIMCOMPLETE_N`"]
pub struct PA20DBMTRIMCOMPLETE_N_W<'a> {
    w: &'a mut W,
}
impl<'a> PA20DBMTRIMCOMPLETE_N_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `CTL_PA_20DBM_TRIM`"]
pub type CTL_PA_20DBM_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTL_PA_20DBM_TRIM`"]
pub struct CTL_PA_20DBM_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CTL_PA_20DBM_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `RFLDO_TRIM_OUTPUT`"]
pub type RFLDO_TRIM_OUTPUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RFLDO_TRIM_OUTPUT`"]
pub struct RFLDO_TRIM_OUTPUT_W<'a> {
    w: &'a mut W,
}
impl<'a> RFLDO_TRIM_OUTPUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 9)) | (((value as u32) & 0x7f) << 9);
        self.w
    }
}
#[doc = "Reader of field `QUANTCTLTHRES`"]
pub type QUANTCTLTHRES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `QUANTCTLTHRES`"]
pub struct QUANTCTLTHRES_W<'a> {
    w: &'a mut W,
}
impl<'a> QUANTCTLTHRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Reader of field `DACTRIM`"]
pub type DACTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DACTRIM`"]
pub struct DACTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DACTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn disable_corner_cap(&self) -> DISABLE_CORNER_CAP_R {
        DISABLE_CORNER_CAP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 25:30 - 30:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sldo_trim_output(&self) -> SLDO_TRIM_OUTPUT_R {
        SLDO_TRIM_OUTPUT_R::new(((self.bits >> 25) & 0x3f) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pa20dbmtrimcomplete_n(&self) -> PA20DBMTRIMCOMPLETE_N_R {
        PA20DBMTRIMCOMPLETE_N_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctl_pa_20dbm_trim(&self) -> CTL_PA_20DBM_TRIM_R {
        CTL_PA_20DBM_TRIM_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rfldo_trim_output(&self) -> RFLDO_TRIM_OUTPUT_R {
        RFLDO_TRIM_OUTPUT_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 6:8 - 8:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn quantctlthres(&self) -> QUANTCTLTHRES_R {
        QUANTCTLTHRES_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dactrim(&self) -> DACTRIM_R {
        DACTRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn disable_corner_cap(&mut self) -> DISABLE_CORNER_CAP_W {
        DISABLE_CORNER_CAP_W { w: self }
    }
    #[doc = "Bits 25:30 - 30:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sldo_trim_output(&mut self) -> SLDO_TRIM_OUTPUT_W {
        SLDO_TRIM_OUTPUT_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pa20dbmtrimcomplete_n(&mut self) -> PA20DBMTRIMCOMPLETE_N_W {
        PA20DBMTRIMCOMPLETE_N_W { w: self }
    }
    #[doc = "Bits 16:20 - 20:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctl_pa_20dbm_trim(&mut self) -> CTL_PA_20DBM_TRIM_W {
        CTL_PA_20DBM_TRIM_W { w: self }
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rfldo_trim_output(&mut self) -> RFLDO_TRIM_OUTPUT_W {
        RFLDO_TRIM_OUTPUT_W { w: self }
    }
    #[doc = "Bits 6:8 - 8:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn quantctlthres(&mut self) -> QUANTCTLTHRES_W {
        QUANTCTLTHRES_W { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dactrim(&mut self) -> DACTRIM_W {
        DACTRIM_W { w: self }
    }
}
