#[doc = "Reader of register FVHVCT1"]
pub type R = crate::R<u32, super::FVHVCT1>;
#[doc = "Writer for register FVHVCT1"]
pub type W = crate::W<u32, super::FVHVCT1>;
#[doc = "Register FVHVCT1 `reset()`'s with value 0x0084_0088"]
impl crate::ResetValue for super::FVHVCT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0084_0088
    }
}
#[doc = "Reader of field `RESERVED24`"]
pub type RESERVED24_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED24`"]
pub struct RESERVED24_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `TRIM13_E`"]
pub type TRIM13_E_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIM13_E`"]
pub struct TRIM13_E_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM13_E_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `VHVCT_E`"]
pub type VHVCT_E_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VHVCT_E`"]
pub struct VHVCT_E_W<'a> {
    w: &'a mut W,
}
impl<'a> VHVCT_E_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `RESERVED8`"]
pub type RESERVED8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED8`"]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `TRIM13_PV`"]
pub type TRIM13_PV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIM13_PV`"]
pub struct TRIM13_PV_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM13_PV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `VHVCT_PV`"]
pub type VHVCT_PV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VHVCT_PV`"]
pub struct VHVCT_PV_W<'a> {
    w: &'a mut W,
}
impl<'a> VHVCT_PV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim13_e(&self) -> TRIM13_E_R {
        TRIM13_E_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhvct_e(&self) -> VHVCT_E_R {
        VHVCT_E_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim13_pv(&self) -> TRIM13_PV_R {
        TRIM13_PV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhvct_pv(&self) -> VHVCT_PV_R {
        VHVCT_PV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved24(&mut self) -> RESERVED24_W {
        RESERVED24_W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim13_e(&mut self) -> TRIM13_E_W {
        TRIM13_E_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhvct_e(&mut self) -> VHVCT_E_W {
        VHVCT_E_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim13_pv(&mut self) -> TRIM13_PV_W {
        TRIM13_PV_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhvct_pv(&mut self) -> VHVCT_PV_W {
        VHVCT_PV_W { w: self }
    }
}
