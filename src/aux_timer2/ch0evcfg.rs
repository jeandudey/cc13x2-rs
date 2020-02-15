#[doc = "Reader of register CH0EVCFG"]
pub type R = crate::R<u32, super::CH0EVCFG>;
#[doc = "Writer for register CH0EVCFG"]
pub type W = crate::W<u32, super::CH0EVCFG>;
#[doc = "Register CH0EVCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CH0EVCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED8`"]
pub type RESERVED8_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED8`"]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Reader of field `EV3_GEN`"]
pub type EV3_GEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EV3_GEN`"]
pub struct EV3_GEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EV3_GEN_W<'a> {
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
#[doc = "Reader of field `EV2_GEN`"]
pub type EV2_GEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EV2_GEN`"]
pub struct EV2_GEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EV2_GEN_W<'a> {
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
#[doc = "Reader of field `EV1_GEN`"]
pub type EV1_GEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EV1_GEN`"]
pub struct EV1_GEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EV1_GEN_W<'a> {
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
#[doc = "Reader of field `EV0_GEN`"]
pub type EV0_GEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EV0_GEN`"]
pub struct EV0_GEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EV0_GEN_W<'a> {
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
#[doc = "3:0\\]
Capture-Compare action. Capture-Compare action defines 15 different channel functions that utilize capture, compare, and zero events.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CCACT_A {
    #[doc = "15: Pulse on compare repeatedly. \n\nChannel function sequence: \n- Pulse enabled events when CH0CC.VALUE = CNTR.VALUE.\n\n The event is high for two timer clock periods."]
    PULSE_ON_CMP = 15,
    #[doc = "14: Toggle on compare repeatedly.\n\nChannel function sequence: \n- Toggle enabled events  when CH0CC.VALUE = CNTR.VALUE."]
    TGL_ON_CMP = 14,
    #[doc = "13: Set on compare repeatedly.\n\nChannel function sequence: \n- Set enabled events  when CH0CC.VALUE = CNTR.VALUE."]
    SET_ON_CMP = 13,
    #[doc = "12: Clear on compare repeatedly.\n\nChannel function sequence: \n- Clear enabled events  when CH0CC.VALUE = CNTR.VALUE."]
    CLR_ON_CMP = 12,
    #[doc = "11: Set on zero, toggle on compare repeatedly.\n\nChannel function sequence: \n- Set enabled events when CNTR.VALUE = 0.\n- Toggle enabled events when CH0CC.VALUE = CNTR.VALUE.\n\nSet CTL.MODE to UP_PER for edge-aligned PWM generation. Duty cycle is given by: \n\nWhen CH0CC.VALUE <= TARGET.VALUE: \n   Duty cycle = CH0CC.VALUE / ( TARGET.VALUE + 1 ).\n\nWhen CH0CC.VALUE > TARGET.VALUE: \n   Duty cycle = 1.\n\nEnabled events are cleared when CH0CC.VALUE = 0 and CNTR.VALUE = 0."]
    SET_ON_0_TGL_ON_CMP = 11,
    #[doc = "10: Clear on zero, toggle on compare repeatedly.\n \nChannel function sequence: \n- Clear enabled events when CNTR.VALUE = 0.\n- Toggle enabled events when CH0CC.VALUE = CNTR.VALUE.\n\nSet CTL.MODE to UPDWN_PER for center-aligned PWM generation. Duty cycle is given by: \n\nWhen CH0CC.VALUE <= TARGET.VALUE: \n   Duty cycle = 1 - ( CH0CC.VALUE / TARGET.VALUE ).\n\nWhen CH0CC.VALUE > TARGET.VALUE: \n   Duty cycle = 0.\n\nEnabled events are set when CH0CC.VALUE = 0 and CNTR.VALUE = 0."]
    CLR_ON_0_TGL_ON_CMP = 10,
    #[doc = "9: Set on capture repeatedly.\n\nChannel function sequence: \n- Set enabled events on capture event and copy CNTR.VALUE to CH0CC.VALUE.\n\nPrimary use scenario is to select this function before you start the timer.\nFollow these steps if you need to select this function while CTL.MODE is different from DIS: \n - Select this function with no event enable.\n - Configure CH0CCFG (optional).\n - Wait for three timer clock periods as defined in PRECFG before you enable events.\n\nThese steps prevent capture events caused by expired signal values in edge-detection circuit."]
    SET_ON_CAPT = 9,
    #[doc = "8: Period and pulse width measurement.\n\nContinuously capture period and pulse width of the signal selected by CH0CCFG.CAPT_SRC relative to the signal edge given by CH0CCFG.EDGE. \n\nSet enabled events when CH0CC.VALUE contains signal period and CH0PCC.VALUE contains signal pulse width. \n\nNotes: \n- Make sure that you configure CH0CCFG.CAPT_SRC and CCACT when CTL.MODE equals DIS, then set CTL.MODE to UP_ONCE or UP_PER. \n- The counter restarts in the selected timer mode when CH0CC.VALUE contains the signal period.\n- If more than one channel uses this function, the channels will perform this function one at a time. The channel with lowest number has priority and performs the function first. Next measurement starts when current measurement completes successfully or times out. A timeout occurs when counter equals target. \n- If you want to observe a timeout event configure another channel to SET_ON_CAPT.\n\nSignal property requirements:\n- Signal Period >= 2 * ( 1 + PRECFG.CLKDIV ) * timer clock period.\n- Signal Period <= 65535 * (1 + PRECFG.CLKDIV ) * timer clock period.\n- Signal low and high phase >= (1 + PRECFG.CLKDIV ) * timer clock period."]
    PER_PULSE_WIDTH_MEAS = 8,
    #[doc = "7: Pulse on compare, and then disable channel.\n\nChannel function sequence: \n- Pulse enabled events when CH0CC.VALUE = CNTR.VALUE.\n- Disable channel.\n\n The event is high for two timer clock periods."]
    PULSE_ON_CMP_DIS = 7,
    #[doc = "6: Toggle on compare, and then disable channel.\n\nChannel function sequence: \n- Toggle enabled events when CH0CC.VALUE = CNTR.VALUE.\n- Disable channel."]
    TGL_ON_CMP_DIS = 6,
    #[doc = "5: Set on compare, and then disable channel.\n\nChannel function sequence: \n- Set enabled events when CH0CC.VALUE = CNTR.VALUE.\n- Disable channel."]
    SET_ON_CMP_DIS = 5,
    #[doc = "4: Clear on compare, and then disable channel.\n\nChannel function sequence: \n- Clear enabled events when CH0CC.VALUE = CNTR.VALUE.\n- Disable channel."]
    CLR_ON_CMP_DIS = 4,
    #[doc = "3: Set on zero, toggle on compare, and then disable channel.\n\nChannel function sequence: \n- Set enabled events when CNTR.VALUE = 0.\n- Toggle enabled events when CH0CC.VALUE = CNTR.VALUE.\n- Disable channel.\n\nEnabled events are cleared when CH0CC.VALUE = 0 and CNTR.VALUE = 0."]
    SET_ON_0_TGL_ON_CMP_DIS = 3,
    #[doc = "2: Clear on zero, toggle on compare, and then disable channel.\n\nChannel function sequence: \n- Clear enabled events when CNTR.VALUE = 0.\n- Toggle enabled events when CH0CC.VALUE = CNTR.VALUE.\n- Disable channel.\n\nEnabled events are set when CH0CC.VALUE = 0 and CNTR.VALUE = 0."]
    CLR_ON_0_TGL_ON_CMP_DIS = 2,
    #[doc = "1: Set on capture, and then disable channel.\n\nChannel function sequence: \n- Set enabled events on capture event and copy CNTR.VALUE to CH0CC.VALUE.\n- Disable channel. \n\nPrimary use scenario is to select this function before you start the timer.\nFollow these steps if you need to select this function while CTL.MODE is different from DIS:  \n - Set CCACT to SET_ON_CAPT with no event enable.\n - Configure CH0CCFG (optional).\n - Wait for three timer clock periods as defined in PRECFG before you set CCACT to SET_ON_CAPT_DIS. Event enable is optional.\n\nThese steps prevent capture events caused by expired signal values in edge-detection circuit."]
    SET_ON_CAPT_DIS = 1,
    #[doc = "0: Disable channel."]
    DIS = 0,
}
impl From<CCACT_A> for u8 {
    #[inline(always)]
    fn from(variant: CCACT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CCACT`"]
