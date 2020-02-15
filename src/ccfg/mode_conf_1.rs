#[doc = "Reader of register MODE_CONF_1"]
pub type R = crate::R<u32, super::MODE_CONF_1>;
#[doc = "Writer for register MODE_CONF_1"]
pub type W = crate::W<u32, super::MODE_CONF_1>;
#[doc = "Register MODE_CONF_1 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MODE_CONF_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `ALT_DCDC_VMIN`"]
pub type ALT_DCDC_VMIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALT_DCDC_VMIN`"]
pub struct ALT_DCDC_VMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALT_DCDC_VMIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `ALT_DCDC_DITHER_EN`"]
pub type ALT_DCDC_DITHER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALT_DCDC_DITHER_EN`"]
pub struct ALT_DCDC_DITHER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALT_DCDC_DITHER_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `ALT_DCDC_IPEAK`"]
pub type ALT_DCDC_IPEAK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALT_DCDC_IPEAK`"]
pub struct ALT_DCDC_IPEAK_W<'a> {
    w: &'a mut W,
}
impl<'a> ALT_DCDC_IPEAK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `DELTA_IBIAS_INIT`"]
pub type DELTA_IBIAS_INIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DELTA_IBIAS_INIT`"]
pub struct DELTA_IBIAS_INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> DELTA_IBIAS_INIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `DELTA_IBIAS_OFFSET`"]
pub type DELTA_IBIAS_OFFSET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DELTA_IBIAS_OFFSET`"]
pub struct DELTA_IBIAS_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> DELTA_IBIAS_OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `XOSC_MAX_START`"]
pub type XOSC_MAX_START_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XOSC_MAX_START`"]
pub struct XOSC_MAX_START_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_MAX_START_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:23 - 23:20\\]
Minimum voltage for when DC/DC should be used if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). Voltage = (28 + ALT_DCDC_VMIN) / 16. 0: 1.75V 1: 1.8125V ... 14: 2.625V 15: 2.6875V NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline(always)]
    pub fn alt_dcdc_vmin(&self) -> ALT_DCDC_VMIN_R {
        ALT_DCDC_VMIN_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 19 - 19:19\\]
Enable DC/DC dithering if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). 0: Dither disable 1: Dither enable"]
    #[inline(always)]
    pub fn alt_dcdc_dither_en(&self) -> ALT_DCDC_DITHER_EN_R {
        ALT_DCDC_DITHER_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Inductor peak current if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). Assuming 10uH external inductor! Peak current = 31 + ( 4 * ALT_DCDC_IPEAK ) : 0: 31mA (min) ... 4: 47mA ... 7: 59mA (max)"]
    #[inline(always)]
    pub fn alt_dcdc_ipeak(&self) -> ALT_DCDC_IPEAK_R {
        ALT_DCDC_IPEAK_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Signed delta value for IBIAS_INIT. Delta value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0. See FCFG1:AMPCOMP_CTRL1.IBIAS_INIT"]
    #[inline(always)]
    pub fn delta_ibias_init(&self) -> DELTA_IBIAS_INIT_R {
        DELTA_IBIAS_INIT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Signed delta value for IBIAS_OFFSET. Delta value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0. See FCFG1:AMPCOMP_CTRL1.IBIAS_OFFSET"]
    #[inline(always)]
    pub fn delta_ibias_offset(&self) -> DELTA_IBIAS_OFFSET_R {
        DELTA_IBIAS_OFFSET_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Unsigned value of maximum XOSC startup time (worst case) in units of 100us. Value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0."]
    #[inline(always)]
    pub fn xosc_max_start(&self) -> XOSC_MAX_START_R {
        XOSC_MAX_START_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 20:23 - 23:20\\]
Minimum voltage for when DC/DC should be used if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). Voltage = (28 + ALT_DCDC_VMIN) / 16. 0: 1.75V 1: 1.8125V ... 14: 2.625V 15: 2.6875V NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline(always)]
    pub fn alt_dcdc_vmin(&mut self) -> ALT_DCDC_VMIN_W {
        ALT_DCDC_VMIN_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\]
Enable DC/DC dithering if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). 0: Dither disable 1: Dither enable"]
    #[inline(always)]
    pub fn alt_dcdc_dither_en(&mut self) -> ALT_DCDC_DITHER_EN_W {
        ALT_DCDC_DITHER_EN_W { w: self }
    }
    #[doc = "Bits 16:18 - 18:16\\]
Inductor peak current if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). Assuming 10uH external inductor! Peak current = 31 + ( 4 * ALT_DCDC_IPEAK ) : 0: 31mA (min) ... 4: 47mA ... 7: 59mA (max)"]
    #[inline(always)]
    pub fn alt_dcdc_ipeak(&mut self) -> ALT_DCDC_IPEAK_W {
        ALT_DCDC_IPEAK_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\]
Signed delta value for IBIAS_INIT. Delta value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0. See FCFG1:AMPCOMP_CTRL1.IBIAS_INIT"]
    #[inline(always)]
    pub fn delta_ibias_init(&mut self) -> DELTA_IBIAS_INIT_W {
        DELTA_IBIAS_INIT_W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\]
Signed delta value for IBIAS_OFFSET. Delta value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0. See FCFG1:AMPCOMP_CTRL1.IBIAS_OFFSET"]
    #[inline(always)]
    pub fn delta_ibias_offset(&mut self) -> DELTA_IBIAS_OFFSET_W {
        DELTA_IBIAS_OFFSET_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Unsigned value of maximum XOSC startup time (worst case) in units of 100us. Value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0."]
    #[inline(always)]
    pub fn xosc_max_start(&mut self) -> XOSC_MAX_START_W {
        XOSC_MAX_START_W { w: self }
    }
}
