#[doc = "Reader of register FLASH_PP"]
pub type R = crate::R<u32, super::FLASH_PP>;
#[doc = "Writer for register FLASH_PP"]
pub type W = crate::W<u32, super::FLASH_PP>;
#[doc = "Register FLASH_PP `reset()`'s with value 0x14"]
impl crate::ResetValue for super::FLASH_PP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x14
    }
}
#[doc = "Reader of field `PUMP_SU`"]
pub type PUMP_SU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PUMP_SU`"]
pub struct PUMP_SU_W<'a> {
    w: &'a mut W,
}
impl<'a> PUMP_SU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `TRIM3P4`"]
pub type TRIM3P4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIM3P4`"]
pub struct TRIM3P4_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM3P4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `MAX_PP`"]
pub type MAX_PP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MAX_PP`"]
pub struct MAX_PP_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_PP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pump_su(&self) -> PUMP_SU_R {
        PUMP_SU_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim3p4(&self) -> TRIM3P4_R {
        TRIM3P4_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn max_pp(&self) -> MAX_PP_R {
        MAX_PP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pump_su(&mut self) -> PUMP_SU_W {
        PUMP_SU_W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim3p4(&mut self) -> TRIM3P4_W {
        TRIM3P4_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn max_pp(&mut self) -> MAX_PP_W {
        MAX_PP_W { w: self }
    }
}
