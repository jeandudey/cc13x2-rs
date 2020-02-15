#[doc = "Reader of register FLASH_P_R_PV"]
pub type R = crate::R<u32, super::FLASH_P_R_PV>;
#[doc = "Writer for register FLASH_P_R_PV"]
pub type W = crate::W<u32, super::FLASH_P_R_PV>;
#[doc = "Register FLASH_P_R_PV `reset()`'s with value 0x02c1_0200"]
impl crate::ResetValue for super::FLASH_P_R_PV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02c1_0200
    }
}
#[doc = "Reader of field `PH`"]
pub type PH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PH`"]
pub struct PH_W<'a> {
    w: &'a mut W,
}
impl<'a> PH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `RH`"]
pub type RH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RH`"]
pub struct RH_W<'a> {
    w: &'a mut W,
}
impl<'a> RH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `PVH`"]
pub type PVH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PVH`"]
pub struct PVH_W<'a> {
    w: &'a mut W,
}
impl<'a> PVH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `PVH2`"]
pub type PVH2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PVH2`"]
pub struct PVH2_W<'a> {
    w: &'a mut W,
}
impl<'a> PVH2_W<'a> {
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
    pub fn ph(&self) -> PH_R {
        PH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rh(&self) -> RH_R {
        RH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pvh(&self) -> PVH_R {
        PVH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pvh2(&self) -> PVH2_R {
        PVH2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ph(&mut self) -> PH_W {
        PH_W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rh(&mut self) -> RH_W {
        RH_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pvh(&mut self) -> PVH_W {
        PVH_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pvh2(&mut self) -> PVH2_W {
        PVH2_W { w: self }
    }
}
