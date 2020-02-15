#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration"]
    pub cfg: CFG,
    #[doc = "0x04 - Timer A Mode"]
    pub tamr: TAMR,
    #[doc = "0x08 - Timer B Mode"]
    pub tbmr: TBMR,
    #[doc = "0x0c - Control"]
    pub ctl: CTL,
    #[doc = "0x10 - Synch Register"]
    pub sync: SYNC,
    _reserved5: [u8; 4usize],
    #[doc = "0x18 - Interrupt Mask This register is used to enable the interrupts. Associated registers: RIS, MIS, ICLR"]
    pub imr: IMR,
    #[doc = "0x1c - Raw Interrupt Status Associated registers: IMR, MIS, ICLR"]
    pub ris: RIS,
    #[doc = "0x20 - Masked Interrupt Status Values are result of bitwise AND operation between RIS and IMR Assosciated clear register: ICLR"]
    pub mis: MIS,
    #[doc = "0x24 - Interrupt Clear This register is used to clear status bits in the RIS and MIS registers"]
    pub iclr: ICLR,
    #[doc = "0x28 - Timer A Interval Load Register"]
    pub tailr: TAILR,
    #[doc = "0x2c - Timer B Interval Load Register"]
    pub tbilr: TBILR,
    #[doc = "0x30 - Timer A Match Register Interrupts can be generated when the timer value is equal to the value in this register in one-shot or periodic mode. In Edge-Count mode, this register along with TAILR, determines how many edge events are counted. The total number of edge events counted is equal to the value in TAILR minus this value. Note that in edge-count mode, when executing an up-count, the value of TAPR and TAILR must be greater than the value of TAPMR and this register. In PWM mode, this value along with TAILR, determines the duty cycle of the output PWM signal. When a 16/32-bit GPT is configured to one of the 32-bit modes, TAMATCHR appears as a 32-bit register. (The upper 16-bits correspond to the contents TBMATCHR). In a 16-bit mode, the upper 16 bits of this register read as 0s and have no effect on the state of TBMATCHR. Note : This register is updated internally (takes effect) based on TAMR.TAMRSU"]
    pub tamatchr: TAMATCHR,
    #[doc = "0x34 - Timer B Match Register When a GPT is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of TAMATCHR. Reads from this register return the current match value of Timer B and writes are ignored. In a 16-bit mode, bits 15:0 are used for the match value. Bits 31:16 are reserved in both cases. Note : This register is updated internally (takes effect) based on TBMR.TBMRSU"]
    pub tbmatchr: TBMATCHR,
    #[doc = "0x38 - Timer A Pre-scale This register allows software to extend the range of the timers when they are used individually. When in one-shot or periodic down count modes, this register acts as a true prescaler for the timer counter. When acting as a true prescaler, the prescaler counts down to 0 before the value in TAR and TAV registers are incremented. In all other individual/split modes, this register is a linear extension of the upper range of the timer counter, holding bits 23:16 in the 16-bit modes of the 16/32-bit GPT."]
    pub tapr: TAPR,
    #[doc = "0x3c - Timer B Pre-scale This register allows software to extend the range of the timers when they are used individually. When in one-shot or periodic down count modes, this register acts as a true prescaler for the timer counter. When acting as a true prescaler, the prescaler counts down to 0 before the value in TBR and TBV registers are incremented. In all other individual/split modes, this register is a linear extension of the upper range of the timer counter, holding bits 23:16 in the 16-bit modes of the 16/32-bit GPT."]
    pub tbpr: TBPR,
    #[doc = "0x40 - Timer A Pre-scale Match This register allows software to extend the range of the TAMATCHR when used individually."]
    pub tapmr: TAPMR,
    #[doc = "0x44 - Timer B Pre-scale Match This register allows software to extend the range of the TBMATCHR when used individually."]
    pub tbpmr: TBPMR,
    #[doc = "0x48 - Timer A Register This register shows the current value of the Timer A counter in all cases except for Input Edge Count and Time modes. In the Input Edge Count mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place. When a GPT is configured to one of the 32-bit modes, this register appears as a 32-bit register (the upper 16-bits correspond to the contents of the Timer B (TBR) register). In the16-bit Input Edge Count, Input Edge Time, and PWM modes, bits 15:0 contain the value of the counter and bits 23:16 contain the value of the prescaler, which is the upper 8 bits of the count. Bits 31:24 always read as 0. To read the value of the prescaler in 16-bit One-Shot and Periodic modes, read bits \\[23:16\\]
