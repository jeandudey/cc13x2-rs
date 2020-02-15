#[doc = "Reader of register CONFIG_OSC_TOP"]
pub type R = crate::R<u32, super::CONFIG_OSC_TOP>;
#[doc = "Writer for register CONFIG_OSC_TOP"]
pub type W = crate::W<u32, super::CONFIG_OSC_TOP>;
#[doc = "Register CONFIG_OSC_TOP `reset()`'s with value 0xdc07_fc00"]
impl crate::ResetValue for super::CONFIG_OSC_TOP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xdc07_fc00
    }
}
#[doc = "Reader of field `XOSC_HF_ROW_Q12`"]
pub type XOSC_HF_ROW_Q12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XOSC_HF_ROW_Q12`"]
pub struct XOSC_HF_ROW_Q12_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_HF_ROW_Q12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 26)) | (((value as u32) & 0x0f) << 26);
        self.w
    }
}
#[doc = "Reader of field `XOSC_HF_COLUMN_Q12`"]
pub type XOSC_HF_COLUMN_Q12_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `XOSC_HF_COLUMN_Q12`"]
pub struct XOSC_HF_COLUMN_Q12_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_HF_COLUMN_Q12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 10)) | (((value as u32) & 0xffff) << 10);
        self.w
    }
}
#[doc = "Reader of field `RCOSCLF_CTUNE_TRIM`"]
pub type RCOSCLF_CTUNE_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RCOSCLF_CTUNE_TRIM`"]
pub struct RCOSCLF_CTUNE_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCLF_CTUNE_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 2)) | (((value as u32) & 0xff) << 2);
        self.w
    }
}
#[doc = "Reader of field `RCOSCLF_RTUNE_TRIM`"]
pub type RCOSCLF_RTUNE_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RCOSCLF_RTUNE_TRIM`"]
pub struct RCOSCLF_RTUNE_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCLF_RTUNE_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 26:29 - 29:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_row_q12(&self) -> XOSC_HF_ROW_Q12_R {
        XOSC_HF_ROW_Q12_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bits 10:25 - 25:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_column_q12(&self) -> XOSC_HF_COLUMN_Q12_R {
        XOSC_HF_COLUMN_Q12_R::new(((self.bits >> 10) & 0xffff) as u16)
    }
    #[doc = "Bits 2:9 - 9:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosclf_ctune_trim(&self) -> RCOSCLF_CTUNE_TRIM_R {
        RCOSCLF_CTUNE_TRIM_R::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosclf_rtune_trim(&self) -> RCOSCLF_RTUNE_TRIM_R {
        RCOSCLF_RTUNE_TRIM_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 26:29 - 29:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_row_q12(&mut self) -> XOSC_HF_ROW_Q12_W {
        XOSC_HF_ROW_Q12_W { w: self }
    }
    #[doc = "Bits 10:25 - 25:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_column_q12(&mut self) -> XOSC_HF_COLUMN_Q12_W {
        XOSC_HF_COLUMN_Q12_W { w: self }
    }
    #[doc = "Bits 2:9 - 9:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosclf_ctune_trim(&mut self) -> RCOSCLF_CTUNE_TRIM_W {
        RCOSCLF_CTUNE_TRIM_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosclf_rtune_trim(&mut self) -> RCOSCLF_RTUNE_TRIM_W {
        RCOSCLF_RTUNE_TRIM_W { w: self }
    }
}
