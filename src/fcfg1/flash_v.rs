#[doc = "Reader of register FLASH_V"]
pub type R = crate::R<u32, super::FLASH_V>;
#[doc = "Writer for register FLASH_V"]
pub type W = crate::W<u32, super::FLASH_V>;
#[doc = "Register FLASH_V `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASH_V {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VSL_P`"]
pub type VSL_P_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VSL_P`"]
pub struct VSL_P_W<'a> {
    w: &'a mut W,
}
impl<'a> VSL_P_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `VWL_P`"]
pub type VWL_P_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VWL_P`"]
pub struct VWL_P_W<'a> {
    w: &'a mut W,
}
impl<'a> VWL_P_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `V_READ`"]
pub type V_READ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `V_READ`"]
pub struct V_READ_W<'a> {
    w: &'a mut W,
}
impl<'a> V_READ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `TRIM0P8`"]
pub type TRIM0P8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIM0P8`"]
pub struct TRIM0P8_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM0P8_W<'a> {
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
    pub fn vsl_p(&self) -> VSL_P_R {
        VSL_P_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vwl_p(&self) -> VWL_P_R {
        VWL_P_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn v_read(&self) -> V_READ_R {
        V_READ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim0p8(&self) -> TRIM0P8_R {
        TRIM0P8_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vsl_p(&mut self) -> VSL_P_W {
        VSL_P_W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vwl_p(&mut self) -> VWL_P_W {
        VWL_P_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn v_read(&mut self) -> V_READ_W {
        V_READ_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim0p8(&mut self) -> TRIM0P8_W {
        TRIM0P8_W { w: self }
    }
}
