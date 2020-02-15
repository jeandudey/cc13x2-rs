#[doc = "Reader of register CCFG_TAP_DAP_0"]
pub type R = crate::R<u32, super::CCFG_TAP_DAP_0>;
#[doc = "Writer for register CCFG_TAP_DAP_0"]
pub type W = crate::W<u32, super::CCFG_TAP_DAP_0>;
#[doc = "Register CCFG_TAP_DAP_0 `reset()`'s with value 0xffc5_c5c5"]
impl crate::ResetValue for super::CCFG_TAP_DAP_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffc5_c5c5
    }
}
#[doc = "Reader of field `CPU_DAP_ENABLE`"]
pub type CPU_DAP_ENABLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CPU_DAP_ENABLE`"]
pub struct CPU_DAP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_DAP_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `PWRPROF_TAP_ENABLE`"]
pub type PWRPROF_TAP_ENABLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PWRPROF_TAP_ENABLE`"]
pub struct PWRPROF_TAP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRPROF_TAP_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `TEST_TAP_ENABLE`"]
pub type TEST_TAP_ENABLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TEST_TAP_ENABLE`"]
pub struct TEST_TAP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_TAP_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - 23:16\\]
Enable CPU DAP. 0xC5: Main CPU DAP access is enabled during power-up/system-reset by ROM boot FW. Any other value: Main CPU DAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn cpu_dap_enable(&self) -> CPU_DAP_ENABLE_R {
        CPU_DAP_ENABLE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Enable PWRPROF TAP. 0xC5: PWRPROF TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PWRPROF TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn pwrprof_tap_enable(&self) -> PWRPROF_TAP_ENABLE_R {
        PWRPROF_TAP_ENABLE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Enable Test TAP. 0xC5: TEST TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: TEST TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn test_tap_enable(&self) -> TEST_TAP_ENABLE_R {
        TEST_TAP_ENABLE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - 23:16\\]
Enable CPU DAP. 0xC5: Main CPU DAP access is enabled during power-up/system-reset by ROM boot FW. Any other value: Main CPU DAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn cpu_dap_enable(&mut self) -> CPU_DAP_ENABLE_W {
        CPU_DAP_ENABLE_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Enable PWRPROF TAP. 0xC5: PWRPROF TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PWRPROF TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn pwrprof_tap_enable(&mut self) -> PWRPROF_TAP_ENABLE_W {
        PWRPROF_TAP_ENABLE_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Enable Test TAP. 0xC5: TEST TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: TEST TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn test_tap_enable(&mut self) -> TEST_TAP_ENABLE_W {
        TEST_TAP_ENABLE_W { w: self }
    }
}
