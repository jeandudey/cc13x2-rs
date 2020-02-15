#[doc = "Reader of register FCFG_B7_START"]
pub type R = crate::R<u32, super::FCFG_B7_START>;
#[doc = "Writer for register FCFG_B7_START"]
pub type W = crate::W<u32, super::FCFG_B7_START>;
#[doc = "Register FCFG_B7_START `reset()`'s with value 0"]
impl crate::ResetValue for super::FCFG_B7_START {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B7_MAX_SECTOR`"]
pub type B7_MAX_SECTOR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `B7_MAX_SECTOR`"]
pub struct B7_MAX_SECTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> B7_MAX_SECTOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `B7_MUX_FACTOR`"]
pub type B7_MUX_FACTOR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `B7_MUX_FACTOR`"]
pub struct B7_MUX_FACTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> B7_MUX_FACTOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `B7_START_ADDR`"]
pub type B7_START_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `B7_START_ADDR`"]
pub struct B7_START_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> B7_START_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b7_max_sector(&self) -> B7_MAX_SECTOR_R {
        B7_MAX_SECTOR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b7_mux_factor(&self) -> B7_MUX_FACTOR_R {
        B7_MUX_FACTOR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b7_start_addr(&self) -> B7_START_ADDR_R {
        B7_START_ADDR_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b7_max_sector(&mut self) -> B7_MAX_SECTOR_W {
        B7_MAX_SECTOR_W { w: self }
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b7_mux_factor(&mut self) -> B7_MUX_FACTOR_W {
        B7_MUX_FACTOR_W { w: self }
    }
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b7_start_addr(&mut self) -> B7_START_ADDR_W {
        B7_START_ADDR_W { w: self }
    }
}
