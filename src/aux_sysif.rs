#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Operational Mode Request AUX can operate in three operational modes. Each mode is associated with: - a SCE clock source or rate, given by AON_PMCTL:AUXSCECLK. This rate is termed SCE_RATE. - a system power supply state request. AUX can request powerdown (uLDO) or active (GLDO or DCDC) system power supply state. - a specific system response to an active AUX wakeup flag. The response is dependent on what operational mode is requested. uLDO power supply state offers limited current supply. AUX_SCE cannot use certain peripherals and functions such as AUX_DDI0_OSC, AUX_TDC and AUX_ANAIF ADC interface in this power supply state. Follow these rules: - It is not allowed to change a request until it has been acknowledged through OPMODEACK. - A change in mode request must happen stepwise along this sequence, the direction is irrelevant: PDA - A - LP - PDLP. Failure to follow these rules might result in unexpected behavior and must be avoided."]
    pub opmodereq: OPMODEREQ,
    #[doc = "0x04 - Operational Mode Acknowledgement AUX_SCE program must assume that the current operational mode is the one acknowledged."]
    pub opmodeack: OPMODEACK,
    #[doc = "0x08 - Programmable Wakeup 0 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU0 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
    pub progwu0cfg: PROGWU0CFG,
    #[doc = "0x0c - Programmable Wakeup 1 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU1 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
    pub progwu1cfg: PROGWU1CFG,
    #[doc = "0x10 - Programmable Wakeup 2 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU2 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
    pub progwu2cfg: PROGWU2CFG,
    #[doc = "0x14 - Programmable Wakeup 3 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU3 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
    pub progwu3cfg: PROGWU3CFG,
    #[doc = "0x18 - Software Wakeup Triggers System CPU uses these wakeup flags to perform handshaking with AUX_SCE. The wakeup flags can change the operational mode of AUX and guarantees a non-zero SCE clock rate. AUX_SCE wakeup vectors are configured in VECCFGn."]
    pub swwutrig: SWWUTRIG,
    #[doc = "0x1c - Wakeup Flags This register holds the eight AUX wakeup flags. Each flag can cause AUX operational mode to change as given in OPMODEREQ. To clear flag n you must set bit n in WUFLAGSCLR until flag n is read as 0. You must clear bit n in WUFLAGSCLR before flag n can be set again."]
    pub wuflags: WUFLAGS,
    #[doc = "0x20 - Wakeup Flags Clear This register clears AUX wakeup flags WUFLAGS. To clear programmable wakeup flags you must disable the AUX wakeup output first. After the programmable wakeup flags are cleared you must re-enable the AUX wakeup output. Write WUGATE to disable or enable the AUX wakeup output. This procedure is not required when you want to clear a software-triggered wakeup."]
    pub wuflagsclr: WUFLAGSCLR,
    #[doc = "0x24 - Wakeup Gate You must disable the AUX wakeup output: - Before you clear a programmable wakeup flag. - Before you change the value of \\[PROGWUnCFG.EN\\]
