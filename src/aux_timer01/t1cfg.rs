#[doc = "Reader of register T1CFG"]
pub type R = crate::R<u32, super::T1CFG>;
#[doc = "Writer for register T1CFG"]
pub type W = crate::W<u32, super::T1CFG>;
#[doc = "Register T1CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::T1CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED15`"]
pub type RESERVED15_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED15`"]
pub struct RESERVED15_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0001_ffff << 15)) | (((value as u32) & 0x0001_ffff) << 15);
        self.w
    }
}
#[doc = "14:14\\]
Tick source polarity for Timer 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TICK_SRC_POL_A {
    #[doc = "1: Count on falling edges of TICK_SRC."]
    FALL = 1,
    #[doc = "0: Count on rising edges of TICK_SRC."]
    RISE = 0,
}
impl From<TICK_SRC_POL_A> for bool {
    #[inline(always)]
    fn from(variant: TICK_SRC_POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TICK_SRC_POL`"]
pub type TICK_SRC_POL_R = crate::R<bool, TICK_SRC_POL_A>;
impl TICK_SRC_POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TICK_SRC_POL_A {
        match self.bits {
            true => TICK_SRC_POL_A::FALL,
            false => TICK_SRC_POL_A::RISE,
        }
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == TICK_SRC_POL_A::FALL
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == TICK_SRC_POL_A::RISE
    }
}
#[doc = "Write proxy for field `TICK_SRC_POL`"]
pub struct TICK_SRC_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> TICK_SRC_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TICK_SRC_POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Count on falling edges of TICK_SRC."]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(TICK_SRC_POL_A::FALL)
    }
    #[doc = "Count on rising edges of TICK_SRC."]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(TICK_SRC_POL_A::RISE)
    }
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
#[doc = "13:8\\]
Select Timer 1 tick source from the synchronous event bus.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TICK_SRC_A {
    #[doc = "63: AUX_EVCTL:EVSTAT3.AUX_TIMER2_CLKSWITCH_RDY"]
    AUX_TIMER2_CLKSW_RDY = 63,
    #[doc = "62: AUX_EVCTL:EVSTAT3.AUX_DAC_HOLD_ACTIVE"]
    AUX_DAC_HOLD_ACTIVE = 62,
    #[doc = "61: AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    AUX_SMPH_AUTOTAKE_DONE = 61,
    #[doc = "60: AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_NOT_EMPTY"]
    AUX_ADC_FIFO_NOT_EMPTY = 60,
    #[doc = "59: AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_ALMOST_FULL"]
    AUX_ADC_FIFO_ALMOST_FULL = 59,
    #[doc = "58: AUX_EVCTL:EVSTAT3.AUX_ADC_IRQ"]
    AUX_ADC_IRQ = 58,
    #[doc = "57: AUX_EVCTL:EVSTAT3.AUX_ADC_DONE"]
    AUX_ADC_DONE = 57,
    #[doc = "56: AUX_EVCTL:EVSTAT3.AUX_ISRC_RESET_N"]
    AUX_ISRC_RESET_N = 56,
    #[doc = "55: AUX_EVCTL:EVSTAT3.AUX_TDC_DONE"]
    AUX_TDC_DONE = 55,
    #[doc = "54: AUX_EVCTL:EVSTAT3.AUX_TIMER0_EV"]
    AUX_TIMER0_EV = 54,
    #[doc = "53: No event."]
    NO_EVENT = 53,
    #[doc = "52: AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE"]
    AUX_TIMER2_PULSE = 52,
    #[doc = "51: AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3"]
    AUX_TIMER2_EV3 = 51,
    #[doc = "50: AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2"]
    AUX_TIMER2_EV2 = 50,
    #[doc = "49: AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1"]
    AUX_TIMER2_EV1 = 49,
    #[doc = "48: AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0"]
    AUX_TIMER2_EV0 = 48,
    #[doc = "47: AUX_EVCTL:EVSTAT2.AUX_COMPB"]
    AUX_COMPB = 47,
    #[doc = "46: AUX_EVCTL:EVSTAT2.AUX_COMPA"]
    AUX_COMPA = 46,
    #[doc = "45: AUX_EVCTL:EVSTAT2.MCU_OBSMUX1"]
    MCU_OBSMUX1 = 45,
    #[doc = "44: AUX_EVCTL:EVSTAT2.MCU_OBSMUX0"]
    MCU_OBSMUX0 = 44,
    #[doc = "43: AUX_EVCTL:EVSTAT2.MCU_EV"]
    MCU_EV = 43,
    #[doc = "42: AUX_EVCTL:EVSTAT2.ACLK_REF"]
    ACLK_REF = 42,
    #[doc = "41: AUX_EVCTL:EVSTAT2.VDDR_RECHARGE"]
    VDDR_RECHARGE = 41,
    #[doc = "40: AUX_EVCTL:EVSTAT2.MCU_ACTIVE"]
    MCU_ACTIVE = 40,
    #[doc = "39: AUX_EVCTL:EVSTAT2.PWR_DWN"]
    PWR_DWN = 39,
    #[doc = "38: AUX_EVCTL:EVSTAT2.SCLK_LF"]
    SCLK_LF = 38,
    #[doc = "37: AUX_EVCTL:EVSTAT2.AON_BATMON_TEMP_UPD"]
    AON_BATMON_TEMP_UPD = 37,
    #[doc = "36: AUX_EVCTL:EVSTAT2.AON_BATMON_BAT_UPD"]
    AON_BATMON_BAT_UPD = 36,
    #[doc = "35: AUX_EVCTL:EVSTAT2.AON_RTC_4KHZ"]
    AON_RTC_4KHZ = 35,
    #[doc = "34: AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY"]
    AON_RTC_CH2_DLY = 34,
    #[doc = "33: AUX_EVCTL:EVSTAT2.AON_RTC_CH2"]
    AON_RTC_CH2 = 33,
    #[doc = "32: AUX_EVCTL:EVSTAT2.MANUAL_EV"]
    MANUAL_EV = 32,
    #[doc = "31: AUX_EVCTL:EVSTAT1.AUXIO31"]
    AUXIO31 = 31,
    #[doc = "30: AUX_EVCTL:EVSTAT1.AUXIO30"]
    AUXIO30 = 30,
    #[doc = "29: AUX_EVCTL:EVSTAT1.AUXIO29"]
    AUXIO29 = 29,
    #[doc = "28: AUX_EVCTL:EVSTAT1.AUXIO28"]
    AUXIO28 = 28,
    #[doc = "27: AUX_EVCTL:EVSTAT1.AUXIO27"]
    AUXIO27 = 27,
    #[doc = "26: AUX_EVCTL:EVSTAT1.AUXIO26"]
    AUXIO26 = 26,
    #[doc = "25: AUX_EVCTL:EVSTAT1.AUXIO25"]
    AUXIO25 = 25,
    #[doc = "24: AUX_EVCTL:EVSTAT1.AUXIO24"]
    AUXIO24 = 24,
    #[doc = "23: AUX_EVCTL:EVSTAT1.AUXIO23"]
    AUXIO23 = 23,
    #[doc = "22: AUX_EVCTL:EVSTAT1.AUXIO22"]
    AUXIO22 = 22,
    #[doc = "21: AUX_EVCTL:EVSTAT1.AUXIO21"]
    AUXIO21 = 21,
    #[doc = "20: AUX_EVCTL:EVSTAT1.AUXIO20"]
    AUXIO20 = 20,
    #[doc = "19: AUX_EVCTL:EVSTAT1.AUXIO19"]
    AUXIO19 = 19,
    #[doc = "18: AUX_EVCTL:EVSTAT1.AUXIO18"]
    AUXIO18 = 18,
    #[doc = "17: AUX_EVCTL:EVSTAT1.AUXIO17"]
    AUXIO17 = 17,
    #[doc = "16: AUX_EVCTL:EVSTAT1.AUXIO16"]
    AUXIO16 = 16,
    #[doc = "15: AUX_EVCTL:EVSTAT0.AUXIO15"]
    AUXIO15 = 15,
    #[doc = "14: AUX_EVCTL:EVSTAT0.AUXIO14"]
    AUXIO14 = 14,
    #[doc = "13: AUX_EVCTL:EVSTAT0.AUXIO13"]
    AUXIO13 = 13,
    #[doc = "12: AUX_EVCTL:EVSTAT0.AUXIO12"]
    AUXIO12 = 12,
    #[doc = "11: AUX_EVCTL:EVSTAT0.AUXIO11"]
    AUXIO11 = 11,
    #[doc = "10: AUX_EVCTL:EVSTAT0.AUXIO10"]
    AUXIO10 = 10,
    #[doc = "9: AUX_EVCTL:EVSTAT0.AUXIO9"]
    AUXIO9 = 9,
    #[doc = "8: AUX_EVCTL:EVSTAT0.AUXIO8"]
    AUXIO8 = 8,
    #[doc = "7: AUX_EVCTL:EVSTAT0.AUXIO7"]
    AUXIO7 = 7,
    #[doc = "6: AUX_EVCTL:EVSTAT0.AUXIO6"]
    AUXIO6 = 6,
    #[doc = "5: AUX_EVCTL:EVSTAT0.AUXIO5"]
    AUXIO5 = 5,
    #[doc = "4: AUX_EVCTL:EVSTAT0.AUXIO4"]
    AUXIO4 = 4,
    #[doc = "3: AUX_EVCTL:EVSTAT0.AUXIO3"]
    AUXIO3 = 3,
    #[doc = "2: AUX_EVCTL:EVSTAT0.AUXIO2"]
    AUXIO2 = 2,
    #[doc = "1: AUX_EVCTL:EVSTAT0.AUXIO1"]
    AUXIO1 = 1,
    #[doc = "0: AUX_EVCTL:EVSTAT0.AUXIO0"]
    AUXIO0 = 0,
}
impl From<TICK_SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: TICK_SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TICK_SRC`"]
pub type TICK_SRC_R = crate::R<u8, TICK_SRC_A>;
impl TICK_SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TICK_SRC_A {
        match self.bits {
            63 => TICK_SRC_A::AUX_TIMER2_CLKSW_RDY,
            62 => TICK_SRC_A::AUX_DAC_HOLD_ACTIVE,
            61 => TICK_SRC_A::AUX_SMPH_AUTOTAKE_DONE,
            60 => TICK_SRC_A::AUX_ADC_FIFO_NOT_EMPTY,
            59 => TICK_SRC_A::AUX_ADC_FIFO_ALMOST_FULL,
            58 => TICK_SRC_A::AUX_ADC_IRQ,
            57 => TICK_SRC_A::AUX_ADC_DONE,
            56 => TICK_SRC_A::AUX_ISRC_RESET_N,
            55 => TICK_SRC_A::AUX_TDC_DONE,
            54 => TICK_SRC_A::AUX_TIMER0_EV,
            53 => TICK_SRC_A::NO_EVENT,
            52 => TICK_SRC_A::AUX_TIMER2_PULSE,
            51 => TICK_SRC_A::AUX_TIMER2_EV3,
            50 => TICK_SRC_A::AUX_TIMER2_EV2,
            49 => TICK_SRC_A::AUX_TIMER2_EV1,
            48 => TICK_SRC_A::AUX_TIMER2_EV0,
            47 => TICK_SRC_A::AUX_COMPB,
            46 => TICK_SRC_A::AUX_COMPA,
            45 => TICK_SRC_A::MCU_OBSMUX1,
            44 => TICK_SRC_A::MCU_OBSMUX0,
            43 => TICK_SRC_A::MCU_EV,
            42 => TICK_SRC_A::ACLK_REF,
            41 => TICK_SRC_A::VDDR_RECHARGE,
            40 => TICK_SRC_A::MCU_ACTIVE,
            39 => TICK_SRC_A::PWR_DWN,
            38 => TICK_SRC_A::SCLK_LF,
            37 => TICK_SRC_A::AON_BATMON_TEMP_UPD,
            36 => TICK_SRC_A::AON_BATMON_BAT_UPD,
            35 => TICK_SRC_A::AON_RTC_4KHZ,
            34 => TICK_SRC_A::AON_RTC_CH2_DLY,
            33 => TICK_SRC_A::AON_RTC_CH2,
            32 => TICK_SRC_A::MANUAL_EV,
            31 => TICK_SRC_A::AUXIO31,
            30 => TICK_SRC_A::AUXIO30,
            29 => TICK_SRC_A::AUXIO29,
            28 => TICK_SRC_A::AUXIO28,
            27 => TICK_SRC_A::AUXIO27,
            26 => TICK_SRC_A::AUXIO26,
            25 => TICK_SRC_A::AUXIO25,
            24 => TICK_SRC_A::AUXIO24,
            23 => TICK_SRC_A::AUXIO23,
            22 => TICK_SRC_A::AUXIO22,
            21 => TICK_SRC_A::AUXIO21,
            20 => TICK_SRC_A::AUXIO20,
            19 => TICK_SRC_A::AUXIO19,
            18 => TICK_SRC_A::AUXIO18,
            17 => TICK_SRC_A::AUXIO17,
            16 => TICK_SRC_A::AUXIO16,
            15 => TICK_SRC_A::AUXIO15,
            14 => TICK_SRC_A::AUXIO14,
            13 => TICK_SRC_A::AUXIO13,
            12 => TICK_SRC_A::AUXIO12,
            11 => TICK_SRC_A::AUXIO11,
            10 => TICK_SRC_A::AUXIO10,
            9 => TICK_SRC_A::AUXIO9,
            8 => TICK_SRC_A::AUXIO8,
            7 => TICK_SRC_A::AUXIO7,
            6 => TICK_SRC_A::AUXIO6,
            5 => TICK_SRC_A::AUXIO5,
            4 => TICK_SRC_A::AUXIO4,
            3 => TICK_SRC_A::AUXIO3,
            2 => TICK_SRC_A::AUXIO2,
            1 => TICK_SRC_A::AUXIO1,
            0 => TICK_SRC_A::AUXIO0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_CLKSW_RDY`"]
    #[inline(always)]
    pub fn is_aux_timer2_clksw_rdy(&self) -> bool {
        *self == TICK_SRC_A::AUX_TIMER2_CLKSW_RDY
    }
    #[doc = "Checks if the value of the field is `AUX_DAC_HOLD_ACTIVE`"]
    #[inline(always)]
    pub fn is_aux_dac_hold_active(&self) -> bool {
        *self == TICK_SRC_A::AUX_DAC_HOLD_ACTIVE
    }
    #[doc = "Checks if the value of the field is `AUX_SMPH_AUTOTAKE_DONE`"]
    #[inline(always)]
    pub fn is_aux_smph_autotake_done(&self) -> bool {
        *self == TICK_SRC_A::AUX_SMPH_AUTOTAKE_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_FIFO_NOT_EMPTY`"]
    #[inline(always)]
    pub fn is_aux_adc_fifo_not_empty(&self) -> bool {
        *self == TICK_SRC_A::AUX_ADC_FIFO_NOT_EMPTY
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_FIFO_ALMOST_FULL`"]
    #[inline(always)]
    pub fn is_aux_adc_fifo_almost_full(&self) -> bool {
        *self == TICK_SRC_A::AUX_ADC_FIFO_ALMOST_FULL
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_IRQ`"]
    #[inline(always)]
    pub fn is_aux_adc_irq(&self) -> bool {
        *self == TICK_SRC_A::AUX_ADC_IRQ
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == TICK_SRC_A::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ISRC_RESET_N`"]
    #[inline(always)]
    pub fn is_aux_isrc_reset_n(&self) -> bool {
        *self == TICK_SRC_A::AUX_ISRC_RESET_N
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == TICK_SRC_A::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == TICK_SRC_A::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `NO_EVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == TICK_SRC_A::NO_EVENT
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_PULSE`"]
    #[inline(always)]
    pub fn is_aux_timer2_pulse(&self) -> bool {
        *self == TICK_SRC_A::AUX_TIMER2_PULSE
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == TICK_SRC_A::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == TICK_SRC_A::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == TICK_SRC_A::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == TICK_SRC_A::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == TICK_SRC_A::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == TICK_SRC_A::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `MCU_OBSMUX1`"]
    #[inline(always)]
    pub fn is_mcu_obsmux1(&self) -> bool {
        *self == TICK_SRC_A::MCU_OBSMUX1
    }
    #[doc = "Checks if the value of the field is `MCU_OBSMUX0`"]
    #[inline(always)]
    pub fn is_mcu_obsmux0(&self) -> bool {
        *self == TICK_SRC_A::MCU_OBSMUX0
    }
    #[doc = "Checks if the value of the field is `MCU_EV`"]
    #[inline(always)]
    pub fn is_mcu_ev(&self) -> bool {
        *self == TICK_SRC_A::MCU_EV
    }
    #[doc = "Checks if the value of the field is `ACLK_REF`"]
    #[inline(always)]
    pub fn is_aclk_ref(&self) -> bool {
        *self == TICK_SRC_A::ACLK_REF
    }
    #[doc = "Checks if the value of the field is `VDDR_RECHARGE`"]
    #[inline(always)]
    pub fn is_vddr_recharge(&self) -> bool {
        *self == TICK_SRC_A::VDDR_RECHARGE
    }
    #[doc = "Checks if the value of the field is `MCU_ACTIVE`"]
    #[inline(always)]
    pub fn is_mcu_active(&self) -> bool {
        *self == TICK_SRC_A::MCU_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PWR_DWN`"]
    #[inline(always)]
    pub fn is_pwr_dwn(&self) -> bool {
        *self == TICK_SRC_A::PWR_DWN
    }
    #[doc = "Checks if the value of the field is `SCLK_LF`"]
    #[inline(always)]
    pub fn is_sclk_lf(&self) -> bool {
        *self == TICK_SRC_A::SCLK_LF
    }
    #[doc = "Checks if the value of the field is `AON_BATMON_TEMP_UPD`"]
    #[inline(always)]
    pub fn is_aon_batmon_temp_upd(&self) -> bool {
        *self == TICK_SRC_A::AON_BATMON_TEMP_UPD
    }
    #[doc = "Checks if the value of the field is `AON_BATMON_BAT_UPD`"]
    #[inline(always)]
    pub fn is_aon_batmon_bat_upd(&self) -> bool {
        *self == TICK_SRC_A::AON_BATMON_BAT_UPD
    }
    #[doc = "Checks if the value of the field is `AON_RTC_4KHZ`"]
    #[inline(always)]
    pub fn is_aon_rtc_4khz(&self) -> bool {
        *self == TICK_SRC_A::AON_RTC_4KHZ
    }
    #[doc = "Checks if the value of the field is `AON_RTC_CH2_DLY`"]
    #[inline(always)]
    pub fn is_aon_rtc_ch2_dly(&self) -> bool {
        *self == TICK_SRC_A::AON_RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `AON_RTC_CH2`"]
    #[inline(always)]
    pub fn is_aon_rtc_ch2(&self) -> bool {
        *self == TICK_SRC_A::AON_RTC_CH2
    }
    #[doc = "Checks if the value of the field is `MANUAL_EV`"]
    #[inline(always)]
    pub fn is_manual_ev(&self) -> bool {
        *self == TICK_SRC_A::MANUAL_EV
    }
    #[doc = "Checks if the value of the field is `AUXIO31`"]
    #[inline(always)]
    pub fn is_auxio31(&self) -> bool {
        *self == TICK_SRC_A::AUXIO31
    }
    #[doc = "Checks if the value of the field is `AUXIO30`"]
    #[inline(always)]
    pub fn is_auxio30(&self) -> bool {
        *self == TICK_SRC_A::AUXIO30
    }
    #[doc = "Checks if the value of the field is `AUXIO29`"]
    #[inline(always)]
    pub fn is_auxio29(&self) -> bool {
        *self == TICK_SRC_A::AUXIO29
    }
    #[doc = "Checks if the value of the field is `AUXIO28`"]
    #[inline(always)]
    pub fn is_auxio28(&self) -> bool {
        *self == TICK_SRC_A::AUXIO28
    }
    #[doc = "Checks if the value of the field is `AUXIO27`"]
    #[inline(always)]
    pub fn is_auxio27(&self) -> bool {
        *self == TICK_SRC_A::AUXIO27
    }
    #[doc = "Checks if the value of the field is `AUXIO26`"]
    #[inline(always)]
    pub fn is_auxio26(&self) -> bool {
        *self == TICK_SRC_A::AUXIO26
    }
    #[doc = "Checks if the value of the field is `AUXIO25`"]
    #[inline(always)]
    pub fn is_auxio25(&self) -> bool {
        *self == TICK_SRC_A::AUXIO25
    }
    #[doc = "Checks if the value of the field is `AUXIO24`"]
    #[inline(always)]
    pub fn is_auxio24(&self) -> bool {
        *self == TICK_SRC_A::AUXIO24
    }
    #[doc = "Checks if the value of the field is `AUXIO23`"]
    #[inline(always)]
    pub fn is_auxio23(&self) -> bool {
        *self == TICK_SRC_A::AUXIO23
    }
    #[doc = "Checks if the value of the field is `AUXIO22`"]
    #[inline(always)]
    pub fn is_auxio22(&self) -> bool {
        *self == TICK_SRC_A::AUXIO22
    }
    #[doc = "Checks if the value of the field is `AUXIO21`"]
    #[inline(always)]
    pub fn is_auxio21(&self) -> bool {
        *self == TICK_SRC_A::AUXIO21
    }
    #[doc = "Checks if the value of the field is `AUXIO20`"]
    #[inline(always)]
    pub fn is_auxio20(&self) -> bool {
        *self == TICK_SRC_A::AUXIO20
    }
    #[doc = "Checks if the value of the field is `AUXIO19`"]
    #[inline(always)]
    pub fn is_auxio19(&self) -> bool {
        *self == TICK_SRC_A::AUXIO19
    }
    #[doc = "Checks if the value of the field is `AUXIO18`"]
    #[inline(always)]
    pub fn is_auxio18(&self) -> bool {
        *self == TICK_SRC_A::AUXIO18
    }
    #[doc = "Checks if the value of the field is `AUXIO17`"]
    #[inline(always)]
    pub fn is_auxio17(&self) -> bool {
        *self == TICK_SRC_A::AUXIO17
    }
    #[doc = "Checks if the value of the field is `AUXIO16`"]
    #[inline(always)]
    pub fn is_auxio16(&self) -> bool {
        *self == TICK_SRC_A::AUXIO16
    }
    #[doc = "Checks if the value of the field is `AUXIO15`"]
    #[inline(always)]
    pub fn is_auxio15(&self) -> bool {
        *self == TICK_SRC_A::AUXIO15
    }
    #[doc = "Checks if the value of the field is `AUXIO14`"]
    #[inline(always)]
    pub fn is_auxio14(&self) -> bool {
        *self == TICK_SRC_A::AUXIO14
    }
    #[doc = "Checks if the value of the field is `AUXIO13`"]
    #[inline(always)]
    pub fn is_auxio13(&self) -> bool {
        *self == TICK_SRC_A::AUXIO13
    }
    #[doc = "Checks if the value of the field is `AUXIO12`"]
    #[inline(always)]
    pub fn is_auxio12(&self) -> bool {
        *self == TICK_SRC_A::AUXIO12
    }
    #[doc = "Checks if the value of the field is `AUXIO11`"]
    #[inline(always)]
    pub fn is_auxio11(&self) -> bool {
        *self == TICK_SRC_A::AUXIO11
    }
    #[doc = "Checks if the value of the field is `AUXIO10`"]
    #[inline(always)]
    pub fn is_auxio10(&self) -> bool {
        *self == TICK_SRC_A::AUXIO10
    }
    #[doc = "Checks if the value of the field is `AUXIO9`"]
    #[inline(always)]
    pub fn is_auxio9(&self) -> bool {
        *self == TICK_SRC_A::AUXIO9
    }
    #[doc = "Checks if the value of the field is `AUXIO8`"]
    #[inline(always)]
    pub fn is_auxio8(&self) -> bool {
        *self == TICK_SRC_A::AUXIO8
    }
    #[doc = "Checks if the value of the field is `AUXIO7`"]
    #[inline(always)]
    pub fn is_auxio7(&self) -> bool {
        *self == TICK_SRC_A::AUXIO7
    }
    #[doc = "Checks if the value of the field is `AUXIO6`"]
    #[inline(always)]
    pub fn is_auxio6(&self) -> bool {
        *self == TICK_SRC_A::AUXIO6
    }
    #[doc = "Checks if the value of the field is `AUXIO5`"]
    #[inline(always)]
    pub fn is_auxio5(&self) -> bool {
        *self == TICK_SRC_A::AUXIO5
    }
    #[doc = "Checks if the value of the field is `AUXIO4`"]
    #[inline(always)]
    pub fn is_auxio4(&self) -> bool {
        *self == TICK_SRC_A::AUXIO4
    }
    #[doc = "Checks if the value of the field is `AUXIO3`"]
    #[inline(always)]
    pub fn is_auxio3(&self) -> bool {
        *self == TICK_SRC_A::AUXIO3
    }
    #[doc = "Checks if the value of the field is `AUXIO2`"]
    #[inline(always)]
    pub fn is_auxio2(&self) -> bool {
        *self == TICK_SRC_A::AUXIO2
    }
    #[doc = "Checks if the value of the field is `AUXIO1`"]
    #[inline(always)]
    pub fn is_auxio1(&self) -> bool {
        *self == TICK_SRC_A::AUXIO1
    }
    #[doc = "Checks if the value of the field is `AUXIO0`"]
    #[inline(always)]
    pub fn is_auxio0(&self) -> bool {
        *self == TICK_SRC_A::AUXIO0
    }
}
#[doc = "Write proxy for field `TICK_SRC`"]
pub struct TICK_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TICK_SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TICK_SRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_CLKSWITCH_RDY"]
    #[inline(always)]
    pub fn aux_timer2_clksw_rdy(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUX_TIMER2_CLKSW_RDY)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_DAC_HOLD_ACTIVE"]
    #[inline(always)]
    pub fn aux_dac_hold_active(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUX_DAC_HOLD_ACTIVE)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn aux_smph_autotake_done(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUX_SMPH_AUTOTAKE_DONE)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_NOT_EMPTY"]
    #[inline(always)]
    pub fn aux_adc_fifo_not_empty(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUX_ADC_FIFO_NOT_EMPTY)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn aux_adc_fifo_almost_full(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUX_ADC_FIFO_ALMOST_FULL)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_IRQ"]
    #[inline(always)]
    pub fn aux_adc_irq(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUX_ADC_IRQ)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_DONE"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUX_ADC_DONE)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ISRC_RESET_N"]
    #[inline(always)]
    pub fn aux_isrc_reset_n(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUX_ISRC_RESET_N)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TDC_DONE"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUX_TDC_DONE)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER0_EV"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUX_TIMER0_EV)
    }
    #[doc = "No event."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(TICK_SRC_A::NO_EVENT)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE"]
    #[inline(always)]
    pub fn aux_timer2_pulse(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUX_TIMER2_PULSE)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUX_TIMER2_EV3)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUX_TIMER2_EV2)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUX_TIMER2_EV1)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUX_TIMER2_EV0)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPB"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUX_COMPB)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPA"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUX_COMPA)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_OBSMUX1"]
    #[inline(always)]
    pub fn mcu_obsmux1(self) -> &'a mut W {
        self.variant(TICK_SRC_A::MCU_OBSMUX1)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_OBSMUX0"]
    #[inline(always)]
    pub fn mcu_obsmux0(self) -> &'a mut W {
        self.variant(TICK_SRC_A::MCU_OBSMUX0)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_EV"]
    #[inline(always)]
    pub fn mcu_ev(self) -> &'a mut W {
        self.variant(TICK_SRC_A::MCU_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.ACLK_REF"]
    #[inline(always)]
    pub fn aclk_ref(self) -> &'a mut W {
        self.variant(TICK_SRC_A::ACLK_REF)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.VDDR_RECHARGE"]
    #[inline(always)]
    pub fn vddr_recharge(self) -> &'a mut W {
        self.variant(TICK_SRC_A::VDDR_RECHARGE)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_ACTIVE"]
    #[inline(always)]
    pub fn mcu_active(self) -> &'a mut W {
        self.variant(TICK_SRC_A::MCU_ACTIVE)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.PWR_DWN"]
    #[inline(always)]
    pub fn pwr_dwn(self) -> &'a mut W {
        self.variant(TICK_SRC_A::PWR_DWN)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.SCLK_LF"]
    #[inline(always)]
    pub fn sclk_lf(self) -> &'a mut W {
        self.variant(TICK_SRC_A::SCLK_LF)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_BATMON_TEMP_UPD"]
    #[inline(always)]
    pub fn aon_batmon_temp_upd(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AON_BATMON_TEMP_UPD)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_BATMON_BAT_UPD"]
    #[inline(always)]
    pub fn aon_batmon_bat_upd(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AON_BATMON_BAT_UPD)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_4KHZ"]
    #[inline(always)]
    pub fn aon_rtc_4khz(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AON_RTC_4KHZ)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY"]
    #[inline(always)]
    pub fn aon_rtc_ch2_dly(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AON_RTC_CH2_DLY)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2"]
    #[inline(always)]
    pub fn aon_rtc_ch2(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AON_RTC_CH2)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MANUAL_EV"]
    #[inline(always)]
    pub fn manual_ev(self) -> &'a mut W {
        self.variant(TICK_SRC_A::MANUAL_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO31"]
    #[inline(always)]
    pub fn auxio31(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO31)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO30"]
    #[inline(always)]
    pub fn auxio30(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO30)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO29"]
    #[inline(always)]
    pub fn auxio29(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO29)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO28"]
    #[inline(always)]
    pub fn auxio28(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO28)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO27"]
    #[inline(always)]
    pub fn auxio27(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO27)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO26"]
    #[inline(always)]
    pub fn auxio26(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO26)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO25"]
    #[inline(always)]
    pub fn auxio25(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO25)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO24"]
    #[inline(always)]
    pub fn auxio24(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO24)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO23"]
    #[inline(always)]
    pub fn auxio23(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO23)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO22"]
    #[inline(always)]
    pub fn auxio22(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO22)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO21"]
    #[inline(always)]
    pub fn auxio21(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO21)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO20"]
    #[inline(always)]
    pub fn auxio20(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO20)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO19"]
    #[inline(always)]
    pub fn auxio19(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO19)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO18"]
    #[inline(always)]
    pub fn auxio18(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO18)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO17"]
    #[inline(always)]
    pub fn auxio17(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO17)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO16"]
    #[inline(always)]
    pub fn auxio16(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO16)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO15"]
    #[inline(always)]
    pub fn auxio15(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO15)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO14"]
    #[inline(always)]
    pub fn auxio14(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO14)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO13"]
    #[inline(always)]
    pub fn auxio13(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO13)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO12"]
    #[inline(always)]
    pub fn auxio12(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO12)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO11"]
    #[inline(always)]
    pub fn auxio11(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO11)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO10"]
    #[inline(always)]
    pub fn auxio10(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO10)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO9"]
    #[inline(always)]
    pub fn auxio9(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO9)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO8"]
    #[inline(always)]
    pub fn auxio8(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO8)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO7"]
    #[inline(always)]
    pub fn auxio7(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO7)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO6"]
    #[inline(always)]
    pub fn auxio6(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO6)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO5"]
    #[inline(always)]
    pub fn auxio5(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO5)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO4"]
    #[inline(always)]
    pub fn auxio4(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO4)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO3"]
    #[inline(always)]
    pub fn auxio3(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO3)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn auxio2(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO2)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn auxio1(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO1)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn auxio0(self) -> &'a mut W {
        self.variant(TICK_SRC_A::AUXIO0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `PRE`"]
pub type PRE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRE`"]
pub struct PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `RESERVED2`"]
pub type RESERVED2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED2`"]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "1:1\\]
Timer 1 mode. Configure source for Timer 1 prescaler.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "1: Use event set by TICK_SRC as source for prescaler."]
    TICK = 1,
    #[doc = "0: Use clock as source for prescaler. Note that AUX_SYSIF:PEROPRATE.TIMER01_OP_RATE sets the clock frequency."]
    CLK = 0,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<bool, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            true => MODE_A::TICK,
            false => MODE_A::CLK,
        }
    }
    #[doc = "Checks if the value of the field is `TICK`"]
    #[inline(always)]
    pub fn is_tick(&self) -> bool {
        *self == MODE_A::TICK
    }
    #[doc = "Checks if the value of the field is `CLK`"]
    #[inline(always)]
    pub fn is_clk(&self) -> bool {
        *self == MODE_A::CLK
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use event set by TICK_SRC as source for prescaler."]
    #[inline(always)]
    pub fn tick(self) -> &'a mut W {
        self.variant(MODE_A::TICK)
    }
    #[doc = "Use clock as source for prescaler. Note that AUX_SYSIF:PEROPRATE.TIMER01_OP_RATE sets the clock frequency."]
    #[inline(always)]
    pub fn clk(self) -> &'a mut W {
        self.variant(MODE_A::CLK)
    }
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
#[doc = "0:0\\]
Timer 1 reload mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RELOAD_A {
    #[doc = "1: Continuous mode.\n\nTimer 1 restarts when the counter value becomes equal to or greater than ( T1TARGET.VALUE - 1)."]
    CONT = 1,
    #[doc = "0: Manual mode.\n\nTimer 1 stops and T1CTL.EN becomes 0 when the counter value becomes equal to or greater than T1TARGET.VALUE."]
    MAN = 0,
}
impl From<RELOAD_A> for bool {
    #[inline(always)]
    fn from(variant: RELOAD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RELOAD`"]
pub type RELOAD_R = crate::R<bool, RELOAD_A>;
impl RELOAD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RELOAD_A {
        match self.bits {
            true => RELOAD_A::CONT,
            false => RELOAD_A::MAN,
        }
    }
    #[doc = "Checks if the value of the field is `CONT`"]
    #[inline(always)]
    pub fn is_cont(&self) -> bool {
        *self == RELOAD_A::CONT
    }
    #[doc = "Checks if the value of the field is `MAN`"]
    #[inline(always)]
    pub fn is_man(&self) -> bool {
        *self == RELOAD_A::MAN
    }
}
#[doc = "Write proxy for field `RELOAD`"]
pub struct RELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RELOAD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Continuous mode. Timer 1 restarts when the counter value becomes equal to or greater than ( T1TARGET.VALUE - 1)."]
    #[inline(always)]
    pub fn cont(self) -> &'a mut W {
        self.variant(RELOAD_A::CONT)
    }
    #[doc = "Manual mode. Timer 1 stops and T1CTL.EN becomes 0 when the counter value becomes equal to or greater than T1TARGET.VALUE."]
    #[inline(always)]
    pub fn man(self) -> &'a mut W {
        self.variant(RELOAD_A::MAN)
    }
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
    #[doc = "Bits 15:31 - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&self) -> RESERVED15_R {
        RESERVED15_R::new(((self.bits >> 15) & 0x0001_ffff) as u32)
    }
    #[doc = "Bit 14 - 14:14\\]
Tick source polarity for Timer 1."]
    #[inline(always)]
    pub fn tick_src_pol(&self) -> TICK_SRC_POL_R {
        TICK_SRC_POL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Select Timer 1 tick source from the synchronous event bus."]
    #[inline(always)]
    pub fn tick_src(&self) -> TICK_SRC_R {
        TICK_SRC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Prescaler division ratio is 2^PRE: 0x0: Divide by 1. 0x1: Divide by 2. 0x2: Divide by 4. ... 0xF: Divide by 32,768."]
    #[inline(always)]
    pub fn pre(&self) -> PRE_R {
        PRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - 1:1\\]
Timer 1 mode. Configure source for Timer 1 prescaler."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Timer 1 reload mode."]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 15:31 - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&mut self) -> RESERVED15_W {
        RESERVED15_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
Tick source polarity for Timer 1."]
    #[inline(always)]
    pub fn tick_src_pol(&mut self) -> TICK_SRC_POL_W {
        TICK_SRC_POL_W { w: self }
    }
    #[doc = "Bits 8:13 - 13:8\\]
Select Timer 1 tick source from the synchronous event bus."]
    #[inline(always)]
    pub fn tick_src(&mut self) -> TICK_SRC_W {
        TICK_SRC_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\]
Prescaler division ratio is 2^PRE: 0x0: Divide by 1. 0x1: Divide by 2. 0x2: Divide by 4. ... 0xF: Divide by 32,768."]
    #[inline(always)]
    pub fn pre(&mut self) -> PRE_W {
        PRE_W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Timer 1 mode. Configure source for Timer 1 prescaler."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Timer 1 reload mode."]
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W {
        RELOAD_W { w: self }
    }
}
