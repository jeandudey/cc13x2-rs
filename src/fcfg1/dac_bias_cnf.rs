#[doc = "Reader of register DAC_BIAS_CNF"]
pub type R = crate::R<u32, super::DAC_BIAS_CNF>;
#[doc = "Writer for register DAC_BIAS_CNF"]
pub type W = crate::W<u32, super::DAC_BIAS_CNF>;
#[doc = "Register DAC_BIAS_CNF `reset()`'s with value 0xfffc_00ff"]
impl crate::ResetValue for super::DAC_BIAS_CNF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xfffc_00ff
    }
}
#[doc = "Reader of field `LPM_TRIM_IOUT`"]
pub type LPM_TRIM_IOUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPM_TRIM_IOUT`"]
pub struct LPM_TRIM_IOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_TRIM_IOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Reader of field `LPM_BIAS_WIDTH_TRIM`"]
pub type LPM_BIAS_WIDTH_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPM_BIAS_WIDTH_TRIM`"]
pub struct LPM_BIAS_WIDTH_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_BIAS_WIDTH_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Reader of field `LPM_BIAS_BACKUP_EN`"]
pub type LPM_BIAS_BACKUP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPM_BIAS_BACKUP_EN`"]
pub struct LPM_BIAS_BACKUP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_BIAS_BACKUP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:17 - 17:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_trim_iout(&self) -> LPM_TRIM_IOUT_R {
        LPM_TRIM_IOUT_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 9:11 - 11:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_bias_width_trim(&self) -> LPM_BIAS_WIDTH_TRIM_R {
        LPM_BIAS_WIDTH_TRIM_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_bias_backup_en(&self) -> LPM_BIAS_BACKUP_EN_R {
        LPM_BIAS_BACKUP_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 12:17 - 17:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_trim_iout(&mut self) -> LPM_TRIM_IOUT_W {
        LPM_TRIM_IOUT_W { w: self }
    }
    #[doc = "Bits 9:11 - 11:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_bias_width_trim(&mut self) -> LPM_BIAS_WIDTH_TRIM_W {
        LPM_BIAS_WIDTH_TRIM_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_bias_backup_en(&mut self) -> LPM_BIAS_BACKUP_EN_W {
        LPM_BIAS_BACKUP_EN_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
}
