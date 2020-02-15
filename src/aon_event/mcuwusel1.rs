#[doc = "Reader of register MCUWUSEL1"]
pub type R = crate::R<u32, super::MCUWUSEL1>;
#[doc = "Writer for register MCUWUSEL1"]
pub type W = crate::W<u32, super::MCUWUSEL1>;
#[doc = "Register MCUWUSEL1 `reset()`'s with value 0x3f3f_3f3f"]
impl crate::ResetValue for super::MCUWUSEL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3f3f_3f3f
    }
}
#[doc = "Reader of field `RESERVED30`"]
pub type RESERVED30_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED30`"]
pub struct RESERVED30_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED30_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "29:24\\]
MCU Wakeup Source #7 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WU7_EV_A {
    #[doc = "63: No event, always low"]
    NONE = 63,
    #[doc = "56: Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC_N = 56,
    #[doc = "55: Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC = 55,
    #[doc = "54: BATMON voltage update event"]
    BATMON_VOLT = 54,
    #[doc = "53: BATMON temperature update event"]
    BATMON_TEMP = 53,
    #[doc = "52: AUX Timer 1 Event"]
    AUX_TIMER1_EV = 52,
    #[doc = "51: AUX Timer 0 Event"]
    AUX_TIMER0_EV = 51,
    #[doc = "50: TDC completed or timed out"]
    AUX_TDC_DONE = 50,
    #[doc = "49: ADC conversion completed"]
    AUX_ADC_DONE = 49,
    #[doc = "48: Comparator B triggered"]
    AUX_COMPB = 48,
    #[doc = "47: Comparator A triggered"]
    AUX_COMPA = 47,
    #[doc = "46: AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    AUX_SWEV2 = 46,
    #[doc = "45: AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    AUX_SWEV1 = 45,
    #[doc = "44: AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    AUX_SWEV0 = 44,
    #[doc = "43: JTAG generated event"]
    JTAG = 43,
    #[doc = "42: RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    RTC_UPD = 42,
    #[doc = "41: RTC combined delayed event"]
    RTC_COMB_DLY = 41,
    #[doc = "40: RTC channel 2 - delayed event"]
    RTC_CH2_DLY = 40,
    #[doc = "39: RTC channel 1 - delayed event"]
    RTC_CH1_DLY = 39,
    #[doc = "38: RTC channel 0 - delayed event"]
    RTC_CH0_DLY = 38,
    #[doc = "37: RTC channel 2 event"]
    RTC_CH2 = 37,
    #[doc = "36: RTC channel 1 event"]
    RTC_CH1 = 36,
    #[doc = "35: RTC channel 0 event"]
    RTC_CH0 = 35,
    #[doc = "32: Edge detect on any PAD"]
    PAD = 32,
    #[doc = "9: Combined event from BATMON"]
    BATMON_COMBINED = 9,
    #[doc = "8: BATMON event: Temperature level below lower limit"]
    BATMON_TEMP_LL = 8,
    #[doc = "7: BATMON event: Temperature level above upper limit"]
    BATMON_TEMP_UL = 7,
    #[doc = "6: BATMON event: Battery level below lower limit"]
    BATMON_BATT_LL = 6,
    #[doc = "5: BATMON event: Battery level above upper limit"]
    BATMON_BATT_UL = 5,
    #[doc = "4: Event 3 from AUX TImer2"]
    AUX_TIMER2_EV3 = 4,
    #[doc = "3: Event 2 from AUX TImer2"]
    AUX_TIMER2_EV2 = 3,
    #[doc = "2: Event 1 from AUX TImer2"]
    AUX_TIMER2_EV1 = 2,
    #[doc = "1: Event 0 from AUX TImer2"]
    AUX_TIMER2_EV0 = 1,
    #[doc = "0: Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    IOEV_MCU_WU = 0,
}
impl From<WU7_EV_A> for u8 {
    #[inline(always)]
    fn from(variant: WU7_EV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WU7_EV`"]
