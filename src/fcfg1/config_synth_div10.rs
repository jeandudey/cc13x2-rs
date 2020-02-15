#[doc = "Reader of register CONFIG_SYNTH_DIV10"]
pub type R = crate::R<u32, super::CONFIG_SYNTH_DIV10>;
#[doc = "Writer for register CONFIG_SYNTH_DIV10"]
pub type W = crate::W<u32, super::CONFIG_SYNTH_DIV10>;
#[doc = "Register CONFIG_SYNTH_DIV10 `reset()`'s with value 0x1f"]
impl crate::ResetValue for super::CONFIG_SYNTH_DIV10 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1f
    }
}
#[doc = "Reader of field `MIN_ALLOWED_RTRIM`"]
pub type MIN_ALLOWED_RTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MIN_ALLOWED_RTRIM`"]
pub struct MIN_ALLOWED_RTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MIN_ALLOWED_RTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `RFC_MDM_DEMIQMC0`"]
pub type RFC_MDM_DEMIQMC0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RFC_MDM_DEMIQMC0`"]
pub struct RFC_MDM_DEMIQMC0_W<'a> {
    w: &'a mut W,
}
impl<'a> RFC_MDM_DEMIQMC0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 12)) | (((value as u32) & 0xffff) << 12);
        self.w
    }
}
#[doc = "Reader of field `LDOVCO_TRIM_OUTPUT`"]
pub type LDOVCO_TRIM_OUTPUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LDOVCO_TRIM_OUTPUT`"]
pub struct LDOVCO_TRIM_OUTPUT_W<'a> {
    w: &'a mut W,
}
impl<'a> LDOVCO_TRIM_OUTPUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
#[doc = "Reader of field `RFC_MDM_DEMIQMC0_TRIMCOMPLETE_N`"]
pub type RFC_MDM_DEMIQMC0_TRIMCOMPLETE_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFC_MDM_DEMIQMC0_TRIMCOMPLETE_N`"]
pub struct RFC_MDM_DEMIQMC0_TRIMCOMPLETE_N_W<'a> {
    w: &'a mut W,
}
impl<'a> RFC_MDM_DEMIQMC0_TRIMCOMPLETE_N_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn min_allowed_rtrim(&self) -> MIN_ALLOWED_RTRIM_R {
        MIN_ALLOWED_RTRIM_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 12:27 - 27:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rfc_mdm_demiqmc0(&self) -> RFC_MDM_DEMIQMC0_R {
        RFC_MDM_DEMIQMC0_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bits 6:11 - 11:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ldovco_trim_output(&self) -> LDOVCO_TRIM_OUTPUT_R {
        LDOVCO_TRIM_OUTPUT_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rfc_mdm_demiqmc0_trimcomplete_n(&self) -> RFC_MDM_DEMIQMC0_TRIMCOMPLETE_N_R {
        RFC_MDM_DEMIQMC0_TRIMCOMPLETE_N_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn min_allowed_rtrim(&mut self) -> MIN_ALLOWED_RTRIM_W {
        MIN_ALLOWED_RTRIM_W { w: self }
    }
    #[doc = "Bits 12:27 - 27:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rfc_mdm_demiqmc0(&mut self) -> RFC_MDM_DEMIQMC0_W {
        RFC_MDM_DEMIQMC0_W { w: self }
    }
    #[doc = "Bits 6:11 - 11:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ldovco_trim_output(&mut self) -> LDOVCO_TRIM_OUTPUT_W {
        LDOVCO_TRIM_OUTPUT_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rfc_mdm_demiqmc0_trimcomplete_n(&mut self) -> RFC_MDM_DEMIQMC0_TRIMCOMPLETE_N_W {
        RFC_MDM_DEMIQMC0_TRIMCOMPLETE_N_W { w: self }
    }
}
