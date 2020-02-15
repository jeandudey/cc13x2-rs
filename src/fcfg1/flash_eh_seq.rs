#[doc = "Reader of register FLASH_EH_SEQ"]
pub type R = crate::R<u32, super::FLASH_EH_SEQ>;
#[doc = "Writer for register FLASH_EH_SEQ"]
pub type W = crate::W<u32, super::FLASH_EH_SEQ>;
#[doc = "Register FLASH_EH_SEQ `reset()`'s with value 0x0200_f000"]
impl crate::ResetValue for super::FLASH_EH_SEQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200_f000
    }
}
#[doc = "Reader of field `EH`"]
pub type EH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EH`"]
pub struct EH_W<'a> {
    w: &'a mut W,
}
impl<'a> EH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `SEQ`"]
pub type SEQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEQ`"]
pub struct SEQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `VSTAT`"]
pub type VSTAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VSTAT`"]
pub struct VSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> VSTAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `SM_FREQUENCY`"]
pub type SM_FREQUENCY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SM_FREQUENCY`"]
pub struct SM_FREQUENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> SM_FREQUENCY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn eh(&self) -> EH_R {
        EH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn seq(&self) -> SEQ_R {
        SEQ_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vstat(&self) -> VSTAT_R {
        VSTAT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sm_frequency(&self) -> SM_FREQUENCY_R {
        SM_FREQUENCY_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn eh(&mut self) -> EH_W {
        EH_W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn seq(&mut self) -> SEQ_W {
        SEQ_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vstat(&mut self) -> VSTAT_W {
        VSTAT_W { w: self }
    }
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sm_frequency(&mut self) -> SM_FREQUENCY_W {
        SM_FREQUENCY_W { w: self }
    }
}