in the TAV register. To read the value of the prescalar in periodic snapshot mode, read the Timer A Prescale Snapshot (TAPS) register."]
    pub tar: TAR,
    #[doc = "0x4c - Timer B Register This register shows the current value of the Timer B counter in all cases except for Input Edge Count and Time modes. In the Input Edge Count mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place. When a GPTM is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of the TAR register. Reads from this register return the current value of Timer B. In a 16-bit mode, bits 15:0 contain the value of the counter and bits 23:16 contain the value of the prescaler in Input Edge Count, Input Edge Time, and PWM modes, which is the upper 8 bits of the count. Bits 31:24 always read as 0. To read the value of the prescaler in 16-bit One-Shot and Periodic modes, read bits \\[23:16\\]
in the TBV register. To read the value of the prescalar in periodic snapshot mode, read the Timer B Prescale Snapshot (TBPS) register."]
    pub tbr: TBR,
    #[doc = "0x50 - Timer A Value When read, this register shows the current, free-running value of Timer A in all modes. Softwarecan use this value to determine the time elapsed between an interrupt and the ISR entry when using the snapshot feature with the periodic operating mode. When written, the value written into this register is loaded into the TAR register on the next clock cycle. When a 16/32-bit GPTM is configured to one of the 32-bit modes, this register appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B Value (TBV) register). In a 16-bit mode, bits 15:0 contain the value of the counter and bits 23:16 contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in Input Edge Count, Input Edge Time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in 23:16 is a true prescaler, meaning bits 23:16 count down before decrementing the value in bits 15:0. The prescaler in bits 31:24 always reads as 0."]
    pub tav: TAV,
    #[doc = "0x54 - Timer B Value When read, this register shows the current, free-running value of Timer B in all modes. Software can use this value to determine the time elapsed between an interrupt and the ISR entry. When written, the value written into this register is loaded into the TBR register on the next clock cycle. When a 16/32-bit GPTM is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of the TAV register. Reads from this register return the current free-running value of Timer B. In a 16-bit mode, bits 15:0 contain the value of the counter and bits 23:16 contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in Input Edge Count, Input Edge Time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in 23:16 is a true prescaler, meaning bits 23:16 count down before decrementing the value in bits 15:0. The prescaler in bits 31:24 always reads as 0."]
    pub tbv: TBV,
    _reserved21: [u8; 4usize],
    #[doc = "0x5c - Timer A Pre-scale Snap-shot Based on the value in the register field TAMR.TAILD, this register is updated with the value from TAPR register either on the next cycle or on the next timeout. This register shows the current value of the Timer A pre-scaler in the 16-bit mode."]
    pub taps: TAPS,
    #[doc = "0x60 - Timer B Pre-scale Snap-shot Based on the value in the register field TBMR.TBILD, this register is updated with the value from TBPR register either on the next cycle or on the next timeout. This register shows the current value of the Timer B pre-scaler in the 16-bit mode."]
    pub tbps: TBPS,
    #[doc = "0x64 - Timer A Pre-scale Value This register shows the current value of the Timer A free running pre-scaler in the 16-bit mode."]
    pub tapv: TAPV,
    #[doc = "0x68 - Timer B Pre-scale Value This register shows the current value of the Timer B free running pre-scaler in the 16-bit mode."]
    pub tbpv: TBPV,
    #[doc = "0x6c - DMA Event This register allows software to enable/disable GPT DMA trigger events."]
    pub dmaev: DMAEV,
    _reserved26: [u8; 3904usize],
    #[doc = "0xfb0 - Peripheral Version This register provides information regarding the GPT version"]
    pub version: VERSION,
    #[doc = "0xfb4 - Combined CCP Output This register is used to logically AND CCP output pairs for each timer"]
    pub andccp: ANDCCP,
}
#[doc = "Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "Configuration"]
pub mod cfg;
#[doc = "Timer A Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamr](tamr) module"]
pub type TAMR = crate::Reg<u32, _TAMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMR;
#[doc = "`read()` method returns [tamr::R](tamr::R) reader structure"]
impl crate::Readable for TAMR {}
#[doc = "`write(|w| ..)` method takes [tamr::W](tamr::W) writer structure"]
impl crate::Writable for TAMR {}
#[doc = "Timer A Mode"]
pub mod tamr;
#[doc = "Timer B Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbmr](tbmr) module"]
pub type TBMR = crate::Reg<u32, _TBMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBMR;
#[doc = "`read()` method returns [tbmr::R](tbmr::R) reader structure"]
impl crate::Readable for TBMR {}
#[doc = "`write(|w| ..)` method takes [tbmr::W](tbmr::W) writer structure"]
impl crate::Writable for TBMR {}
#[doc = "Timer B Mode"]
pub mod tbmr;
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "Control"]
pub mod ctl;
#[doc = "Synch Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sync](sync) module"]
pub type SYNC = crate::Reg<u32, _SYNC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNC;
#[doc = "`read()` method returns [sync::R](sync::R) reader structure"]
impl crate::Readable for SYNC {}
#[doc = "`write(|w| ..)` method takes [sync::W](sync::W) writer structure"]
impl crate::Writable for SYNC {}
#[doc = "Synch Register"]
pub mod sync;
#[doc = "Interrupt Mask This register is used to enable the interrupts. Associated registers: RIS, MIS, ICLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](imr) module"]
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "`write(|w| ..)` method takes [imr::W](imr::W) writer structure"]
impl crate::Writable for IMR {}
#[doc = "Interrupt Mask This register is used to enable the interrupts. Associated registers: RIS, MIS, ICLR"]
pub mod imr;
#[doc = "Raw Interrupt Status Associated registers: IMR, MIS, ICLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](ris) module"]
pub type RIS = crate::Reg<u32, _RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIS;
#[doc = "`read()` method returns [ris::R](ris::R) reader structure"]
impl crate::Readable for RIS {}
#[doc = "`write(|w| ..)` method takes [ris::W](ris::W) writer structure"]
impl crate::Writable for RIS {}
#[doc = "Raw Interrupt Status Associated registers: IMR, MIS, ICLR"]
pub mod ris;
#[doc = "Masked Interrupt Status Values are result of bitwise AND operation between RIS and IMR Assosciated clear register: ICLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mis](mis) module"]
pub type MIS = crate::Reg<u32, _MIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIS;
#[doc = "`read()` method returns [mis::R](mis::R) reader structure"]
impl crate::Readable for MIS {}
#[doc = "`write(|w| ..)` method takes [mis::W](mis::W) writer structure"]
impl crate::Writable for MIS {}
#[doc = "Masked Interrupt Status Values are result of bitwise AND operation between RIS and IMR Assosciated clear register: ICLR"]
pub mod mis;
#[doc = "Interrupt Clear This register is used to clear status bits in the RIS and MIS registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iclr](iclr) module"]
pub type ICLR = crate::Reg<u32, _ICLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICLR;
#[doc = "`read()` method returns [iclr::R](iclr::R) reader structure"]
impl crate::Readable for ICLR {}
#[doc = "`write(|w| ..)` method takes [iclr::W](iclr::W) writer structure"]
impl crate::Writable for ICLR {}
#[doc = "Interrupt Clear This register is used to clear status bits in the RIS and MIS registers"]
pub mod iclr;
#[doc = "Timer A Interval Load Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tailr](tailr) module"]
pub type TAILR = crate::Reg<u32, _TAILR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAILR;
#[doc = "`read()` method returns [tailr::R](tailr::R) reader structure"]
impl crate::Readable for TAILR {}
#[doc = "`write(|w| ..)` method takes [tailr::W](tailr::W) writer structure"]
impl crate::Writable for TAILR {}
#[doc = "Timer A Interval Load Register"]
pub mod tailr;
#[doc = "Timer B Interval Load Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbilr](tbilr) module"]
pub type TBILR = crate::Reg<u32, _TBILR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBILR;
#[doc = "`read()` method returns [tbilr::R](tbilr::R) reader structure"]
impl crate::Readable for TBILR {}
#[doc = "`write(|w| ..)` method takes [tbilr::W](tbilr::W) writer structure"]
impl crate::Writable for TBILR {}
#[doc = "Timer B Interval Load Register"]
pub mod tbilr;
#[doc = "Timer A Match Register Interrupts can be generated when the timer value is equal to the value in this register in one-shot or periodic mode. In Edge-Count mode, this register along with TAILR, determines how many edge events are counted. The total number of edge events counted is equal to the value in TAILR minus this value. Note that in edge-count mode, when executing an up-count, the value of TAPR and TAILR must be greater than the value of TAPMR and this register. In PWM mode, this value along with TAILR, determines the duty cycle of the output PWM signal. When a 16/32-bit GPT is configured to one of the 32-bit modes, TAMATCHR appears as a 32-bit register. (The upper 16-bits correspond to the contents TBMATCHR). In a 16-bit mode, the upper 16 bits of this register read as 0s and have no effect on the state of TBMATCHR. Note : This register is updated internally (takes effect) based on TAMR.TAMRSU\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamatchr](tamatchr) module"]
pub type TAMATCHR = crate::Reg<u32, _TAMATCHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMATCHR;
#[doc = "`read()` method returns [tamatchr::R](tamatchr::R) reader structure"]
impl crate::Readable for TAMATCHR {}
#[doc = "`write(|w| ..)` method takes [tamatchr::W](tamatchr::W) writer structure"]
impl crate::Writable for TAMATCHR {}
#[doc = "Timer A Match Register Interrupts can be generated when the timer value is equal to the value in this register in one-shot or periodic mode. In Edge-Count mode, this register along with TAILR, determines how many edge events are counted. The total number of edge events counted is equal to the value in TAILR minus this value. Note that in edge-count mode, when executing an up-count, the value of TAPR and TAILR must be greater than the value of TAPMR and this register. In PWM mode, this value along with TAILR, determines the duty cycle of the output PWM signal. When a 16/32-bit GPT is configured to one of the 32-bit modes, TAMATCHR appears as a 32-bit register. (The upper 16-bits correspond to the contents TBMATCHR). In a 16-bit mode, the upper 16 bits of this register read as 0s and have no effect on the state of TBMATCHR. Note : This register is updated internally (takes effect) based on TAMR.TAMRSU"]
pub mod tamatchr;
#[doc = "Timer B Match Register When a GPT is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of TAMATCHR. Reads from this register return the current match value of Timer B and writes are ignored. In a 16-bit mode, bits 15:0 are used for the match value. Bits 31:16 are reserved in both cases. Note : This register is updated internally (takes effect) based on TBMR.TBMRSU\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbmatchr](tbmatchr) module"]
pub type TBMATCHR = crate::Reg<u32, _TBMATCHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBMATCHR;
#[doc = "`read()` method returns [tbmatchr::R](tbmatchr::R) reader structure"]
impl crate::Readable for TBMATCHR {}
#[doc = "`write(|w| ..)` method takes [tbmatchr::W](tbmatchr::W) writer structure"]
impl crate::Writable for TBMATCHR {}
#[doc = "Timer B Match Register When a GPT is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of TAMATCHR. Reads from this register return the current match value of Timer B and writes are ignored. In a 16-bit mode, bits 15:0 are used for the match value. Bits 31:16 are reserved in both cases. Note : This register is updated internally (takes effect) based on TBMR.TBMRSU"]
pub mod tbmatchr;
#[doc = "Timer A Pre-scale This register allows software to extend the range of the timers when they are used individually. When in one-shot or periodic down count modes, this register acts as a true prescaler for the timer counter. When acting as a true prescaler, the prescaler counts down to 0 before the value in TAR and TAV registers are incremented. In all other individual/split modes, this register is a linear extension of the upper range of the timer counter, holding bits 23:16 in the 16-bit modes of the 16/32-bit GPT.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tapr](tapr) module"]
pub type TAPR = crate::Reg<u32, _TAPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAPR;
#[doc = "`read()` method returns [tapr::R](tapr::R) reader structure"]
impl crate::Readable for TAPR {}
#[doc = "`write(|w| ..)` method takes [tapr::W](tapr::W) writer structure"]
impl crate::Writable for TAPR {}
#[doc = "Timer A Pre-scale This register allows software to extend the range of the timers when they are used individually. When in one-shot or periodic down count modes, this register acts as a true prescaler for the timer counter. When acting as a true prescaler, the prescaler counts down to 0 before the value in TAR and TAV registers are incremented. In all other individual/split modes, this register is a linear extension of the upper range of the timer counter, holding bits 23:16 in the 16-bit modes of the 16/32-bit GPT."]
pub mod tapr;
#[doc = "Timer B Pre-scale This register allows software to extend the range of the timers when they are used individually. When in one-shot or periodic down count modes, this register acts as a true prescaler for the timer counter. When acting as a true prescaler, the prescaler counts down to 0 before the value in TBR and TBV registers are incremented. In all other individual/split modes, this register is a linear extension of the upper range of the timer counter, holding bits 23:16 in the 16-bit modes of the 16/32-bit GPT.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbpr](tbpr) module"]
pub type TBPR = crate::Reg<u32, _TBPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBPR;
#[doc = "`read()` method returns [tbpr::R](tbpr::R) reader structure"]
impl crate::Readable for TBPR {}
#[doc = "`write(|w| ..)` method takes [tbpr::W](tbpr::W) writer structure"]
impl crate::Writable for TBPR {}
#[doc = "Timer B Pre-scale This register allows software to extend the range of the timers when they are used individually. When in one-shot or periodic down count modes, this register acts as a true prescaler for the timer counter. When acting as a true prescaler, the prescaler counts down to 0 before the value in TBR and TBV registers are incremented. In all other individual/split modes, this register is a linear extension of the upper range of the timer counter, holding bits 23:16 in the 16-bit modes of the 16/32-bit GPT."]
pub mod tbpr;
#[doc = "Timer A Pre-scale Match This register allows software to extend the range of the TAMATCHR when used individually.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tapmr](tapmr) module"]
pub type TAPMR = crate::Reg<u32, _TAPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAPMR;
#[doc = "`read()` method returns [tapmr::R](tapmr::R) reader structure"]
impl crate::Readable for TAPMR {}
#[doc = "`write(|w| ..)` method takes [tapmr::W](tapmr::W) writer structure"]
impl crate::Writable for TAPMR {}
#[doc = "Timer A Pre-scale Match This register allows software to extend the range of the TAMATCHR when used individually."]
pub mod tapmr;
#[doc = "Timer B Pre-scale Match This register allows software to extend the range of the TBMATCHR when used individually.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbpmr](tbpmr) module"]
pub type TBPMR = crate::Reg<u32, _TBPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBPMR;
#[doc = "`read()` method returns [tbpmr::R](tbpmr::R) reader structure"]
impl crate::Readable for TBPMR {}
#[doc = "`write(|w| ..)` method takes [tbpmr::W](tbpmr::W) writer structure"]
impl crate::Writable for TBPMR {}
#[doc = "Timer B Pre-scale Match This register allows software to extend the range of the TBMATCHR when used individually."]
pub mod tbpmr;
#[doc = "Timer A Register This register shows the current value of the Timer A counter in all cases except for Input Edge Count and Time modes. In the Input Edge Count mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place. When a GPT is configured to one of the 32-bit modes, this register appears as a 32-bit register (the upper 16-bits correspond to the contents of the Timer B (TBR) register). In the16-bit Input Edge Count, Input Edge Time, and PWM modes, bits 15:0 contain the value of the counter and bits 23:16 contain the value of the prescaler, which is the upper 8 bits of the count. Bits 31:24 always read as 0. To read the value of the prescaler in 16-bit One-Shot and Periodic modes, read bits \\[23:16\\]
in the TAV register. To read the value of the prescalar in periodic snapshot mode, read the Timer A Prescale Snapshot (TAPS) register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tar](tar) module"]
pub type TAR = crate::Reg<u32, _TAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAR;
#[doc = "`read()` method returns [tar::R](tar::R) reader structure"]
impl crate::Readable for TAR {}
#[doc = "`write(|w| ..)` method takes [tar::W](tar::W) writer structure"]
impl crate::Writable for TAR {}
#[doc = "Timer A Register This register shows the current value of the Timer A counter in all cases except for Input Edge Count and Time modes. In the Input Edge Count mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place. When a GPT is configured to one of the 32-bit modes, this register appears as a 32-bit register (the upper 16-bits correspond to the contents of the Timer B (TBR) register). In the16-bit Input Edge Count, Input Edge Time, and PWM modes, bits 15:0 contain the value of the counter and bits 23:16 contain the value of the prescaler, which is the upper 8 bits of the count. Bits 31:24 always read as 0. To read the value of the prescaler in 16-bit One-Shot and Periodic modes, read bits \\[23:16\\]
in the TAV register. To read the value of the prescalar in periodic snapshot mode, read the Timer A Prescale Snapshot (TAPS) register."]
pub mod tar;
#[doc = "Timer B Register This register shows the current value of the Timer B counter in all cases except for Input Edge Count and Time modes. In the Input Edge Count mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place. When a GPTM is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of the TAR register. Reads from this register return the current value of Timer B. In a 16-bit mode, bits 15:0 contain the value of the counter and bits 23:16 contain the value of the prescaler in Input Edge Count, Input Edge Time, and PWM modes, which is the upper 8 bits of the count. Bits 31:24 always read as 0. To read the value of the prescaler in 16-bit One-Shot and Periodic modes, read bits \\[23:16\\]
in the TBV register. To read the value of the prescalar in periodic snapshot mode, read the Timer B Prescale Snapshot (TBPS) register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbr](tbr) module"]
pub type TBR = crate::Reg<u32, _TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBR;
#[doc = "`read()` method returns [tbr::R](tbr::R) reader structure"]
impl crate::Readable for TBR {}
#[doc = "`write(|w| ..)` method takes [tbr::W](tbr::W) writer structure"]
impl crate::Writable for TBR {}
#[doc = "Timer B Register This register shows the current value of the Timer B counter in all cases except for Input Edge Count and Time modes. In the Input Edge Count mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place. When a GPTM is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of the TAR register. Reads from this register return the current value of Timer B. In a 16-bit mode, bits 15:0 contain the value of the counter and bits 23:16 contain the value of the prescaler in Input Edge Count, Input Edge Time, and PWM modes, which is the upper 8 bits of the count. Bits 31:24 always read as 0. To read the value of the prescaler in 16-bit One-Shot and Periodic modes, read bits \\[23:16\\]
in the TBV register. To read the value of the prescalar in periodic snapshot mode, read the Timer B Prescale Snapshot (TBPS) register."]
pub mod tbr;
#[doc = "Timer A Value When read, this register shows the current, free-running value of Timer A in all modes. Softwarecan use this value to determine the time elapsed between an interrupt and the ISR entry when using the snapshot feature with the periodic operating mode. When written, the value written into this register is loaded into the TAR register on the next clock cycle. When a 16/32-bit GPTM is configured to one of the 32-bit modes, this register appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B Value (TBV) register). In a 16-bit mode, bits 15:0 contain the value of the counter and bits 23:16 contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in Input Edge Count, Input Edge Time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in 23:16 is a true prescaler, meaning bits 23:16 count down before decrementing the value in bits 15:0. The prescaler in bits 31:24 always reads as 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tav](tav) module"]
pub type TAV = crate::Reg<u32, _TAV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAV;
#[doc = "`read()` method returns [tav::R](tav::R) reader structure"]
impl crate::Readable for TAV {}
#[doc = "`write(|w| ..)` method takes [tav::W](tav::W) writer structure"]
impl crate::Writable for TAV {}
#[doc = "Timer A Value When read, this register shows the current, free-running value of Timer A in all modes. Softwarecan use this value to determine the time elapsed between an interrupt and the ISR entry when using the snapshot feature with the periodic operating mode. When written, the value written into this register is loaded into the TAR register on the next clock cycle. When a 16/32-bit GPTM is configured to one of the 32-bit modes, this register appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B Value (TBV) register). In a 16-bit mode, bits 15:0 contain the value of the counter and bits 23:16 contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in Input Edge Count, Input Edge Time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in 23:16 is a true prescaler, meaning bits 23:16 count down before decrementing the value in bits 15:0. The prescaler in bits 31:24 always reads as 0."]
pub mod tav;
#[doc = "Timer B Value When read, this register shows the current, free-running value of Timer B in all modes. Software can use this value to determine the time elapsed between an interrupt and the ISR entry. When written, the value written into this register is loaded into the TBR register on the next clock cycle. When a 16/32-bit GPTM is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of the TAV register. Reads from this register return the current free-running value of Timer B. In a 16-bit mode, bits 15:0 contain the value of the counter and bits 23:16 contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in Input Edge Count, Input Edge Time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in 23:16 is a true prescaler, meaning bits 23:16 count down before decrementing the value in bits 15:0. The prescaler in bits 31:24 always reads as 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbv](tbv) module"]
pub type TBV = crate::Reg<u32, _TBV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBV;
#[doc = "`read()` method returns [tbv::R](tbv::R) reader structure"]
impl crate::Readable for TBV {}
#[doc = "`write(|w| ..)` method takes [tbv::W](tbv::W) writer structure"]
impl crate::Writable for TBV {}
#[doc = "Timer B Value When read, this register shows the current, free-running value of Timer B in all modes. Software can use this value to determine the time elapsed between an interrupt and the ISR entry. When written, the value written into this register is loaded into the TBR register on the next clock cycle. When a 16/32-bit GPTM is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of the TAV register. Reads from this register return the current free-running value of Timer B. In a 16-bit mode, bits 15:0 contain the value of the counter and bits 23:16 contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in Input Edge Count, Input Edge Time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in 23:16 is a true prescaler, meaning bits 23:16 count down before decrementing the value in bits 15:0. The prescaler in bits 31:24 always reads as 0."]
pub mod tbv;
#[doc = "Timer A Pre-scale Snap-shot Based on the value in the register field TAMR.TAILD, this register is updated with the value from TAPR register either on the next cycle or on the next timeout. This register shows the current value of the Timer A pre-scaler in the 16-bit mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [taps](taps) module"]
pub type TAPS = crate::Reg<u32, _TAPS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAPS;
#[doc = "`read()` method returns [taps::R](taps::R) reader structure"]
impl crate::Readable for TAPS {}
#[doc = "`write(|w| ..)` method takes [taps::W](taps::W) writer structure"]
impl crate::Writable for TAPS {}
#[doc = "Timer A Pre-scale Snap-shot Based on the value in the register field TAMR.TAILD, this register is updated with the value from TAPR register either on the next cycle or on the next timeout. This register shows the current value of the Timer A pre-scaler in the 16-bit mode."]
pub mod taps;
#[doc = "Timer B Pre-scale Snap-shot Based on the value in the register field TBMR.TBILD, this register is updated with the value from TBPR register either on the next cycle or on the next timeout. This register shows the current value of the Timer B pre-scaler in the 16-bit mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbps](tbps) module"]
pub type TBPS = crate::Reg<u32, _TBPS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBPS;
#[doc = "`read()` method returns [tbps::R](tbps::R) reader structure"]
impl crate::Readable for TBPS {}
#[doc = "`write(|w| ..)` method takes [tbps::W](tbps::W) writer structure"]
impl crate::Writable for TBPS {}
#[doc = "Timer B Pre-scale Snap-shot Based on the value in the register field TBMR.TBILD, this register is updated with the value from TBPR register either on the next cycle or on the next timeout. This register shows the current value of the Timer B pre-scaler in the 16-bit mode."]
pub mod tbps;
#[doc = "Timer A Pre-scale Value This register shows the current value of the Timer A free running pre-scaler in the 16-bit mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tapv](tapv) module"]
pub type TAPV = crate::Reg<u32, _TAPV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAPV;
#[doc = "`read()` method returns [tapv::R](tapv::R) reader structure"]
impl crate::Readable for TAPV {}
#[doc = "`write(|w| ..)` method takes [tapv::W](tapv::W) writer structure"]
impl crate::Writable for TAPV {}
#[doc = "Timer A Pre-scale Value This register shows the current value of the Timer A free running pre-scaler in the 16-bit mode."]
pub mod tapv;
#[doc = "Timer B Pre-scale Value This register shows the current value of the Timer B free running pre-scaler in the 16-bit mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbpv](tbpv) module"]
pub type TBPV = crate::Reg<u32, _TBPV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBPV;
#[doc = "`read()` method returns [tbpv::R](tbpv::R) reader structure"]
impl crate::Readable for TBPV {}
#[doc = "`write(|w| ..)` method takes [tbpv::W](tbpv::W) writer structure"]
impl crate::Writable for TBPV {}
#[doc = "Timer B Pre-scale Value This register shows the current value of the Timer B free running pre-scaler in the 16-bit mode."]
pub mod tbpv;
#[doc = "DMA Event This register allows software to enable/disable GPT DMA trigger events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaev](dmaev) module"]
pub type DMAEV = crate::Reg<u32, _DMAEV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAEV;
#[doc = "`read()` method returns [dmaev::R](dmaev::R) reader structure"]
impl crate::Readable for DMAEV {}
#[doc = "`write(|w| ..)` method takes [dmaev::W](dmaev::W) writer structure"]
impl crate::Writable for DMAEV {}
#[doc = "DMA Event This register allows software to enable/disable GPT DMA trigger events."]
pub mod dmaev;
#[doc = "Peripheral Version This register provides information regarding the GPT version\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [version](version) module"]
pub type VERSION = crate::Reg<u32, _VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERSION;
#[doc = "`read()` method returns [version::R](version::R) reader structure"]
impl crate::Readable for VERSION {}
#[doc = "`write(|w| ..)` method takes [version::W](version::W) writer structure"]
impl crate::Writable for VERSION {}
#[doc = "Peripheral Version This register provides information regarding the GPT version"]
pub mod version;
#[doc = "Combined CCP Output This register is used to logically AND CCP output pairs for each timer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [andccp](andccp) module"]
pub type ANDCCP = crate::Reg<u32, _ANDCCP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANDCCP;
#[doc = "`read()` method returns [andccp::R](andccp::R) reader structure"]
impl crate::Readable for ANDCCP {}
#[doc = "`write(|w| ..)` method takes [andccp::W](andccp::W) writer structure"]
impl crate::Writable for ANDCCP {}
#[doc = "Combined CCP Output This register is used to logically AND CCP output pairs for each timer"]
pub mod andccp;
