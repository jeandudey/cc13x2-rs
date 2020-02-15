#[doc = "Reader of register CPUIRQSEL30"]
pub type R = crate::R<u32, super::CPUIRQSEL30>;
#[doc = "Writer for register CPUIRQSEL30"]
pub type W = crate::W<u32, super::CPUIRQSEL30>;
#[doc = "Register CPUIRQSEL30 `reset()`'s with value 0"]
impl crate::ResetValue for super::CPUIRQSEL30 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EV_A {
    #[doc = "121: Always asserted"]
    ALWAYS_ACTIVE = 121,
    #[doc = "119: RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN"]
    AON_RTC_UPD = 119,
    #[doc = "114: Loopback of OBSMUX0 through AUX, corresponds to AUX_EVCTL:EVTOMCUFLAGS.MCU_OBSMUX0"]
    AUX_OBSMUX0 = 114,
    #[doc = "113: AUX ADC FIFO watermark event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL"]
    AUX_ADC_FIFO_ALMOST_FULL = 113,
    #[doc = "112: AUX ADC done, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_ADC_DONE"]
    AUX_ADC_DONE = 112,
    #[doc = "111: Autotake event from AUX semaphore, configured by AUX_SMPH:AUTOTAKE"]
    AUX_SMPH_AUTOTAKE_DONE = 111,
    #[doc = "110: AUX timer 1 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER1_EV"]
    AUX_TIMER1_EV = 110,
    #[doc = "109: AUX timer 0 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER0_EV"]
    AUX_TIMER0_EV = 109,
    #[doc = "108: AUX TDC measurement done event, corresponds to the flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TDC_DONE and the AUX_TDC status AUX_TDC:STAT.DONE"]
    AUX_TDC_DONE = 108,
    #[doc = "107: AUX Compare B event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPB"]
    AUX_COMPB = 107,
    #[doc = "105: AON wakeup event, the corresponding flag is here AUX_EVCTL:EVTOMCUFLAGS.AUX_WU_EV"]
    AUX_AON_WU_EV = 105,
    #[doc = "94: CRYPTO DMA input done event, the correspondingg flag is CRYPTO:IRQSTAT.DMA_IN_DONE. Controlled by CRYPTO:IRQEN.DMA_IN_DONE"]
    CRYPTO_DMA_DONE_IRQ = 94,
    #[doc = "60: AUX Timer2 pulse, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_PULSE"]
    AUX_TIMER2_PULSE = 60,
    #[doc = "59: AUX Timer2 event 3, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV3"]
    AUX_TIMER2_EV3 = 59,
    #[doc = "58: AUX Timer2 event 2, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV2"]
    AUX_TIMER2_EV2 = 58,
    #[doc = "57: AUX Timer2 event 1, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV1"]
    AUX_TIMER2_EV1 = 57,
    #[doc = "56: AUX Timer2 event 0, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV0"]
    AUX_TIMER2_EV0 = 56,
    #[doc = "22: DMA done for software tiggered UDMA channel 18, see UDMA0:SOFTREQ"]
    DMA_CH18_DONE = 22,
    #[doc = "20: DMA done for software tiggered UDMA channel 0, see UDMA0:SOFTREQ"]
    DMA_CH0_DONE = 20,
    #[doc = "10: AUX Software event 0, AUX_EVCTL:SWEVSET.SWEV0"]
    AON_AUX_SWEV0 = 10,
    #[doc = "8: Interrupt event from I2S"]
    I2S_IRQ = 8,
    #[doc = "3: AON programmable event 2. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG2_EV"]
    AON_PROG2 = 3,
    #[doc = "2: AON programmable event 1. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG1_EV"]
    AON_PROG1 = 2,
    #[doc = "0: Always inactive"]
    NONE = 0,
}
impl From<EV_A> for u8 {
    #[inline(always)]
    fn from(variant: EV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EV`"]
pub type EV_R = crate::R<u8, EV_A>;
impl EV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EV_A> {
        use crate::Variant::*;
        match self.bits {
            121 => Val(EV_A::ALWAYS_ACTIVE),
            119 => Val(EV_A::AON_RTC_UPD),
            114 => Val(EV_A::AUX_OBSMUX0),
            113 => Val(EV_A::AUX_ADC_FIFO_ALMOST_FULL),
            112 => Val(EV_A::AUX_ADC_DONE),
            111 => Val(EV_A::AUX_SMPH_AUTOTAKE_DONE),
            110 => Val(EV_A::AUX_TIMER1_EV),
            109 => Val(EV_A::AUX_TIMER0_EV),
            108 => Val(EV_A::AUX_TDC_DONE),
            107 => Val(EV_A::AUX_COMPB),
            105 => Val(EV_A::AUX_AON_WU_EV),
            94 => Val(EV_A::CRYPTO_DMA_DONE_IRQ),
            60 => Val(EV_A::AUX_TIMER2_PULSE),
            59 => Val(EV_A::AUX_TIMER2_EV3),
            58 => Val(EV_A::AUX_TIMER2_EV2),
            57 => Val(EV_A::AUX_TIMER2_EV1),
            56 => Val(EV_A::AUX_TIMER2_EV0),
            22 => Val(EV_A::DMA_CH18_DONE),
            20 => Val(EV_A::DMA_CH0_DONE),
            10 => Val(EV_A::AON_AUX_SWEV0),
            8 => Val(EV_A::I2S_IRQ),
            3 => Val(EV_A::AON_PROG2),
            2 => Val(EV_A::AON_PROG1),
            0 => Val(EV_A::NONE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALWAYS_ACTIVE`"]
    #[inline(always)]
    pub fn is_always_active(&self) -> bool {
        *self == EV_A::ALWAYS_ACTIVE
    }
    #[doc = "Checks if the value of the field is `AON_RTC_UPD`"]
    #[inline(always)]
    pub fn is_aon_rtc_upd(&self) -> bool {
        *self == EV_A::AON_RTC_UPD
    }
    #[doc = "Checks if the value of the field is `AUX_OBSMUX0`"]
    #[inline(always)]
    pub fn is_aux_obsmux0(&self) -> bool {
        *self == EV_A::AUX_OBSMUX0
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_FIFO_ALMOST_FULL`"]
    #[inline(always)]
    pub fn is_aux_adc_fifo_almost_full(&self) -> bool {
        *self == EV_A::AUX_ADC_FIFO_ALMOST_FULL
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == EV_A::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_SMPH_AUTOTAKE_DONE`"]
    #[inline(always)]
    pub fn is_aux_smph_autotake_done(&self) -> bool {
        *self == EV_A::AUX_SMPH_AUTOTAKE_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == EV_A::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == EV_A::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == EV_A::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == EV_A::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_AON_WU_EV`"]
    #[inline(always)]
    pub fn is_aux_aon_wu_ev(&self) -> bool {
        *self == EV_A::AUX_AON_WU_EV
    }
    #[doc = "Checks if the value of the field is `CRYPTO_DMA_DONE_IRQ`"]
    #[inline(always)]
    pub fn is_crypto_dma_done_irq(&self) -> bool {
        *self == EV_A::CRYPTO_DMA_DONE_IRQ
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_PULSE`"]
    #[inline(always)]
    pub fn is_aux_timer2_pulse(&self) -> bool {
        *self == EV_A::AUX_TIMER2_PULSE
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == EV_A::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == EV_A::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == EV_A::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == EV_A::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `DMA_CH18_DONE`"]
    #[inline(always)]
    pub fn is_dma_ch18_done(&self) -> bool {
        *self == EV_A::DMA_CH18_DONE
    }
    #[doc = "Checks if the value of the field is `DMA_CH0_DONE`"]
    #[inline(always)]
    pub fn is_dma_ch0_done(&self) -> bool {
        *self == EV_A::DMA_CH0_DONE
    }
    #[doc = "Checks if the value of the field is `AON_AUX_SWEV0`"]
    #[inline(always)]
    pub fn is_aon_aux_swev0(&self) -> bool {
        *self == EV_A::AON_AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `I2S_IRQ`"]
    #[inline(always)]
    pub fn is_i2s_irq(&self) -> bool {
        *self == EV_A::I2S_IRQ
    }
    #[doc = "Checks if the value of the field is `AON_PROG2`"]
    #[inline(always)]
    pub fn is_aon_prog2(&self) -> bool {
        *self == EV_A::AON_PROG2
    }
    #[doc = "Checks if the value of the field is `AON_PROG1`"]
    #[inline(always)]
    pub fn is_aon_prog1(&self) -> bool {
        *self == EV_A::AON_PROG1
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == EV_A::NONE
    }
}
#[doc = "Write proxy for field `EV`"]
pub struct EV_W<'a> {
    w: &'a mut W,
}
impl<'a> EV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Always asserted"]
    #[inline(always)]
    pub fn always_active(self) -> &'a mut W {
        self.variant(EV_A::ALWAYS_ACTIVE)
    }
    #[doc = "RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN"]
    #[inline(always)]
    pub fn aon_rtc_upd(self) -> &'a mut W {
        self.variant(EV_A::AON_RTC_UPD)
    }
    #[doc = "Loopback of OBSMUX0 through AUX, corresponds to AUX_EVCTL:EVTOMCUFLAGS.MCU_OBSMUX0"]
    #[inline(always)]
    pub fn aux_obsmux0(self) -> &'a mut W {
        self.variant(EV_A::AUX_OBSMUX0)
    }
    #[doc = "AUX ADC FIFO watermark event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn aux_adc_fifo_almost_full(self) -> &'a mut W {
        self.variant(EV_A::AUX_ADC_FIFO_ALMOST_FULL)
    }
    #[doc = "AUX ADC done, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_ADC_DONE"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(EV_A::AUX_ADC_DONE)
    }
    #[doc = "Autotake event from AUX semaphore, configured by AUX_SMPH:AUTOTAKE"]
    #[inline(always)]
    pub fn aux_smph_autotake_done(self) -> &'a mut W {
        self.variant(EV_A::AUX_SMPH_AUTOTAKE_DONE)
    }
    #[doc = "AUX timer 1 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER1_EV"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(EV_A::AUX_TIMER1_EV)
    }
    #[doc = "AUX timer 0 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER0_EV"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(EV_A::AUX_TIMER0_EV)
    }
    #[doc = "AUX TDC measurement done event, corresponds to the flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TDC_DONE and the AUX_TDC status AUX_TDC:STAT.DONE"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(EV_A::AUX_TDC_DONE)
    }
    #[doc = "AUX Compare B event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPB"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(EV_A::AUX_COMPB)
    }
    #[doc = "AON wakeup event, the corresponding flag is here AUX_EVCTL:EVTOMCUFLAGS.AUX_WU_EV"]
    #[inline(always)]
    pub fn aux_aon_wu_ev(self) -> &'a mut W {
        self.variant(EV_A::AUX_AON_WU_EV)
    }
    #[doc = "CRYPTO DMA input done event, the correspondingg flag is CRYPTO:IRQSTAT.DMA_IN_DONE. Controlled by CRYPTO:IRQEN.DMA_IN_DONE"]
    #[inline(always)]
    pub fn crypto_dma_done_irq(self) -> &'a mut W {
        self.variant(EV_A::CRYPTO_DMA_DONE_IRQ)
    }
    #[doc = "AUX Timer2 pulse, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_PULSE"]
    #[inline(always)]
    pub fn aux_timer2_pulse(self) -> &'a mut W {
        self.variant(EV_A::AUX_TIMER2_PULSE)
    }
    #[doc = "AUX Timer2 event 3, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV3"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(EV_A::AUX_TIMER2_EV3)
    }
    #[doc = "AUX Timer2 event 2, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV2"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(EV_A::AUX_TIMER2_EV2)
    }
    #[doc = "AUX Timer2 event 1, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV1"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(EV_A::AUX_TIMER2_EV1)
    }
    #[doc = "AUX Timer2 event 0, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV0"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(EV_A::AUX_TIMER2_EV0)
    }
    #[doc = "DMA done for software tiggered UDMA channel 18, see UDMA0:SOFTREQ"]
    #[inline(always)]
    pub fn dma_ch18_done(self) -> &'a mut W {
        self.variant(EV_A::DMA_CH18_DONE)
    }
    #[doc = "DMA done for software tiggered UDMA channel 0, see UDMA0:SOFTREQ"]
    #[inline(always)]
    pub fn dma_ch0_done(self) -> &'a mut W {
        self.variant(EV_A::DMA_CH0_DONE)
    }
    #[doc = "AUX Software event 0, AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn aon_aux_swev0(self) -> &'a mut W {
        self.variant(EV_A::AON_AUX_SWEV0)
    }
    #[doc = "Interrupt event from I2S"]
    #[inline(always)]
    pub fn i2s_irq(self) -> &'a mut W {
        self.variant(EV_A::I2S_IRQ)
    }
    #[doc = "AON programmable event 2. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG2_EV"]
    #[inline(always)]
    pub fn aon_prog2(self) -> &'a mut W {
        self.variant(EV_A::AON_PROG2)
    }
    #[doc = "AON programmable event 1. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG1_EV"]
    #[inline(always)]
    pub fn aon_prog1(self) -> &'a mut W {
        self.variant(EV_A::AON_PROG1)
    }
    #[doc = "Always inactive"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(EV_A::NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub fn ev(&self) -> EV_R {
        EV_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub fn ev(&mut self) -> EV_W {
        EV_W { w: self }
    }
}
