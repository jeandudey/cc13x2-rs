#[doc = "Reader of register SIZE_AND_DIS_FLAGS"]
pub type R = crate::R<u32, super::SIZE_AND_DIS_FLAGS>;
#[doc = "Writer for register SIZE_AND_DIS_FLAGS"]
pub type W = crate::W<u32, super::SIZE_AND_DIS_FLAGS>;
#[doc = "Register SIZE_AND_DIS_FLAGS `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::SIZE_AND_DIS_FLAGS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `SIZE_OF_CCFG`"]
pub type SIZE_OF_CCFG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SIZE_OF_CCFG`"]
pub struct SIZE_OF_CCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE_OF_CCFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DISABLE_FLAGS`"]
pub type DISABLE_FLAGS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DISABLE_FLAGS`"]
pub struct DISABLE_FLAGS_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_FLAGS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 4)) | (((value as u32) & 0x0fff) << 4);
        self.w
    }
}
#[doc = "Reader of field `DIS_TCXO`"]
pub type DIS_TCXO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_TCXO`"]
pub struct DIS_TCXO_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_TCXO_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `DIS_GPRAM`"]
pub type DIS_GPRAM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_GPRAM`"]
pub struct DIS_GPRAM_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_GPRAM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `DIS_ALT_DCDC_SETTING`"]
pub type DIS_ALT_DCDC_SETTING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_ALT_DCDC_SETTING`"]
pub struct DIS_ALT_DCDC_SETTING_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_ALT_DCDC_SETTING_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `DIS_XOSC_OVR`"]
pub type DIS_XOSC_OVR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_XOSC_OVR`"]
pub struct DIS_XOSC_OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_XOSC_OVR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Total size of CCFG in bytes."]
    #[inline(always)]
    pub fn size_of_ccfg(&self) -> SIZE_OF_CCFG_R {
        SIZE_OF_CCFG_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn disable_flags(&self) -> DISABLE_FLAGS_R {
        DISABLE_FLAGS_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bit 3 - 3:3\\]
Disable TCXO. 0: TCXO functionality enabled. 1: TCXO functionality disabled. Note: An external TCXO is required if DIS_TCXO = 0."]
    #[inline(always)]
    pub fn dis_tcxo(&self) -> DIS_TCXO_R {
        DIS_TCXO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Disable GPRAM (or use the 8K VIMS RAM as CACHE RAM). 0: GPRAM is enabled and hence CACHE disabled. 1: GPRAM is disabled and instead CACHE is enabled (default). Notes: - Disabling CACHE will reduce CPU execution speed (up to 60%). - GPRAM is 8 K-bytes in size and located at 0x11000000-0x11001FFF if enabled. See: VIMS:CTL.MODE"]
    #[inline(always)]
    pub fn dis_gpram(&self) -> DIS_GPRAM_R {
        DIS_GPRAM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Disable alternate DC/DC settings. 0: Enable alternate DC/DC settings. 1: Disable alternate DC/DC settings. See: MODE_CONF_1.ALT_DCDC_VMIN MODE_CONF_1.ALT_DCDC_DITHER_EN MODE_CONF_1.ALT_DCDC_IPEAK NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline(always)]
    pub fn dis_alt_dcdc_setting(&self) -> DIS_ALT_DCDC_SETTING_R {
        DIS_ALT_DCDC_SETTING_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Disable XOSC override functionality. 0: Enable XOSC override functionality. 1: Disable XOSC override functionality. See: MODE_CONF_1.DELTA_IBIAS_INIT MODE_CONF_1.DELTA_IBIAS_OFFSET MODE_CONF_1.XOSC_MAX_START"]
    #[inline(always)]
    pub fn dis_xosc_ovr(&self) -> DIS_XOSC_OVR_R {
        DIS_XOSC_OVR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Total size of CCFG in bytes."]
    #[inline(always)]
    pub fn size_of_ccfg(&mut self) -> SIZE_OF_CCFG_W {
        SIZE_OF_CCFG_W { w: self }
    }
    #[doc = "Bits 4:15 - 15:4\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn disable_flags(&mut self) -> DISABLE_FLAGS_W {
        DISABLE_FLAGS_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Disable TCXO. 0: TCXO functionality enabled. 1: TCXO functionality disabled. Note: An external TCXO is required if DIS_TCXO = 0."]
    #[inline(always)]
    pub fn dis_tcxo(&mut self) -> DIS_TCXO_W {
        DIS_TCXO_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Disable GPRAM (or use the 8K VIMS RAM as CACHE RAM). 0: GPRAM is enabled and hence CACHE disabled. 1: GPRAM is disabled and instead CACHE is enabled (default). Notes: - Disabling CACHE will reduce CPU execution speed (up to 60%). - GPRAM is 8 K-bytes in size and located at 0x11000000-0x11001FFF if enabled. See: VIMS:CTL.MODE"]
    #[inline(always)]
    pub fn dis_gpram(&mut self) -> DIS_GPRAM_W {
        DIS_GPRAM_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Disable alternate DC/DC settings. 0: Enable alternate DC/DC settings. 1: Disable alternate DC/DC settings. See: MODE_CONF_1.ALT_DCDC_VMIN MODE_CONF_1.ALT_DCDC_DITHER_EN MODE_CONF_1.ALT_DCDC_IPEAK NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline(always)]
    pub fn dis_alt_dcdc_setting(&mut self) -> DIS_ALT_DCDC_SETTING_W {
        DIS_ALT_DCDC_SETTING_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Disable XOSC override functionality. 0: Enable XOSC override functionality. 1: Disable XOSC override functionality. See: MODE_CONF_1.DELTA_IBIAS_INIT MODE_CONF_1.DELTA_IBIAS_OFFSET MODE_CONF_1.XOSC_MAX_START"]
    #[inline(always)]
    pub fn dis_xosc_ovr(&mut self) -> DIS_XOSC_OVR_W {
        DIS_XOSC_OVR_W { w: self }
    }
}
