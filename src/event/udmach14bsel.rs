#[doc = "Reader of register UDMACH14BSEL"]
pub type R = crate::R<u32, super::UDMACH14BSEL>;
#[doc = "Writer for register UDMACH14BSEL"]
pub type W = crate::W<u32, super::UDMACH14BSEL>;
#[doc = "Register UDMACH14BSEL `reset()`'s with value 0x01"]
impl crate::ResetValue for super::UDMACH14BSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EV_A {
    #[doc = "121: Always asserted"]
    ALWAYS_ACTIVE = 121,
    #[doc = "120: CPU halted"]
    CPU_HALTED = 120,
    #[doc = "119: RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN"]
    AON_RTC_UPD = 119,
    #[doc = "118: DMA burst request event from AUX, configured by AUX_EVCTL:DMACTL"]
    AUX_DMABREQ = 118,
    #[doc = "117: DMA single request event from AUX, configured by AUX_EVCTL:DMACTL"]
    AUX_DMASREQ = 117,
    #[doc = "116: DMA sofware trigger from AUX, triggered by AUX_EVCTL:DMASWREQ.START"]
    AUX_SW_DMABREQ = 116,
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
    #[doc = "104: TRNG Interrupt event, controlled by TRNG:IRQEN.EN"]
    TRNG_IRQ = 104,
    #[doc = "103: Software event 3, triggered by SWEV.SWEV3"]
    SWEV3 = 103,
    #[doc = "102: Software event 2, triggered by SWEV.SWEV2"]
    SWEV2 = 102,
    #[doc = "101: Software event 1, triggered by SWEV.SWEV1"]
    SWEV1 = 101,
    #[doc = "100: Software event 0, triggered by SWEV.SWEV0"]
    SWEV0 = 100,
    #[doc = "99: Watchdog non maskable interrupt event, controlled by WDT:CTL.INTTYPE"]
    WDT_NMI = 99,
    #[doc = "94: CRYPTO DMA input done event, the correspondingg flag is CRYPTO:IRQSTAT.DMA_IN_DONE. Controlled by CRYPTO:IRQEN.DMA_IN_DONE"]
    CRYPTO_DMA_DONE_IRQ = 94,
    #[doc = "93: CRYPTO result available interupt event, the corresponding flag is found here CRYPTO:IRQSTAT.RESULT_AVAIL. Controlled by CRYPTO:IRQSTAT.RESULT_AVAIL"]
    CRYPTO_RESULT_AVAIL_IRQ = 93,
    #[doc = "92: Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT7 wil be routed here."]
    PORT_EVENT7 = 92,
    #[doc = "91: Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT6 wil be routed here."]
    PORT_EVENT6 = 91,
    #[doc = "90: Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT4 wil be routed here."]
    PORT_EVENT5 = 90,
    #[doc = "89: Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT4 wil be routed here."]
    PORT_EVENT4 = 89,
    #[doc = "88: Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT3 wil be routed here."]
    PORT_EVENT3 = 88,
    #[doc = "87: Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT2 wil be routed here."]
    PORT_EVENT2 = 87,
    #[doc = "86: Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT1 wil be routed here."]
    PORT_EVENT1 = 86,
    #[doc = "85: Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT0 wil be routed here."]
    PORT_EVENT0 = 85,
    #[doc = "84: GPT3B DMA trigger event. Configured by GPT3:DMAEV"]
    GPT3B_DMABREQ = 84,
    #[doc = "83: GPT3A DMA trigger event. Configured by GPT3:DMAEV"]
    GPT3A_DMABREQ = 83,
    #[doc = "82: GPT2B DMA trigger event. Configured by GPT2:DMAEV"]
    GPT2B_DMABREQ = 82,
    #[doc = "81: GPT2A DMA trigger event. Configured by GPT2:DMAEV"]
    GPT2A_DMABREQ = 81,
    #[doc = "80: GPT1B DMA trigger event. Configured by GPT1:DMAEV"]
    GPT1B_DMABREQ = 80,
    #[doc = "79: GPT1A DMA trigger event. Configured by GPT1:DMAEV"]
    GPT1A_DMABREQ = 79,
    #[doc = "78: GPT0B DMA trigger event. Configured by GPT0:DMAEV"]
    GPT0B_DMABREQ = 78,
    #[doc = "77: GPT0A DMA trigger event. Configured by GPT0:DMAEV"]
    GPT0A_DMABREQ = 77,
    #[doc = "68: GPT3B compare event. Configured by GPT3:TBMR.TCACT"]
    GPT3B_CMP = 68,
    #[doc = "67: GPT3A compare event. Configured by GPT3:TAMR.TCACT"]
    GPT3A_CMP = 67,
    #[doc = "66: GPT2B compare event. Configured by GPT2:TBMR.TCACT"]
    GPT2B_CMP = 66,
    #[doc = "65: GPT2A compare event. Configured by GPT2:TAMR.TCACT"]
    GPT2A_CMP = 65,
    #[doc = "64: GPT1B compare event. Configured by GPT1:TBMR.TCACT"]
    GPT1B_CMP = 64,
    #[doc = "63: GPT1A compare event. Configured by GPT1:TAMR.TCACT"]
    GPT1A_CMP = 63,
    #[doc = "62: GPT0B compare event. Configured by GPT0:TBMR.TCACT"]
    GPT0B_CMP = 62,
    #[doc = "61: GPT0A compare event. Configured by GPT0:TAMR.TCACT"]
    GPT0A_CMP = 61,
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
    #[doc = "55: UART1 TX DMA single request, controlled by UART1:DMACTL.TXDMAE"]
    UART1_TX_DMASREQ = 55,
    #[doc = "54: UART1 TX DMA burst request, controlled by UART1:DMACTL.TXDMAE"]
    UART1_TX_DMABREQ = 54,
    #[doc = "53: UART1 RX DMA single request, controlled by UART1:DMACTL.RXDMAE"]
    UART1_RX_DMASREQ = 53,
    #[doc = "52: UART1 RX DMA burst request, controlled by UART1:DMACTL.RXDMAE"]
    UART1_RX_DMABREQ = 52,
    #[doc = "51: UART0 TX DMA single request, controlled by UART0:DMACTL.TXDMAE"]
    UART0_TX_DMASREQ = 51,
    #[doc = "50: UART0 TX DMA burst request, controlled by UART0:DMACTL.TXDMAE"]
    UART0_TX_DMABREQ = 50,
    #[doc = "49: UART0 RX DMA single request, controlled by UART0:DMACTL.RXDMAE"]
    UART0_RX_DMASREQ = 49,
    #[doc = "48: UART0 RX DMA burst request, controlled by UART0:DMACTL.RXDMAE"]
    UART0_RX_DMABREQ = 48,
    #[doc = "47: SSI1 TX DMA single request, controlled by SSI0:DMACR.TXDMAE"]
    SSI1_TX_DMASREQ = 47,
    #[doc = "46: SSI1 TX DMA burst request , controlled by SSI0:DMACR.TXDMAE"]
    SSI1_TX_DMABREQ = 46,
    #[doc = "45: SSI1 RX DMA single request, controlled by SSI0:DMACR.RXDMAE"]
    SSI1_RX_DMASREQ = 45,
    #[doc = "44: SSI1 RX DMA burst request , controlled by SSI0:DMACR.RXDMAE"]
    SSI1_RX_DMABREQ = 44,
    #[doc = "43: SSI0 TX DMA single request, controlled by SSI0:DMACR.TXDMAE"]
    SSI0_TX_DMASREQ = 43,
    #[doc = "42: SSI0 TX DMA burst request , controlled by SSI0:DMACR.TXDMAE"]
    SSI0_TX_DMABREQ = 42,
    #[doc = "41: SSI0 RX DMA single request, controlled by SSI0:DMACR.RXDMAE"]
    SSI0_RX_DMASREQ = 41,
    #[doc = "40: SSI0 RX DMA burst request , controlled by SSI0:DMACR.RXDMAE"]
    SSI0_RX_DMABREQ = 40,
    #[doc = "39: Combined DMA done, corresponding flags are here UDMA0:REQDONE"]
    DMA_DONE_COMB = 39,
    #[doc = "38: DMA bus error, corresponds to UDMA0:ERROR.STATUS"]
    DMA_ERR = 38,
    #[doc = "37: UART1 combined interrupt, interrupt flags are found here UART1:MIS"]
    UART1_COMB = 37,
    #[doc = "36: UART0 combined interrupt, interrupt flags are found here UART0:MIS"]
    UART0_COMB = 36,
    #[doc = "35: SSI1 combined interrupt, interrupt flags are found here SSI1:MIS"]
    SSI1_COMB = 35,
    #[doc = "34: SSI0 combined interrupt, interrupt flags are found here SSI0:MIS"]
    SSI0_COMB = 34,
    #[doc = "31: PKA Interrupt event"]
    PKA_IRQ = 31,
    #[doc = "30: Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE1 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_1 event"]
    RFC_CPE_1 = 30,
    #[doc = "29: AUX software event 1, triggered by AUX_EVCTL:SWEVSET.SWEV1, also available as AUX_EVENT2 AON wake up event.\nMCU domain wakeup control AON_EVENT:MCUWUSEL"]
    AUX_SWEV1 = 29,
    #[doc = "27: Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE0 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_0 event"]
    RFC_CPE_0 = 27,
    #[doc = "26: Combined RFC hardware interrupt, corresponding flag is here RFC_DBELL:RFHWIFG"]
    RFC_HW_COMB = 26,
    #[doc = "25: RFC Doorbell Command Acknowledgement Interrupt, equvialent to RFC_DBELL:RFACKIFG.ACKFLAG"]
    RFC_CMD_ACK = 25,
    #[doc = "24: Watchdog interrupt event, controlled by WDT:CTL.INTEN"]
    WDT_IRQ = 24,
    #[doc = "22: DMA done for software tiggered UDMA channel 18, see UDMA0:SOFTREQ"]
    DMA_CH18_DONE = 22,
    #[doc = "21: FLASH controller error event,  the status flags are FLASH:FEDACSTAT.FSM_DONE and FLASH:FEDACSTAT.RVF_INT"]
    FLASH = 21,
    #[doc = "20: DMA done for software tiggered UDMA channel 0, see UDMA0:SOFTREQ"]
    DMA_CH0_DONE = 20,
    #[doc = "19: GPT1B interrupt event, controlled by GPT1:TBMR"]
    GPT1B = 19,
    #[doc = "18: GPT1A interrupt event, controlled by GPT1:TAMR"]
    GPT1A = 18,
    #[doc = "17: GPT0B interrupt event, controlled by GPT0:TBMR"]
    GPT0B = 17,
    #[doc = "16: GPT0A interrupt event, controlled by GPT0:TAMR"]
    GPT0A = 16,
    #[doc = "15: GPT3B interrupt event, controlled by GPT3:TBMR"]
    GPT3B = 15,
    #[doc = "14: GPT3A interrupt event, controlled by GPT3:TAMR"]
    GPT3A = 14,
    #[doc = "13: GPT2B interrupt event, controlled by GPT2:TBMR"]
    GPT2B = 13,
    #[doc = "12: GPT2A interrupt event, controlled by GPT2:TAMR"]
    GPT2A = 12,
    #[doc = "11: AUX combined event, the corresponding flag register is here AUX_EVCTL:EVTOMCUFLAGS"]
    AUX_COMB = 11,
    #[doc = "10: AUX Software event 0, AUX_EVCTL:SWEVSET.SWEV0"]
    AON_AUX_SWEV0 = 10,
    #[doc = "9: Interrupt event from I2C"]
    I2C_IRQ = 9,
    #[doc = "8: Interrupt event from I2S"]
    I2S_IRQ = 8,
    #[doc = "7: Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting"]
    AON_RTC_COMB = 7,
    #[doc = "6: Combined event from Oscillator control"]
    OSC_COMB = 6,
    #[doc = "5: Combined event from battery monitor"]
    BATMON_COMB = 5,
    #[doc = "4: Edge detect event from IOC. Configureded by the IOC:IOCFGn.EDGE_IRQ_EN and  IOC:IOCFGn.EDGE_DET settings"]
    AON_GPIO_EDGE = 4,
    #[doc = "3: AON programmable event 2. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG2_EV"]
    AON_PROG2 = 3,
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
            120 => Val(EV_A::CPU_HALTED),
            119 => Val(EV_A::AON_RTC_UPD),
            118 => Val(EV_A::AUX_DMABREQ),
            117 => Val(EV_A::AUX_DMASREQ),
            116 => Val(EV_A::AUX_SW_DMABREQ),
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
            104 => Val(EV_A::TRNG_IRQ),
            103 => Val(EV_A::SWEV3),
            102 => Val(EV_A::SWEV2),
            101 => Val(EV_A::SWEV1),
            100 => Val(EV_A::SWEV0),
            99 => Val(EV_A::WDT_NMI),
            94 => Val(EV_A::CRYPTO_DMA_DONE_IRQ),
            93 => Val(EV_A::CRYPTO_RESULT_AVAIL_IRQ),
            92 => Val(EV_A::PORT_EVENT7),
            91 => Val(EV_A::PORT_EVENT6),
            90 => Val(EV_A::PORT_EVENT5),
            89 => Val(EV_A::PORT_EVENT4),
            88 => Val(EV_A::PORT_EVENT3),
            87 => Val(EV_A::PORT_EVENT2),
            86 => Val(EV_A::PORT_EVENT1),
            85 => Val(EV_A::PORT_EVENT0),
            84 => Val(EV_A::GPT3B_DMABREQ),
            83 => Val(EV_A::GPT3A_DMABREQ),
            82 => Val(EV_A::GPT2B_DMABREQ),
            81 => Val(EV_A::GPT2A_DMABREQ),
            80 => Val(EV_A::GPT1B_DMABREQ),
            79 => Val(EV_A::GPT1A_DMABREQ),
            78 => Val(EV_A::GPT0B_DMABREQ),
            77 => Val(EV_A::GPT0A_DMABREQ),
            68 => Val(EV_A::GPT3B_CMP),
            67 => Val(EV_A::GPT3A_CMP),
            66 => Val(EV_A::GPT2B_CMP),
            65 => Val(EV_A::GPT2A_CMP),
            64 => Val(EV_A::GPT1B_CMP),
            63 => Val(EV_A::GPT1A_CMP),
            62 => Val(EV_A::GPT0B_CMP),
            61 => Val(EV_A::GPT0A_CMP),
            60 => Val(EV_A::AUX_TIMER2_PULSE),
            59 => Val(EV_A::AUX_TIMER2_EV3),
            58 => Val(EV_A::AUX_TIMER2_EV2),
            57 => Val(EV_A::AUX_TIMER2_EV1),
            56 => Val(EV_A::AUX_TIMER2_EV0),
            55 => Val(EV_A::UART1_TX_DMASREQ),
            54 => Val(EV_A::UART1_TX_DMABREQ),
            53 => Val(EV_A::UART1_RX_DMASREQ),
            52 => Val(EV_A::UART1_RX_DMABREQ),
            51 => Val(EV_A::UART0_TX_DMASREQ),
            50 => Val(EV_A::UART0_TX_DMABREQ),
            49 => Val(EV_A::UART0_RX_DMASREQ),
            48 => Val(EV_A::UART0_RX_DMABREQ),
            47 => Val(EV_A::SSI1_TX_DMASREQ),
            46 => Val(EV_A::SSI1_TX_DMABREQ),
            45 => Val(EV_A::SSI1_RX_DMASREQ),
            44 => Val(EV_A::SSI1_RX_DMABREQ),
            43 => Val(EV_A::SSI0_TX_DMASREQ),
            42 => Val(EV_A::SSI0_TX_DMABREQ),
            41 => Val(EV_A::SSI0_RX_DMASREQ),
            40 => Val(EV_A::SSI0_RX_DMABREQ),
            39 => Val(EV_A::DMA_DONE_COMB),
            38 => Val(EV_A::DMA_ERR),
            37 => Val(EV_A::UART1_COMB),
            36 => Val(EV_A::UART0_COMB),
            35 => Val(EV_A::SSI1_COMB),
            34 => Val(EV_A::SSI0_COMB),
            31 => Val(EV_A::PKA_IRQ),
            30 => Val(EV_A::RFC_CPE_1),
            29 => Val(EV_A::AUX_SWEV1),
            27 => Val(EV_A::RFC_CPE_0),
            26 => Val(EV_A::RFC_HW_COMB),
            25 => Val(EV_A::RFC_CMD_ACK),
            24 => Val(EV_A::WDT_IRQ),
            22 => Val(EV_A::DMA_CH18_DONE),
            21 => Val(EV_A::FLASH),
            20 => Val(EV_A::DMA_CH0_DONE),
            19 => Val(EV_A::GPT1B),
            18 => Val(EV_A::GPT1A),
            17 => Val(EV_A::GPT0B),
            16 => Val(EV_A::GPT0A),
            15 => Val(EV_A::GPT3B),
            14 => Val(EV_A::GPT3A),
            13 => Val(EV_A::GPT2B),
            12 => Val(EV_A::GPT2A),
            11 => Val(EV_A::AUX_COMB),
            10 => Val(EV_A::AON_AUX_SWEV0),
            9 => Val(EV_A::I2C_IRQ),
            8 => Val(EV_A::I2S_IRQ),
            7 => Val(EV_A::AON_RTC_COMB),
            6 => Val(EV_A::OSC_COMB),
            5 => Val(EV_A::BATMON_COMB),
            4 => Val(EV_A::AON_GPIO_EDGE),
            3 => Val(EV_A::AON_PROG2),
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
    #[doc = "Checks if the value of the field is `CPU_HALTED`"]
    #[inline(always)]
    pub fn is_cpu_halted(&self) -> bool {
        *self == EV_A::CPU_HALTED
    }
    #[doc = "Checks if the value of the field is `AON_RTC_UPD`"]
    #[inline(always)]
    pub fn is_aon_rtc_upd(&self) -> bool {
        *self == EV_A::AON_RTC_UPD
    }
    #[doc = "Checks if the value of the field is `AUX_DMABREQ`"]
    #[inline(always)]
    pub fn is_aux_dmabreq(&self) -> bool {
        *self == EV_A::AUX_DMABREQ
    }
    #[doc = "Checks if the value of the field is `AUX_DMASREQ`"]
    #[inline(always)]
    pub fn is_aux_dmasreq(&self) -> bool {
        *self == EV_A::AUX_DMASREQ
    }
    #[doc = "Checks if the value of the field is `AUX_SW_DMABREQ`"]
    #[inline(always)]
    pub fn is_aux_sw_dmabreq(&self) -> bool {
        *self == EV_A::AUX_SW_DMABREQ
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
    #[doc = "Checks if the value of the field is `TRNG_IRQ`"]
    #[inline(always)]
    pub fn is_trng_irq(&self) -> bool {
        *self == EV_A::TRNG_IRQ
    }
    #[doc = "Checks if the value of the field is `SWEV3`"]
    #[inline(always)]
    pub fn is_swev3(&self) -> bool {
        *self == EV_A::SWEV3
    }
    #[doc = "Checks if the value of the field is `SWEV2`"]
    #[inline(always)]
    pub fn is_swev2(&self) -> bool {
        *self == EV_A::SWEV2
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
    #[doc = "Checks if the value of the field is `WDT_NMI`"]
    #[inline(always)]
    pub fn is_wdt_nmi(&self) -> bool {
        *self == EV_A::WDT_NMI
    }
    #[doc = "Checks if the value of the field is `CRYPTO_DMA_DONE_IRQ`"]
    #[inline(always)]
    pub fn is_crypto_dma_done_irq(&self) -> bool {
        *self == EV_A::CRYPTO_DMA_DONE_IRQ
    }
    #[doc = "Checks if the value of the field is `CRYPTO_RESULT_AVAIL_IRQ`"]
    #[inline(always)]
    pub fn is_crypto_result_avail_irq(&self) -> bool {
        *self == EV_A::CRYPTO_RESULT_AVAIL_IRQ
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT7`"]
    #[inline(always)]
    pub fn is_port_event7(&self) -> bool {
        *self == EV_A::PORT_EVENT7
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT6`"]
    #[inline(always)]
    pub fn is_port_event6(&self) -> bool {
        *self == EV_A::PORT_EVENT6
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT5`"]
    #[inline(always)]
    pub fn is_port_event5(&self) -> bool {
        *self == EV_A::PORT_EVENT5
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT4`"]
    #[inline(always)]
    pub fn is_port_event4(&self) -> bool {
        *self == EV_A::PORT_EVENT4
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT3`"]
    #[inline(always)]
    pub fn is_port_event3(&self) -> bool {
        *self == EV_A::PORT_EVENT3
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT2`"]
    #[inline(always)]
    pub fn is_port_event2(&self) -> bool {
        *self == EV_A::PORT_EVENT2
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT1`"]
    #[inline(always)]
    pub fn is_port_event1(&self) -> bool {
        *self == EV_A::PORT_EVENT1
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT0`"]
    #[inline(always)]
    pub fn is_port_event0(&self) -> bool {
        *self == EV_A::PORT_EVENT0
    }
    #[doc = "Checks if the value of the field is `GPT3B_DMABREQ`"]
    #[inline(always)]
    pub fn is_gpt3b_dmabreq(&self) -> bool {
        *self == EV_A::GPT3B_DMABREQ
    }
    #[doc = "Checks if the value of the field is `GPT3A_DMABREQ`"]
    #[inline(always)]
    pub fn is_gpt3a_dmabreq(&self) -> bool {
        *self == EV_A::GPT3A_DMABREQ
    }
    #[doc = "Checks if the value of the field is `GPT2B_DMABREQ`"]
    #[inline(always)]
    pub fn is_gpt2b_dmabreq(&self) -> bool {
        *self == EV_A::GPT2B_DMABREQ
    }
    #[doc = "Checks if the value of the field is `GPT2A_DMABREQ`"]
    #[inline(always)]
    pub fn is_gpt2a_dmabreq(&self) -> bool {
        *self == EV_A::GPT2A_DMABREQ
    }
    #[doc = "Checks if the value of the field is `GPT1B_DMABREQ`"]
    #[inline(always)]
    pub fn is_gpt1b_dmabreq(&self) -> bool {
        *self == EV_A::GPT1B_DMABREQ
    }
    #[doc = "Checks if the value of the field is `GPT1A_DMABREQ`"]
    #[inline(always)]
    pub fn is_gpt1a_dmabreq(&self) -> bool {
        *self == EV_A::GPT1A_DMABREQ
    }
    #[doc = "Checks if the value of the field is `GPT0B_DMABREQ`"]
    #[inline(always)]
    pub fn is_gpt0b_dmabreq(&self) -> bool {
        *self == EV_A::GPT0B_DMABREQ
    }
    #[doc = "Checks if the value of the field is `GPT0A_DMABREQ`"]
    #[inline(always)]
    pub fn is_gpt0a_dmabreq(&self) -> bool {
        *self == EV_A::GPT0A_DMABREQ
    }
    #[doc = "Checks if the value of the field is `GPT3B_CMP`"]
    #[inline(always)]
    pub fn is_gpt3b_cmp(&self) -> bool {
        *self == EV_A::GPT3B_CMP
    }
    #[doc = "Checks if the value of the field is `GPT3A_CMP`"]
    #[inline(always)]
    pub fn is_gpt3a_cmp(&self) -> bool {
        *self == EV_A::GPT3A_CMP
    }
    #[doc = "Checks if the value of the field is `GPT2B_CMP`"]
    #[inline(always)]
    pub fn is_gpt2b_cmp(&self) -> bool {
        *self == EV_A::GPT2B_CMP
    }
    #[doc = "Checks if the value of the field is `GPT2A_CMP`"]
    #[inline(always)]
    pub fn is_gpt2a_cmp(&self) -> bool {
        *self == EV_A::GPT2A_CMP
    }
    #[doc = "Checks if the value of the field is `GPT1B_CMP`"]
    #[inline(always)]
    pub fn is_gpt1b_cmp(&self) -> bool {
        *self == EV_A::GPT1B_CMP
    }
    #[doc = "Checks if the value of the field is `GPT1A_CMP`"]
    #[inline(always)]
    pub fn is_gpt1a_cmp(&self) -> bool {
        *self == EV_A::GPT1A_CMP
    }
    #[doc = "Checks if the value of the field is `GPT0B_CMP`"]
    #[inline(always)]
    pub fn is_gpt0b_cmp(&self) -> bool {
        *self == EV_A::GPT0B_CMP
    }
    #[doc = "Checks if the value of the field is `GPT0A_CMP`"]
    #[inline(always)]
    pub fn is_gpt0a_cmp(&self) -> bool {
        *self == EV_A::GPT0A_CMP
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
    #[doc = "Checks if the value of the field is `UART1_TX_DMASREQ`"]
    #[inline(always)]
    pub fn is_uart1_tx_dmasreq(&self) -> bool {
        *self == EV_A::UART1_TX_DMASREQ
    }
    #[doc = "Checks if the value of the field is `UART1_TX_DMABREQ`"]
    #[inline(always)]
    pub fn is_uart1_tx_dmabreq(&self) -> bool {
        *self == EV_A::UART1_TX_DMABREQ
    }
    #[doc = "Checks if the value of the field is `UART1_RX_DMASREQ`"]
    #[inline(always)]
    pub fn is_uart1_rx_dmasreq(&self) -> bool {
        *self == EV_A::UART1_RX_DMASREQ
    }
    #[doc = "Checks if the value of the field is `UART1_RX_DMABREQ`"]
    #[inline(always)]
    pub fn is_uart1_rx_dmabreq(&self) -> bool {
        *self == EV_A::UART1_RX_DMABREQ
    }
    #[doc = "Checks if the value of the field is `UART0_TX_DMASREQ`"]
    #[inline(always)]
    pub fn is_uart0_tx_dmasreq(&self) -> bool {
        *self == EV_A::UART0_TX_DMASREQ
    }
    #[doc = "Checks if the value of the field is `UART0_TX_DMABREQ`"]
    #[inline(always)]
    pub fn is_uart0_tx_dmabreq(&self) -> bool {
        *self == EV_A::UART0_TX_DMABREQ
    }
    #[doc = "Checks if the value of the field is `UART0_RX_DMASREQ`"]
    #[inline(always)]
    pub fn is_uart0_rx_dmasreq(&self) -> bool {
        *self == EV_A::UART0_RX_DMASREQ
    }
    #[doc = "Checks if the value of the field is `UART0_RX_DMABREQ`"]
    #[inline(always)]
    pub fn is_uart0_rx_dmabreq(&self) -> bool {
        *self == EV_A::UART0_RX_DMABREQ
    }
    #[doc = "Checks if the value of the field is `SSI1_TX_DMASREQ`"]
    #[inline(always)]
    pub fn is_ssi1_tx_dmasreq(&self) -> bool {
        *self == EV_A::SSI1_TX_DMASREQ
    }
    #[doc = "Checks if the value of the field is `SSI1_TX_DMABREQ`"]
    #[inline(always)]
    pub fn is_ssi1_tx_dmabreq(&self) -> bool {
        *self == EV_A::SSI1_TX_DMABREQ
    }
    #[doc = "Checks if the value of the field is `SSI1_RX_DMASREQ`"]
    #[inline(always)]
    pub fn is_ssi1_rx_dmasreq(&self) -> bool {
        *self == EV_A::SSI1_RX_DMASREQ
    }
    #[doc = "Checks if the value of the field is `SSI1_RX_DMABREQ`"]
    #[inline(always)]
    pub fn is_ssi1_rx_dmabreq(&self) -> bool {
        *self == EV_A::SSI1_RX_DMABREQ
    }
    #[doc = "Checks if the value of the field is `SSI0_TX_DMASREQ`"]
    #[inline(always)]
    pub fn is_ssi0_tx_dmasreq(&self) -> bool {
        *self == EV_A::SSI0_TX_DMASREQ
    }
    #[doc = "Checks if the value of the field is `SSI0_TX_DMABREQ`"]
    #[inline(always)]
    pub fn is_ssi0_tx_dmabreq(&self) -> bool {
        *self == EV_A::SSI0_TX_DMABREQ
    }
    #[doc = "Checks if the value of the field is `SSI0_RX_DMASREQ`"]
    #[inline(always)]
    pub fn is_ssi0_rx_dmasreq(&self) -> bool {
        *self == EV_A::SSI0_RX_DMASREQ
    }
    #[doc = "Checks if the value of the field is `SSI0_RX_DMABREQ`"]
    #[inline(always)]
    pub fn is_ssi0_rx_dmabreq(&self) -> bool {
        *self == EV_A::SSI0_RX_DMABREQ
    }
    #[doc = "Checks if the value of the field is `DMA_DONE_COMB`"]
    #[inline(always)]
    pub fn is_dma_done_comb(&self) -> bool {
        *self == EV_A::DMA_DONE_COMB
    }
    #[doc = "Checks if the value of the field is `DMA_ERR`"]
    #[inline(always)]
    pub fn is_dma_err(&self) -> bool {
        *self == EV_A::DMA_ERR
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
    #[doc = "Checks if the value of the field is `PKA_IRQ`"]
    #[inline(always)]
    pub fn is_pka_irq(&self) -> bool {
        *self == EV_A::PKA_IRQ
    }
    #[doc = "Checks if the value of the field is `RFC_CPE_1`"]
    #[inline(always)]
    pub fn is_rfc_cpe_1(&self) -> bool {
        *self == EV_A::RFC_CPE_1
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV1`"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == EV_A::AUX_SWEV1
    }
    #[doc = "Checks if the value of the field is `RFC_CPE_0`"]
    #[inline(always)]
    pub fn is_rfc_cpe_0(&self) -> bool {
        *self == EV_A::RFC_CPE_0
    }
    #[doc = "Checks if the value of the field is `RFC_HW_COMB`"]
    #[inline(always)]
    pub fn is_rfc_hw_comb(&self) -> bool {
        *self == EV_A::RFC_HW_COMB
    }
    #[doc = "Checks if the value of the field is `RFC_CMD_ACK`"]
    #[inline(always)]
    pub fn is_rfc_cmd_ack(&self) -> bool {
        *self == EV_A::RFC_CMD_ACK
    }
    #[doc = "Checks if the value of the field is `WDT_IRQ`"]
    #[inline(always)]
    pub fn is_wdt_irq(&self) -> bool {
        *self == EV_A::WDT_IRQ
    }
    #[doc = "Checks if the value of the field is `DMA_CH18_DONE`"]
    #[inline(always)]
    pub fn is_dma_ch18_done(&self) -> bool {
        *self == EV_A::DMA_CH18_DONE
    }
    #[doc = "Checks if the value of the field is `FLASH`"]
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == EV_A::FLASH
    }
    #[doc = "Checks if the value of the field is `DMA_CH0_DONE`"]
    #[inline(always)]
    pub fn is_dma_ch0_done(&self) -> bool {
        *self == EV_A::DMA_CH0_DONE
    }
    #[doc = "Checks if the value of the field is `GPT1B`"]
    #[inline(always)]
    pub fn is_gpt1b(&self) -> bool {
        *self == EV_A::GPT1B
    }
    #[doc = "Checks if the value of the field is `GPT1A`"]
    #[inline(always)]
    pub fn is_gpt1a(&self) -> bool {
        *self == EV_A::GPT1A
    }
    #[doc = "Checks if the value of the field is `GPT0B`"]
    #[inline(always)]
    pub fn is_gpt0b(&self) -> bool {
        *self == EV_A::GPT0B
    }
    #[doc = "Checks if the value of the field is `GPT0A`"]
    #[inline(always)]
    pub fn is_gpt0a(&self) -> bool {
        *self == EV_A::GPT0A
    }
    #[doc = "Checks if the value of the field is `GPT3B`"]
    #[inline(always)]
    pub fn is_gpt3b(&self) -> bool {
        *self == EV_A::GPT3B
    }
    #[doc = "Checks if the value of the field is `GPT3A`"]
    #[inline(always)]
    pub fn is_gpt3a(&self) -> bool {
        *self == EV_A::GPT3A
    }
    #[doc = "Checks if the value of the field is `GPT2B`"]
    #[inline(always)]
    pub fn is_gpt2b(&self) -> bool {
        *self == EV_A::GPT2B
    }
    #[doc = "Checks if the value of the field is `GPT2A`"]
    #[inline(always)]
    pub fn is_gpt2a(&self) -> bool {
        *self == EV_A::GPT2A
    }
    #[doc = "Checks if the value of the field is `AUX_COMB`"]
    #[inline(always)]
    pub fn is_aux_comb(&self) -> bool {
        *self == EV_A::AUX_COMB
    }
    #[doc = "Checks if the value of the field is `AON_AUX_SWEV0`"]
    #[inline(always)]
    pub fn is_aon_aux_swev0(&self) -> bool {
        *self == EV_A::AON_AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `I2C_IRQ`"]
    #[inline(always)]
    pub fn is_i2c_irq(&self) -> bool {
        *self == EV_A::I2C_IRQ
    }
    #[doc = "Checks if the value of the field is `I2S_IRQ`"]
    #[inline(always)]
    pub fn is_i2s_irq(&self) -> bool {
        *self == EV_A::I2S_IRQ
    }
    #[doc = "Checks if the value of the field is `AON_RTC_COMB`"]
    #[inline(always)]
    pub fn is_aon_rtc_comb(&self) -> bool {
        *self == EV_A::AON_RTC_COMB
    }
    #[doc = "Checks if the value of the field is `OSC_COMB`"]
    #[inline(always)]
    pub fn is_osc_comb(&self) -> bool {
        *self == EV_A::OSC_COMB
    }
    #[doc = "Checks if the value of the field is `BATMON_COMB`"]
    #[inline(always)]
    pub fn is_batmon_comb(&self) -> bool {
        *self == EV_A::BATMON_COMB
    }
    #[doc = "Checks if the value of the field is `AON_GPIO_EDGE`"]
    #[inline(always)]
    pub fn is_aon_gpio_edge(&self) -> bool {
        *self == EV_A::AON_GPIO_EDGE
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
    #[doc = "CPU halted"]
    #[inline(always)]
    pub fn cpu_halted(self) -> &'a mut W {
        self.variant(EV_A::CPU_HALTED)
    }
    #[doc = "RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN"]
    #[inline(always)]
    pub fn aon_rtc_upd(self) -> &'a mut W {
        self.variant(EV_A::AON_RTC_UPD)
    }
    #[doc = "DMA burst request event from AUX, configured by AUX_EVCTL:DMACTL"]
    #[inline(always)]
    pub fn aux_dmabreq(self) -> &'a mut W {
        self.variant(EV_A::AUX_DMABREQ)
    }
    #[doc = "DMA single request event from AUX, configured by AUX_EVCTL:DMACTL"]
    #[inline(always)]
    pub fn aux_dmasreq(self) -> &'a mut W {
        self.variant(EV_A::AUX_DMASREQ)
    }
    #[doc = "DMA sofware trigger from AUX, triggered by AUX_EVCTL:DMASWREQ.START"]
    #[inline(always)]
    pub fn aux_sw_dmabreq(self) -> &'a mut W {
        self.variant(EV_A::AUX_SW_DMABREQ)
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
    #[doc = "TRNG Interrupt event, controlled by TRNG:IRQEN.EN"]
    #[inline(always)]
    pub fn trng_irq(self) -> &'a mut W {
        self.variant(EV_A::TRNG_IRQ)
    }
    #[doc = "Software event 3, triggered by SWEV.SWEV3"]
    #[inline(always)]
    pub fn swev3(self) -> &'a mut W {
        self.variant(EV_A::SWEV3)
    }
    #[doc = "Software event 2, triggered by SWEV.SWEV2"]
    #[inline(always)]
    pub fn swev2(self) -> &'a mut W {
        self.variant(EV_A::SWEV2)
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
    #[doc = "Watchdog non maskable interrupt event, controlled by WDT:CTL.INTTYPE"]
    #[inline(always)]
    pub fn wdt_nmi(self) -> &'a mut W {
        self.variant(EV_A::WDT_NMI)
    }
    #[doc = "CRYPTO DMA input done event, the correspondingg flag is CRYPTO:IRQSTAT.DMA_IN_DONE. Controlled by CRYPTO:IRQEN.DMA_IN_DONE"]
    #[inline(always)]
    pub fn crypto_dma_done_irq(self) -> &'a mut W {
        self.variant(EV_A::CRYPTO_DMA_DONE_IRQ)
    }
    #[doc = "CRYPTO result available interupt event, the corresponding flag is found here CRYPTO:IRQSTAT.RESULT_AVAIL. Controlled by CRYPTO:IRQSTAT.RESULT_AVAIL"]
    #[inline(always)]
    pub fn crypto_result_avail_irq(self) -> &'a mut W {
        self.variant(EV_A::CRYPTO_RESULT_AVAIL_IRQ)
    }
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT7 wil be routed here."]
    #[inline(always)]
    pub fn port_event7(self) -> &'a mut W {
        self.variant(EV_A::PORT_EVENT7)
    }
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT6 wil be routed here."]
    #[inline(always)]
    pub fn port_event6(self) -> &'a mut W {
        self.variant(EV_A::PORT_EVENT6)
    }
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT4 wil be routed here."]
    #[inline(always)]
    pub fn port_event5(self) -> &'a mut W {
        self.variant(EV_A::PORT_EVENT5)
    }
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT4 wil be routed here."]
    #[inline(always)]
    pub fn port_event4(self) -> &'a mut W {
        self.variant(EV_A::PORT_EVENT4)
    }
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT3 wil be routed here."]
    #[inline(always)]
    pub fn port_event3(self) -> &'a mut W {
        self.variant(EV_A::PORT_EVENT3)
    }
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT2 wil be routed here."]
    #[inline(always)]
    pub fn port_event2(self) -> &'a mut W {
        self.variant(EV_A::PORT_EVENT2)
    }
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT1 wil be routed here."]
    #[inline(always)]
    pub fn port_event1(self) -> &'a mut W {
        self.variant(EV_A::PORT_EVENT1)
    }
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT0 wil be routed here."]
    #[inline(always)]
    pub fn port_event0(self) -> &'a mut W {
        self.variant(EV_A::PORT_EVENT0)
    }
    #[doc = "GPT3B DMA trigger event. Configured by GPT3:DMAEV"]
    #[inline(always)]
    pub fn gpt3b_dmabreq(self) -> &'a mut W {
        self.variant(EV_A::GPT3B_DMABREQ)
    }
    #[doc = "GPT3A DMA trigger event. Configured by GPT3:DMAEV"]
    #[inline(always)]
    pub fn gpt3a_dmabreq(self) -> &'a mut W {
        self.variant(EV_A::GPT3A_DMABREQ)
    }
    #[doc = "GPT2B DMA trigger event. Configured by GPT2:DMAEV"]
    #[inline(always)]
    pub fn gpt2b_dmabreq(self) -> &'a mut W {
        self.variant(EV_A::GPT2B_DMABREQ)
    }
    #[doc = "GPT2A DMA trigger event. Configured by GPT2:DMAEV"]
    #[inline(always)]
    pub fn gpt2a_dmabreq(self) -> &'a mut W {
        self.variant(EV_A::GPT2A_DMABREQ)
    }
    #[doc = "GPT1B DMA trigger event. Configured by GPT1:DMAEV"]
    #[inline(always)]
    pub fn gpt1b_dmabreq(self) -> &'a mut W {
        self.variant(EV_A::GPT1B_DMABREQ)
    }
    #[doc = "GPT1A DMA trigger event. Configured by GPT1:DMAEV"]
    #[inline(always)]
    pub fn gpt1a_dmabreq(self) -> &'a mut W {
        self.variant(EV_A::GPT1A_DMABREQ)
    }
    #[doc = "GPT0B DMA trigger event. Configured by GPT0:DMAEV"]
    #[inline(always)]
    pub fn gpt0b_dmabreq(self) -> &'a mut W {
        self.variant(EV_A::GPT0B_DMABREQ)
    }
    #[doc = "GPT0A DMA trigger event. Configured by GPT0:DMAEV"]
    #[inline(always)]
    pub fn gpt0a_dmabreq(self) -> &'a mut W {
        self.variant(EV_A::GPT0A_DMABREQ)
    }
    #[doc = "GPT3B compare event. Configured by GPT3:TBMR.TCACT"]
    #[inline(always)]
    pub fn gpt3b_cmp(self) -> &'a mut W {
        self.variant(EV_A::GPT3B_CMP)
    }
    #[doc = "GPT3A compare event. Configured by GPT3:TAMR.TCACT"]
    #[inline(always)]
    pub fn gpt3a_cmp(self) -> &'a mut W {
        self.variant(EV_A::GPT3A_CMP)
    }
    #[doc = "GPT2B compare event. Configured by GPT2:TBMR.TCACT"]
    #[inline(always)]
    pub fn gpt2b_cmp(self) -> &'a mut W {
        self.variant(EV_A::GPT2B_CMP)
    }
    #[doc = "GPT2A compare event. Configured by GPT2:TAMR.TCACT"]
    #[inline(always)]
    pub fn gpt2a_cmp(self) -> &'a mut W {
        self.variant(EV_A::GPT2A_CMP)
    }
    #[doc = "GPT1B compare event. Configured by GPT1:TBMR.TCACT"]
    #[inline(always)]
    pub fn gpt1b_cmp(self) -> &'a mut W {
        self.variant(EV_A::GPT1B_CMP)
    }
    #[doc = "GPT1A compare event. Configured by GPT1:TAMR.TCACT"]
    #[inline(always)]
    pub fn gpt1a_cmp(self) -> &'a mut W {
        self.variant(EV_A::GPT1A_CMP)
    }
    #[doc = "GPT0B compare event. Configured by GPT0:TBMR.TCACT"]
    #[inline(always)]
    pub fn gpt0b_cmp(self) -> &'a mut W {
        self.variant(EV_A::GPT0B_CMP)
    }
    #[doc = "GPT0A compare event. Configured by GPT0:TAMR.TCACT"]
    #[inline(always)]
    pub fn gpt0a_cmp(self) -> &'a mut W {
        self.variant(EV_A::GPT0A_CMP)
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
    #[doc = "UART1 TX DMA single request, controlled by UART1:DMACTL.TXDMAE"]
    #[inline(always)]
    pub fn uart1_tx_dmasreq(self) -> &'a mut W {
        self.variant(EV_A::UART1_TX_DMASREQ)
    }
    #[doc = "UART1 TX DMA burst request, controlled by UART1:DMACTL.TXDMAE"]
    #[inline(always)]
    pub fn uart1_tx_dmabreq(self) -> &'a mut W {
        self.variant(EV_A::UART1_TX_DMABREQ)
    }
    #[doc = "UART1 RX DMA single request, controlled by UART1:DMACTL.RXDMAE"]
    #[inline(always)]
    pub fn uart1_rx_dmasreq(self) -> &'a mut W {
        self.variant(EV_A::UART1_RX_DMASREQ)
    }
    #[doc = "UART1 RX DMA burst request, controlled by UART1:DMACTL.RXDMAE"]
    #[inline(always)]
    pub fn uart1_rx_dmabreq(self) -> &'a mut W {
        self.variant(EV_A::UART1_RX_DMABREQ)
    }
    #[doc = "UART0 TX DMA single request, controlled by UART0:DMACTL.TXDMAE"]
    #[inline(always)]
    pub fn uart0_tx_dmasreq(self) -> &'a mut W {
        self.variant(EV_A::UART0_TX_DMASREQ)
    }
    #[doc = "UART0 TX DMA burst request, controlled by UART0:DMACTL.TXDMAE"]
    #[inline(always)]
    pub fn uart0_tx_dmabreq(self) -> &'a mut W {
        self.variant(EV_A::UART0_TX_DMABREQ)
    }
    #[doc = "UART0 RX DMA single request, controlled by UART0:DMACTL.RXDMAE"]
    #[inline(always)]
    pub fn uart0_rx_dmasreq(self) -> &'a mut W {
        self.variant(EV_A::UART0_RX_DMASREQ)
    }
    #[doc = "UART0 RX DMA burst request, controlled by UART0:DMACTL.RXDMAE"]
    #[inline(always)]
    pub fn uart0_rx_dmabreq(self) -> &'a mut W {
        self.variant(EV_A::UART0_RX_DMABREQ)
    }
    #[doc = "SSI1 TX DMA single request, controlled by SSI0:DMACR.TXDMAE"]
    #[inline(always)]
    pub fn ssi1_tx_dmasreq(self) -> &'a mut W {
        self.variant(EV_A::SSI1_TX_DMASREQ)
    }
    #[doc = "SSI1 TX DMA burst request , controlled by SSI0:DMACR.TXDMAE"]
    #[inline(always)]
    pub fn ssi1_tx_dmabreq(self) -> &'a mut W {
        self.variant(EV_A::SSI1_TX_DMABREQ)
    }
    #[doc = "SSI1 RX DMA single request, controlled by SSI0:DMACR.RXDMAE"]
    #[inline(always)]
    pub fn ssi1_rx_dmasreq(self) -> &'a mut W {
        self.variant(EV_A::SSI1_RX_DMASREQ)
    }
    #[doc = "SSI1 RX DMA burst request , controlled by SSI0:DMACR.RXDMAE"]
    #[inline(always)]
    pub fn ssi1_rx_dmabreq(self) -> &'a mut W {
        self.variant(EV_A::SSI1_RX_DMABREQ)
    }
    #[doc = "SSI0 TX DMA single request, controlled by SSI0:DMACR.TXDMAE"]
    #[inline(always)]
    pub fn ssi0_tx_dmasreq(self) -> &'a mut W {
        self.variant(EV_A::SSI0_TX_DMASREQ)
    }
    #[doc = "SSI0 TX DMA burst request , controlled by SSI0:DMACR.TXDMAE"]
    #[inline(always)]
    pub fn ssi0_tx_dmabreq(self) -> &'a mut W {
        self.variant(EV_A::SSI0_TX_DMABREQ)
    }
    #[doc = "SSI0 RX DMA single request, controlled by SSI0:DMACR.RXDMAE"]
    #[inline(always)]
    pub fn ssi0_rx_dmasreq(self) -> &'a mut W {
        self.variant(EV_A::SSI0_RX_DMASREQ)
    }
    #[doc = "SSI0 RX DMA burst request , controlled by SSI0:DMACR.RXDMAE"]
    #[inline(always)]
    pub fn ssi0_rx_dmabreq(self) -> &'a mut W {
        self.variant(EV_A::SSI0_RX_DMABREQ)
    }
    #[doc = "Combined DMA done, corresponding flags are here UDMA0:REQDONE"]
    #[inline(always)]
    pub fn dma_done_comb(self) -> &'a mut W {
        self.variant(EV_A::DMA_DONE_COMB)
    }
    #[doc = "DMA bus error, corresponds to UDMA0:ERROR.STATUS"]
    #[inline(always)]
    pub fn dma_err(self) -> &'a mut W {
        self.variant(EV_A::DMA_ERR)
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
    #[doc = "PKA Interrupt event"]
    #[inline(always)]
    pub fn pka_irq(self) -> &'a mut W {
        self.variant(EV_A::PKA_IRQ)
    }
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE1 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_1 event"]
    #[inline(always)]
    pub fn rfc_cpe_1(self) -> &'a mut W {
        self.variant(EV_A::RFC_CPE_1)
    }
    #[doc = "AUX software event 1, triggered by AUX_EVCTL:SWEVSET.SWEV1, also available as AUX_EVENT2 AON wake up event. MCU domain wakeup control AON_EVENT:MCUWUSEL"]
    #[inline(always)]
    pub fn aux_swev1(self) -> &'a mut W {
        self.variant(EV_A::AUX_SWEV1)
    }
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE0 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_0 event"]
    #[inline(always)]
    pub fn rfc_cpe_0(self) -> &'a mut W {
        self.variant(EV_A::RFC_CPE_0)
    }
    #[doc = "Combined RFC hardware interrupt, corresponding flag is here RFC_DBELL:RFHWIFG"]
    #[inline(always)]
    pub fn rfc_hw_comb(self) -> &'a mut W {
        self.variant(EV_A::RFC_HW_COMB)
    }
    #[doc = "RFC Doorbell Command Acknowledgement Interrupt, equvialent to RFC_DBELL:RFACKIFG.ACKFLAG"]
    #[inline(always)]
    pub fn rfc_cmd_ack(self) -> &'a mut W {
        self.variant(EV_A::RFC_CMD_ACK)
    }
    #[doc = "Watchdog interrupt event, controlled by WDT:CTL.INTEN"]
    #[inline(always)]
    pub fn wdt_irq(self) -> &'a mut W {
        self.variant(EV_A::WDT_IRQ)
    }
    #[doc = "DMA done for software tiggered UDMA channel 18, see UDMA0:SOFTREQ"]
    #[inline(always)]
    pub fn dma_ch18_done(self) -> &'a mut W {
        self.variant(EV_A::DMA_CH18_DONE)
    }
    #[doc = "FLASH controller error event, the status flags are FLASH:FEDACSTAT.FSM_DONE and FLASH:FEDACSTAT.RVF_INT"]
    #[inline(always)]
    pub fn flash(self) -> &'a mut W {
        self.variant(EV_A::FLASH)
    }
    #[doc = "DMA done for software tiggered UDMA channel 0, see UDMA0:SOFTREQ"]
    #[inline(always)]
    pub fn dma_ch0_done(self) -> &'a mut W {
        self.variant(EV_A::DMA_CH0_DONE)
    }
    #[doc = "GPT1B interrupt event, controlled by GPT1:TBMR"]
    #[inline(always)]
    pub fn gpt1b(self) -> &'a mut W {
        self.variant(EV_A::GPT1B)
    }
    #[doc = "GPT1A interrupt event, controlled by GPT1:TAMR"]
    #[inline(always)]
    pub fn gpt1a(self) -> &'a mut W {
        self.variant(EV_A::GPT1A)
    }
    #[doc = "GPT0B interrupt event, controlled by GPT0:TBMR"]
    #[inline(always)]
    pub fn gpt0b(self) -> &'a mut W {
        self.variant(EV_A::GPT0B)
    }
    #[doc = "GPT0A interrupt event, controlled by GPT0:TAMR"]
    #[inline(always)]
    pub fn gpt0a(self) -> &'a mut W {
        self.variant(EV_A::GPT0A)
    }
    #[doc = "GPT3B interrupt event, controlled by GPT3:TBMR"]
    #[inline(always)]
    pub fn gpt3b(self) -> &'a mut W {
        self.variant(EV_A::GPT3B)
    }
    #[doc = "GPT3A interrupt event, controlled by GPT3:TAMR"]
    #[inline(always)]
    pub fn gpt3a(self) -> &'a mut W {
        self.variant(EV_A::GPT3A)
    }
    #[doc = "GPT2B interrupt event, controlled by GPT2:TBMR"]
    #[inline(always)]
    pub fn gpt2b(self) -> &'a mut W {
        self.variant(EV_A::GPT2B)
    }
    #[doc = "GPT2A interrupt event, controlled by GPT2:TAMR"]
    #[inline(always)]
    pub fn gpt2a(self) -> &'a mut W {
        self.variant(EV_A::GPT2A)
    }
    #[doc = "AUX combined event, the corresponding flag register is here AUX_EVCTL:EVTOMCUFLAGS"]
    #[inline(always)]
    pub fn aux_comb(self) -> &'a mut W {
        self.variant(EV_A::AUX_COMB)
    }
    #[doc = "AUX Software event 0, AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn aon_aux_swev0(self) -> &'a mut W {
        self.variant(EV_A::AON_AUX_SWEV0)
    }
    #[doc = "Interrupt event from I2C"]
    #[inline(always)]
    pub fn i2c_irq(self) -> &'a mut W {
        self.variant(EV_A::I2C_IRQ)
    }
    #[doc = "Interrupt event from I2S"]
    #[inline(always)]
    pub fn i2s_irq(self) -> &'a mut W {
        self.variant(EV_A::I2S_IRQ)
    }
    #[doc = "Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting"]
    #[inline(always)]
    pub fn aon_rtc_comb(self) -> &'a mut W {
        self.variant(EV_A::AON_RTC_COMB)
    }
    #[doc = "Combined event from Oscillator control"]
    #[inline(always)]
    pub fn osc_comb(self) -> &'a mut W {
        self.variant(EV_A::OSC_COMB)
    }
    #[doc = "Combined event from battery monitor"]
    #[inline(always)]
    pub fn batmon_comb(self) -> &'a mut W {
        self.variant(EV_A::BATMON_COMB)
    }
    #[doc = "Edge detect event from IOC. Configureded by the IOC:IOCFGn.EDGE_IRQ_EN and IOC:IOCFGn.EDGE_DET settings"]
    #[inline(always)]
    pub fn aon_gpio_edge(self) -> &'a mut W {
        self.variant(EV_A::AON_GPIO_EDGE)
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
