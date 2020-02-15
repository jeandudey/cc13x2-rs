#[doc = "Reader of register EVSTAT3"]
pub type R = crate::R<u32, super::EVSTAT3>;
#[doc = "Writer for register EVSTAT3"]
pub type W = crate::W<u32, super::EVSTAT3>;
#[doc = "Register EVSTAT3 `reset()`'s with value 0"]
impl crate::ResetValue for super::EVSTAT3 {
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
#[doc = "Reader of field `AUX_TIMER2_CLKSWITCH_RDY`"]
pub type AUX_TIMER2_CLKSWITCH_RDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_TIMER2_CLKSWITCH_RDY`"]
pub struct AUX_TIMER2_CLKSWITCH_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_TIMER2_CLKSWITCH_RDY_W<'a> {
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
#[doc = "Reader of field `AUX_DAC_HOLD_ACTIVE`"]
pub type AUX_DAC_HOLD_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_DAC_HOLD_ACTIVE`"]
pub struct AUX_DAC_HOLD_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_DAC_HOLD_ACTIVE_W<'a> {
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
#[doc = "Reader of field `AUX_SMPH_AUTOTAKE_DONE`"]
pub type AUX_SMPH_AUTOTAKE_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_SMPH_AUTOTAKE_DONE`"]
pub struct AUX_SMPH_AUTOTAKE_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_SMPH_AUTOTAKE_DONE_W<'a> {
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
#[doc = "Reader of field `AUX_ADC_FIFO_NOT_EMPTY`"]
pub type AUX_ADC_FIFO_NOT_EMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_ADC_FIFO_NOT_EMPTY`"]
pub struct AUX_ADC_FIFO_NOT_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_ADC_FIFO_NOT_EMPTY_W<'a> {
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
#[doc = "Reader of field `AUX_ADC_FIFO_ALMOST_FULL`"]
pub type AUX_ADC_FIFO_ALMOST_FULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_ADC_FIFO_ALMOST_FULL`"]
pub struct AUX_ADC_FIFO_ALMOST_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_ADC_FIFO_ALMOST_FULL_W<'a> {
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
#[doc = "Reader of field `AUX_ADC_IRQ`"]
pub type AUX_ADC_IRQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_ADC_IRQ`"]
pub struct AUX_ADC_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_ADC_IRQ_W<'a> {
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
#[doc = "Reader of field `AUX_ADC_DONE`"]
pub type AUX_ADC_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_ADC_DONE`"]
pub struct AUX_ADC_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_ADC_DONE_W<'a> {
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
#[doc = "Reader of field `AUX_ISRC_RESET_N`"]
pub type AUX_ISRC_RESET_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_ISRC_RESET_N`"]
pub struct AUX_ISRC_RESET_N_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_ISRC_RESET_N_W<'a> {
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
#[doc = "Reader of field `AUX_TDC_DONE`"]
pub type AUX_TDC_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_TDC_DONE`"]
pub struct AUX_TDC_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_TDC_DONE_W<'a> {
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
#[doc = "Reader of field `AUX_TIMER0_EV`"]
pub type AUX_TIMER0_EV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_TIMER0_EV`"]
pub struct AUX_TIMER0_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_TIMER0_EV_W<'a> {
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
#[doc = "Reader of field `AUX_TIMER1_EV`"]
pub type AUX_TIMER1_EV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_TIMER1_EV`"]
pub struct AUX_TIMER1_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_TIMER1_EV_W<'a> {
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
#[doc = "Reader of field `AUX_TIMER2_PULSE`"]
pub type AUX_TIMER2_PULSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_TIMER2_PULSE`"]
pub struct AUX_TIMER2_PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_TIMER2_PULSE_W<'a> {
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
#[doc = "Reader of field `AUX_TIMER2_EV3`"]
pub type AUX_TIMER2_EV3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_TIMER2_EV3`"]
pub struct AUX_TIMER2_EV3_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_TIMER2_EV3_W<'a> {
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
#[doc = "Reader of field `AUX_TIMER2_EV2`"]
pub type AUX_TIMER2_EV2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_TIMER2_EV2`"]
pub struct AUX_TIMER2_EV2_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_TIMER2_EV2_W<'a> {
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
#[doc = "Reader of field `AUX_TIMER2_EV1`"]
pub type AUX_TIMER2_EV1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_TIMER2_EV1`"]
pub struct AUX_TIMER2_EV1_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_TIMER2_EV1_W<'a> {
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
#[doc = "Reader of field `AUX_TIMER2_EV0`"]
pub type AUX_TIMER2_EV0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_TIMER2_EV0`"]
pub struct AUX_TIMER2_EV0_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_TIMER2_EV0_W<'a> {
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
AUX_SYSIF:TIMER2CLKSWITCH.RDY"]
    #[inline(always)]
    pub fn aux_timer2_clkswitch_rdy(&self) -> AUX_TIMER2_CLKSWITCH_RDY_R {
        AUX_TIMER2_CLKSWITCH_RDY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
AUX_ANAIF:DACSTAT.HOLD_ACTIVE"]
    #[inline(always)]
    pub fn aux_dac_hold_active(&self) -> AUX_DAC_HOLD_ACTIVE_R {
        AUX_DAC_HOLD_ACTIVE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
See AUX_SMPH:AUTOTAKE.SMPH_ID for description."]
    #[inline(always)]
    pub fn aux_smph_autotake_done(&self) -> AUX_SMPH_AUTOTAKE_DONE_R {
        AUX_SMPH_AUTOTAKE_DONE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
AUX_ANAIF:ADCFIFOSTAT.EMPTY negated"]
    #[inline(always)]
    pub fn aux_adc_fifo_not_empty(&self) -> AUX_ADC_FIFO_NOT_EMPTY_R {
        AUX_ADC_FIFO_NOT_EMPTY_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
AUX_ANAIF:ADCFIFOSTAT.ALMOST_FULL"]
    #[inline(always)]
    pub fn aux_adc_fifo_almost_full(&self) -> AUX_ADC_FIFO_ALMOST_FULL_R {
        AUX_ADC_FIFO_ALMOST_FULL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
The logical function for this event is configurable. When DMACTL.EN = 1 : Event = UDMA0 Channel 7 done event OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW When DMACTL.EN = 0 : Event = (NOT AUX_ANAIF:ADCFIFOSTAT.EMPTY) OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW Bit 7 in UDMA0:DONEMASK must be 0."]
    #[inline(always)]
    pub fn aux_adc_irq(&self) -> AUX_ADC_IRQ_R {
        AUX_ADC_IRQ_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
AUX_ANAIF ADC conversion done event. Event is synchronized at AUX bus rate."]
    #[inline(always)]
    pub fn aux_adc_done(&self) -> AUX_ADC_DONE_R {
        AUX_ADC_DONE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
AUX_ANAIF:ISRCCTL.RESET_N"]
    #[inline(always)]
    pub fn aux_isrc_reset_n(&self) -> AUX_ISRC_RESET_N_R {
        AUX_ISRC_RESET_N_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
AUX_TDC:STAT.DONE"]
    #[inline(always)]
    pub fn aux_tdc_done(&self) -> AUX_TDC_DONE_R {
        AUX_TDC_DONE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
AUX_TIMER0_EV event, see AUX_TIMER01:T0TARGET for description."]
    #[inline(always)]
    pub fn aux_timer0_ev(&self) -> AUX_TIMER0_EV_R {
        AUX_TIMER0_EV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
AUX_TIMER1_EV event, see AUX_TIMER01:T1TARGET for description."]
    #[inline(always)]
    pub fn aux_timer1_ev(&self) -> AUX_TIMER1_EV_R {
        AUX_TIMER1_EV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
AUX_TIMER2 pulse event. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
    #[inline(always)]
    pub fn aux_timer2_pulse(&self) -> AUX_TIMER2_PULSE_R {
        AUX_TIMER2_PULSE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
AUX_TIMER2 event output 3. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
    #[inline(always)]
    pub fn aux_timer2_ev3(&self) -> AUX_TIMER2_EV3_R {
        AUX_TIMER2_EV3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
AUX_TIMER2 event output 2. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
    #[inline(always)]
    pub fn aux_timer2_ev2(&self) -> AUX_TIMER2_EV2_R {
        AUX_TIMER2_EV2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
AUX_TIMER2 event output 1. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
    #[inline(always)]
    pub fn aux_timer2_ev1(&self) -> AUX_TIMER2_EV1_R {
        AUX_TIMER2_EV1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
AUX_TIMER2 event output 0. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
    #[inline(always)]
    pub fn aux_timer2_ev0(&self) -> AUX_TIMER2_EV0_R {
        AUX_TIMER2_EV0_R::new((self.bits & 0x01) != 0)
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
AUX_SYSIF:TIMER2CLKSWITCH.RDY"]
    #[inline(always)]
    pub fn aux_timer2_clkswitch_rdy(&mut self) -> AUX_TIMER2_CLKSWITCH_RDY_W {
        AUX_TIMER2_CLKSWITCH_RDY_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
AUX_ANAIF:DACSTAT.HOLD_ACTIVE"]
    #[inline(always)]
    pub fn aux_dac_hold_active(&mut self) -> AUX_DAC_HOLD_ACTIVE_W {
        AUX_DAC_HOLD_ACTIVE_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
See AUX_SMPH:AUTOTAKE.SMPH_ID for description."]
    #[inline(always)]
    pub fn aux_smph_autotake_done(&mut self) -> AUX_SMPH_AUTOTAKE_DONE_W {
        AUX_SMPH_AUTOTAKE_DONE_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
AUX_ANAIF:ADCFIFOSTAT.EMPTY negated"]
    #[inline(always)]
    pub fn aux_adc_fifo_not_empty(&mut self) -> AUX_ADC_FIFO_NOT_EMPTY_W {
        AUX_ADC_FIFO_NOT_EMPTY_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
AUX_ANAIF:ADCFIFOSTAT.ALMOST_FULL"]
    #[inline(always)]
    pub fn aux_adc_fifo_almost_full(&mut self) -> AUX_ADC_FIFO_ALMOST_FULL_W {
        AUX_ADC_FIFO_ALMOST_FULL_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
The logical function for this event is configurable. When DMACTL.EN = 1 : Event = UDMA0 Channel 7 done event OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW When DMACTL.EN = 0 : Event = (NOT AUX_ANAIF:ADCFIFOSTAT.EMPTY) OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW Bit 7 in UDMA0:DONEMASK must be 0."]
    #[inline(always)]
    pub fn aux_adc_irq(&mut self) -> AUX_ADC_IRQ_W {
        AUX_ADC_IRQ_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
AUX_ANAIF ADC conversion done event. Event is synchronized at AUX bus rate."]
    #[inline(always)]
    pub fn aux_adc_done(&mut self) -> AUX_ADC_DONE_W {
        AUX_ADC_DONE_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
AUX_ANAIF:ISRCCTL.RESET_N"]
    #[inline(always)]
    pub fn aux_isrc_reset_n(&mut self) -> AUX_ISRC_RESET_N_W {
        AUX_ISRC_RESET_N_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
AUX_TDC:STAT.DONE"]
    #[inline(always)]
    pub fn aux_tdc_done(&mut self) -> AUX_TDC_DONE_W {
        AUX_TDC_DONE_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
AUX_TIMER0_EV event, see AUX_TIMER01:T0TARGET for description."]
    #[inline(always)]
    pub fn aux_timer0_ev(&mut self) -> AUX_TIMER0_EV_W {
        AUX_TIMER0_EV_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
AUX_TIMER1_EV event, see AUX_TIMER01:T1TARGET for description."]
    #[inline(always)]
    pub fn aux_timer1_ev(&mut self) -> AUX_TIMER1_EV_W {
        AUX_TIMER1_EV_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
AUX_TIMER2 pulse event. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
    #[inline(always)]
    pub fn aux_timer2_pulse(&mut self) -> AUX_TIMER2_PULSE_W {
        AUX_TIMER2_PULSE_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
AUX_TIMER2 event output 3. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
    #[inline(always)]
    pub fn aux_timer2_ev3(&mut self) -> AUX_TIMER2_EV3_W {
        AUX_TIMER2_EV3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
AUX_TIMER2 event output 2. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
    #[inline(always)]
    pub fn aux_timer2_ev2(&mut self) -> AUX_TIMER2_EV2_W {
        AUX_TIMER2_EV2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
AUX_TIMER2 event output 1. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
    #[inline(always)]
    pub fn aux_timer2_ev1(&mut self) -> AUX_TIMER2_EV1_W {
        AUX_TIMER2_EV1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
AUX_TIMER2 event output 0. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_TIMER2_SYNC_RATE sets the synchronization rate for this event."]
    #[inline(always)]
    pub fn aux_timer2_ev0(&mut self) -> AUX_TIMER2_EV0_W {
        AUX_TIMER2_EV0_W { w: self }
    }
}
