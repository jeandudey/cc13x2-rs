#[doc = "Reader of register SHDW_OSC_BIAS_LDO_TRIM"]
pub type R = crate::R<u32, super::SHDW_OSC_BIAS_LDO_TRIM>;
#[doc = "Writer for register SHDW_OSC_BIAS_LDO_TRIM"]
pub type W = crate::W<u32, super::SHDW_OSC_BIAS_LDO_TRIM>;
#[doc = "Register SHDW_OSC_BIAS_LDO_TRIM `reset()`'s with value 0"]
impl crate::ResetValue for super::SHDW_OSC_BIAS_LDO_TRIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRIMMAG`"]
pub type TRIMMAG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIMMAG`"]
pub struct TRIMMAG_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMMAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 23)) | (((value as u32) & 0x0f) << 23);
        self.w
    }
}
#[doc = "Reader of field `TRIMIREF`"]
pub type TRIMIREF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIMIREF`"]
pub struct TRIMIREF_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMIREF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 18)) | (((value as u32) & 0x1f) << 18);
        self.w
    }
}
#[doc = "Reader of field `ITRIM_DIG_LDO`"]
pub type ITRIM_DIG_LDO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ITRIM_DIG_LDO`"]
pub struct ITRIM_DIG_LDO_W<'a> {
    w: &'a mut W,
}
impl<'a> ITRIM_DIG_LDO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `VTRIM_DIG`"]
pub type VTRIM_DIG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VTRIM_DIG`"]
pub struct VTRIM_DIG_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIM_DIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `VTRIM_COARSE`"]
pub type VTRIM_COARSE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VTRIM_COARSE`"]
pub struct VTRIM_COARSE_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIM_COARSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `RCOSCHF_CTRIM`"]
pub type RCOSCHF_CTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RCOSCHF_CTRIM`"]
pub struct RCOSCHF_CTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCHF_CTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 23:26 - 26:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimmag(&self) -> TRIMMAG_R {
        TRIMMAG_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bits 18:22 - 22:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimiref(&self) -> TRIMIREF_R {
        TRIMIREF_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn itrim_dig_ldo(&self) -> ITRIM_DIG_LDO_R {
        ITRIM_DIG_LDO_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vtrim_dig(&self) -> VTRIM_DIG_R {
        VTRIM_DIG_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vtrim_coarse(&self) -> VTRIM_COARSE_R {
        VTRIM_COARSE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschf_ctrim(&self) -> RCOSCHF_CTRIM_R {
        RCOSCHF_CTRIM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 23:26 - 26:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimmag(&mut self) -> TRIMMAG_W {
        TRIMMAG_W { w: self }
    }
    #[doc = "Bits 18:22 - 22:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimiref(&mut self) -> TRIMIREF_W {
        TRIMIREF_W { w: self }
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn itrim_dig_ldo(&mut self) -> ITRIM_DIG_LDO_W {
        ITRIM_DIG_LDO_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vtrim_dig(&mut self) -> VTRIM_DIG_W {
        VTRIM_DIG_W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vtrim_coarse(&mut self) -> VTRIM_COARSE_W {
        VTRIM_COARSE_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschf_ctrim(&mut self) -> RCOSCHF_CTRIM_W {
        RCOSCHF_CTRIM_W { w: self }
    }
}
