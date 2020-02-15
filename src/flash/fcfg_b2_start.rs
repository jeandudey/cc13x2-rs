#[doc = "Reader of register FCFG_B2_START"]
pub type R = crate::R<u32, super::FCFG_B2_START>;
#[doc = "Writer for register FCFG_B2_START"]
pub type W = crate::W<u32, super::FCFG_B2_START>;
#[doc = "Register FCFG_B2_START `reset()`'s with value 0"]
impl crate::ResetValue for super::FCFG_B2_START {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B2_MAX_SECTOR`"]
pub type B2_MAX_SECTOR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `B2_MAX_SECTOR`"]
pub struct B2_MAX_SECTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> B2_MAX_SECTOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `B2_MUX_FACTOR`"]
pub type B2_MUX_FACTOR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `B2_MUX_FACTOR`"]
pub struct B2_MUX_FACTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> B2_MUX_FACTOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `B2_START_ADDR`"]
pub type B2_START_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `B2_START_ADDR`"]
pub struct B2_START_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> B2_START_ADDR_W<'a> {
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
    pub fn b2_max_sector(&self) -> B2_MAX_SECTOR_R {
        B2_MAX_SECTOR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b2_mux_factor(&self) -> B2_MUX_FACTOR_R {
        B2_MUX_FACTOR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b2_start_addr(&self) -> B2_START_ADDR_R {
        B2_START_ADDR_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b2_max_sector(&mut self) -> B2_MAX_SECTOR_W {
        B2_MAX_SECTOR_W { w: self }
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b2_mux_factor(&mut self) -> B2_MUX_FACTOR_W {
        B2_MUX_FACTOR_W { w: self }
    }
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b2_start_addr(&mut self) -> B2_START_ADDR_W {
        B2_START_ADDR_W { w: self }
    }
}
