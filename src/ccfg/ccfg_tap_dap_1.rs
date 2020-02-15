#[doc = "Reader of register CCFG_TAP_DAP_1"]
pub type R = crate::R<u32, super::CCFG_TAP_DAP_1>;
#[doc = "Writer for register CCFG_TAP_DAP_1"]
pub type W = crate::W<u32, super::CCFG_TAP_DAP_1>;
#[doc = "Register CCFG_TAP_DAP_1 `reset()`'s with value 0xffc5_c5c5"]
impl crate::ResetValue for super::CCFG_TAP_DAP_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffc5_c5c5
    }
}
#[doc = "Reader of field `PBIST2_TAP_ENABLE`"]
pub type PBIST2_TAP_ENABLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PBIST2_TAP_ENABLE`"]
pub struct PBIST2_TAP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIST2_TAP_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `PBIST1_TAP_ENABLE`"]
pub type PBIST1_TAP_ENABLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PBIST1_TAP_ENABLE`"]
pub struct PBIST1_TAP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIST1_TAP_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `AON_TAP_ENABLE`"]
pub type AON_TAP_ENABLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AON_TAP_ENABLE`"]
pub struct AON_TAP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> AON_TAP_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - 23:16\\]
Enable PBIST2 TAP. 0xC5: PBIST2 TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PBIST2 TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn pbist2_tap_enable(&self) -> PBIST2_TAP_ENABLE_R {
        PBIST2_TAP_ENABLE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Enable PBIST1 TAP. 0xC5: PBIST1 TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PBIST1 TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn pbist1_tap_enable(&self) -> PBIST1_TAP_ENABLE_R {
        PBIST1_TAP_ENABLE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Enable AON TAP 0xC5: AON TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: AON TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn aon_tap_enable(&self) -> AON_TAP_ENABLE_R {
        AON_TAP_ENABLE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - 23:16\\]
Enable PBIST2 TAP. 0xC5: PBIST2 TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PBIST2 TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn pbist2_tap_enable(&mut self) -> PBIST2_TAP_ENABLE_W {
        PBIST2_TAP_ENABLE_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Enable PBIST1 TAP. 0xC5: PBIST1 TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PBIST1 TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn pbist1_tap_enable(&mut self) -> PBIST1_TAP_ENABLE_W {
        PBIST1_TAP_ENABLE_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Enable AON TAP 0xC5: AON TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: AON TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn aon_tap_enable(&mut self) -> AON_TAP_ENABLE_W {
        AON_TAP_ENABLE_W { w: self }
    }
}