pub type CCACT_R = crate::R<u8, CCACT_A>;
impl CCACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCACT_A {
        match self.bits {
            15 => CCACT_A::PULSE_ON_CMP,
            14 => CCACT_A::TGL_ON_CMP,
            13 => CCACT_A::SET_ON_CMP,
            12 => CCACT_A::CLR_ON_CMP,
            11 => CCACT_A::SET_ON_0_TGL_ON_CMP,
            10 => CCACT_A::CLR_ON_0_TGL_ON_CMP,
            9 => CCACT_A::SET_ON_CAPT,
            8 => CCACT_A::PER_PULSE_WIDTH_MEAS,
            7 => CCACT_A::PULSE_ON_CMP_DIS,
            6 => CCACT_A::TGL_ON_CMP_DIS,
            5 => CCACT_A::SET_ON_CMP_DIS,
            4 => CCACT_A::CLR_ON_CMP_DIS,
            3 => CCACT_A::SET_ON_0_TGL_ON_CMP_DIS,
            2 => CCACT_A::CLR_ON_0_TGL_ON_CMP_DIS,
            1 => CCACT_A::SET_ON_CAPT_DIS,
            0 => CCACT_A::DIS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULSE_ON_CMP`"]
    #[inline(always)]
    pub fn is_pulse_on_cmp(&self) -> bool {
        *self == CCACT_A::PULSE_ON_CMP
    }
    #[doc = "Checks if the value of the field is `TGL_ON_CMP`"]
    #[inline(always)]
    pub fn is_tgl_on_cmp(&self) -> bool {
        *self == CCACT_A::TGL_ON_CMP
    }
    #[doc = "Checks if the value of the field is `SET_ON_CMP`"]
    #[inline(always)]
    pub fn is_set_on_cmp(&self) -> bool {
        *self == CCACT_A::SET_ON_CMP
    }
    #[doc = "Checks if the value of the field is `CLR_ON_CMP`"]
    #[inline(always)]
    pub fn is_clr_on_cmp(&self) -> bool {
        *self == CCACT_A::CLR_ON_CMP
    }
    #[doc = "Checks if the value of the field is `SET_ON_0_TGL_ON_CMP`"]
    #[inline(always)]
    pub fn is_set_on_0_tgl_on_cmp(&self) -> bool {
        *self == CCACT_A::SET_ON_0_TGL_ON_CMP
    }
    #[doc = "Checks if the value of the field is `CLR_ON_0_TGL_ON_CMP`"]
    #[inline(always)]
    pub fn is_clr_on_0_tgl_on_cmp(&self) -> bool {
        *self == CCACT_A::CLR_ON_0_TGL_ON_CMP
    }
    #[doc = "Checks if the value of the field is `SET_ON_CAPT`"]
    #[inline(always)]
    pub fn is_set_on_capt(&self) -> bool {
        *self == CCACT_A::SET_ON_CAPT
    }
    #[doc = "Checks if the value of the field is `PER_PULSE_WIDTH_MEAS`"]
    #[inline(always)]
    pub fn is_per_pulse_width_meas(&self) -> bool {
        *self == CCACT_A::PER_PULSE_WIDTH_MEAS
    }
    #[doc = "Checks if the value of the field is `PULSE_ON_CMP_DIS`"]
    #[inline(always)]
    pub fn is_pulse_on_cmp_dis(&self) -> bool {
        *self == CCACT_A::PULSE_ON_CMP_DIS
    }
    #[doc = "Checks if the value of the field is `TGL_ON_CMP_DIS`"]
    #[inline(always)]
    pub fn is_tgl_on_cmp_dis(&self) -> bool {
        *self == CCACT_A::TGL_ON_CMP_DIS
    }
    #[doc = "Checks if the value of the field is `SET_ON_CMP_DIS`"]
    #[inline(always)]
    pub fn is_set_on_cmp_dis(&self) -> bool {
        *self == CCACT_A::SET_ON_CMP_DIS
    }
    #[doc = "Checks if the value of the field is `CLR_ON_CMP_DIS`"]
    #[inline(always)]
    pub fn is_clr_on_cmp_dis(&self) -> bool {
        *self == CCACT_A::CLR_ON_CMP_DIS
    }
    #[doc = "Checks if the value of the field is `SET_ON_0_TGL_ON_CMP_DIS`"]
    #[inline(always)]
    pub fn is_set_on_0_tgl_on_cmp_dis(&self) -> bool {
        *self == CCACT_A::SET_ON_0_TGL_ON_CMP_DIS
    }
    #[doc = "Checks if the value of the field is `CLR_ON_0_TGL_ON_CMP_DIS`"]
    #[inline(always)]
    pub fn is_clr_on_0_tgl_on_cmp_dis(&self) -> bool {
        *self == CCACT_A::CLR_ON_0_TGL_ON_CMP_DIS
    }
    #[doc = "Checks if the value of the field is `SET_ON_CAPT_DIS`"]
    #[inline(always)]
    pub fn is_set_on_capt_dis(&self) -> bool {
        *self == CCACT_A::SET_ON_CAPT_DIS
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CCACT_A::DIS
    }
}
#[doc = "Write proxy for field `CCACT`"]
pub struct CCACT_W<'a> {
    w: &'a mut W,
}
impl<'a> CCACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCACT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pulse on compare repeatedly. Channel function sequence: - Pulse enabled events when CH0CC.VALUE = CNTR.VALUE. The event is high for two timer clock periods."]
    #[inline(always)]
    pub fn pulse_on_cmp(self) -> &'a mut W {
        self.variant(CCACT_A::PULSE_ON_CMP)
    }
    #[doc = "Toggle on compare repeatedly. Channel function sequence: - Toggle enabled events when CH0CC.VALUE = CNTR.VALUE."]
    #[inline(always)]
    pub fn tgl_on_cmp(self) -> &'a mut W {
        self.variant(CCACT_A::TGL_ON_CMP)
    }
    #[doc = "Set on compare repeatedly. Channel function sequence: - Set enabled events when CH0CC.VALUE = CNTR.VALUE."]
    #[inline(always)]
    pub fn set_on_cmp(self) -> &'a mut W {
        self.variant(CCACT_A::SET_ON_CMP)
    }
    #[doc = "Clear on compare repeatedly. Channel function sequence: - Clear enabled events when CH0CC.VALUE = CNTR.VALUE."]
    #[inline(always)]
    pub fn clr_on_cmp(self) -> &'a mut W {
        self.variant(CCACT_A::CLR_ON_CMP)
    }
    #[doc = "Set on zero, toggle on compare repeatedly. Channel function sequence: - Set enabled events when CNTR.VALUE = 0. - Toggle enabled events when CH0CC.VALUE = CNTR.VALUE. Set CTL.MODE to UP_PER for edge-aligned PWM generation. Duty cycle is given by: When CH0CC.VALUE <= TARGET.VALUE: Duty cycle = CH0CC.VALUE / ( TARGET.VALUE + 1 ). When CH0CC.VALUE > TARGET.VALUE: Duty cycle = 1. Enabled events are cleared when CH0CC.VALUE = 0 and CNTR.VALUE = 0."]
    #[inline(always)]
    pub fn set_on_0_tgl_on_cmp(self) -> &'a mut W {
        self.variant(CCACT_A::SET_ON_0_TGL_ON_CMP)
    }
    #[doc = "Clear on zero, toggle on compare repeatedly. Channel function sequence: - Clear enabled events when CNTR.VALUE = 0. - Toggle enabled events when CH0CC.VALUE = CNTR.VALUE. Set CTL.MODE to UPDWN_PER for center-aligned PWM generation. Duty cycle is given by: When CH0CC.VALUE <= TARGET.VALUE: Duty cycle = 1 - ( CH0CC.VALUE / TARGET.VALUE ). When CH0CC.VALUE > TARGET.VALUE: Duty cycle = 0. Enabled events are set when CH0CC.VALUE = 0 and CNTR.VALUE = 0."]
    #[inline(always)]
    pub fn clr_on_0_tgl_on_cmp(self) -> &'a mut W {
        self.variant(CCACT_A::CLR_ON_0_TGL_ON_CMP)
    }
    #[doc = "Set on capture repeatedly. Channel function sequence: - Set enabled events on capture event and copy CNTR.VALUE to CH0CC.VALUE. Primary use scenario is to select this function before you start the timer. Follow these steps if you need to select this function while CTL.MODE is different from DIS: - Select this function with no event enable. - Configure CH0CCFG (optional). - Wait for three timer clock periods as defined in PRECFG before you enable events. These steps prevent capture events caused by expired signal values in edge-detection circuit."]
    #[inline(always)]
    pub fn set_on_capt(self) -> &'a mut W {
        self.variant(CCACT_A::SET_ON_CAPT)
    }
    #[doc = "Period and pulse width measurement. Continuously capture period and pulse width of the signal selected by CH0CCFG.CAPT_SRC relative to the signal edge given by CH0CCFG.EDGE. Set enabled events when CH0CC.VALUE contains signal period and CH0PCC.VALUE contains signal pulse width. Notes: - Make sure that you configure CH0CCFG.CAPT_SRC and CCACT when CTL.MODE equals DIS, then set CTL.MODE to UP_ONCE or UP_PER. - The counter restarts in the selected timer mode when CH0CC.VALUE contains the signal period. - If more than one channel uses this function, the channels will perform this function one at a time. The channel with lowest number has priority and performs the function first. Next measurement starts when current measurement completes successfully or times out. A timeout occurs when counter equals target. - If you want to observe a timeout event configure another channel to SET_ON_CAPT. Signal property requirements: - Signal Period >= 2 * ( 1 + PRECFG.CLKDIV ) * timer clock period. - Signal Period <= 65535 * (1 + PRECFG.CLKDIV ) * timer clock period. - Signal low and high phase >= (1 + PRECFG.CLKDIV ) * timer clock period."]
    #[inline(always)]
    pub fn per_pulse_width_meas(self) -> &'a mut W {
        self.variant(CCACT_A::PER_PULSE_WIDTH_MEAS)
    }
    #[doc = "Pulse on compare, and then disable channel. Channel function sequence: - Pulse enabled events when CH0CC.VALUE = CNTR.VALUE. - Disable channel. The event is high for two timer clock periods."]
    #[inline(always)]
    pub fn pulse_on_cmp_dis(self) -> &'a mut W {
        self.variant(CCACT_A::PULSE_ON_CMP_DIS)
    }
    #[doc = "Toggle on compare, and then disable channel. Channel function sequence: - Toggle enabled events when CH0CC.VALUE = CNTR.VALUE. - Disable channel."]
    #[inline(always)]
    pub fn tgl_on_cmp_dis(self) -> &'a mut W {
        self.variant(CCACT_A::TGL_ON_CMP_DIS)
    }
    #[doc = "Set on compare, and then disable channel. Channel function sequence: - Set enabled events when CH0CC.VALUE = CNTR.VALUE. - Disable channel."]
    #[inline(always)]
    pub fn set_on_cmp_dis(self) -> &'a mut W {
        self.variant(CCACT_A::SET_ON_CMP_DIS)
    }
    #[doc = "Clear on compare, and then disable channel. Channel function sequence: - Clear enabled events when CH0CC.VALUE = CNTR.VALUE. - Disable channel."]
    #[inline(always)]
    pub fn clr_on_cmp_dis(self) -> &'a mut W {
        self.variant(CCACT_A::CLR_ON_CMP_DIS)
    }
    #[doc = "Set on zero, toggle on compare, and then disable channel. Channel function sequence: - Set enabled events when CNTR.VALUE = 0. - Toggle enabled events when CH0CC.VALUE = CNTR.VALUE. - Disable channel. Enabled events are cleared when CH0CC.VALUE = 0 and CNTR.VALUE = 0."]
    #[inline(always)]
    pub fn set_on_0_tgl_on_cmp_dis(self) -> &'a mut W {
        self.variant(CCACT_A::SET_ON_0_TGL_ON_CMP_DIS)
    }
    #[doc = "Clear on zero, toggle on compare, and then disable channel. Channel function sequence: - Clear enabled events when CNTR.VALUE = 0. - Toggle enabled events when CH0CC.VALUE = CNTR.VALUE. - Disable channel. Enabled events are set when CH0CC.VALUE = 0 and CNTR.VALUE = 0."]
    #[inline(always)]
    pub fn clr_on_0_tgl_on_cmp_dis(self) -> &'a mut W {
        self.variant(CCACT_A::CLR_ON_0_TGL_ON_CMP_DIS)
    }
    #[doc = "Set on capture, and then disable channel. Channel function sequence: - Set enabled events on capture event and copy CNTR.VALUE to CH0CC.VALUE. - Disable channel. Primary use scenario is to select this function before you start the timer. Follow these steps if you need to select this function while CTL.MODE is different from DIS: - Set CCACT to SET_ON_CAPT with no event enable. - Configure CH0CCFG (optional). - Wait for three timer clock periods as defined in PRECFG before you set CCACT to SET_ON_CAPT_DIS. Event enable is optional. These steps prevent capture events caused by expired signal values in edge-detection circuit."]
    #[inline(always)]
    pub fn set_on_capt_dis(self) -> &'a mut W {
        self.variant(CCACT_A::SET_ON_CAPT_DIS)
    }
    #[doc = "Disable channel."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CCACT_A::DIS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 7 - 7:7\\]
Event 3 enable. 0: Channel 0 does not control event 3. 1: Channel 0 controls event 3. When 0 < CCACT < 8, EV3_GEN becomes zero after a capture or compare event."]
    #[inline(always)]
    pub fn ev3_gen(&self) -> EV3_GEN_R {
        EV3_GEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Event 2 enable. 0: Channel 0 does not control event 2. 1: Channel 0 controls event 2. When 0 < CCACT < 8, EV2_GEN becomes zero after a capture or compare event."]
    #[inline(always)]
    pub fn ev2_gen(&self) -> EV2_GEN_R {
        EV2_GEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Event 1 enable. 0: Channel 0 does not control event 1. 1: Channel 0 controls event 1. When 0 < CCACT < 8, EV1_GEN becomes zero after a capture or compare event."]
    #[inline(always)]
    pub fn ev1_gen(&self) -> EV1_GEN_R {
        EV1_GEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Event 0 enable. 0: Channel 0 does not control event 0. 1: Channel 0 controls event 0. When 0 < CCACT < 8, EV0_GEN becomes zero after a capture or compare event."]
    #[inline(always)]
    pub fn ev0_gen(&self) -> EV0_GEN_R {
        EV0_GEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Capture-Compare action. Capture-Compare action defines 15 different channel functions that utilize capture, compare, and zero events."]
    #[inline(always)]
    pub fn ccact(&self) -> CCACT_R {
        CCACT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Event 3 enable. 0: Channel 0 does not control event 3. 1: Channel 0 controls event 3. When 0 < CCACT < 8, EV3_GEN becomes zero after a capture or compare event."]
    #[inline(always)]
    pub fn ev3_gen(&mut self) -> EV3_GEN_W {
        EV3_GEN_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Event 2 enable. 0: Channel 0 does not control event 2. 1: Channel 0 controls event 2. When 0 < CCACT < 8, EV2_GEN becomes zero after a capture or compare event."]
    #[inline(always)]
    pub fn ev2_gen(&mut self) -> EV2_GEN_W {
        EV2_GEN_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Event 1 enable. 0: Channel 0 does not control event 1. 1: Channel 0 controls event 1. When 0 < CCACT < 8, EV1_GEN becomes zero after a capture or compare event."]
    #[inline(always)]
    pub fn ev1_gen(&mut self) -> EV1_GEN_W {
        EV1_GEN_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Event 0 enable. 0: Channel 0 does not control event 0. 1: Channel 0 controls event 0. When 0 < CCACT < 8, EV0_GEN becomes zero after a capture or compare event."]
    #[inline(always)]
    pub fn ev0_gen(&mut self) -> EV0_GEN_W {
        EV0_GEN_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Capture-Compare action. Capture-Compare action defines 15 different channel functions that utilize capture, compare, and zero events."]
    #[inline(always)]
    pub fn ccact(&mut self) -> CCACT_W {
        CCACT_W { w: self }
    }
}
