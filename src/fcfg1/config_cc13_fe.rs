#[doc = "Reader of register CONFIG_CC13_FE"]
pub type R = crate::R<u32, super::CONFIG_CC13_FE>;
#[doc = "Writer for register CONFIG_CC13_FE"]
pub type W = crate::W<u32, super::CONFIG_CC13_FE>;
#[doc = "Register CONFIG_CC13_FE `reset()`'s with value 0x7000_0f00"]
impl crate::ResetValue for super::CONFIG_CC13_FE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7000_0f00
    }
}
#[doc = "Reader of field `IFAMP_IB`"]
pub type IFAMP_IB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IFAMP_IB`"]
pub struct IFAMP_IB_W<'a> {
    w: &'a mut W,
}
impl<'a> IFAMP_IB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `LNA_IB`"]
pub type LNA_IB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LNA_IB`"]
pub struct LNA_IB_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_IB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `IFAMP_TRIM`"]
pub type IFAMP_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IFAMP_TRIM`"]
pub struct IFAMP_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> IFAMP_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | (((value as u32) & 0x1f) << 19);
        self.w
    }
}
#[doc = "Reader of field `CTL_PA0_TRIM`"]
pub type CTL_PA0_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTL_PA0_TRIM`"]
pub struct CTL_PA0_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CTL_PA0_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 14)) | (((value as u32) & 0x1f) << 14);
        self.w
    }
}
#[doc = "Reader of field `PATRIMCOMPLETE_N`"]
pub type PATRIMCOMPLETE_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PATRIMCOMPLETE_N`"]
pub struct PATRIMCOMPLETE_N_W<'a> {
    w: &'a mut W,
}
impl<'a> PATRIMCOMPLETE_N_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `RSSITRIMCOMPLETE_N`"]
pub type RSSITRIMCOMPLETE_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSSITRIMCOMPLETE_N`"]
pub struct RSSITRIMCOMPLETE_N_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSITRIMCOMPLETE_N_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `RSSI_OFFSET`"]
pub type RSSI_OFFSET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSSI_OFFSET`"]
pub struct RSSI_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSI_OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ifamp_ib(&self) -> IFAMP_IB_R {
        IFAMP_IB_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lna_ib(&self) -> LNA_IB_R {
        LNA_IB_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ifamp_trim(&self) -> IFAMP_TRIM_R {
        IFAMP_TRIM_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 14:18 - 18:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctl_pa0_trim(&self) -> CTL_PA0_TRIM_R {
        CTL_PA0_TRIM_R::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn patrimcomplete_n(&self) -> PATRIMCOMPLETE_N_R {
        PATRIMCOMPLETE_N_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rssitrimcomplete_n(&self) -> RSSITRIMCOMPLETE_N_R {
        RSSITRIMCOMPLETE_N_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rssi_offset(&self) -> RSSI_OFFSET_R {
        RSSI_OFFSET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ifamp_ib(&mut self) -> IFAMP_IB_W {
        IFAMP_IB_W { w: self }
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lna_ib(&mut self) -> LNA_IB_W {
        LNA_IB_W { w: self }
    }
    #[doc = "Bits 19:23 - 23:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ifamp_trim(&mut self) -> IFAMP_TRIM_W {
        IFAMP_TRIM_W { w: self }
    }
    #[doc = "Bits 14:18 - 18:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctl_pa0_trim(&mut self) -> CTL_PA0_TRIM_W {
        CTL_PA0_TRIM_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn patrimcomplete_n(&mut self) -> PATRIMCOMPLETE_N_W {
        PATRIMCOMPLETE_N_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rssitrimcomplete_n(&mut self) -> RSSITRIMCOMPLETE_N_W {
        RSSITRIMCOMPLETE_N_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rssi_offset(&mut self) -> RSSI_OFFSET_W {
        RSSI_OFFSET_W { w: self }
    }
}
