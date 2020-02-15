#[doc = "Reader of register EVSTAT2"]
pub type R = crate::R<u32, super::EVSTAT2>;
#[doc = "Writer for register EVSTAT2"]
pub type W = crate::W<u32, super::EVSTAT2>;
#[doc = "Register EVSTAT2 `reset()`'s with value 0"]
impl crate::ResetValue for super::EVSTAT2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED16`"]
pub type RESERVED16_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED16`"]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `AUX_COMPB`"]
pub type AUX_COMPB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_COMPB`"]
pub struct AUX_COMPB_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_COMPB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `AUX_COMPA`"]
pub type AUX_COMPA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_COMPA`"]
pub struct AUX_COMPA_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_COMPA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `MCU_OBSMUX1`"]
pub type MCU_OBSMUX1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCU_OBSMUX1`"]
pub struct MCU_OBSMUX1_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_OBSMUX1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `MCU_OBSMUX0`"]
pub type MCU_OBSMUX0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCU_OBSMUX0`"]
pub struct MCU_OBSMUX0_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_OBSMUX0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `MCU_EV`"]
pub type MCU_EV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCU_EV`"]
pub struct MCU_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_EV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `ACLK_REF`"]
pub type ACLK_REF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACLK_REF`"]
pub struct ACLK_REF_W<'a> {
    w: &'a mut W,
}
impl<'a> ACLK_REF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `VDDR_RECHARGE`"]
pub type VDDR_RECHARGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDDR_RECHARGE`"]
pub struct VDDR_RECHARGE_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_RECHARGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `MCU_ACTIVE`"]
pub type MCU_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCU_ACTIVE`"]
pub struct MCU_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_ACTIVE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `PWR_DWN`"]
pub type PWR_DWN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWR_DWN`"]
pub struct PWR_DWN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_DWN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `SCLK_LF`"]
pub type SCLK_LF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCLK_LF`"]
pub struct SCLK_LF_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_LF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `AON_BATMON_TEMP_UPD`"]
pub type AON_BATMON_TEMP_UPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AON_BATMON_TEMP_UPD`"]
pub struct AON_BATMON_TEMP_UPD_W<'a> {
    w: &'a mut W,
}
impl<'a> AON_BATMON_TEMP_UPD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `AON_BATMON_BAT_UPD`"]
pub type AON_BATMON_BAT_UPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AON_BATMON_BAT_UPD`"]
pub struct AON_BATMON_BAT_UPD_W<'a> {
    w: &'a mut W,
}
impl<'a> AON_BATMON_BAT_UPD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `AON_RTC_4KHZ`"]
pub type AON_RTC_4KHZ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AON_RTC_4KHZ`"]
pub struct AON_RTC_4KHZ_W<'a> {
    w: &'a mut W,
}
impl<'a> AON_RTC_4KHZ_W<'a> {
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
#[doc = "Reader of field `AON_RTC_CH2_DLY`"]
pub type AON_RTC_CH2_DLY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AON_RTC_CH2_DLY`"]
pub struct AON_RTC_CH2_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> AON_RTC_CH2_DLY_W<'a> {
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
#[doc = "Reader of field `AON_RTC_CH2`"]
pub type AON_RTC_CH2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AON_RTC_CH2`"]
pub struct AON_RTC_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> AON_RTC_CH2_W<'a> {
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
#[doc = "Reader of field `MANUAL_EV`"]
pub type MANUAL_EV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MANUAL_EV`"]
pub struct MANUAL_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> MANUAL_EV_W<'a> {
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
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 15 - 15:15\\]
Comparator B output. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_COMPB_SYNC_RATE sets the synchronization rate for this event."]
    #[inline(always)]
    pub fn aux_compb(&self) -> AUX_COMPB_R {
        AUX_COMPB_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Comparator A output. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_COMPA_SYNC_RATE sets the synchronization rate for this event."]
    #[inline(always)]
    pub fn aux_compa(&self) -> AUX_COMPA_R {
        AUX_COMPA_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Observation input 1 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL1."]
    #[inline(always)]
    pub fn mcu_obsmux1(&self) -> MCU_OBSMUX1_R {
        MCU_OBSMUX1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Observation input 0 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL0 and can be overridden by IOC:OBSAUXOUTPUT.SEL_MISC."]
    #[inline(always)]
    pub fn mcu_obsmux0(&self) -> MCU_OBSMUX0_R {
        MCU_OBSMUX0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Event from EVENT configured by EVENT:AUXSEL0."]
    #[inline(always)]
    pub fn mcu_ev(&self) -> MCU_EV_R {
        MCU_EV_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
TDC reference clock. It is configured by DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL and enabled by AUX_SYSIF:TDCREFCLKCTL.REQ."]
    #[inline(always)]
    pub fn aclk_ref(&self) -> ACLK_REF_R {
        ACLK_REF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Event is high during VDDR recharge."]
    #[inline(always)]
    pub fn vddr_recharge(&self) -> VDDR_RECHARGE_R {
        VDDR_RECHARGE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Event is high while system(MCU, AUX, or JTAG domains) is active or transitions to active (GLDO or DCDC power supply state). Event is not high during VDDR recharge."]
    #[inline(always)]
    pub fn mcu_active(&self) -> MCU_ACTIVE_R {
        MCU_ACTIVE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Event is high while system(MCU, AUX, or JTAG domains) is in powerdown (uLDO power supply)."]
    #[inline(always)]
    pub fn pwr_dwn(&self) -> PWR_DWN_R {
        PWR_DWN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
SCLK_LF clock"]
    #[inline(always)]
    pub fn sclk_lf(&self) -> SCLK_LF_R {
        SCLK_LF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Event is high for two SCLK_MF clock periods when there is an update of AON_BATMON:TEMP."]
    #[inline(always)]
    pub fn aon_batmon_temp_upd(&self) -> AON_BATMON_TEMP_UPD_R {
        AON_BATMON_TEMP_UPD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Event is high for two SCLK_MF clock periods when there is an update of AON_BATMON:BAT."]
    #[inline(always)]
    pub fn aon_batmon_bat_upd(&self) -> AON_BATMON_BAT_UPD_R {
        AON_BATMON_BAT_UPD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
AON_RTC:SUBSEC.VALUE bit 19. AON_RTC:CTL.RTC_4KHZ_EN enables this event."]
    #[inline(always)]
    pub fn aon_rtc_4khz(&self) -> AON_RTC_4KHZ_R {
        AON_RTC_4KHZ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
AON_RTC:EVFLAGS.CH2 delayed by AON_RTC:CTL.EV_DELAY configuration."]
    #[inline(always)]
    pub fn aon_rtc_ch2_dly(&self) -> AON_RTC_CH2_DLY_R {
        AON_RTC_CH2_DLY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
AON_RTC:EVFLAGS.CH2."]
    #[inline(always)]
    pub fn aon_rtc_ch2(&self) -> AON_RTC_CH2_R {
        AON_RTC_CH2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Programmable event. See MANUAL for description."]
    #[inline(always)]
    pub fn manual_ev(&self) -> MANUAL_EV_R {
        MANUAL_EV_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
Comparator B output. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_COMPB_SYNC_RATE sets the synchronization rate for this event."]
    #[inline(always)]
    pub fn aux_compb(&mut self) -> AUX_COMPB_W {
        AUX_COMPB_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
Comparator A output. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_COMPA_SYNC_RATE sets the synchronization rate for this event."]
    #[inline(always)]
    pub fn aux_compa(&mut self) -> AUX_COMPA_W {
        AUX_COMPA_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
Observation input 1 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL1."]
    #[inline(always)]
    pub fn mcu_obsmux1(&mut self) -> MCU_OBSMUX1_W {
        MCU_OBSMUX1_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Observation input 0 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL0 and can be overridden by IOC:OBSAUXOUTPUT.SEL_MISC."]
    #[inline(always)]
    pub fn mcu_obsmux0(&mut self) -> MCU_OBSMUX0_W {
        MCU_OBSMUX0_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Event from EVENT configured by EVENT:AUXSEL0."]
    #[inline(always)]
    pub fn mcu_ev(&mut self) -> MCU_EV_W {
        MCU_EV_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
TDC reference clock. It is configured by DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL and enabled by AUX_SYSIF:TDCREFCLKCTL.REQ."]
    #[inline(always)]
    pub fn aclk_ref(&mut self) -> ACLK_REF_W {
        ACLK_REF_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Event is high during VDDR recharge."]
    #[inline(always)]
    pub fn vddr_recharge(&mut self) -> VDDR_RECHARGE_W {
        VDDR_RECHARGE_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Event is high while system(MCU, AUX, or JTAG domains) is active or transitions to active (GLDO or DCDC power supply state). Event is not high during VDDR recharge."]
    #[inline(always)]
    pub fn mcu_active(&mut self) -> MCU_ACTIVE_W {
        MCU_ACTIVE_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Event is high while system(MCU, AUX, or JTAG domains) is in powerdown (uLDO power supply)."]
    #[inline(always)]
    pub fn pwr_dwn(&mut self) -> PWR_DWN_W {
        PWR_DWN_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
SCLK_LF clock"]
    #[inline(always)]
    pub fn sclk_lf(&mut self) -> SCLK_LF_W {
        SCLK_LF_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Event is high for two SCLK_MF clock periods when there is an update of AON_BATMON:TEMP."]
    #[inline(always)]
    pub fn aon_batmon_temp_upd(&mut self) -> AON_BATMON_TEMP_UPD_W {
        AON_BATMON_TEMP_UPD_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Event is high for two SCLK_MF clock periods when there is an update of AON_BATMON:BAT."]
    #[inline(always)]
    pub fn aon_batmon_bat_upd(&mut self) -> AON_BATMON_BAT_UPD_W {
        AON_BATMON_BAT_UPD_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
AON_RTC:SUBSEC.VALUE bit 19. AON_RTC:CTL.RTC_4KHZ_EN enables this event."]
    #[inline(always)]
    pub fn aon_rtc_4khz(&mut self) -> AON_RTC_4KHZ_W {
        AON_RTC_4KHZ_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
AON_RTC:EVFLAGS.CH2 delayed by AON_RTC:CTL.EV_DELAY configuration."]
    #[inline(always)]
    pub fn aon_rtc_ch2_dly(&mut self) -> AON_RTC_CH2_DLY_W {
        AON_RTC_CH2_DLY_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
AON_RTC:EVFLAGS.CH2."]
    #[inline(always)]
    pub fn aon_rtc_ch2(&mut self) -> AON_RTC_CH2_W {
        AON_RTC_CH2_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Programmable event. See MANUAL for description."]
    #[inline(always)]
    pub fn manual_ev(&mut self) -> MANUAL_EV_W {
        MANUAL_EV_W { w: self }
    }
}
