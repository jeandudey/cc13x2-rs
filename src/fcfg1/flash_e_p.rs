#[doc = "Reader of register FLASH_E_P"]
pub type R = crate::R<u32, super::FLASH_E_P>;
#[doc = "Writer for register FLASH_E_P"]
pub type W = crate::W<u32, super::FLASH_E_P>;
#[doc = "Register FLASH_E_P `reset()`'s with value 0x4c64_4c64"]
impl crate::ResetValue for super::FLASH_E_P {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4c64_4c64
    }
}
#[doc = "Reader of field `PSU`"]
pub type PSU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PSU`"]
pub struct PSU_W<'a> {
    w: &'a mut W,
}
impl<'a> PSU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `ESU`"]
pub type ESU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ESU`"]
pub struct ESU_W<'a> {
    w: &'a mut W,
}
impl<'a> ESU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `PVSU`"]
pub type PVSU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PVSU`"]
pub struct PVSU_W<'a> {
    w: &'a mut W,
}
impl<'a> PVSU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `EVSU`"]
pub type EVSU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EVSU`"]
pub struct EVSU_W<'a> {
    w: &'a mut W,
}
impl<'a> EVSU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn psu(&self) -> PSU_R {
        PSU_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn esu(&self) -> ESU_R {
        ESU_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pvsu(&self) -> PVSU_R {
        PVSU_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn evsu(&self) -> EVSU_R {
        EVSU_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn psu(&mut self) -> PSU_W {
        PSU_W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn esu(&mut self) -> ESU_W {
        ESU_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pvsu(&mut self) -> PVSU_W {
        PVSU_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn evsu(&mut self) -> EVSU_W {
        EVSU_W { w: self }
    }
}
