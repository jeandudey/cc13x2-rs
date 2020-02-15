#[doc = "Reader of register RFCSEL9"]
pub type R = crate::R<u32, super::RFCSEL9>;
#[doc = "Writer for register RFCSEL9"]
pub type W = crate::W<u32, super::RFCSEL9>;
#[doc = "Register RFCSEL9 `reset()`'s with value 0x02"]
impl crate::ResetValue for super::RFCSEL9 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EV_A {
    #[doc = "121: Always asserted"]
    ALWAYS_ACTIVE = 121,
    #[doc = "115: AUX ADC interrupt event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_ADC_IRQ. Status flags are found here AUX_EVCTL:EVTOMCUFLAGS"]
    AUX_ADC_IRQ = 115,
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
    #[doc = "106: AUX Compare A event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPA"]
    AUX_COMPA = 106,
    #[doc = "105: AON wakeup event, the corresponding flag is here AUX_EVCTL:EVTOMCUFLAGS.AUX_WU_EV"]
    AUX_AON_WU_EV = 105,
    #[doc = "101: Software event 1, triggered by SWEV.SWEV1"]
    SWEV1 = 101,
    #[doc = "100: Software event 0, triggered by SWEV.SWEV0"]
    SWEV0 = 100,
    #[doc = "93: CRYPTO result available interupt event, the corresponding flag is found here CRYPTO:IRQSTAT.RESULT_AVAIL. Controlled by CRYPTO:IRQSTAT.RESULT_AVAIL"]
    CRYPTO_RESULT_AVAIL_IRQ = 93,
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
    #[doc = "39: Combined DMA done, corresponding flags are here UDMA0:REQDONE"]
    DMA_DONE_COMB = 39,
    #[doc = "37: UART1 combined interrupt, interrupt flags are found here UART1:MIS"]
    UART1_COMB = 37,
    #[doc = "36: UART0 combined interrupt, interrupt flags are found here UART0:MIS"]
    UART0_COMB = 36,
    #[doc = "35: SSI1 combined interrupt, interrupt flags are found here SSI1:MIS"]
    SSI1_COMB = 35,
    #[doc = "34: SSI0 combined interrupt, interrupt flags are found here SSI0:MIS"]
    SSI0_COMB = 34,
    #[doc = "24: Watchdog interrupt event, controlled by WDT:CTL.INTEN"]
    WDT_IRQ = 24,
    #[doc = "10: AUX Software event 0, AUX_EVCTL:SWEVSET.SWEV0"]
    AON_AUX_SWEV0 = 10,
    #[doc = "8: Interrupt event from I2S"]
    I2S_IRQ = 8,
    #[doc = "2: AON programmable event 1. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG1_EV"]
    AON_PROG1 = 2,
    #[doc = "1: AON programmable event 0. Event selected by AON_EVENT  MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG0_EV"]
    AON_PROG0 = 1,
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
            115 => Val(EV_A::AUX_ADC_IRQ),
            114 => Val(EV_A::AUX_OBSMUX0),
            113 => Val(EV_A::AUX_ADC_FIFO_ALMOST_FULL),
            112 => Val(EV_A::AUX_ADC_DONE),
            111 => Val(EV_A::AUX_SMPH_AUTOTAKE_DONE),
            110 => Val(EV_A::AUX_TIMER1_EV),
            109 => Val(EV_A::AUX_TIMER0_EV),
            108 => Val(EV_A::AUX_TDC_DONE),
            107 => Val(EV_A::AUX_COMPB),
            106 => Val(EV_A::AUX_COMPA),
            105 => Val(EV_A::AUX_AON_WU_EV),
            101 => Val(EV_A::SWEV1),
            100 => Val(EV_A::SWEV0),
            93 => Val(EV_A::CRYPTO_RESULT_AVAIL_IRQ),
            60 => Val(EV_A::AUX_TIMER2_PULSE),
            59 => Val(EV_A::AUX_TIMER2_EV3),
            58 => Val(EV_A::AUX_TIMER2_EV2),
            57 => Val(EV_A::AUX_TIMER2_EV1),
            56 => Val(EV_A::AUX_TIMER2_EV0),
            39 => Val(EV_A::DMA_DONE_COMB),
            37 => Val(EV_A::UART1_COMB),
            36 => Val(EV_A::UART0_COMB),
            35 => Val(EV_A::SSI1_COMB),
            34 => Val(EV_A::SSI0_COMB),
            24 => Val(EV_A::WDT_IRQ),
            10 => Val(EV_A::AON_AUX_SWEV0),
            8 => Val(EV_A::I2S_IRQ),
            2 => Val(EV_A::AON_PROG1),
            1 => Val(EV_A::AON_PROG0),
            0 => Val(EV_A::NONE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALWAYS_ACTIVE`"]
    #[inline(always)]
    pub fn is_always_active(&self) -> bool {
        *self == EV_A::ALWAYS_ACTIVE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_IRQ`"]
    #[inline(always)]
    pub fn is_aux_adc_irq(&self) -> bool {
        *self == EV_A::AUX_ADC_IRQ
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
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == EV_A::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_AON_WU_EV`"]
    #[inline(always)]
    pub fn is_aux_aon_wu_ev(&self) -> bool {
        *self == EV_A::AUX_AON_WU_EV
    }
    #[doc = "Checks if the value of the field is `SWEV1`"]
    #[inline(always)]
    pub fn is_swev1(&self) -> bool {
        *self == EV_A::SWEV1
    }
    #[doc = "Checks if the value of the field is `SWEV0`"]
    #[inline(always)]
    pub fn is_swev0(&self) -> bool {
        *self == EV_A::SWEV0
    }
    #[doc = "Checks if the value of the field is `CRYPTO_RESULT_AVAIL_IRQ`"]
    #[inline(always)]
    pub fn is_crypto_result_avail_irq(&self) -> bool {
        *self == EV_A::CRYPTO_RESULT_AVAIL_IRQ
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
    #[doc = "Checks if the value of the field is `DMA_DONE_COMB`"]
    #[inline(always)]
    pub fn is_dma_done_comb(&self) -> bool {
        *self == EV_A::DMA_DONE_COMB
    }
    #[doc = "Checks if the value of the field is `UART1_COMB`"]
    #[inline(always)]
    pub fn is_uart1_comb(&self) -> bool {
        *self == EV_A::UART1_COMB
    }
    #[doc = "Checks if the value of the field is `UART0_COMB`"]
    #[inline(always)]
    pub fn is_uart0_comb(&self) -> bool {
        *self == EV_A::UART0_COMB
    }
    #[doc = "Checks if the value of the field is `SSI1_COMB`"]
    #[inline(always)]
    pub fn is_ssi1_comb(&self) -> bool {
        *self == EV_A::SSI1_COMB
    }
    #[doc = "Checks if the value of the field is `SSI0_COMB`"]
    #[inline(always)]
    pub fn is_ssi0_comb(&self) -> bool {
        *self == EV_A::SSI0_COMB
    }
    #[doc = "Checks if the value of the field is `WDT_IRQ`"]
    #[inline(always)]
    pub fn is_wdt_irq(&self) -> bool {
        *self == EV_A::WDT_IRQ
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
    #[doc = "Checks if the value of the field is `AON_PROG1`"]
    #[inline(always)]
    pub fn is_aon_prog1(&self) -> bool {
        *self == EV_A::AON_PROG1
    }
    #[doc = "Checks if the value of the field is `AON_PROG0`"]
    #[inline(always)]
    pub fn is_aon_prog0(&self) -> bool {
        *self == EV_A::AON_PROG0
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
    #[doc = "AUX ADC interrupt event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_ADC_IRQ. Status flags are found here AUX_EVCTL:EVTOMCUFLAGS"]
    #[inline(always)]
    pub fn aux_adc_irq(self) -> &'a mut W {
        self.variant(EV_A::AUX_ADC_IRQ)
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
    #[doc = "AUX Compare A event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPA"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(EV_A::AUX_COMPA)
    }
    #[doc = "AON wakeup event, the corresponding flag is here AUX_EVCTL:EVTOMCUFLAGS.AUX_WU_EV"]
    #[inline(always)]
    pub fn aux_aon_wu_ev(self) -> &'a mut W {
        self.variant(EV_A::AUX_AON_WU_EV)
    }
    #[doc = "Software event 1, triggered by SWEV.SWEV1"]
    #[inline(always)]
    pub fn swev1(self) -> &'a mut W {
        self.variant(EV_A::SWEV1)
    }
    #[doc = "Software event 0, triggered by SWEV.SWEV0"]
    #[inline(always)]
    pub fn swev0(self) -> &'a mut W {
        self.variant(EV_A::SWEV0)
    }
    #[doc = "CRYPTO result available interupt event, the corresponding flag is found here CRYPTO:IRQSTAT.RESULT_AVAIL. Controlled by CRYPTO:IRQSTAT.RESULT_AVAIL"]
    #[inline(always)]
    pub fn crypto_result_avail_irq(self) -> &'a mut W {
        self.variant(EV_A::CRYPTO_RESULT_AVAIL_IRQ)
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
    #[doc = "Combined DMA done, corresponding flags are here UDMA0:REQDONE"]
    #[inline(always)]
    pub fn dma_done_comb(self) -> &'a mut W {
        self.variant(EV_A::DMA_DONE_COMB)
    }
    #[doc = "UART1 combined interrupt, interrupt flags are found here UART1:MIS"]
    #[inline(always)]
    pub fn uart1_comb(self) -> &'a mut W {
        self.variant(EV_A::UART1_COMB)
    }
    #[doc = "UART0 combined interrupt, interrupt flags are found here UART0:MIS"]
    #[inline(always)]
    pub fn uart0_comb(self) -> &'a mut W {
        self.variant(EV_A::UART0_COMB)
    }
    #[doc = "SSI1 combined interrupt, interrupt flags are found here SSI1:MIS"]
    #[inline(always)]
    pub fn ssi1_comb(self) -> &'a mut W {
        self.variant(EV_A::SSI1_COMB)
    }
    #[doc = "SSI0 combined interrupt, interrupt flags are found here SSI0:MIS"]
    #[inline(always)]
    pub fn ssi0_comb(self) -> &'a mut W {
        self.variant(EV_A::SSI0_COMB)
    }
    #[doc = "Watchdog interrupt event, controlled by WDT:CTL.INTEN"]
    #[inline(always)]
    pub fn wdt_irq(self) -> &'a mut W {
        self.variant(EV_A::WDT_IRQ)
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
    #[doc = "AON programmable event 1. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG1_EV"]
    #[inline(always)]
    pub fn aon_prog1(self) -> &'a mut W {
        self.variant(EV_A::AON_PROG1)
    }
    #[doc = "AON programmable event 0. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG0_EV"]
    #[inline(always)]
    pub fn aon_prog0(self) -> &'a mut W {
        self.variant(EV_A::AON_PROG0)
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
