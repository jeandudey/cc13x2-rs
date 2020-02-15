#[doc = "Reader of register FLASH_C_E_P_R"]
pub type R = crate::R<u32, super::FLASH_C_E_P_R>;
#[doc = "Writer for register FLASH_C_E_P_R"]
pub type W = crate::W<u32, super::FLASH_C_E_P_R>;
#[doc = "Register FLASH_C_E_P_R `reset()`'s with value 0x0a0a_2000"]
impl crate::ResetValue for super::FLASH_C_E_P_R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0a0a_2000
    }
}
#[doc = "Reader of field `RVSU`"]
pub type RVSU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RVSU`"]
pub struct RVSU_W<'a> {
    w: &'a mut W,
}
impl<'a> RVSU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `PV_ACCESS`"]
pub type PV_ACCESS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PV_ACCESS`"]
pub struct PV_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> PV_ACCESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `A_EXEZ_SETUP`"]
pub type A_EXEZ_SETUP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `A_EXEZ_SETUP`"]
pub struct A_EXEZ_SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> A_EXEZ_SETUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `CVSU`"]
pub type CVSU_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CVSU`"]
pub struct CVSU_W<'a> {
    w: &'a mut W,
}
impl<'a> CVSU_W<'a> {
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
    pub fn rvsu(&self) -> RVSU_R {
        RVSU_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pv_access(&self) -> PV_ACCESS_R {
        PV_ACCESS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn a_exez_setup(&self) -> A_EXEZ_SETUP_R {
        A_EXEZ_SETUP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cvsu(&self) -> CVSU_R {
        CVSU_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rvsu(&mut self) -> RVSU_W {
        RVSU_W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pv_access(&mut self) -> PV_ACCESS_W {
        PV_ACCESS_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn a_exez_setup(&mut self) -> A_EXEZ_SETUP_W {
        A_EXEZ_SETUP_W { w: self }
    }
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cvsu(&mut self) -> CVSU_W {
        CVSU_W { w: self }
    }
}
