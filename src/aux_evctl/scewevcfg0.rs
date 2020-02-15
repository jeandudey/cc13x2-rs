#[doc = "Reader of register SCEWEVCFG0"]
pub type R = crate::R<u32, super::SCEWEVCFG0>;
#[doc = "Writer for register SCEWEVCFG0"]
pub type W = crate::W<u32, super::SCEWEVCFG0>;
#[doc = "Register SCEWEVCFG0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SCEWEVCFG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED7`"]
pub type RESERVED7_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED7`"]
pub struct RESERVED7_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff_ffff << 7)) | (((value as u32) & 0x01ff_ffff) << 7);
        self.w
    }
}
#[doc = "Reader of field `COMB_EV_EN`"]
pub type COMB_EV_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMB_EV_EN`"]
pub struct COMB_EV_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMB_EV_EN_W<'a> {
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
#[doc = "5:0\\]
Select the event source from the synchronous event bus to be used in event equation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EV0_SEL_A {
    #[doc = "63: EVSTAT3.AUX_TIMER2_CLKSWITCH_RDY"]
    AUX_TIMER2_CLKSWITCH_RDY = 63,
    #[doc = "62: EVSTAT3.AUX_DAC_HOLD_ACTIVE"]
    AUX_DAC_HOLD_ACTIVE = 62,
    #[doc = "61: EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    AUX_SMPH_AUTOTAKE_DONE = 61,
    #[doc = "60: EVSTAT3.AUX_ADC_FIFO_NOT_EMPTY"]
    AUX_ADC_FIFO_NOT_EMPTY = 60,
    #[doc = "59: EVSTAT3.AUX_ADC_FIFO_ALMOST_FULL"]
    AUX_ADC_FIFO_ALMOST_FULL = 59,
    #[doc = "58: EVSTAT3.AUX_ADC_IRQ"]
    AUX_ADC_IRQ = 58,
    #[doc = "57: EVSTAT3.AUX_ADC_DONE"]
    AUX_ADC_DONE = 57,
    #[doc = "56: EVSTAT3.AUX_ISRC_RESET_N"]
    AUX_ISRC_RESET_N = 56,
    #[doc = "55: EVSTAT3.AUX_TDC_DONE"]
    AUX_TDC_DONE = 55,
    #[doc = "54: EVSTAT3.AUX_TIMER0_EV"]
    AUX_TIMER0_EV = 54,
    #[doc = "53: EVSTAT3.AUX_TIMER1_EV"]
    AUX_TIMER1_EV = 53,
    #[doc = "52: EVSTAT3.AUX_TIMER2_PULSE"]
    AUX_TIMER2_PULSE = 52,
    #[doc = "51: EVSTAT3.AUX_TIMER2_EV3"]
    AUX_TIMER2_EV3 = 51,
    #[doc = "50: EVSTAT3.AUX_TIMER2_EV2"]
    AUX_TIMER2_EV2 = 50,
    #[doc = "49: EVSTAT3.AUX_TIMER2_EV1"]
    AUX_TIMER2_EV1 = 49,
    #[doc = "48: EVSTAT3.AUX_TIMER2_EV0"]
    AUX_TIMER2_EV0 = 48,
    #[doc = "47: EVSTAT2.AUX_COMPB"]
    AUX_COMPB = 47,
    #[doc = "46: EVSTAT2.AUX_COMPA"]
    AUX_COMPA = 46,
    #[doc = "45: EVSTAT2.MCU_OBSMUX1"]
    MCU_OBSMUX1 = 45,
    #[doc = "44: EVSTAT2.MCU_OBSMUX0"]
    MCU_OBSMUX0 = 44,
    #[doc = "43: EVSTAT2.MCU_EV"]
    MCU_EV = 43,
    #[doc = "42: EVSTAT2.ACLK_REF"]
    ACLK_REF = 42,
    #[doc = "41: EVSTAT2.VDDR_RECHARGE"]
    VDDR_RECHARGE = 41,
    #[doc = "40: EVSTAT2.MCU_ACTIVE"]
    MCU_ACTIVE = 40,
    #[doc = "39: EVSTAT2.PWR_DWN"]
    PWR_DWN = 39,
    #[doc = "38: EVSTAT2.SCLK_LF"]
    SCLK_LF = 38,
    #[doc = "37: EVSTAT2.AON_BATMON_TEMP_UPD"]
    AON_BATMON_TEMP_UPD = 37,
    #[doc = "36: EVSTAT2.AON_BATMON_BAT_UPD"]
    AON_BATMON_BAT_UPD = 36,
    #[doc = "35: EVSTAT2.AON_RTC_4KHZ"]
    AON_RTC_4KHZ = 35,
    #[doc = "34: EVSTAT2.AON_RTC_CH2_DLY"]
    AON_RTC_CH2_DLY = 34,
    #[doc = "33: EVSTAT2.AON_RTC_CH2"]
    AON_RTC_CH2 = 33,
    #[doc = "32: Programmable delay event as described in PROGDLY"]
    AUX_PROG_DLY_IDLE = 32,
    #[doc = "31: EVSTAT1.AUXIO31"]
    AUXIO31 = 31,
    #[doc = "30: EVSTAT1.AUXIO30"]
    AUXIO30 = 30,
    #[doc = "29: EVSTAT1.AUXIO29"]
    AUXIO29 = 29,
    #[doc = "28: EVSTAT1.AUXIO28"]
    AUXIO28 = 28,
    #[doc = "27: EVSTAT1.AUXIO27"]
    AUXIO27 = 27,
    #[doc = "26: EVSTAT1.AUXIO26"]
    AUXIO26 = 26,
    #[doc = "25: EVSTAT1.AUXIO25"]
    AUXIO25 = 25,
    #[doc = "24: EVSTAT1.AUXIO24"]
    AUXIO24 = 24,
    #[doc = "23: EVSTAT1.AUXIO23"]
    AUXIO23 = 23,
    #[doc = "22: EVSTAT1.AUXIO22"]
    AUXIO22 = 22,
    #[doc = "21: EVSTAT1.AUXIO21"]
    AUXIO21 = 21,
    #[doc = "20: EVSTAT1.AUXIO20"]
    AUXIO20 = 20,
    #[doc = "19: EVSTAT1.AUXIO19"]
    AUXIO19 = 19,
    #[doc = "18: EVSTAT1.AUXIO18"]
    AUXIO18 = 18,
    #[doc = "17: EVSTAT1.AUXIO17"]
    AUXIO17 = 17,
    #[doc = "16: EVSTAT1.AUXIO16"]
    AUXIO16 = 16,
    #[doc = "15: EVSTAT0.AUXIO15"]
    AUXIO15 = 15,
    #[doc = "14: EVSTAT0.AUXIO14"]
    AUXIO14 = 14,
    #[doc = "13: EVSTAT0.AUXIO13"]
    AUXIO13 = 13,
    #[doc = "12: EVSTAT0.AUXIO12"]
    AUXIO12 = 12,
    #[doc = "11: EVSTAT0.AUXIO11"]
    AUXIO11 = 11,
    #[doc = "10: EVSTAT0.AUXIO10"]
    AUXIO10 = 10,
    #[doc = "9: EVSTAT0.AUXIO9"]
    AUXIO9 = 9,
    #[doc = "8: EVSTAT0.AUXIO8"]
    AUXIO8 = 8,
    #[doc = "7: EVSTAT0.AUXIO7"]
    AUXIO7 = 7,
    #[doc = "6: EVSTAT0.AUXIO6"]
    AUXIO6 = 6,
    #[doc = "5: EVSTAT0.AUXIO5"]
    AUXIO5 = 5,
    #[doc = "4: EVSTAT0.AUXIO4"]
    AUXIO4 = 4,
    #[doc = "3: EVSTAT0.AUXIO3"]
    AUXIO3 = 3,
    #[doc = "2: EVSTAT0.AUXIO2"]
    AUXIO2 = 2,
    #[doc = "1: EVSTAT0.AUXIO1"]
    AUXIO1 = 1,
    #[doc = "0: EVSTAT0.AUXIO0"]
    AUXIO0 = 0,
}
impl From<EV0_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EV0_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EV0_SEL`"]
pub type EV0_SEL_R = crate::R<u8, EV0_SEL_A>;
impl EV0_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EV0_SEL_A {
        match self.bits {
            63 => EV0_SEL_A::AUX_TIMER2_CLKSWITCH_RDY,
            62 => EV0_SEL_A::AUX_DAC_HOLD_ACTIVE,
            61 => EV0_SEL_A::AUX_SMPH_AUTOTAKE_DONE,
            60 => EV0_SEL_A::AUX_ADC_FIFO_NOT_EMPTY,
            59 => EV0_SEL_A::AUX_ADC_FIFO_ALMOST_FULL,
            58 => EV0_SEL_A::AUX_ADC_IRQ,
            57 => EV0_SEL_A::AUX_ADC_DONE,
            56 => EV0_SEL_A::AUX_ISRC_RESET_N,
            55 => EV0_SEL_A::AUX_TDC_DONE,
            54 => EV0_SEL_A::AUX_TIMER0_EV,
            53 => EV0_SEL_A::AUX_TIMER1_EV,
            52 => EV0_SEL_A::AUX_TIMER2_PULSE,
            51 => EV0_SEL_A::AUX_TIMER2_EV3,
            50 => EV0_SEL_A::AUX_TIMER2_EV2,
            49 => EV0_SEL_A::AUX_TIMER2_EV1,
            48 => EV0_SEL_A::AUX_TIMER2_EV0,
            47 => EV0_SEL_A::AUX_COMPB,
            46 => EV0_SEL_A::AUX_COMPA,
            45 => EV0_SEL_A::MCU_OBSMUX1,
            44 => EV0_SEL_A::MCU_OBSMUX0,
            43 => EV0_SEL_A::MCU_EV,
            42 => EV0_SEL_A::ACLK_REF,
            41 => EV0_SEL_A::VDDR_RECHARGE,
            40 => EV0_SEL_A::MCU_ACTIVE,
            39 => EV0_SEL_A::PWR_DWN,
            38 => EV0_SEL_A::SCLK_LF,
            37 => EV0_SEL_A::AON_BATMON_TEMP_UPD,
            36 => EV0_SEL_A::AON_BATMON_BAT_UPD,
            35 => EV0_SEL_A::AON_RTC_4KHZ,
            34 => EV0_SEL_A::AON_RTC_CH2_DLY,
            33 => EV0_SEL_A::AON_RTC_CH2,
            32 => EV0_SEL_A::AUX_PROG_DLY_IDLE,
            31 => EV0_SEL_A::AUXIO31,
            30 => EV0_SEL_A::AUXIO30,
            29 => EV0_SEL_A::AUXIO29,
            28 => EV0_SEL_A::AUXIO28,
            27 => EV0_SEL_A::AUXIO27,
            26 => EV0_SEL_A::AUXIO26,
            25 => EV0_SEL_A::AUXIO25,
            24 => EV0_SEL_A::AUXIO24,
            23 => EV0_SEL_A::AUXIO23,
            22 => EV0_SEL_A::AUXIO22,
            21 => EV0_SEL_A::AUXIO21,
            20 => EV0_SEL_A::AUXIO20,
            19 => EV0_SEL_A::AUXIO19,
            18 => EV0_SEL_A::AUXIO18,
            17 => EV0_SEL_A::AUXIO17,
            16 => EV0_SEL_A::AUXIO16,
            15 => EV0_SEL_A::AUXIO15,
            14 => EV0_SEL_A::AUXIO14,
            13 => EV0_SEL_A::AUXIO13,
            12 => EV0_SEL_A::AUXIO12,
            11 => EV0_SEL_A::AUXIO11,
            10 => EV0_SEL_A::AUXIO10,
            9 => EV0_SEL_A::AUXIO9,
            8 => EV0_SEL_A::AUXIO8,
            7 => EV0_SEL_A::AUXIO7,
            6 => EV0_SEL_A::AUXIO6,
            5 => EV0_SEL_A::AUXIO5,
            4 => EV0_SEL_A::AUXIO4,
            3 => EV0_SEL_A::AUXIO3,
            2 => EV0_SEL_A::AUXIO2,
            1 => EV0_SEL_A::AUXIO1,
            0 => EV0_SEL_A::AUXIO0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_CLKSWITCH_RDY`"]
    #[inline(always)]
    pub fn is_aux_timer2_clkswitch_rdy(&self) -> bool {
        *self == EV0_SEL_A::AUX_TIMER2_CLKSWITCH_RDY
    }
    #[doc = "Checks if the value of the field is `AUX_DAC_HOLD_ACTIVE`"]
    #[inline(always)]
    pub fn is_aux_dac_hold_active(&self) -> bool {
        *self == EV0_SEL_A::AUX_DAC_HOLD_ACTIVE
    }
    #[doc = "Checks if the value of the field is `AUX_SMPH_AUTOTAKE_DONE`"]
    #[inline(always)]
    pub fn is_aux_smph_autotake_done(&self) -> bool {
        *self == EV0_SEL_A::AUX_SMPH_AUTOTAKE_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_FIFO_NOT_EMPTY`"]
    #[inline(always)]
    pub fn is_aux_adc_fifo_not_empty(&self) -> bool {
        *self == EV0_SEL_A::AUX_ADC_FIFO_NOT_EMPTY
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_FIFO_ALMOST_FULL`"]
    #[inline(always)]
    pub fn is_aux_adc_fifo_almost_full(&self) -> bool {
        *self == EV0_SEL_A::AUX_ADC_FIFO_ALMOST_FULL
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_IRQ`"]
    #[inline(always)]
    pub fn is_aux_adc_irq(&self) -> bool {
        *self == EV0_SEL_A::AUX_ADC_IRQ
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == EV0_SEL_A::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ISRC_RESET_N`"]
    #[inline(always)]
    pub fn is_aux_isrc_reset_n(&self) -> bool {
        *self == EV0_SEL_A::AUX_ISRC_RESET_N
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == EV0_SEL_A::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == EV0_SEL_A::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == EV0_SEL_A::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_PULSE`"]
    #[inline(always)]
    pub fn is_aux_timer2_pulse(&self) -> bool {
        *self == EV0_SEL_A::AUX_TIMER2_PULSE
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == EV0_SEL_A::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == EV0_SEL_A::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == EV0_SEL_A::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == EV0_SEL_A::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == EV0_SEL_A::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == EV0_SEL_A::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `MCU_OBSMUX1`"]
    #[inline(always)]
    pub fn is_mcu_obsmux1(&self) -> bool {
        *self == EV0_SEL_A::MCU_OBSMUX1
    }
    #[doc = "Checks if the value of the field is `MCU_OBSMUX0`"]
    #[inline(always)]
    pub fn is_mcu_obsmux0(&self) -> bool {
        *self == EV0_SEL_A::MCU_OBSMUX0
    }
    #[doc = "Checks if the value of the field is `MCU_EV`"]
    #[inline(always)]
    pub fn is_mcu_ev(&self) -> bool {
        *self == EV0_SEL_A::MCU_EV
    }
    #[doc = "Checks if the value of the field is `ACLK_REF`"]
    #[inline(always)]
    pub fn is_aclk_ref(&self) -> bool {
        *self == EV0_SEL_A::ACLK_REF
    }
    #[doc = "Checks if the value of the field is `VDDR_RECHARGE`"]
    #[inline(always)]
    pub fn is_vddr_recharge(&self) -> bool {
        *self == EV0_SEL_A::VDDR_RECHARGE
    }
    #[doc = "Checks if the value of the field is `MCU_ACTIVE`"]
    #[inline(always)]
    pub fn is_mcu_active(&self) -> bool {
        *self == EV0_SEL_A::MCU_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PWR_DWN`"]
    #[inline(always)]
    pub fn is_pwr_dwn(&self) -> bool {
        *self == EV0_SEL_A::PWR_DWN
    }
    #[doc = "Checks if the value of the field is `SCLK_LF`"]
    #[inline(always)]
    pub fn is_sclk_lf(&self) -> bool {
        *self == EV0_SEL_A::SCLK_LF
    }
    #[doc = "Checks if the value of the field is `AON_BATMON_TEMP_UPD`"]
    #[inline(always)]
    pub fn is_aon_batmon_temp_upd(&self) -> bool {
        *self == EV0_SEL_A::AON_BATMON_TEMP_UPD
    }
    #[doc = "Checks if the value of the field is `AON_BATMON_BAT_UPD`"]
    #[inline(always)]
    pub fn is_aon_batmon_bat_upd(&self) -> bool {
        *self == EV0_SEL_A::AON_BATMON_BAT_UPD
    }
    #[doc = "Checks if the value of the field is `AON_RTC_4KHZ`"]
    #[inline(always)]
    pub fn is_aon_rtc_4khz(&self) -> bool {
        *self == EV0_SEL_A::AON_RTC_4KHZ
    }
    #[doc = "Checks if the value of the field is `AON_RTC_CH2_DLY`"]
    #[inline(always)]
    pub fn is_aon_rtc_ch2_dly(&self) -> bool {
        *self == EV0_SEL_A::AON_RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `AON_RTC_CH2`"]
    #[inline(always)]
    pub fn is_aon_rtc_ch2(&self) -> bool {
        *self == EV0_SEL_A::AON_RTC_CH2
    }
    #[doc = "Checks if the value of the field is `AUX_PROG_DLY_IDLE`"]
    #[inline(always)]
    pub fn is_aux_prog_dly_idle(&self) -> bool {
        *self == EV0_SEL_A::AUX_PROG_DLY_IDLE
    }
    #[doc = "Checks if the value of the field is `AUXIO31`"]
    #[inline(always)]
    pub fn is_auxio31(&self) -> bool {
        *self == EV0_SEL_A::AUXIO31
    }
    #[doc = "Checks if the value of the field is `AUXIO30`"]
    #[inline(always)]
    pub fn is_auxio30(&self) -> bool {
        *self == EV0_SEL_A::AUXIO30
    }
    #[doc = "Checks if the value of the field is `AUXIO29`"]
    #[inline(always)]
    pub fn is_auxio29(&self) -> bool {
        *self == EV0_SEL_A::AUXIO29
    }
    #[doc = "Checks if the value of the field is `AUXIO28`"]
    #[inline(always)]
    pub fn is_auxio28(&self) -> bool {
        *self == EV0_SEL_A::AUXIO28
    }
    #[doc = "Checks if the value of the field is `AUXIO27`"]
    #[inline(always)]
    pub fn is_auxio27(&self) -> bool {
        *self == EV0_SEL_A::AUXIO27
    }
    #[doc = "Checks if the value of the field is `AUXIO26`"]
    #[inline(always)]
    pub fn is_auxio26(&self) -> bool {
        *self == EV0_SEL_A::AUXIO26
    }
    #[doc = "Checks if the value of the field is `AUXIO25`"]
    #[inline(always)]
    pub fn is_auxio25(&self) -> bool {
        *self == EV0_SEL_A::AUXIO25
    }
    #[doc = "Checks if the value of the field is `AUXIO24`"]
    #[inline(always)]
    pub fn is_auxio24(&self) -> bool {
        *self == EV0_SEL_A::AUXIO24
    }
    #[doc = "Checks if the value of the field is `AUXIO23`"]
    #[inline(always)]
    pub fn is_auxio23(&self) -> bool {
        *self == EV0_SEL_A::AUXIO23
    }
    #[doc = "Checks if the value of the field is `AUXIO22`"]
    #[inline(always)]
    pub fn is_auxio22(&self) -> bool {
        *self == EV0_SEL_A::AUXIO22
    }
    #[doc = "Checks if the value of the field is `AUXIO21`"]
    #[inline(always)]
    pub fn is_auxio21(&self) -> bool {
        *self == EV0_SEL_A::AUXIO21
    }
    #[doc = "Checks if the value of the field is `AUXIO20`"]
    #[inline(always)]
    pub fn is_auxio20(&self) -> bool {
        *self == EV0_SEL_A::AUXIO20
    }
    #[doc = "Checks if the value of the field is `AUXIO19`"]
    #[inline(always)]
    pub fn is_auxio19(&self) -> bool {
        *self == EV0_SEL_A::AUXIO19
    }
    #[doc = "Checks if the value of the field is `AUXIO18`"]
    #[inline(always)]
    pub fn is_auxio18(&self) -> bool {
        *self == EV0_SEL_A::AUXIO18
    }
    #[doc = "Checks if the value of the field is `AUXIO17`"]
    #[inline(always)]
    pub fn is_auxio17(&self) -> bool {
        *self == EV0_SEL_A::AUXIO17
    }
    #[doc = "Checks if the value of the field is `AUXIO16`"]
    #[inline(always)]
    pub fn is_auxio16(&self) -> bool {
        *self == EV0_SEL_A::AUXIO16
    }
    #[doc = "Checks if the value of the field is `AUXIO15`"]
    #[inline(always)]
    pub fn is_auxio15(&self) -> bool {
        *self == EV0_SEL_A::AUXIO15
    }
    #[doc = "Checks if the value of the field is `AUXIO14`"]
    #[inline(always)]
    pub fn is_auxio14(&self) -> bool {
        *self == EV0_SEL_A::AUXIO14
    }
    #[doc = "Checks if the value of the field is `AUXIO13`"]
    #[inline(always)]
    pub fn is_auxio13(&self) -> bool {
        *self == EV0_SEL_A::AUXIO13
    }
    #[doc = "Checks if the value of the field is `AUXIO12`"]
    #[inline(always)]
    pub fn is_auxio12(&self) -> bool {
        *self == EV0_SEL_A::AUXIO12
    }
    #[doc = "Checks if the value of the field is `AUXIO11`"]
    #[inline(always)]
    pub fn is_auxio11(&self) -> bool {
        *self == EV0_SEL_A::AUXIO11
    }
    #[doc = "Checks if the value of the field is `AUXIO10`"]
    #[inline(always)]
    pub fn is_auxio10(&self) -> bool {
        *self == EV0_SEL_A::AUXIO10
    }
    #[doc = "Checks if the value of the field is `AUXIO9`"]
    #[inline(always)]
    pub fn is_auxio9(&self) -> bool {
        *self == EV0_SEL_A::AUXIO9
    }
    #[doc = "Checks if the value of the field is `AUXIO8`"]
    #[inline(always)]
    pub fn is_auxio8(&self) -> bool {
        *self == EV0_SEL_A::AUXIO8
    }
    #[doc = "Checks if the value of the field is `AUXIO7`"]
    #[inline(always)]
    pub fn is_auxio7(&self) -> bool {
        *self == EV0_SEL_A::AUXIO7
    }
    #[doc = "Checks if the value of the field is `AUXIO6`"]
    #[inline(always)]
    pub fn is_auxio6(&self) -> bool {
        *self == EV0_SEL_A::AUXIO6
    }
    #[doc = "Checks if the value of the field is `AUXIO5`"]
    #[inline(always)]
    pub fn is_auxio5(&self) -> bool {
        *self == EV0_SEL_A::AUXIO5
    }
    #[doc = "Checks if the value of the field is `AUXIO4`"]
    #[inline(always)]
    pub fn is_auxio4(&self) -> bool {
        *self == EV0_SEL_A::AUXIO4
    }
    #[doc = "Checks if the value of the field is `AUXIO3`"]
    #[inline(always)]
    pub fn is_auxio3(&self) -> bool {
        *self == EV0_SEL_A::AUXIO3
    }
    #[doc = "Checks if the value of the field is `AUXIO2`"]
    #[inline(always)]
    pub fn is_auxio2(&self) -> bool {
        *self == EV0_SEL_A::AUXIO2
    }
    #[doc = "Checks if the value of the field is `AUXIO1`"]
    #[inline(always)]
    pub fn is_auxio1(&self) -> bool {
        *self == EV0_SEL_A::AUXIO1
    }
    #[doc = "Checks if the value of the field is `AUXIO0`"]
    #[inline(always)]
    pub fn is_auxio0(&self) -> bool {
        *self == EV0_SEL_A::AUXIO0
    }
}
#[doc = "Write proxy for field `EV0_SEL`"]
pub struct EV0_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EV0_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EV0_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "EVSTAT3.AUX_TIMER2_CLKSWITCH_RDY"]
    #[inline(always)]
    pub fn aux_timer2_clkswitch_rdy(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUX_TIMER2_CLKSWITCH_RDY)
    }
    #[doc = "EVSTAT3.AUX_DAC_HOLD_ACTIVE"]
    #[inline(always)]
    pub fn aux_dac_hold_active(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUX_DAC_HOLD_ACTIVE)
    }
    #[doc = "EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn aux_smph_autotake_done(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUX_SMPH_AUTOTAKE_DONE)
    }
    #[doc = "EVSTAT3.AUX_ADC_FIFO_NOT_EMPTY"]
    #[inline(always)]
    pub fn aux_adc_fifo_not_empty(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUX_ADC_FIFO_NOT_EMPTY)
    }
    #[doc = "EVSTAT3.AUX_ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn aux_adc_fifo_almost_full(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUX_ADC_FIFO_ALMOST_FULL)
    }
    #[doc = "EVSTAT3.AUX_ADC_IRQ"]
    #[inline(always)]
    pub fn aux_adc_irq(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUX_ADC_IRQ)
    }
    #[doc = "EVSTAT3.AUX_ADC_DONE"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUX_ADC_DONE)
    }
    #[doc = "EVSTAT3.AUX_ISRC_RESET_N"]
    #[inline(always)]
    pub fn aux_isrc_reset_n(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUX_ISRC_RESET_N)
    }
    #[doc = "EVSTAT3.AUX_TDC_DONE"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUX_TDC_DONE)
    }
    #[doc = "EVSTAT3.AUX_TIMER0_EV"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUX_TIMER0_EV)
    }
    #[doc = "EVSTAT3.AUX_TIMER1_EV"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUX_TIMER1_EV)
    }
    #[doc = "EVSTAT3.AUX_TIMER2_PULSE"]
    #[inline(always)]
    pub fn aux_timer2_pulse(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUX_TIMER2_PULSE)
    }
    #[doc = "EVSTAT3.AUX_TIMER2_EV3"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUX_TIMER2_EV3)
    }
    #[doc = "EVSTAT3.AUX_TIMER2_EV2"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUX_TIMER2_EV2)
    }
    #[doc = "EVSTAT3.AUX_TIMER2_EV1"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUX_TIMER2_EV1)
    }
    #[doc = "EVSTAT3.AUX_TIMER2_EV0"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUX_TIMER2_EV0)
    }
    #[doc = "EVSTAT2.AUX_COMPB"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUX_COMPB)
    }
    #[doc = "EVSTAT2.AUX_COMPA"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUX_COMPA)
    }
    #[doc = "EVSTAT2.MCU_OBSMUX1"]
    #[inline(always)]
    pub fn mcu_obsmux1(self) -> &'a mut W {
        self.variant(EV0_SEL_A::MCU_OBSMUX1)
    }
    #[doc = "EVSTAT2.MCU_OBSMUX0"]
    #[inline(always)]
    pub fn mcu_obsmux0(self) -> &'a mut W {
        self.variant(EV0_SEL_A::MCU_OBSMUX0)
    }
    #[doc = "EVSTAT2.MCU_EV"]
    #[inline(always)]
    pub fn mcu_ev(self) -> &'a mut W {
        self.variant(EV0_SEL_A::MCU_EV)
    }
    #[doc = "EVSTAT2.ACLK_REF"]
    #[inline(always)]
    pub fn aclk_ref(self) -> &'a mut W {
        self.variant(EV0_SEL_A::ACLK_REF)
    }
    #[doc = "EVSTAT2.VDDR_RECHARGE"]
    #[inline(always)]
    pub fn vddr_recharge(self) -> &'a mut W {
        self.variant(EV0_SEL_A::VDDR_RECHARGE)
    }
    #[doc = "EVSTAT2.MCU_ACTIVE"]
    #[inline(always)]
    pub fn mcu_active(self) -> &'a mut W {
        self.variant(EV0_SEL_A::MCU_ACTIVE)
    }
    #[doc = "EVSTAT2.PWR_DWN"]
    #[inline(always)]
    pub fn pwr_dwn(self) -> &'a mut W {
        self.variant(EV0_SEL_A::PWR_DWN)
    }
    #[doc = "EVSTAT2.SCLK_LF"]
    #[inline(always)]
    pub fn sclk_lf(self) -> &'a mut W {
        self.variant(EV0_SEL_A::SCLK_LF)
    }
    #[doc = "EVSTAT2.AON_BATMON_TEMP_UPD"]
    #[inline(always)]
    pub fn aon_batmon_temp_upd(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AON_BATMON_TEMP_UPD)
    }
    #[doc = "EVSTAT2.AON_BATMON_BAT_UPD"]
    #[inline(always)]
    pub fn aon_batmon_bat_upd(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AON_BATMON_BAT_UPD)
    }
    #[doc = "EVSTAT2.AON_RTC_4KHZ"]
    #[inline(always)]
    pub fn aon_rtc_4khz(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AON_RTC_4KHZ)
    }
    #[doc = "EVSTAT2.AON_RTC_CH2_DLY"]
    #[inline(always)]
    pub fn aon_rtc_ch2_dly(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AON_RTC_CH2_DLY)
    }
    #[doc = "EVSTAT2.AON_RTC_CH2"]
    #[inline(always)]
    pub fn aon_rtc_ch2(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AON_RTC_CH2)
    }
    #[doc = "Programmable delay event as described in PROGDLY"]
    #[inline(always)]
    pub fn aux_prog_dly_idle(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUX_PROG_DLY_IDLE)
    }
    #[doc = "EVSTAT1.AUXIO31"]
    #[inline(always)]
    pub fn auxio31(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUXIO31)
    }
    #[doc = "EVSTAT1.AUXIO30"]
    #[inline(always)]
    pub fn auxio30(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUXIO30)
    }
    #[doc = "EVSTAT1.AUXIO29"]
    #[inline(always)]
    pub fn auxio29(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUXIO29)
    }
    #[doc = "EVSTAT1.AUXIO28"]
    #[inline(always)]
    pub fn auxio28(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUXIO28)
    }
    #[doc = "EVSTAT1.AUXIO27"]
    #[inline(always)]
    pub fn auxio27(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUXIO27)
    }
    #[doc = "EVSTAT1.AUXIO26"]
    #[inline(always)]
    pub fn auxio26(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUXIO26)
    }
    #[doc = "EVSTAT1.AUXIO25"]
    #[inline(always)]
    pub fn auxio25(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUXIO25)
    }
    #[doc = "EVSTAT1.AUXIO24"]
    #[inline(always)]
    pub fn auxio24(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUXIO24)
    }
    #[doc = "EVSTAT1.AUXIO23"]
    #[inline(always)]
    pub fn auxio23(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUXIO23)
    }
    #[doc = "EVSTAT1.AUXIO22"]
    #[inline(always)]
    pub fn auxio22(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUXIO22)
    }
    #[doc = "EVSTAT1.AUXIO21"]
    #[inline(always)]
    pub fn auxio21(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUXIO21)
    }
    #[doc = "EVSTAT1.AUXIO20"]
    #[inline(always)]
    pub fn auxio20(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUXIO20)
    }
    #[doc = "EVSTAT1.AUXIO19"]
    #[inline(always)]
    pub fn auxio19(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUXIO19)
    }
    #[doc = "EVSTAT1.AUXIO18"]
    #[inline(always)]
    pub fn auxio18(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUXIO18)
    }
    #[doc = "EVSTAT1.AUXIO17"]
    #[inline(always)]
    pub fn auxio17(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUXIO17)
    }
    #[doc = "EVSTAT1.AUXIO16"]
    #[inline(always)]
    pub fn auxio16(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUXIO16)
    }
    #[doc = "EVSTAT0.AUXIO15"]
    #[inline(always)]
    pub fn auxio15(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUXIO15)
    }
    #[doc = "EVSTAT0.AUXIO14"]
    #[inline(always)]
    pub fn auxio14(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUXIO14)
    }
    #[doc = "EVSTAT0.AUXIO13"]
    #[inline(always)]
    pub fn auxio13(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUXIO13)
    }
    #[doc = "EVSTAT0.AUXIO12"]
    #[inline(always)]
    pub fn auxio12(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUXIO12)
    }
    #[doc = "EVSTAT0.AUXIO11"]
    #[inline(always)]
    pub fn auxio11(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUXIO11)
    }
    #[doc = "EVSTAT0.AUXIO10"]
    #[inline(always)]
    pub fn auxio10(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUXIO10)
    }
    #[doc = "EVSTAT0.AUXIO9"]
    #[inline(always)]
    pub fn auxio9(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUXIO9)
    }
    #[doc = "EVSTAT0.AUXIO8"]
    #[inline(always)]
    pub fn auxio8(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUXIO8)
    }
    #[doc = "EVSTAT0.AUXIO7"]
    #[inline(always)]
    pub fn auxio7(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUXIO7)
    }
    #[doc = "EVSTAT0.AUXIO6"]
    #[inline(always)]
    pub fn auxio6(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUXIO6)
    }
    #[doc = "EVSTAT0.AUXIO5"]
    #[inline(always)]
    pub fn auxio5(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUXIO5)
    }
    #[doc = "EVSTAT0.AUXIO4"]
    #[inline(always)]
    pub fn auxio4(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUXIO4)
    }
    #[doc = "EVSTAT0.AUXIO3"]
    #[inline(always)]
    pub fn auxio3(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUXIO3)
    }
    #[doc = "EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn auxio2(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUXIO2)
    }
    #[doc = "EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn auxio1(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUXIO1)
    }
    #[doc = "EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn auxio0(self) -> &'a mut W {
        self.variant(EV0_SEL_A::AUXIO0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 7:31 - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 7) & 0x01ff_ffff) as u32)
    }
    #[doc = "Bit 6 - 6:6\\]
Event combination control: 0: Disable event combination. 1: Enable event combination."]
    #[inline(always)]
    pub fn comb_ev_en(&self) -> COMB_EV_EN_R {
        COMB_EV_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 0:5 - 5:0\\]
Select the event source from the synchronous event bus to be used in event equation."]
    #[inline(always)]
    pub fn ev0_sel(&self) -> EV0_SEL_R {
        EV0_SEL_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 7:31 - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&mut self) -> RESERVED7_W {
        RESERVED7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Event combination control: 0: Disable event combination. 1: Enable event combination."]
    #[inline(always)]
    pub fn comb_ev_en(&mut self) -> COMB_EV_EN_W {
        COMB_EV_EN_W { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\]
Select the event source from the synchronous event bus to be used in event equation."]
    #[inline(always)]
    pub fn ev0_sel(&mut self) -> EV0_SEL_W {
        EV0_SEL_W { w: self }
    }
}