pub type WU7_EV_R = crate::R<u8, WU7_EV_A>;
impl WU7_EV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WU7_EV_A> {
        use crate::Variant::*;
        match self.bits {
            63 => Val(WU7_EV_A::NONE),
            56 => Val(WU7_EV_A::AUX_COMPB_ASYNC_N),
            55 => Val(WU7_EV_A::AUX_COMPB_ASYNC),
            54 => Val(WU7_EV_A::BATMON_VOLT),
            53 => Val(WU7_EV_A::BATMON_TEMP),
            52 => Val(WU7_EV_A::AUX_TIMER1_EV),
            51 => Val(WU7_EV_A::AUX_TIMER0_EV),
            50 => Val(WU7_EV_A::AUX_TDC_DONE),
            49 => Val(WU7_EV_A::AUX_ADC_DONE),
            48 => Val(WU7_EV_A::AUX_COMPB),
            47 => Val(WU7_EV_A::AUX_COMPA),
            46 => Val(WU7_EV_A::AUX_SWEV2),
            45 => Val(WU7_EV_A::AUX_SWEV1),
            44 => Val(WU7_EV_A::AUX_SWEV0),
            43 => Val(WU7_EV_A::JTAG),
            42 => Val(WU7_EV_A::RTC_UPD),
            41 => Val(WU7_EV_A::RTC_COMB_DLY),
            40 => Val(WU7_EV_A::RTC_CH2_DLY),
            39 => Val(WU7_EV_A::RTC_CH1_DLY),
            38 => Val(WU7_EV_A::RTC_CH0_DLY),
            37 => Val(WU7_EV_A::RTC_CH2),
            36 => Val(WU7_EV_A::RTC_CH1),
            35 => Val(WU7_EV_A::RTC_CH0),
            32 => Val(WU7_EV_A::PAD),
            9 => Val(WU7_EV_A::BATMON_COMBINED),
            8 => Val(WU7_EV_A::BATMON_TEMP_LL),
            7 => Val(WU7_EV_A::BATMON_TEMP_UL),
            6 => Val(WU7_EV_A::BATMON_BATT_LL),
            5 => Val(WU7_EV_A::BATMON_BATT_UL),
            4 => Val(WU7_EV_A::AUX_TIMER2_EV3),
            3 => Val(WU7_EV_A::AUX_TIMER2_EV2),
            2 => Val(WU7_EV_A::AUX_TIMER2_EV1),
            1 => Val(WU7_EV_A::AUX_TIMER2_EV0),
            0 => Val(WU7_EV_A::IOEV_MCU_WU),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == WU7_EV_A::NONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC_N`"]
    #[inline(always)]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == WU7_EV_A::AUX_COMPB_ASYNC_N
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC`"]
    #[inline(always)]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == WU7_EV_A::AUX_COMPB_ASYNC
    }
    #[doc = "Checks if the value of the field is `BATMON_VOLT`"]
    #[inline(always)]
    pub fn is_batmon_volt(&self) -> bool {
        *self == WU7_EV_A::BATMON_VOLT
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP`"]
    #[inline(always)]
    pub fn is_batmon_temp(&self) -> bool {
        *self == WU7_EV_A::BATMON_TEMP
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == WU7_EV_A::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == WU7_EV_A::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == WU7_EV_A::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == WU7_EV_A::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == WU7_EV_A::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == WU7_EV_A::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV2`"]
    #[inline(always)]
    pub fn is_aux_swev2(&self) -> bool {
        *self == WU7_EV_A::AUX_SWEV2
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV1`"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == WU7_EV_A::AUX_SWEV1
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV0`"]
    #[inline(always)]
    pub fn is_aux_swev0(&self) -> bool {
        *self == WU7_EV_A::AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `JTAG`"]
    #[inline(always)]
    pub fn is_jtag(&self) -> bool {
        *self == WU7_EV_A::JTAG
    }
    #[doc = "Checks if the value of the field is `RTC_UPD`"]
    #[inline(always)]
    pub fn is_rtc_upd(&self) -> bool {
        *self == WU7_EV_A::RTC_UPD
    }
    #[doc = "Checks if the value of the field is `RTC_COMB_DLY`"]
    #[inline(always)]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == WU7_EV_A::RTC_COMB_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == WU7_EV_A::RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH1_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == WU7_EV_A::RTC_CH1_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH0_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == WU7_EV_A::RTC_CH0_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2`"]
    #[inline(always)]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == WU7_EV_A::RTC_CH2
    }
    #[doc = "Checks if the value of the field is `RTC_CH1`"]
    #[inline(always)]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == WU7_EV_A::RTC_CH1
    }
    #[doc = "Checks if the value of the field is `RTC_CH0`"]
    #[inline(always)]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == WU7_EV_A::RTC_CH0
    }
    #[doc = "Checks if the value of the field is `PAD`"]
    #[inline(always)]
    pub fn is_pad(&self) -> bool {
        *self == WU7_EV_A::PAD
    }
    #[doc = "Checks if the value of the field is `BATMON_COMBINED`"]
    #[inline(always)]
    pub fn is_batmon_combined(&self) -> bool {
        *self == WU7_EV_A::BATMON_COMBINED
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_LL`"]
    #[inline(always)]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == WU7_EV_A::BATMON_TEMP_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_UL`"]
    #[inline(always)]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == WU7_EV_A::BATMON_TEMP_UL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_LL`"]
    #[inline(always)]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == WU7_EV_A::BATMON_BATT_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_UL`"]
    #[inline(always)]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == WU7_EV_A::BATMON_BATT_UL
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == WU7_EV_A::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == WU7_EV_A::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == WU7_EV_A::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == WU7_EV_A::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `IOEV_MCU_WU`"]
    #[inline(always)]
    pub fn is_ioev_mcu_wu(&self) -> bool {
        *self == WU7_EV_A::IOEV_MCU_WU
    }
}
#[doc = "Write proxy for field `WU7_EV`"]
pub struct WU7_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> WU7_EV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WU7_EV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(WU7_EV_A::NONE)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async_n(self) -> &'a mut W {
        self.variant(WU7_EV_A::AUX_COMPB_ASYNC_N)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async(self) -> &'a mut W {
        self.variant(WU7_EV_A::AUX_COMPB_ASYNC)
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn batmon_volt(self) -> &'a mut W {
        self.variant(WU7_EV_A::BATMON_VOLT)
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn batmon_temp(self) -> &'a mut W {
        self.variant(WU7_EV_A::BATMON_TEMP)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(WU7_EV_A::AUX_TIMER1_EV)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(WU7_EV_A::AUX_TIMER0_EV)
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(WU7_EV_A::AUX_TDC_DONE)
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(WU7_EV_A::AUX_ADC_DONE)
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(WU7_EV_A::AUX_COMPB)
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(WU7_EV_A::AUX_COMPA)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn aux_swev2(self) -> &'a mut W {
        self.variant(WU7_EV_A::AUX_SWEV2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn aux_swev1(self) -> &'a mut W {
        self.variant(WU7_EV_A::AUX_SWEV1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn aux_swev0(self) -> &'a mut W {
        self.variant(WU7_EV_A::AUX_SWEV0)
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn jtag(self) -> &'a mut W {
        self.variant(WU7_EV_A::JTAG)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn rtc_upd(self) -> &'a mut W {
        self.variant(WU7_EV_A::RTC_UPD)
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn rtc_comb_dly(self) -> &'a mut W {
        self.variant(WU7_EV_A::RTC_COMB_DLY)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch2_dly(self) -> &'a mut W {
        self.variant(WU7_EV_A::RTC_CH2_DLY)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch1_dly(self) -> &'a mut W {
        self.variant(WU7_EV_A::RTC_CH1_DLY)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch0_dly(self) -> &'a mut W {
        self.variant(WU7_EV_A::RTC_CH0_DLY)
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn rtc_ch2(self) -> &'a mut W {
        self.variant(WU7_EV_A::RTC_CH2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn rtc_ch1(self) -> &'a mut W {
        self.variant(WU7_EV_A::RTC_CH1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn rtc_ch0(self) -> &'a mut W {
        self.variant(WU7_EV_A::RTC_CH0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn pad(self) -> &'a mut W {
        self.variant(WU7_EV_A::PAD)
    }
    #[doc = "Combined event from BATMON"]
    #[inline(always)]
    pub fn batmon_combined(self) -> &'a mut W {
        self.variant(WU7_EV_A::BATMON_COMBINED)
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline(always)]
    pub fn batmon_temp_ll(self) -> &'a mut W {
        self.variant(WU7_EV_A::BATMON_TEMP_LL)
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline(always)]
    pub fn batmon_temp_ul(self) -> &'a mut W {
        self.variant(WU7_EV_A::BATMON_TEMP_UL)
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline(always)]
    pub fn batmon_batt_ll(self) -> &'a mut W {
        self.variant(WU7_EV_A::BATMON_BATT_LL)
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline(always)]
    pub fn batmon_batt_ul(self) -> &'a mut W {
        self.variant(WU7_EV_A::BATMON_BATT_UL)
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(WU7_EV_A::AUX_TIMER2_EV3)
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(WU7_EV_A::AUX_TIMER2_EV2)
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(WU7_EV_A::AUX_TIMER2_EV1)
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(WU7_EV_A::AUX_TIMER2_EV0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    #[inline(always)]
    pub fn ioev_mcu_wu(self) -> &'a mut W {
        self.variant(WU7_EV_A::IOEV_MCU_WU)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "Reader of field `RESERVED22`"]
pub type RESERVED22_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED22`"]
pub struct RESERVED22_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED22_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "21:16\\]
MCU Wakeup Source #6 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WU6_EV_A {
    #[doc = "63: No event, always low"]
    NONE = 63,
    #[doc = "56: Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC_N = 56,
    #[doc = "55: Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC = 55,
    #[doc = "54: BATMON voltage update event"]
    BATMON_VOLT = 54,
    #[doc = "53: BATMON temperature update event"]
    BATMON_TEMP = 53,
    #[doc = "52: AUX Timer 1 Event"]
    AUX_TIMER1_EV = 52,
    #[doc = "51: AUX Timer 0 Event"]
    AUX_TIMER0_EV = 51,
    #[doc = "50: TDC completed or timed out"]
    AUX_TDC_DONE = 50,
    #[doc = "49: ADC conversion completed"]
    AUX_ADC_DONE = 49,
    #[doc = "48: Comparator B triggered"]
    AUX_COMPB = 48,
    #[doc = "47: Comparator A triggered"]
    AUX_COMPA = 47,
    #[doc = "46: AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    AUX_SWEV2 = 46,
    #[doc = "45: AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    AUX_SWEV1 = 45,
    #[doc = "44: AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    AUX_SWEV0 = 44,
    #[doc = "43: JTAG generated event"]
    JTAG = 43,
    #[doc = "42: RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    RTC_UPD = 42,
    #[doc = "41: RTC combined delayed event"]
    RTC_COMB_DLY = 41,
    #[doc = "40: RTC channel 2 - delayed event"]
    RTC_CH2_DLY = 40,
    #[doc = "39: RTC channel 1 - delayed event"]
    RTC_CH1_DLY = 39,
    #[doc = "38: RTC channel 0 - delayed event"]
    RTC_CH0_DLY = 38,
    #[doc = "37: RTC channel 2 event"]
    RTC_CH2 = 37,
    #[doc = "36: RTC channel 1 event"]
    RTC_CH1 = 36,
    #[doc = "35: RTC channel 0 event"]
    RTC_CH0 = 35,
    #[doc = "32: Edge detect on any PAD"]
    PAD = 32,
    #[doc = "9: Combined event from BATMON"]
    BATMON_COMBINED = 9,
    #[doc = "8: BATMON event: Temperature level below lower limit"]
    BATMON_TEMP_LL = 8,
    #[doc = "7: BATMON event: Temperature level above upper limit"]
    BATMON_TEMP_UL = 7,
    #[doc = "6: BATMON event: Battery level below lower limit"]
    BATMON_BATT_LL = 6,
    #[doc = "5: BATMON event: Battery level above upper limit"]
    BATMON_BATT_UL = 5,
    #[doc = "4: Event 3 from AUX TImer2"]
    AUX_TIMER2_EV3 = 4,
    #[doc = "3: Event 2 from AUX TImer2"]
    AUX_TIMER2_EV2 = 3,
    #[doc = "2: Event 1 from AUX TImer2"]
    AUX_TIMER2_EV1 = 2,
    #[doc = "1: Event 0 from AUX TImer2"]
    AUX_TIMER2_EV0 = 1,
    #[doc = "0: Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    IOEV_MCU_WU = 0,
}
impl From<WU6_EV_A> for u8 {
    #[inline(always)]
    fn from(variant: WU6_EV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WU6_EV`"]
pub type WU6_EV_R = crate::R<u8, WU6_EV_A>;
impl WU6_EV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WU6_EV_A> {
        use crate::Variant::*;
        match self.bits {
            63 => Val(WU6_EV_A::NONE),
            56 => Val(WU6_EV_A::AUX_COMPB_ASYNC_N),
            55 => Val(WU6_EV_A::AUX_COMPB_ASYNC),
            54 => Val(WU6_EV_A::BATMON_VOLT),
            53 => Val(WU6_EV_A::BATMON_TEMP),
            52 => Val(WU6_EV_A::AUX_TIMER1_EV),
            51 => Val(WU6_EV_A::AUX_TIMER0_EV),
            50 => Val(WU6_EV_A::AUX_TDC_DONE),
            49 => Val(WU6_EV_A::AUX_ADC_DONE),
            48 => Val(WU6_EV_A::AUX_COMPB),
            47 => Val(WU6_EV_A::AUX_COMPA),
            46 => Val(WU6_EV_A::AUX_SWEV2),
            45 => Val(WU6_EV_A::AUX_SWEV1),
            44 => Val(WU6_EV_A::AUX_SWEV0),
            43 => Val(WU6_EV_A::JTAG),
            42 => Val(WU6_EV_A::RTC_UPD),
            41 => Val(WU6_EV_A::RTC_COMB_DLY),
            40 => Val(WU6_EV_A::RTC_CH2_DLY),
            39 => Val(WU6_EV_A::RTC_CH1_DLY),
            38 => Val(WU6_EV_A::RTC_CH0_DLY),
            37 => Val(WU6_EV_A::RTC_CH2),
            36 => Val(WU6_EV_A::RTC_CH1),
            35 => Val(WU6_EV_A::RTC_CH0),
            32 => Val(WU6_EV_A::PAD),
            9 => Val(WU6_EV_A::BATMON_COMBINED),
            8 => Val(WU6_EV_A::BATMON_TEMP_LL),
            7 => Val(WU6_EV_A::BATMON_TEMP_UL),
            6 => Val(WU6_EV_A::BATMON_BATT_LL),
            5 => Val(WU6_EV_A::BATMON_BATT_UL),
            4 => Val(WU6_EV_A::AUX_TIMER2_EV3),
            3 => Val(WU6_EV_A::AUX_TIMER2_EV2),
            2 => Val(WU6_EV_A::AUX_TIMER2_EV1),
            1 => Val(WU6_EV_A::AUX_TIMER2_EV0),
            0 => Val(WU6_EV_A::IOEV_MCU_WU),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == WU6_EV_A::NONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC_N`"]
    #[inline(always)]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == WU6_EV_A::AUX_COMPB_ASYNC_N
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC`"]
    #[inline(always)]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == WU6_EV_A::AUX_COMPB_ASYNC
    }
    #[doc = "Checks if the value of the field is `BATMON_VOLT`"]
    #[inline(always)]
    pub fn is_batmon_volt(&self) -> bool {
        *self == WU6_EV_A::BATMON_VOLT
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP`"]
    #[inline(always)]
    pub fn is_batmon_temp(&self) -> bool {
        *self == WU6_EV_A::BATMON_TEMP
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == WU6_EV_A::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == WU6_EV_A::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == WU6_EV_A::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == WU6_EV_A::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == WU6_EV_A::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == WU6_EV_A::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV2`"]
    #[inline(always)]
    pub fn is_aux_swev2(&self) -> bool {
        *self == WU6_EV_A::AUX_SWEV2
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV1`"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == WU6_EV_A::AUX_SWEV1
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV0`"]
    #[inline(always)]
    pub fn is_aux_swev0(&self) -> bool {
        *self == WU6_EV_A::AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `JTAG`"]
    #[inline(always)]
    pub fn is_jtag(&self) -> bool {
        *self == WU6_EV_A::JTAG
    }
    #[doc = "Checks if the value of the field is `RTC_UPD`"]
    #[inline(always)]
    pub fn is_rtc_upd(&self) -> bool {
        *self == WU6_EV_A::RTC_UPD
    }
    #[doc = "Checks if the value of the field is `RTC_COMB_DLY`"]
    #[inline(always)]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == WU6_EV_A::RTC_COMB_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == WU6_EV_A::RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH1_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == WU6_EV_A::RTC_CH1_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH0_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == WU6_EV_A::RTC_CH0_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2`"]
    #[inline(always)]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == WU6_EV_A::RTC_CH2
    }
    #[doc = "Checks if the value of the field is `RTC_CH1`"]
    #[inline(always)]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == WU6_EV_A::RTC_CH1
    }
    #[doc = "Checks if the value of the field is `RTC_CH0`"]
    #[inline(always)]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == WU6_EV_A::RTC_CH0
    }
    #[doc = "Checks if the value of the field is `PAD`"]
    #[inline(always)]
    pub fn is_pad(&self) -> bool {
        *self == WU6_EV_A::PAD
    }
    #[doc = "Checks if the value of the field is `BATMON_COMBINED`"]
    #[inline(always)]
    pub fn is_batmon_combined(&self) -> bool {
        *self == WU6_EV_A::BATMON_COMBINED
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_LL`"]
    #[inline(always)]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == WU6_EV_A::BATMON_TEMP_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_UL`"]
    #[inline(always)]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == WU6_EV_A::BATMON_TEMP_UL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_LL`"]
    #[inline(always)]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == WU6_EV_A::BATMON_BATT_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_UL`"]
    #[inline(always)]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == WU6_EV_A::BATMON_BATT_UL
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == WU6_EV_A::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == WU6_EV_A::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == WU6_EV_A::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == WU6_EV_A::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `IOEV_MCU_WU`"]
    #[inline(always)]
    pub fn is_ioev_mcu_wu(&self) -> bool {
        *self == WU6_EV_A::IOEV_MCU_WU
    }
}
#[doc = "Write proxy for field `WU6_EV`"]
pub struct WU6_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> WU6_EV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WU6_EV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(WU6_EV_A::NONE)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async_n(self) -> &'a mut W {
        self.variant(WU6_EV_A::AUX_COMPB_ASYNC_N)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async(self) -> &'a mut W {
        self.variant(WU6_EV_A::AUX_COMPB_ASYNC)
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn batmon_volt(self) -> &'a mut W {
        self.variant(WU6_EV_A::BATMON_VOLT)
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn batmon_temp(self) -> &'a mut W {
        self.variant(WU6_EV_A::BATMON_TEMP)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(WU6_EV_A::AUX_TIMER1_EV)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(WU6_EV_A::AUX_TIMER0_EV)
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(WU6_EV_A::AUX_TDC_DONE)
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(WU6_EV_A::AUX_ADC_DONE)
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(WU6_EV_A::AUX_COMPB)
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(WU6_EV_A::AUX_COMPA)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn aux_swev2(self) -> &'a mut W {
        self.variant(WU6_EV_A::AUX_SWEV2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn aux_swev1(self) -> &'a mut W {
        self.variant(WU6_EV_A::AUX_SWEV1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn aux_swev0(self) -> &'a mut W {
        self.variant(WU6_EV_A::AUX_SWEV0)
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn jtag(self) -> &'a mut W {
        self.variant(WU6_EV_A::JTAG)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn rtc_upd(self) -> &'a mut W {
        self.variant(WU6_EV_A::RTC_UPD)
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn rtc_comb_dly(self) -> &'a mut W {
        self.variant(WU6_EV_A::RTC_COMB_DLY)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch2_dly(self) -> &'a mut W {
        self.variant(WU6_EV_A::RTC_CH2_DLY)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch1_dly(self) -> &'a mut W {
        self.variant(WU6_EV_A::RTC_CH1_DLY)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch0_dly(self) -> &'a mut W {
        self.variant(WU6_EV_A::RTC_CH0_DLY)
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn rtc_ch2(self) -> &'a mut W {
        self.variant(WU6_EV_A::RTC_CH2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn rtc_ch1(self) -> &'a mut W {
        self.variant(WU6_EV_A::RTC_CH1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn rtc_ch0(self) -> &'a mut W {
        self.variant(WU6_EV_A::RTC_CH0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn pad(self) -> &'a mut W {
        self.variant(WU6_EV_A::PAD)
    }
    #[doc = "Combined event from BATMON"]
    #[inline(always)]
    pub fn batmon_combined(self) -> &'a mut W {
        self.variant(WU6_EV_A::BATMON_COMBINED)
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline(always)]
    pub fn batmon_temp_ll(self) -> &'a mut W {
        self.variant(WU6_EV_A::BATMON_TEMP_LL)
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline(always)]
    pub fn batmon_temp_ul(self) -> &'a mut W {
        self.variant(WU6_EV_A::BATMON_TEMP_UL)
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline(always)]
    pub fn batmon_batt_ll(self) -> &'a mut W {
        self.variant(WU6_EV_A::BATMON_BATT_LL)
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline(always)]
    pub fn batmon_batt_ul(self) -> &'a mut W {
        self.variant(WU6_EV_A::BATMON_BATT_UL)
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(WU6_EV_A::AUX_TIMER2_EV3)
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(WU6_EV_A::AUX_TIMER2_EV2)
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(WU6_EV_A::AUX_TIMER2_EV1)
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(WU6_EV_A::AUX_TIMER2_EV0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    #[inline(always)]
    pub fn ioev_mcu_wu(self) -> &'a mut W {
        self.variant(WU6_EV_A::IOEV_MCU_WU)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `RESERVED14`"]
pub type RESERVED14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED14`"]
pub struct RESERVED14_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "13:8\\]
MCU Wakeup Source #5 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WU5_EV_A {
    #[doc = "63: No event, always low"]
    NONE = 63,
    #[doc = "56: Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC_N = 56,
    #[doc = "55: Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC = 55,
    #[doc = "54: BATMON voltage update event"]
    BATMON_VOLT = 54,
    #[doc = "53: BATMON temperature update event"]
    BATMON_TEMP = 53,
    #[doc = "52: AUX Timer 1 Event"]
    AUX_TIMER1_EV = 52,
    #[doc = "51: AUX Timer 0 Event"]
    AUX_TIMER0_EV = 51,
    #[doc = "50: TDC completed or timed out"]
    AUX_TDC_DONE = 50,
    #[doc = "49: ADC conversion completed"]
    AUX_ADC_DONE = 49,
    #[doc = "48: Comparator B triggered"]
    AUX_COMPB = 48,
    #[doc = "47: Comparator A triggered"]
    AUX_COMPA = 47,
    #[doc = "46: AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    AUX_SWEV2 = 46,
    #[doc = "45: AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    AUX_SWEV1 = 45,
    #[doc = "44: AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    AUX_SWEV0 = 44,
    #[doc = "43: JTAG generated event"]
    JTAG = 43,
    #[doc = "42: RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    RTC_UPD = 42,
    #[doc = "41: RTC combined delayed event"]
    RTC_COMB_DLY = 41,
    #[doc = "40: RTC channel 2 - delayed event"]
    RTC_CH2_DLY = 40,
    #[doc = "39: RTC channel 1 - delayed event"]
    RTC_CH1_DLY = 39,
    #[doc = "38: RTC channel 0 - delayed event"]
    RTC_CH0_DLY = 38,
    #[doc = "37: RTC channel 2 event"]
    RTC_CH2 = 37,
    #[doc = "36: RTC channel 1 event"]
    RTC_CH1 = 36,
    #[doc = "35: RTC channel 0 event"]
    RTC_CH0 = 35,
    #[doc = "32: Edge detect on any PAD"]
    PAD = 32,
    #[doc = "9: Combined event from BATMON"]
    BATMON_COMBINED = 9,
    #[doc = "8: BATMON event: Temperature level below lower limit"]
    BATMON_TEMP_LL = 8,
    #[doc = "7: BATMON event: Temperature level above upper limit"]
    BATMON_TEMP_UL = 7,
    #[doc = "6: BATMON event: Battery level below lower limit"]
    BATMON_BATT_LL = 6,
    #[doc = "5: BATMON event: Battery level above upper limit"]
    BATMON_BATT_UL = 5,
    #[doc = "4: Event 3 from AUX TImer2"]
    AUX_TIMER2_EV3 = 4,
    #[doc = "3: Event 2 from AUX TImer2"]
    AUX_TIMER2_EV2 = 3,
    #[doc = "2: Event 1 from AUX TImer2"]
    AUX_TIMER2_EV1 = 2,
    #[doc = "1: Event 0 from AUX TImer2"]
    AUX_TIMER2_EV0 = 1,
    #[doc = "0: Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    IOEV_MCU_WU = 0,
}
impl From<WU5_EV_A> for u8 {
    #[inline(always)]
    fn from(variant: WU5_EV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WU5_EV`"]
pub type WU5_EV_R = crate::R<u8, WU5_EV_A>;
impl WU5_EV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WU5_EV_A> {
        use crate::Variant::*;
        match self.bits {
            63 => Val(WU5_EV_A::NONE),
            56 => Val(WU5_EV_A::AUX_COMPB_ASYNC_N),
            55 => Val(WU5_EV_A::AUX_COMPB_ASYNC),
            54 => Val(WU5_EV_A::BATMON_VOLT),
            53 => Val(WU5_EV_A::BATMON_TEMP),
            52 => Val(WU5_EV_A::AUX_TIMER1_EV),
            51 => Val(WU5_EV_A::AUX_TIMER0_EV),
            50 => Val(WU5_EV_A::AUX_TDC_DONE),
            49 => Val(WU5_EV_A::AUX_ADC_DONE),
            48 => Val(WU5_EV_A::AUX_COMPB),
            47 => Val(WU5_EV_A::AUX_COMPA),
            46 => Val(WU5_EV_A::AUX_SWEV2),
            45 => Val(WU5_EV_A::AUX_SWEV1),
            44 => Val(WU5_EV_A::AUX_SWEV0),
            43 => Val(WU5_EV_A::JTAG),
            42 => Val(WU5_EV_A::RTC_UPD),
            41 => Val(WU5_EV_A::RTC_COMB_DLY),
            40 => Val(WU5_EV_A::RTC_CH2_DLY),
            39 => Val(WU5_EV_A::RTC_CH1_DLY),
            38 => Val(WU5_EV_A::RTC_CH0_DLY),
            37 => Val(WU5_EV_A::RTC_CH2),
            36 => Val(WU5_EV_A::RTC_CH1),
            35 => Val(WU5_EV_A::RTC_CH0),
            32 => Val(WU5_EV_A::PAD),
            9 => Val(WU5_EV_A::BATMON_COMBINED),
            8 => Val(WU5_EV_A::BATMON_TEMP_LL),
            7 => Val(WU5_EV_A::BATMON_TEMP_UL),
            6 => Val(WU5_EV_A::BATMON_BATT_LL),
            5 => Val(WU5_EV_A::BATMON_BATT_UL),
            4 => Val(WU5_EV_A::AUX_TIMER2_EV3),
            3 => Val(WU5_EV_A::AUX_TIMER2_EV2),
            2 => Val(WU5_EV_A::AUX_TIMER2_EV1),
            1 => Val(WU5_EV_A::AUX_TIMER2_EV0),
            0 => Val(WU5_EV_A::IOEV_MCU_WU),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == WU5_EV_A::NONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC_N`"]
    #[inline(always)]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == WU5_EV_A::AUX_COMPB_ASYNC_N
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC`"]
    #[inline(always)]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == WU5_EV_A::AUX_COMPB_ASYNC
    }
    #[doc = "Checks if the value of the field is `BATMON_VOLT`"]
    #[inline(always)]
    pub fn is_batmon_volt(&self) -> bool {
        *self == WU5_EV_A::BATMON_VOLT
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP`"]
    #[inline(always)]
    pub fn is_batmon_temp(&self) -> bool {
        *self == WU5_EV_A::BATMON_TEMP
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == WU5_EV_A::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == WU5_EV_A::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == WU5_EV_A::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == WU5_EV_A::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == WU5_EV_A::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == WU5_EV_A::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV2`"]
    #[inline(always)]
    pub fn is_aux_swev2(&self) -> bool {
        *self == WU5_EV_A::AUX_SWEV2
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV1`"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == WU5_EV_A::AUX_SWEV1
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV0`"]
    #[inline(always)]
    pub fn is_aux_swev0(&self) -> bool {
        *self == WU5_EV_A::AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `JTAG`"]
    #[inline(always)]
    pub fn is_jtag(&self) -> bool {
        *self == WU5_EV_A::JTAG
    }
    #[doc = "Checks if the value of the field is `RTC_UPD`"]
    #[inline(always)]
    pub fn is_rtc_upd(&self) -> bool {
        *self == WU5_EV_A::RTC_UPD
    }
    #[doc = "Checks if the value of the field is `RTC_COMB_DLY`"]
    #[inline(always)]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == WU5_EV_A::RTC_COMB_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == WU5_EV_A::RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH1_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == WU5_EV_A::RTC_CH1_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH0_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == WU5_EV_A::RTC_CH0_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2`"]
    #[inline(always)]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == WU5_EV_A::RTC_CH2
    }
    #[doc = "Checks if the value of the field is `RTC_CH1`"]
    #[inline(always)]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == WU5_EV_A::RTC_CH1
    }
    #[doc = "Checks if the value of the field is `RTC_CH0`"]
    #[inline(always)]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == WU5_EV_A::RTC_CH0
    }
    #[doc = "Checks if the value of the field is `PAD`"]
    #[inline(always)]
    pub fn is_pad(&self) -> bool {
        *self == WU5_EV_A::PAD
    }
    #[doc = "Checks if the value of the field is `BATMON_COMBINED`"]
    #[inline(always)]
    pub fn is_batmon_combined(&self) -> bool {
        *self == WU5_EV_A::BATMON_COMBINED
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_LL`"]
    #[inline(always)]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == WU5_EV_A::BATMON_TEMP_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_UL`"]
    #[inline(always)]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == WU5_EV_A::BATMON_TEMP_UL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_LL`"]
    #[inline(always)]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == WU5_EV_A::BATMON_BATT_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_UL`"]
    #[inline(always)]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == WU5_EV_A::BATMON_BATT_UL
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == WU5_EV_A::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == WU5_EV_A::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == WU5_EV_A::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == WU5_EV_A::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `IOEV_MCU_WU`"]
    #[inline(always)]
    pub fn is_ioev_mcu_wu(&self) -> bool {
        *self == WU5_EV_A::IOEV_MCU_WU
    }
}
#[doc = "Write proxy for field `WU5_EV`"]
pub struct WU5_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> WU5_EV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WU5_EV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(WU5_EV_A::NONE)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async_n(self) -> &'a mut W {
        self.variant(WU5_EV_A::AUX_COMPB_ASYNC_N)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async(self) -> &'a mut W {
        self.variant(WU5_EV_A::AUX_COMPB_ASYNC)
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn batmon_volt(self) -> &'a mut W {
        self.variant(WU5_EV_A::BATMON_VOLT)
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn batmon_temp(self) -> &'a mut W {
        self.variant(WU5_EV_A::BATMON_TEMP)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(WU5_EV_A::AUX_TIMER1_EV)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(WU5_EV_A::AUX_TIMER0_EV)
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(WU5_EV_A::AUX_TDC_DONE)
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(WU5_EV_A::AUX_ADC_DONE)
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(WU5_EV_A::AUX_COMPB)
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(WU5_EV_A::AUX_COMPA)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn aux_swev2(self) -> &'a mut W {
        self.variant(WU5_EV_A::AUX_SWEV2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn aux_swev1(self) -> &'a mut W {
        self.variant(WU5_EV_A::AUX_SWEV1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn aux_swev0(self) -> &'a mut W {
        self.variant(WU5_EV_A::AUX_SWEV0)
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn jtag(self) -> &'a mut W {
        self.variant(WU5_EV_A::JTAG)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn rtc_upd(self) -> &'a mut W {
        self.variant(WU5_EV_A::RTC_UPD)
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn rtc_comb_dly(self) -> &'a mut W {
        self.variant(WU5_EV_A::RTC_COMB_DLY)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch2_dly(self) -> &'a mut W {
        self.variant(WU5_EV_A::RTC_CH2_DLY)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch1_dly(self) -> &'a mut W {
        self.variant(WU5_EV_A::RTC_CH1_DLY)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch0_dly(self) -> &'a mut W {
        self.variant(WU5_EV_A::RTC_CH0_DLY)
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn rtc_ch2(self) -> &'a mut W {
        self.variant(WU5_EV_A::RTC_CH2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn rtc_ch1(self) -> &'a mut W {
        self.variant(WU5_EV_A::RTC_CH1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn rtc_ch0(self) -> &'a mut W {
        self.variant(WU5_EV_A::RTC_CH0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn pad(self) -> &'a mut W {
        self.variant(WU5_EV_A::PAD)
    }
    #[doc = "Combined event from BATMON"]
    #[inline(always)]
    pub fn batmon_combined(self) -> &'a mut W {
        self.variant(WU5_EV_A::BATMON_COMBINED)
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline(always)]
    pub fn batmon_temp_ll(self) -> &'a mut W {
        self.variant(WU5_EV_A::BATMON_TEMP_LL)
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline(always)]
    pub fn batmon_temp_ul(self) -> &'a mut W {
        self.variant(WU5_EV_A::BATMON_TEMP_UL)
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline(always)]
    pub fn batmon_batt_ll(self) -> &'a mut W {
        self.variant(WU5_EV_A::BATMON_BATT_LL)
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline(always)]
    pub fn batmon_batt_ul(self) -> &'a mut W {
        self.variant(WU5_EV_A::BATMON_BATT_UL)
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(WU5_EV_A::AUX_TIMER2_EV3)
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(WU5_EV_A::AUX_TIMER2_EV2)
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(WU5_EV_A::AUX_TIMER2_EV1)
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(WU5_EV_A::AUX_TIMER2_EV0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    #[inline(always)]
    pub fn ioev_mcu_wu(self) -> &'a mut W {
        self.variant(WU5_EV_A::IOEV_MCU_WU)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `RESERVED6`"]
pub type RESERVED6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED6`"]
pub struct RESERVED6_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "5:0\\]
MCU Wakeup Source #4 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WU4_EV_A {
    #[doc = "63: No event, always low"]
    NONE = 63,
    #[doc = "56: Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC_N = 56,
    #[doc = "55: Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC = 55,
    #[doc = "54: BATMON voltage update event"]
    BATMON_VOLT = 54,
    #[doc = "53: BATMON temperature update event"]
    BATMON_TEMP = 53,
    #[doc = "52: AUX Timer 1 Event"]
    AUX_TIMER1_EV = 52,
    #[doc = "51: AUX Timer 0 Event"]
    AUX_TIMER0_EV = 51,
    #[doc = "50: TDC completed or timed out"]
    AUX_TDC_DONE = 50,
    #[doc = "49: ADC conversion completed"]
    AUX_ADC_DONE = 49,
    #[doc = "48: Comparator B triggered"]
    AUX_COMPB = 48,
    #[doc = "47: Comparator A triggered"]
    AUX_COMPA = 47,
    #[doc = "46: AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    AUX_SWEV2 = 46,
    #[doc = "45: AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    AUX_SWEV1 = 45,
    #[doc = "44: AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    AUX_SWEV0 = 44,
    #[doc = "43: JTAG generated event"]
    JTAG = 43,
    #[doc = "42: RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    RTC_UPD = 42,
    #[doc = "41: RTC combined delayed event"]
    RTC_COMB_DLY = 41,
    #[doc = "40: RTC channel 2 - delayed event"]
    RTC_CH2_DLY = 40,
    #[doc = "39: RTC channel 1 - delayed event"]
    RTC_CH1_DLY = 39,
    #[doc = "38: RTC channel 0 - delayed event"]
    RTC_CH0_DLY = 38,
    #[doc = "37: RTC channel 2 event"]
    RTC_CH2 = 37,
    #[doc = "36: RTC channel 1 event"]
    RTC_CH1 = 36,
    #[doc = "35: RTC channel 0 event"]
    RTC_CH0 = 35,
    #[doc = "32: Edge detect on any PAD"]
    PAD = 32,
    #[doc = "9: Combined event from BATMON"]
    BATMON_COMBINED = 9,
    #[doc = "8: BATMON event: Temperature level below lower limit"]
    BATMON_TEMP_LL = 8,
    #[doc = "7: BATMON event: Temperature level above upper limit"]
    BATMON_TEMP_UL = 7,
    #[doc = "6: BATMON event: Battery level below lower limit"]
    BATMON_BATT_LL = 6,
    #[doc = "5: BATMON event: Battery level above upper limit"]
    BATMON_BATT_UL = 5,
    #[doc = "4: Event 3 from AUX TImer2"]
    AUX_TIMER2_EV3 = 4,
    #[doc = "3: Event 2 from AUX TImer2"]
    AUX_TIMER2_EV2 = 3,
    #[doc = "2: Event 1 from AUX TImer2"]
    AUX_TIMER2_EV1 = 2,
    #[doc = "1: Event 0 from AUX TImer2"]
    AUX_TIMER2_EV0 = 1,
    #[doc = "0: Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    IOEV_MCU_WU = 0,
}
impl From<WU4_EV_A> for u8 {
    #[inline(always)]
    fn from(variant: WU4_EV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WU4_EV`"]
pub type WU4_EV_R = crate::R<u8, WU4_EV_A>;
impl WU4_EV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WU4_EV_A> {
        use crate::Variant::*;
        match self.bits {
            63 => Val(WU4_EV_A::NONE),
            56 => Val(WU4_EV_A::AUX_COMPB_ASYNC_N),
            55 => Val(WU4_EV_A::AUX_COMPB_ASYNC),
            54 => Val(WU4_EV_A::BATMON_VOLT),
            53 => Val(WU4_EV_A::BATMON_TEMP),
            52 => Val(WU4_EV_A::AUX_TIMER1_EV),
            51 => Val(WU4_EV_A::AUX_TIMER0_EV),
            50 => Val(WU4_EV_A::AUX_TDC_DONE),
            49 => Val(WU4_EV_A::AUX_ADC_DONE),
            48 => Val(WU4_EV_A::AUX_COMPB),
            47 => Val(WU4_EV_A::AUX_COMPA),
            46 => Val(WU4_EV_A::AUX_SWEV2),
            45 => Val(WU4_EV_A::AUX_SWEV1),
            44 => Val(WU4_EV_A::AUX_SWEV0),
            43 => Val(WU4_EV_A::JTAG),
            42 => Val(WU4_EV_A::RTC_UPD),
            41 => Val(WU4_EV_A::RTC_COMB_DLY),
            40 => Val(WU4_EV_A::RTC_CH2_DLY),
            39 => Val(WU4_EV_A::RTC_CH1_DLY),
            38 => Val(WU4_EV_A::RTC_CH0_DLY),
            37 => Val(WU4_EV_A::RTC_CH2),
            36 => Val(WU4_EV_A::RTC_CH1),
            35 => Val(WU4_EV_A::RTC_CH0),
            32 => Val(WU4_EV_A::PAD),
            9 => Val(WU4_EV_A::BATMON_COMBINED),
            8 => Val(WU4_EV_A::BATMON_TEMP_LL),
            7 => Val(WU4_EV_A::BATMON_TEMP_UL),
            6 => Val(WU4_EV_A::BATMON_BATT_LL),
            5 => Val(WU4_EV_A::BATMON_BATT_UL),
            4 => Val(WU4_EV_A::AUX_TIMER2_EV3),
            3 => Val(WU4_EV_A::AUX_TIMER2_EV2),
            2 => Val(WU4_EV_A::AUX_TIMER2_EV1),
            1 => Val(WU4_EV_A::AUX_TIMER2_EV0),
            0 => Val(WU4_EV_A::IOEV_MCU_WU),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == WU4_EV_A::NONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC_N`"]
    #[inline(always)]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == WU4_EV_A::AUX_COMPB_ASYNC_N
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC`"]
    #[inline(always)]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == WU4_EV_A::AUX_COMPB_ASYNC
    }
    #[doc = "Checks if the value of the field is `BATMON_VOLT`"]
    #[inline(always)]
    pub fn is_batmon_volt(&self) -> bool {
        *self == WU4_EV_A::BATMON_VOLT
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP`"]
    #[inline(always)]
    pub fn is_batmon_temp(&self) -> bool {
        *self == WU4_EV_A::BATMON_TEMP
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == WU4_EV_A::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == WU4_EV_A::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == WU4_EV_A::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == WU4_EV_A::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == WU4_EV_A::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == WU4_EV_A::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV2`"]
    #[inline(always)]
    pub fn is_aux_swev2(&self) -> bool {
        *self == WU4_EV_A::AUX_SWEV2
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV1`"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == WU4_EV_A::AUX_SWEV1
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV0`"]
    #[inline(always)]
    pub fn is_aux_swev0(&self) -> bool {
        *self == WU4_EV_A::AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `JTAG`"]
    #[inline(always)]
    pub fn is_jtag(&self) -> bool {
        *self == WU4_EV_A::JTAG
    }
    #[doc = "Checks if the value of the field is `RTC_UPD`"]
    #[inline(always)]
    pub fn is_rtc_upd(&self) -> bool {
        *self == WU4_EV_A::RTC_UPD
    }
    #[doc = "Checks if the value of the field is `RTC_COMB_DLY`"]
    #[inline(always)]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == WU4_EV_A::RTC_COMB_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == WU4_EV_A::RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH1_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == WU4_EV_A::RTC_CH1_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH0_DLY`"]
    #[inline(always)]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == WU4_EV_A::RTC_CH0_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2`"]
    #[inline(always)]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == WU4_EV_A::RTC_CH2
    }
    #[doc = "Checks if the value of the field is `RTC_CH1`"]
    #[inline(always)]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == WU4_EV_A::RTC_CH1
    }
    #[doc = "Checks if the value of the field is `RTC_CH0`"]
    #[inline(always)]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == WU4_EV_A::RTC_CH0
    }
    #[doc = "Checks if the value of the field is `PAD`"]
    #[inline(always)]
    pub fn is_pad(&self) -> bool {
        *self == WU4_EV_A::PAD
    }
    #[doc = "Checks if the value of the field is `BATMON_COMBINED`"]
    #[inline(always)]
    pub fn is_batmon_combined(&self) -> bool {
        *self == WU4_EV_A::BATMON_COMBINED
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_LL`"]
    #[inline(always)]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == WU4_EV_A::BATMON_TEMP_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP_UL`"]
    #[inline(always)]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == WU4_EV_A::BATMON_TEMP_UL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_LL`"]
    #[inline(always)]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == WU4_EV_A::BATMON_BATT_LL
    }
    #[doc = "Checks if the value of the field is `BATMON_BATT_UL`"]
    #[inline(always)]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == WU4_EV_A::BATMON_BATT_UL
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == WU4_EV_A::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == WU4_EV_A::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == WU4_EV_A::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == WU4_EV_A::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `IOEV_MCU_WU`"]
    #[inline(always)]
    pub fn is_ioev_mcu_wu(&self) -> bool {
        *self == WU4_EV_A::IOEV_MCU_WU
    }
}
#[doc = "Write proxy for field `WU4_EV`"]
pub struct WU4_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> WU4_EV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WU4_EV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No event, always low"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(WU4_EV_A::NONE)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async_n(self) -> &'a mut W {
        self.variant(WU4_EV_A::AUX_COMPB_ASYNC_N)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline(always)]
    pub fn aux_compb_async(self) -> &'a mut W {
        self.variant(WU4_EV_A::AUX_COMPB_ASYNC)
    }
    #[doc = "BATMON voltage update event"]
    #[inline(always)]
    pub fn batmon_volt(self) -> &'a mut W {
        self.variant(WU4_EV_A::BATMON_VOLT)
    }
    #[doc = "BATMON temperature update event"]
    #[inline(always)]
    pub fn batmon_temp(self) -> &'a mut W {
        self.variant(WU4_EV_A::BATMON_TEMP)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(WU4_EV_A::AUX_TIMER1_EV)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(WU4_EV_A::AUX_TIMER0_EV)
    }
    #[doc = "TDC completed or timed out"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(WU4_EV_A::AUX_TDC_DONE)
    }
    #[doc = "ADC conversion completed"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(WU4_EV_A::AUX_ADC_DONE)
    }
    #[doc = "Comparator B triggered"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(WU4_EV_A::AUX_COMPB)
    }
    #[doc = "Comparator A triggered"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(WU4_EV_A::AUX_COMPA)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline(always)]
    pub fn aux_swev2(self) -> &'a mut W {
        self.variant(WU4_EV_A::AUX_SWEV2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline(always)]
    pub fn aux_swev1(self) -> &'a mut W {
        self.variant(WU4_EV_A::AUX_SWEV1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn aux_swev0(self) -> &'a mut W {
        self.variant(WU4_EV_A::AUX_SWEV0)
    }
    #[doc = "JTAG generated event"]
    #[inline(always)]
    pub fn jtag(self) -> &'a mut W {
        self.variant(WU4_EV_A::JTAG)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline(always)]
    pub fn rtc_upd(self) -> &'a mut W {
        self.variant(WU4_EV_A::RTC_UPD)
    }
    #[doc = "RTC combined delayed event"]
    #[inline(always)]
    pub fn rtc_comb_dly(self) -> &'a mut W {
        self.variant(WU4_EV_A::RTC_COMB_DLY)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch2_dly(self) -> &'a mut W {
        self.variant(WU4_EV_A::RTC_CH2_DLY)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch1_dly(self) -> &'a mut W {
        self.variant(WU4_EV_A::RTC_CH1_DLY)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline(always)]
    pub fn rtc_ch0_dly(self) -> &'a mut W {
        self.variant(WU4_EV_A::RTC_CH0_DLY)
    }
    #[doc = "RTC channel 2 event"]
    #[inline(always)]
    pub fn rtc_ch2(self) -> &'a mut W {
        self.variant(WU4_EV_A::RTC_CH2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline(always)]
    pub fn rtc_ch1(self) -> &'a mut W {
        self.variant(WU4_EV_A::RTC_CH1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline(always)]
    pub fn rtc_ch0(self) -> &'a mut W {
        self.variant(WU4_EV_A::RTC_CH0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline(always)]
    pub fn pad(self) -> &'a mut W {
        self.variant(WU4_EV_A::PAD)
    }
    #[doc = "Combined event from BATMON"]
    #[inline(always)]
    pub fn batmon_combined(self) -> &'a mut W {
        self.variant(WU4_EV_A::BATMON_COMBINED)
    }
    #[doc = "BATMON event: Temperature level below lower limit"]
    #[inline(always)]
    pub fn batmon_temp_ll(self) -> &'a mut W {
        self.variant(WU4_EV_A::BATMON_TEMP_LL)
    }
    #[doc = "BATMON event: Temperature level above upper limit"]
    #[inline(always)]
    pub fn batmon_temp_ul(self) -> &'a mut W {
        self.variant(WU4_EV_A::BATMON_TEMP_UL)
    }
    #[doc = "BATMON event: Battery level below lower limit"]
    #[inline(always)]
    pub fn batmon_batt_ll(self) -> &'a mut W {
        self.variant(WU4_EV_A::BATMON_BATT_LL)
    }
    #[doc = "BATMON event: Battery level above upper limit"]
    #[inline(always)]
    pub fn batmon_batt_ul(self) -> &'a mut W {
        self.variant(WU4_EV_A::BATMON_BATT_UL)
    }
    #[doc = "Event 3 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(WU4_EV_A::AUX_TIMER2_EV3)
    }
    #[doc = "Event 2 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(WU4_EV_A::AUX_TIMER2_EV2)
    }
    #[doc = "Event 1 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(WU4_EV_A::AUX_TIMER2_EV1)
    }
    #[doc = "Event 0 from AUX TImer2"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(WU4_EV_A::AUX_TIMER2_EV0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_MCU_WU in \\[MCU_IOC:IOCFGx.IOEV_MCU_WU_EN\\]"]
    #[inline(always)]
    pub fn ioev_mcu_wu(self) -> &'a mut W {
        self.variant(WU4_EV_A::IOEV_MCU_WU)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved30(&self) -> RESERVED30_R {
        RESERVED30_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
MCU Wakeup Source #7 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    pub fn wu7_ev(&self) -> WU7_EV_R {
        WU7_EV_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved22(&self) -> RESERVED22_R {
        RESERVED22_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
MCU Wakeup Source #6 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    pub fn wu6_ev(&self) -> WU6_EV_R {
        WU6_EV_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> RESERVED14_R {
        RESERVED14_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
MCU Wakeup Source #5 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    pub fn wu5_ev(&self) -> WU5_EV_R {
        WU5_EV_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 0:5 - 5:0\\]
MCU Wakeup Source #4 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    pub fn wu4_ev(&self) -> WU4_EV_R {
        WU4_EV_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved30(&mut self) -> RESERVED30_W {
        RESERVED30_W { w: self }
    }
    #[doc = "Bits 24:29 - 29:24\\]
MCU Wakeup Source #7 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    pub fn wu7_ev(&mut self) -> WU7_EV_W {
        WU7_EV_W { w: self }
    }
    #[doc = "Bits 22:23 - 23:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved22(&mut self) -> RESERVED22_W {
        RESERVED22_W { w: self }
    }
    #[doc = "Bits 16:21 - 21:16\\]
MCU Wakeup Source #6 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    pub fn wu6_ev(&mut self) -> WU6_EV_W {
        WU6_EV_W { w: self }
    }
    #[doc = "Bits 14:15 - 15:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&mut self) -> RESERVED14_W {
        RESERVED14_W { w: self }
    }
    #[doc = "Bits 8:13 - 13:8\\]
MCU Wakeup Source #5 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    pub fn wu5_ev(&mut self) -> WU5_EV_W {
        WU5_EV_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&mut self) -> RESERVED6_W {
        RESERVED6_W { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\]
MCU Wakeup Source #4 AON Event Source selecting 1 of 8 events routed to AON_PMCTRL for waking up the MCU domain from Power Off or Power Down. Note:"]
    #[inline(always)]
    pub fn wu4_ev(&mut self) -> WU4_EV_W {
        WU4_EV_W { w: self }
    }
}