or \\[PROGWUnCFG.WU_SRC\\]. The AUX wakeup output must be re-enabled after clear operation or programmable wakeup configuration."]
    pub wugate: WUGATE,
    #[doc = "0x28 - Vector Configuration 0 AUX_SCE wakeup vector 0 configuration"]
    pub veccfg0: VECCFG0,
    #[doc = "0x2c - Vector Configuration 1 AUX_SCE wakeup vector 1 configuration"]
    pub veccfg1: VECCFG1,
    #[doc = "0x30 - Vector Configuration 2 AUX_SCE wakeup vector 2 configuration"]
    pub veccfg2: VECCFG2,
    #[doc = "0x34 - Vector Configuration 3 AUX_SCE wakeup vector 3 configuration"]
    pub veccfg3: VECCFG3,
    #[doc = "0x38 - Vector Configuration 4 AUX_SCE wakeup vector 4 configuration"]
    pub veccfg4: VECCFG4,
    #[doc = "0x3c - Vector Configuration 5 AUX_SCE wakeup vector 5 configuration"]
    pub veccfg5: VECCFG5,
    #[doc = "0x40 - Vector Configuration 6 AUX_SCE wakeup vector 6 configuration"]
    pub veccfg6: VECCFG6,
    #[doc = "0x44 - Vector Configuration 7 AUX_SCE wakeup vector 7 configuration"]
    pub veccfg7: VECCFG7,
    #[doc = "0x48 - Event Synchronization Rate Configure synchronization rate for certain events to the synchronous AUX event bus. You must select SCE rate when AUX_SCE uses the event. You must select AUX bus rate when system CPU uses the event. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by two when MCU domain is active."]
    pub evsyncrate: EVSYNCRATE,
    #[doc = "0x4c - Peripheral Operational Rate Some AUX peripherals are operated at either SCE or at AUX bus rate. You must select SCE rate when AUX_SCE uses such peripheral or an event produced by it. You must select AUX bus rate when system CPU uses such peripheral. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by 2 when MCU domain is active."]
    pub peroprate: PEROPRATE,
    #[doc = "0x50 - ADC Clock Control"]
    pub adcclkctl: ADCCLKCTL,
    #[doc = "0x54 - TDC Counter Clock Control Controls if the AUX_TDC counter clock source is enabled. TDC counter clock source is configured in DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL."]
    pub tdcclkctl: TDCCLKCTL,
    #[doc = "0x58 - TDC Reference Clock Control Controls if the AUX_TDC reference clock source is enabled. This clock is compared against the AUX_TDC counter clock. TDC reference clock source is configured in DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL."]
    pub tdcrefclkctl: TDCREFCLKCTL,
    #[doc = "0x5c - AUX_TIMER2 Clock Control Access to AUX_TIMER2 is only possible when TIMER2CLKSTAT.STAT is different from NONE."]
    pub timer2clkctl: TIMER2CLKCTL,
    #[doc = "0x60 - AUX_TIMER2 Clock Status"]
    pub timer2clkstat: TIMER2CLKSTAT,
    #[doc = "0x64 - AUX_TIMER2 Clock Switch"]
    pub timer2clkswitch: TIMER2CLKSWITCH,
    #[doc = "0x68 - AUX_TIMER2 Debug Control"]
    pub timer2dbgctl: TIMER2DBGCTL,
    _reserved27: [u8; 4usize],
    #[doc = "0x70 - Clock Shift Detection A transition in the MCU domain state causes a non-accumulative change to the SCE clock period when the AUX clock rate is derived from SCLK_MF or SCLK_LF: - A single SCE clock cycle is 6 thru 8 SCLK_HF cycles longer when MCU domain enters active state. - A single SCE clock cycle is 6 thru 8 SCLK_HF cycles shorter when MCU domain exits active state. AUX_SCE detects if such events occurred to the SCE clock during the time period between a clear of STAT and a read of STAT."]
    pub clkshiftdet: CLKSHIFTDET,
    #[doc = "0x74 - VDDR Recharge Trigger"]
    pub rechargetrig: RECHARGETRIG,
    #[doc = "0x78 - VDDR Recharge Detection Some applications can be sensitive to power noise caused by recharge of VDDR. You can detect if VDDR recharge occurs."]
    pub rechargedet: RECHARGEDET,
    #[doc = "0x7c - Real Time Counter Sub Second Increment 0 INC15_0 will replace bits 15:0 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set."]
    pub rtcsubsecinc0: RTCSUBSECINC0,
    #[doc = "0x80 - Real Time Counter Sub Second Increment 1 INC23_16 will replace bits 23:16 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set."]
    pub rtcsubsecinc1: RTCSUBSECINC1,
    #[doc = "0x84 - Real Time Counter Sub Second Increment Control"]
    pub rtcsubsecincctl: RTCSUBSECINCCTL,
    #[doc = "0x88 - Real Time Counter Second System CPU must not access this register. Instead, system CPU must access AON_RTC:SEC.VALUE directly."]
    pub rtcsec: RTCSEC,
    #[doc = "0x8c - Real Time Counter Sub-Second System CPU must not access this register. Instead, system CPU must access AON_RTC:SUBSEC.VALUE directly."]
    pub rtcsubsec: RTCSUBSEC,
    #[doc = "0x90 - AON_RTC Event Clear Request to clear events: - AON_RTC:EVFLAGS.CH2. - AON_RTC:EVFLAGS.CH2 delayed version. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY."]
    pub rtcevclr: RTCEVCLR,
    #[doc = "0x94 - AON_BATMON Battery Voltage Value Read access to AON_BATMON:BAT. System CPU must not access this register. Instead, system CPU must access AON_BATMON:BAT directly. AON_BATMON:BAT updates during VDDR recharge or active operational mode."]
    pub batmonbat: BATMONBAT,
    _reserved37: [u8; 4usize],
    #[doc = "0x9c - AON_BATMON Temperature Value Read access to AON_BATMON:TEMP. System CPU must not access this register. Instead, system CPU must access AON_BATMON:TEMP directly. AON_BATMON:TEMP updates during VDDR recharge or active operational mode."]
    pub batmontemp: BATMONTEMP,
    #[doc = "0xa0 - Timer Halt Debug register"]
    pub timerhalt: TIMERHALT,
    _reserved39: [u8; 12usize],
    #[doc = "0xb0 - AUX_TIMER2 Bridge"]
    pub timer2bridge: TIMER2BRIDGE,
    #[doc = "0xb4 - Software Power Profiler"]
    pub swpwrprof: SWPWRPROF,
}
#[doc = "Operational Mode Request AUX can operate in three operational modes. Each mode is associated with: - a SCE clock source or rate, given by AON_PMCTL:AUXSCECLK. This rate is termed SCE_RATE. - a system power supply state request. AUX can request powerdown (uLDO) or active (GLDO or DCDC) system power supply state. - a specific system response to an active AUX wakeup flag. The response is dependent on what operational mode is requested. uLDO power supply state offers limited current supply. AUX_SCE cannot use certain peripherals and functions such as AUX_DDI0_OSC, AUX_TDC and AUX_ANAIF ADC interface in this power supply state. Follow these rules: - It is not allowed to change a request until it has been acknowledged through OPMODEACK. - A change in mode request must happen stepwise along this sequence, the direction is irrelevant: PDA - A - LP - PDLP. Failure to follow these rules might result in unexpected behavior and must be avoided.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opmodereq](opmodereq) module"]
pub type OPMODEREQ = crate::Reg<u32, _OPMODEREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPMODEREQ;
#[doc = "`read()` method returns [opmodereq::R](opmodereq::R) reader structure"]
impl crate::Readable for OPMODEREQ {}
#[doc = "`write(|w| ..)` method takes [opmodereq::W](opmodereq::W) writer structure"]
impl crate::Writable for OPMODEREQ {}
#[doc = "Operational Mode Request AUX can operate in three operational modes. Each mode is associated with: - a SCE clock source or rate, given by AON_PMCTL:AUXSCECLK. This rate is termed SCE_RATE. - a system power supply state request. AUX can request powerdown (uLDO) or active (GLDO or DCDC) system power supply state. - a specific system response to an active AUX wakeup flag. The response is dependent on what operational mode is requested. uLDO power supply state offers limited current supply. AUX_SCE cannot use certain peripherals and functions such as AUX_DDI0_OSC, AUX_TDC and AUX_ANAIF ADC interface in this power supply state. Follow these rules: - It is not allowed to change a request until it has been acknowledged through OPMODEACK. - A change in mode request must happen stepwise along this sequence, the direction is irrelevant: PDA - A - LP - PDLP. Failure to follow these rules might result in unexpected behavior and must be avoided."]
pub mod opmodereq;
#[doc = "Operational Mode Acknowledgement AUX_SCE program must assume that the current operational mode is the one acknowledged.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opmodeack](opmodeack) module"]
pub type OPMODEACK = crate::Reg<u32, _OPMODEACK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPMODEACK;
#[doc = "`read()` method returns [opmodeack::R](opmodeack::R) reader structure"]
impl crate::Readable for OPMODEACK {}
#[doc = "`write(|w| ..)` method takes [opmodeack::W](opmodeack::W) writer structure"]
impl crate::Writable for OPMODEACK {}
#[doc = "Operational Mode Acknowledgement AUX_SCE program must assume that the current operational mode is the one acknowledged."]
pub mod opmodeack;
#[doc = "Programmable Wakeup 0 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU0 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [progwu0cfg](progwu0cfg) module"]
pub type PROGWU0CFG = crate::Reg<u32, _PROGWU0CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROGWU0CFG;
#[doc = "`read()` method returns [progwu0cfg::R](progwu0cfg::R) reader structure"]
impl crate::Readable for PROGWU0CFG {}
#[doc = "`write(|w| ..)` method takes [progwu0cfg::W](progwu0cfg::W) writer structure"]
impl crate::Writable for PROGWU0CFG {}
#[doc = "Programmable Wakeup 0 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU0 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
pub mod progwu0cfg;
#[doc = "Programmable Wakeup 1 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU1 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [progwu1cfg](progwu1cfg) module"]
pub type PROGWU1CFG = crate::Reg<u32, _PROGWU1CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROGWU1CFG;
#[doc = "`read()` method returns [progwu1cfg::R](progwu1cfg::R) reader structure"]
impl crate::Readable for PROGWU1CFG {}
#[doc = "`write(|w| ..)` method takes [progwu1cfg::W](progwu1cfg::W) writer structure"]
impl crate::Writable for PROGWU1CFG {}
#[doc = "Programmable Wakeup 1 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU1 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
pub mod progwu1cfg;
#[doc = "Programmable Wakeup 2 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU2 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [progwu2cfg](progwu2cfg) module"]
pub type PROGWU2CFG = crate::Reg<u32, _PROGWU2CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROGWU2CFG;
#[doc = "`read()` method returns [progwu2cfg::R](progwu2cfg::R) reader structure"]
impl crate::Readable for PROGWU2CFG {}
#[doc = "`write(|w| ..)` method takes [progwu2cfg::W](progwu2cfg::W) writer structure"]
impl crate::Writable for PROGWU2CFG {}
#[doc = "Programmable Wakeup 2 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU2 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
pub mod progwu2cfg;
#[doc = "Programmable Wakeup 3 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU3 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [progwu3cfg](progwu3cfg) module"]
pub type PROGWU3CFG = crate::Reg<u32, _PROGWU3CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROGWU3CFG;
#[doc = "`read()` method returns [progwu3cfg::R](progwu3cfg::R) reader structure"]
impl crate::Readable for PROGWU3CFG {}
#[doc = "`write(|w| ..)` method takes [progwu3cfg::W](progwu3cfg::W) writer structure"]
impl crate::Writable for PROGWU3CFG {}
#[doc = "Programmable Wakeup 3 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU3 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
pub mod progwu3cfg;
#[doc = "Software Wakeup Triggers System CPU uses these wakeup flags to perform handshaking with AUX_SCE. The wakeup flags can change the operational mode of AUX and guarantees a non-zero SCE clock rate. AUX_SCE wakeup vectors are configured in VECCFGn.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swwutrig](swwutrig) module"]
pub type SWWUTRIG = crate::Reg<u32, _SWWUTRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWWUTRIG;
#[doc = "`read()` method returns [swwutrig::R](swwutrig::R) reader structure"]
impl crate::Readable for SWWUTRIG {}
#[doc = "`write(|w| ..)` method takes [swwutrig::W](swwutrig::W) writer structure"]
impl crate::Writable for SWWUTRIG {}
#[doc = "Software Wakeup Triggers System CPU uses these wakeup flags to perform handshaking with AUX_SCE. The wakeup flags can change the operational mode of AUX and guarantees a non-zero SCE clock rate. AUX_SCE wakeup vectors are configured in VECCFGn."]
pub mod swwutrig;
#[doc = "Wakeup Flags This register holds the eight AUX wakeup flags. Each flag can cause AUX operational mode to change as given in OPMODEREQ. To clear flag n you must set bit n in WUFLAGSCLR until flag n is read as 0. You must clear bit n in WUFLAGSCLR before flag n can be set again.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wuflags](wuflags) module"]
pub type WUFLAGS = crate::Reg<u32, _WUFLAGS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WUFLAGS;
#[doc = "`read()` method returns [wuflags::R](wuflags::R) reader structure"]
impl crate::Readable for WUFLAGS {}
#[doc = "`write(|w| ..)` method takes [wuflags::W](wuflags::W) writer structure"]
impl crate::Writable for WUFLAGS {}
#[doc = "Wakeup Flags This register holds the eight AUX wakeup flags. Each flag can cause AUX operational mode to change as given in OPMODEREQ. To clear flag n you must set bit n in WUFLAGSCLR until flag n is read as 0. You must clear bit n in WUFLAGSCLR before flag n can be set again."]
pub mod wuflags;
#[doc = "Wakeup Flags Clear This register clears AUX wakeup flags WUFLAGS. To clear programmable wakeup flags you must disable the AUX wakeup output first. After the programmable wakeup flags are cleared you must re-enable the AUX wakeup output. Write WUGATE to disable or enable the AUX wakeup output. This procedure is not required when you want to clear a software-triggered wakeup.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wuflagsclr](wuflagsclr) module"]
pub type WUFLAGSCLR = crate::Reg<u32, _WUFLAGSCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WUFLAGSCLR;
#[doc = "`read()` method returns [wuflagsclr::R](wuflagsclr::R) reader structure"]
impl crate::Readable for WUFLAGSCLR {}
#[doc = "`write(|w| ..)` method takes [wuflagsclr::W](wuflagsclr::W) writer structure"]
impl crate::Writable for WUFLAGSCLR {}
#[doc = "Wakeup Flags Clear This register clears AUX wakeup flags WUFLAGS. To clear programmable wakeup flags you must disable the AUX wakeup output first. After the programmable wakeup flags are cleared you must re-enable the AUX wakeup output. Write WUGATE to disable or enable the AUX wakeup output. This procedure is not required when you want to clear a software-triggered wakeup."]
pub mod wuflagsclr;
#[doc = "Wakeup Gate You must disable the AUX wakeup output: - Before you clear a programmable wakeup flag. - Before you change the value of \\[PROGWUnCFG.EN\\]
or \\[PROGWUnCFG.WU_SRC\\]. The AUX wakeup output must be re-enabled after clear operation or programmable wakeup configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wugate](wugate) module"]
pub type WUGATE = crate::Reg<u32, _WUGATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WUGATE;
#[doc = "`read()` method returns [wugate::R](wugate::R) reader structure"]
impl crate::Readable for WUGATE {}
#[doc = "`write(|w| ..)` method takes [wugate::W](wugate::W) writer structure"]
impl crate::Writable for WUGATE {}
#[doc = "Wakeup Gate You must disable the AUX wakeup output: - Before you clear a programmable wakeup flag. - Before you change the value of \\[PROGWUnCFG.EN\\]
or \\[PROGWUnCFG.WU_SRC\\]. The AUX wakeup output must be re-enabled after clear operation or programmable wakeup configuration."]
pub mod wugate;
#[doc = "Vector Configuration 0 AUX_SCE wakeup vector 0 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [veccfg0](veccfg0) module"]
pub type VECCFG0 = crate::Reg<u32, _VECCFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VECCFG0;
#[doc = "`read()` method returns [veccfg0::R](veccfg0::R) reader structure"]
impl crate::Readable for VECCFG0 {}
#[doc = "`write(|w| ..)` method takes [veccfg0::W](veccfg0::W) writer structure"]
impl crate::Writable for VECCFG0 {}
#[doc = "Vector Configuration 0 AUX_SCE wakeup vector 0 configuration"]
pub mod veccfg0;
#[doc = "Vector Configuration 1 AUX_SCE wakeup vector 1 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [veccfg1](veccfg1) module"]
pub type VECCFG1 = crate::Reg<u32, _VECCFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VECCFG1;
#[doc = "`read()` method returns [veccfg1::R](veccfg1::R) reader structure"]
impl crate::Readable for VECCFG1 {}
#[doc = "`write(|w| ..)` method takes [veccfg1::W](veccfg1::W) writer structure"]
impl crate::Writable for VECCFG1 {}
#[doc = "Vector Configuration 1 AUX_SCE wakeup vector 1 configuration"]
pub mod veccfg1;
#[doc = "Vector Configuration 2 AUX_SCE wakeup vector 2 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [veccfg2](veccfg2) module"]
pub type VECCFG2 = crate::Reg<u32, _VECCFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VECCFG2;
#[doc = "`read()` method returns [veccfg2::R](veccfg2::R) reader structure"]
impl crate::Readable for VECCFG2 {}
#[doc = "`write(|w| ..)` method takes [veccfg2::W](veccfg2::W) writer structure"]
impl crate::Writable for VECCFG2 {}
#[doc = "Vector Configuration 2 AUX_SCE wakeup vector 2 configuration"]
pub mod veccfg2;
#[doc = "Vector Configuration 3 AUX_SCE wakeup vector 3 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [veccfg3](veccfg3) module"]
pub type VECCFG3 = crate::Reg<u32, _VECCFG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VECCFG3;
#[doc = "`read()` method returns [veccfg3::R](veccfg3::R) reader structure"]
impl crate::Readable for VECCFG3 {}
#[doc = "`write(|w| ..)` method takes [veccfg3::W](veccfg3::W) writer structure"]
impl crate::Writable for VECCFG3 {}
#[doc = "Vector Configuration 3 AUX_SCE wakeup vector 3 configuration"]
pub mod veccfg3;
#[doc = "Vector Configuration 4 AUX_SCE wakeup vector 4 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [veccfg4](veccfg4) module"]
pub type VECCFG4 = crate::Reg<u32, _VECCFG4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VECCFG4;
#[doc = "`read()` method returns [veccfg4::R](veccfg4::R) reader structure"]
impl crate::Readable for VECCFG4 {}
#[doc = "`write(|w| ..)` method takes [veccfg4::W](veccfg4::W) writer structure"]
impl crate::Writable for VECCFG4 {}
#[doc = "Vector Configuration 4 AUX_SCE wakeup vector 4 configuration"]
pub mod veccfg4;
#[doc = "Vector Configuration 5 AUX_SCE wakeup vector 5 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [veccfg5](veccfg5) module"]
pub type VECCFG5 = crate::Reg<u32, _VECCFG5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VECCFG5;
#[doc = "`read()` method returns [veccfg5::R](veccfg5::R) reader structure"]
impl crate::Readable for VECCFG5 {}
#[doc = "`write(|w| ..)` method takes [veccfg5::W](veccfg5::W) writer structure"]
impl crate::Writable for VECCFG5 {}
#[doc = "Vector Configuration 5 AUX_SCE wakeup vector 5 configuration"]
pub mod veccfg5;
#[doc = "Vector Configuration 6 AUX_SCE wakeup vector 6 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [veccfg6](veccfg6) module"]
pub type VECCFG6 = crate::Reg<u32, _VECCFG6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VECCFG6;
#[doc = "`read()` method returns [veccfg6::R](veccfg6::R) reader structure"]
impl crate::Readable for VECCFG6 {}
#[doc = "`write(|w| ..)` method takes [veccfg6::W](veccfg6::W) writer structure"]
impl crate::Writable for VECCFG6 {}
#[doc = "Vector Configuration 6 AUX_SCE wakeup vector 6 configuration"]
pub mod veccfg6;
#[doc = "Vector Configuration 7 AUX_SCE wakeup vector 7 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [veccfg7](veccfg7) module"]
pub type VECCFG7 = crate::Reg<u32, _VECCFG7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VECCFG7;
#[doc = "`read()` method returns [veccfg7::R](veccfg7::R) reader structure"]
impl crate::Readable for VECCFG7 {}
#[doc = "`write(|w| ..)` method takes [veccfg7::W](veccfg7::W) writer structure"]
impl crate::Writable for VECCFG7 {}
#[doc = "Vector Configuration 7 AUX_SCE wakeup vector 7 configuration"]
pub mod veccfg7;
#[doc = "Event Synchronization Rate Configure synchronization rate for certain events to the synchronous AUX event bus. You must select SCE rate when AUX_SCE uses the event. You must select AUX bus rate when system CPU uses the event. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by two when MCU domain is active.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evsyncrate](evsyncrate) module"]
pub type EVSYNCRATE = crate::Reg<u32, _EVSYNCRATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVSYNCRATE;
#[doc = "`read()` method returns [evsyncrate::R](evsyncrate::R) reader structure"]
impl crate::Readable for EVSYNCRATE {}
#[doc = "`write(|w| ..)` method takes [evsyncrate::W](evsyncrate::W) writer structure"]
impl crate::Writable for EVSYNCRATE {}
#[doc = "Event Synchronization Rate Configure synchronization rate for certain events to the synchronous AUX event bus. You must select SCE rate when AUX_SCE uses the event. You must select AUX bus rate when system CPU uses the event. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by two when MCU domain is active."]
pub mod evsyncrate;
#[doc = "Peripheral Operational Rate Some AUX peripherals are operated at either SCE or at AUX bus rate. You must select SCE rate when AUX_SCE uses such peripheral or an event produced by it. You must select AUX bus rate when system CPU uses such peripheral. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by 2 when MCU domain is active.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peroprate](peroprate) module"]
pub type PEROPRATE = crate::Reg<u32, _PEROPRATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEROPRATE;
#[doc = "`read()` method returns [peroprate::R](peroprate::R) reader structure"]
impl crate::Readable for PEROPRATE {}
#[doc = "`write(|w| ..)` method takes [peroprate::W](peroprate::W) writer structure"]
impl crate::Writable for PEROPRATE {}
#[doc = "Peripheral Operational Rate Some AUX peripherals are operated at either SCE or at AUX bus rate. You must select SCE rate when AUX_SCE uses such peripheral or an event produced by it. You must select AUX bus rate when system CPU uses such peripheral. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by 2 when MCU domain is active."]
pub mod peroprate;
#[doc = "ADC Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcclkctl](adcclkctl) module"]
pub type ADCCLKCTL = crate::Reg<u32, _ADCCLKCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCCLKCTL;
#[doc = "`read()` method returns [adcclkctl::R](adcclkctl::R) reader structure"]
impl crate::Readable for ADCCLKCTL {}
#[doc = "`write(|w| ..)` method takes [adcclkctl::W](adcclkctl::W) writer structure"]
impl crate::Writable for ADCCLKCTL {}
#[doc = "ADC Clock Control"]
pub mod adcclkctl;
#[doc = "TDC Counter Clock Control Controls if the AUX_TDC counter clock source is enabled. TDC counter clock source is configured in DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdcclkctl](tdcclkctl) module"]
pub type TDCCLKCTL = crate::Reg<u32, _TDCCLKCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TDCCLKCTL;
#[doc = "`read()` method returns [tdcclkctl::R](tdcclkctl::R) reader structure"]
impl crate::Readable for TDCCLKCTL {}
#[doc = "`write(|w| ..)` method takes [tdcclkctl::W](tdcclkctl::W) writer structure"]
impl crate::Writable for TDCCLKCTL {}
#[doc = "TDC Counter Clock Control Controls if the AUX_TDC counter clock source is enabled. TDC counter clock source is configured in DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL."]
pub mod tdcclkctl;
#[doc = "TDC Reference Clock Control Controls if the AUX_TDC reference clock source is enabled. This clock is compared against the AUX_TDC counter clock. TDC reference clock source is configured in DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdcrefclkctl](tdcrefclkctl) module"]
pub type TDCREFCLKCTL = crate::Reg<u32, _TDCREFCLKCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TDCREFCLKCTL;
#[doc = "`read()` method returns [tdcrefclkctl::R](tdcrefclkctl::R) reader structure"]
impl crate::Readable for TDCREFCLKCTL {}
#[doc = "`write(|w| ..)` method takes [tdcrefclkctl::W](tdcrefclkctl::W) writer structure"]
impl crate::Writable for TDCREFCLKCTL {}
#[doc = "TDC Reference Clock Control Controls if the AUX_TDC reference clock source is enabled. This clock is compared against the AUX_TDC counter clock. TDC reference clock source is configured in DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL."]
pub mod tdcrefclkctl;
#[doc = "AUX_TIMER2 Clock Control Access to AUX_TIMER2 is only possible when TIMER2CLKSTAT.STAT is different from NONE.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer2clkctl](timer2clkctl) module"]
pub type TIMER2CLKCTL = crate::Reg<u32, _TIMER2CLKCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER2CLKCTL;
#[doc = "`read()` method returns [timer2clkctl::R](timer2clkctl::R) reader structure"]
impl crate::Readable for TIMER2CLKCTL {}
#[doc = "`write(|w| ..)` method takes [timer2clkctl::W](timer2clkctl::W) writer structure"]
impl crate::Writable for TIMER2CLKCTL {}
#[doc = "AUX_TIMER2 Clock Control Access to AUX_TIMER2 is only possible when TIMER2CLKSTAT.STAT is different from NONE."]
pub mod timer2clkctl;
#[doc = "AUX_TIMER2 Clock Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer2clkstat](timer2clkstat) module"]
pub type TIMER2CLKSTAT = crate::Reg<u32, _TIMER2CLKSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER2CLKSTAT;
#[doc = "`read()` method returns [timer2clkstat::R](timer2clkstat::R) reader structure"]
impl crate::Readable for TIMER2CLKSTAT {}
#[doc = "`write(|w| ..)` method takes [timer2clkstat::W](timer2clkstat::W) writer structure"]
impl crate::Writable for TIMER2CLKSTAT {}
#[doc = "AUX_TIMER2 Clock Status"]
pub mod timer2clkstat;
#[doc = "AUX_TIMER2 Clock Switch\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer2clkswitch](timer2clkswitch) module"]
pub type TIMER2CLKSWITCH = crate::Reg<u32, _TIMER2CLKSWITCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER2CLKSWITCH;
#[doc = "`read()` method returns [timer2clkswitch::R](timer2clkswitch::R) reader structure"]
impl crate::Readable for TIMER2CLKSWITCH {}
#[doc = "`write(|w| ..)` method takes [timer2clkswitch::W](timer2clkswitch::W) writer structure"]
impl crate::Writable for TIMER2CLKSWITCH {}
#[doc = "AUX_TIMER2 Clock Switch"]
pub mod timer2clkswitch;
#[doc = "AUX_TIMER2 Debug Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer2dbgctl](timer2dbgctl) module"]
pub type TIMER2DBGCTL = crate::Reg<u32, _TIMER2DBGCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER2DBGCTL;
#[doc = "`read()` method returns [timer2dbgctl::R](timer2dbgctl::R) reader structure"]
impl crate::Readable for TIMER2DBGCTL {}
#[doc = "`write(|w| ..)` method takes [timer2dbgctl::W](timer2dbgctl::W) writer structure"]
impl crate::Writable for TIMER2DBGCTL {}
#[doc = "AUX_TIMER2 Debug Control"]
pub mod timer2dbgctl;
#[doc = "Clock Shift Detection A transition in the MCU domain state causes a non-accumulative change to the SCE clock period when the AUX clock rate is derived from SCLK_MF or SCLK_LF: - A single SCE clock cycle is 6 thru 8 SCLK_HF cycles longer when MCU domain enters active state. - A single SCE clock cycle is 6 thru 8 SCLK_HF cycles shorter when MCU domain exits active state. AUX_SCE detects if such events occurred to the SCE clock during the time period between a clear of STAT and a read of STAT.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkshiftdet](clkshiftdet) module"]
pub type CLKSHIFTDET = crate::Reg<u32, _CLKSHIFTDET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKSHIFTDET;
#[doc = "`read()` method returns [clkshiftdet::R](clkshiftdet::R) reader structure"]
impl crate::Readable for CLKSHIFTDET {}
#[doc = "`write(|w| ..)` method takes [clkshiftdet::W](clkshiftdet::W) writer structure"]
impl crate::Writable for CLKSHIFTDET {}
#[doc = "Clock Shift Detection A transition in the MCU domain state causes a non-accumulative change to the SCE clock period when the AUX clock rate is derived from SCLK_MF or SCLK_LF: - A single SCE clock cycle is 6 thru 8 SCLK_HF cycles longer when MCU domain enters active state. - A single SCE clock cycle is 6 thru 8 SCLK_HF cycles shorter when MCU domain exits active state. AUX_SCE detects if such events occurred to the SCE clock during the time period between a clear of STAT and a read of STAT."]
pub mod clkshiftdet;
#[doc = "VDDR Recharge Trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rechargetrig](rechargetrig) module"]
pub type RECHARGETRIG = crate::Reg<u32, _RECHARGETRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RECHARGETRIG;
#[doc = "`read()` method returns [rechargetrig::R](rechargetrig::R) reader structure"]
impl crate::Readable for RECHARGETRIG {}
#[doc = "`write(|w| ..)` method takes [rechargetrig::W](rechargetrig::W) writer structure"]
impl crate::Writable for RECHARGETRIG {}
#[doc = "VDDR Recharge Trigger"]
pub mod rechargetrig;
#[doc = "VDDR Recharge Detection Some applications can be sensitive to power noise caused by recharge of VDDR. You can detect if VDDR recharge occurs.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rechargedet](rechargedet) module"]
pub type RECHARGEDET = crate::Reg<u32, _RECHARGEDET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RECHARGEDET;
#[doc = "`read()` method returns [rechargedet::R](rechargedet::R) reader structure"]
impl crate::Readable for RECHARGEDET {}
#[doc = "`write(|w| ..)` method takes [rechargedet::W](rechargedet::W) writer structure"]
impl crate::Writable for RECHARGEDET {}
#[doc = "VDDR Recharge Detection Some applications can be sensitive to power noise caused by recharge of VDDR. You can detect if VDDR recharge occurs."]
pub mod rechargedet;
#[doc = "Real Time Counter Sub Second Increment 0 INC15_0 will replace bits 15:0 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcsubsecinc0](rtcsubsecinc0) module"]
pub type RTCSUBSECINC0 = crate::Reg<u32, _RTCSUBSECINC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCSUBSECINC0;
#[doc = "`read()` method returns [rtcsubsecinc0::R](rtcsubsecinc0::R) reader structure"]
impl crate::Readable for RTCSUBSECINC0 {}
#[doc = "`write(|w| ..)` method takes [rtcsubsecinc0::W](rtcsubsecinc0::W) writer structure"]
impl crate::Writable for RTCSUBSECINC0 {}
#[doc = "Real Time Counter Sub Second Increment 0 INC15_0 will replace bits 15:0 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set."]
pub mod rtcsubsecinc0;
#[doc = "Real Time Counter Sub Second Increment 1 INC23_16 will replace bits 23:16 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcsubsecinc1](rtcsubsecinc1) module"]
pub type RTCSUBSECINC1 = crate::Reg<u32, _RTCSUBSECINC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCSUBSECINC1;
#[doc = "`read()` method returns [rtcsubsecinc1::R](rtcsubsecinc1::R) reader structure"]
impl crate::Readable for RTCSUBSECINC1 {}
#[doc = "`write(|w| ..)` method takes [rtcsubsecinc1::W](rtcsubsecinc1::W) writer structure"]
impl crate::Writable for RTCSUBSECINC1 {}
#[doc = "Real Time Counter Sub Second Increment 1 INC23_16 will replace bits 23:16 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set."]
pub mod rtcsubsecinc1;
#[doc = "Real Time Counter Sub Second Increment Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcsubsecincctl](rtcsubsecincctl) module"]
pub type RTCSUBSECINCCTL = crate::Reg<u32, _RTCSUBSECINCCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCSUBSECINCCTL;
#[doc = "`read()` method returns [rtcsubsecincctl::R](rtcsubsecincctl::R) reader structure"]
impl crate::Readable for RTCSUBSECINCCTL {}
#[doc = "`write(|w| ..)` method takes [rtcsubsecincctl::W](rtcsubsecincctl::W) writer structure"]
impl crate::Writable for RTCSUBSECINCCTL {}
#[doc = "Real Time Counter Sub Second Increment Control"]
pub mod rtcsubsecincctl;
#[doc = "Real Time Counter Second System CPU must not access this register. Instead, system CPU must access AON_RTC:SEC.VALUE directly.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcsec](rtcsec) module"]
pub type RTCSEC = crate::Reg<u32, _RTCSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCSEC;
#[doc = "`read()` method returns [rtcsec::R](rtcsec::R) reader structure"]
impl crate::Readable for RTCSEC {}
#[doc = "`write(|w| ..)` method takes [rtcsec::W](rtcsec::W) writer structure"]
impl crate::Writable for RTCSEC {}
#[doc = "Real Time Counter Second System CPU must not access this register. Instead, system CPU must access AON_RTC:SEC.VALUE directly."]
pub mod rtcsec;
#[doc = "Real Time Counter Sub-Second System CPU must not access this register. Instead, system CPU must access AON_RTC:SUBSEC.VALUE directly.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcsubsec](rtcsubsec) module"]
pub type RTCSUBSEC = crate::Reg<u32, _RTCSUBSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCSUBSEC;
#[doc = "`read()` method returns [rtcsubsec::R](rtcsubsec::R) reader structure"]
impl crate::Readable for RTCSUBSEC {}
#[doc = "`write(|w| ..)` method takes [rtcsubsec::W](rtcsubsec::W) writer structure"]
impl crate::Writable for RTCSUBSEC {}
#[doc = "Real Time Counter Sub-Second System CPU must not access this register. Instead, system CPU must access AON_RTC:SUBSEC.VALUE directly."]
pub mod rtcsubsec;
#[doc = "AON_RTC Event Clear Request to clear events: - AON_RTC:EVFLAGS.CH2. - AON_RTC:EVFLAGS.CH2 delayed version. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcevclr](rtcevclr) module"]
pub type RTCEVCLR = crate::Reg<u32, _RTCEVCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCEVCLR;
#[doc = "`read()` method returns [rtcevclr::R](rtcevclr::R) reader structure"]
impl crate::Readable for RTCEVCLR {}
#[doc = "`write(|w| ..)` method takes [rtcevclr::W](rtcevclr::W) writer structure"]
impl crate::Writable for RTCEVCLR {}
#[doc = "AON_RTC Event Clear Request to clear events: - AON_RTC:EVFLAGS.CH2. - AON_RTC:EVFLAGS.CH2 delayed version. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY."]
pub mod rtcevclr;
#[doc = "AON_BATMON Battery Voltage Value Read access to AON_BATMON:BAT. System CPU must not access this register. Instead, system CPU must access AON_BATMON:BAT directly. AON_BATMON:BAT updates during VDDR recharge or active operational mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [batmonbat](batmonbat) module"]
pub type BATMONBAT = crate::Reg<u32, _BATMONBAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BATMONBAT;
#[doc = "`read()` method returns [batmonbat::R](batmonbat::R) reader structure"]
impl crate::Readable for BATMONBAT {}
#[doc = "`write(|w| ..)` method takes [batmonbat::W](batmonbat::W) writer structure"]
impl crate::Writable for BATMONBAT {}
#[doc = "AON_BATMON Battery Voltage Value Read access to AON_BATMON:BAT. System CPU must not access this register. Instead, system CPU must access AON_BATMON:BAT directly. AON_BATMON:BAT updates during VDDR recharge or active operational mode."]
pub mod batmonbat;
#[doc = "AON_BATMON Temperature Value Read access to AON_BATMON:TEMP. System CPU must not access this register. Instead, system CPU must access AON_BATMON:TEMP directly. AON_BATMON:TEMP updates during VDDR recharge or active operational mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [batmontemp](batmontemp) module"]
pub type BATMONTEMP = crate::Reg<u32, _BATMONTEMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BATMONTEMP;
#[doc = "`read()` method returns [batmontemp::R](batmontemp::R) reader structure"]
impl crate::Readable for BATMONTEMP {}
#[doc = "`write(|w| ..)` method takes [batmontemp::W](batmontemp::W) writer structure"]
impl crate::Writable for BATMONTEMP {}
#[doc = "AON_BATMON Temperature Value Read access to AON_BATMON:TEMP. System CPU must not access this register. Instead, system CPU must access AON_BATMON:TEMP directly. AON_BATMON:TEMP updates during VDDR recharge or active operational mode."]
pub mod batmontemp;
#[doc = "Timer Halt Debug register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timerhalt](timerhalt) module"]
pub type TIMERHALT = crate::Reg<u32, _TIMERHALT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMERHALT;
#[doc = "`read()` method returns [timerhalt::R](timerhalt::R) reader structure"]
impl crate::Readable for TIMERHALT {}
#[doc = "`write(|w| ..)` method takes [timerhalt::W](timerhalt::W) writer structure"]
impl crate::Writable for TIMERHALT {}
#[doc = "Timer Halt Debug register"]
pub mod timerhalt;
#[doc = "AUX_TIMER2 Bridge\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer2bridge](timer2bridge) module"]
pub type TIMER2BRIDGE = crate::Reg<u32, _TIMER2BRIDGE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER2BRIDGE;
#[doc = "`read()` method returns [timer2bridge::R](timer2bridge::R) reader structure"]
impl crate::Readable for TIMER2BRIDGE {}
#[doc = "`write(|w| ..)` method takes [timer2bridge::W](timer2bridge::W) writer structure"]
impl crate::Writable for TIMER2BRIDGE {}
#[doc = "AUX_TIMER2 Bridge"]
pub mod timer2bridge;
#[doc = "Software Power Profiler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swpwrprof](swpwrprof) module"]
pub type SWPWRPROF = crate::Reg<u32, _SWPWRPROF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWPWRPROF;
#[doc = "`read()` method returns [swpwrprof::R](swpwrprof::R) reader structure"]
impl crate::Readable for SWPWRPROF {}
#[doc = "`write(|w| ..)` method takes [swpwrprof::W](swpwrprof::W) writer structure"]
impl crate::Writable for SWPWRPROF {}
#[doc = "Software Power Profiler"]
pub mod swpwrprof;
