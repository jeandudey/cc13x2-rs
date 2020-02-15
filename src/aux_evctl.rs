#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Event Status 0 Register holds events 0 thru 15 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_TIMER2. - AUX_ANAIF. - AUX_TDC. - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG."]
    pub evstat0: EVSTAT0,
    #[doc = "0x04 - Event Status 1 Register holds events 16 thru 31 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_TIMER2. - AUX_ANAIF. - AUX_TDC. - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG."]
    pub evstat1: EVSTAT1,
    #[doc = "0x08 - Event Status 2 Register holds events 32 thru 47 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_TIMER2. - AUX_ANAIF. - AUX_TDC. - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG."]
    pub evstat2: EVSTAT2,
    #[doc = "0x0c - Event Status 3 Register holds events 48 thru 63 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_TIMER2. - AUX_ANAIF. - AUX_TDC . - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG."]
    pub evstat3: EVSTAT3,
    #[doc = "0x10 - Sensor Controller Engine Wait Event Configuration 0 Configuration of this register and SCEWEVCFG1 controls bit index 7 in AUX_SCE:WUSTAT.EV_SIGNALS. This bit can be used by AUX_SCE WEV0, WEV1, BEV0 and BEV1 instructions. When COMB_EV_EN = 0: AUX_SCE:WUSTAT.EV_SIGNALS (7) = EV0_SEL event When COMB_EV_EN = 1: AUX_SCE:WUSTAT.EV_SIGNALS (7) = ( EV0_SEL event ) OR ( SCEWEVCFG1.EV1_SEL event ) Bit fields SCEWEVCFG1.EV0_POL and SCEWEVCFG1.EV1_POL control the polarity of selected events. Event combination is useful when there is a need to wait for a certain condition with timeout."]
    pub scewevcfg0: SCEWEVCFG0,
    #[doc = "0x14 - Sensor Controller Engine Wait Event Configuration 1 See SCEWEVCFG0 for description."]
    pub scewevcfg1: SCEWEVCFG1,
    #[doc = "0x18 - Direct Memory Access Control"]
    pub dmactl: DMACTL,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - Software Event Set Set software event flags from AUX domain to AON and MCU domains. CPUs in MCU domain can read the event flags from EVTOAONFLAGS and clear them in EVTOAONFLAGSCLR. Use of these event flags is software-defined."]
    pub swevset: SWEVSET,
    #[doc = "0x24 - Events To AON Flags This register contains a collection of event flags routed to AON_EVENT. To clear an event flag, write to EVTOAONFLAGSCLR or write 0 to event flag in this register."]
    pub evtoaonflags: EVTOAONFLAGS,
    #[doc = "0x28 - Events To AON Polarity Event source polarity configuration for EVTOAONFLAGS."]
    pub evtoaonpol: EVTOAONPOL,
    #[doc = "0x2c - Events To AON Clear Clear event flags in EVTOAONFLAGS. In order to clear a level sensitive event flag, the event must be deasserted."]
    pub evtoaonflagsclr: EVTOAONFLAGSCLR,
    #[doc = "0x30 - Events to MCU Flags This register contains a collection of event flags routed to MCU domain. To clear an event flag, write to EVTOMCUFLAGSCLR or write 0 to event flag in this register. Follow procedure described in AUX_SYSIF:WUCLR to clear AUX_WU_EV event flag."]
    pub evtomcuflags: EVTOMCUFLAGS,
    #[doc = "0x34 - Event To MCU Polarity Event source polarity configuration for EVTOMCUFLAGS."]
    pub evtomcupol: EVTOMCUPOL,
    #[doc = "0x38 - Events To MCU Flags Clear Clear event flags in EVTOMCUFLAGS. In order to clear a level sensitive event flag, the event must be deasserted."]
    pub evtomcuflagsclr: EVTOMCUFLAGSCLR,
    #[doc = "0x3c - Combined Event To MCU Mask Select event flags in EVTOMCUFLAGS that contribute to the AUX_COMB event to EVENT and system CPU. The AUX_COMB event is high as long as one or more of the included event flags are set."]
    pub combevtomcumask: COMBEVTOMCUMASK,
    #[doc = "0x40 - Event Observation Configuration"]
    pub evobscfg: EVOBSCFG,
    #[doc = "0x44 - Programmable Delay"]
    pub progdly: PROGDLY,
    #[doc = "0x48 - Manual Programmable event."]
    pub manual: MANUAL,
    #[doc = "0x4c - Event Status 0 Low"]
    pub evstat0l: EVSTAT0L,
    #[doc = "0x50 - Event Status 0 High"]
    pub evstat0h: EVSTAT0H,
    #[doc = "0x54 - Event Status 1 Low"]
    pub evstat1l: EVSTAT1L,
    #[doc = "0x58 - Event Status 1 High"]
    pub evstat1h: EVSTAT1H,
    #[doc = "0x5c - Event Status 2 Low"]
    pub evstat2l: EVSTAT2L,
    #[doc = "0x60 - Event Status 2 High"]
    pub evstat2h: EVSTAT2H,
    #[doc = "0x64 - Event Status 3 Low"]
    pub evstat3l: EVSTAT3L,
    #[doc = "0x68 - Event Status 3 High"]
    pub evstat3h: EVSTAT3H,
}
#[doc = "Event Status 0 Register holds events 0 thru 15 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_TIMER2. - AUX_ANAIF. - AUX_TDC. - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evstat0](evstat0) module"]
pub type EVSTAT0 = crate::Reg<u32, _EVSTAT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVSTAT0;
#[doc = "`read()` method returns [evstat0::R](evstat0::R) reader structure"]
impl crate::Readable for EVSTAT0 {}
#[doc = "`write(|w| ..)` method takes [evstat0::W](evstat0::W) writer structure"]
impl crate::Writable for EVSTAT0 {}
#[doc = "Event Status 0 Register holds events 0 thru 15 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_TIMER2. - AUX_ANAIF. - AUX_TDC. - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG."]
pub mod evstat0;
#[doc = "Event Status 1 Register holds events 16 thru 31 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_TIMER2. - AUX_ANAIF. - AUX_TDC. - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evstat1](evstat1) module"]
pub type EVSTAT1 = crate::Reg<u32, _EVSTAT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVSTAT1;
#[doc = "`read()` method returns [evstat1::R](evstat1::R) reader structure"]
impl crate::Readable for EVSTAT1 {}
#[doc = "`write(|w| ..)` method takes [evstat1::W](evstat1::W) writer structure"]
impl crate::Writable for EVSTAT1 {}
#[doc = "Event Status 1 Register holds events 16 thru 31 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_TIMER2. - AUX_ANAIF. - AUX_TDC. - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG."]
pub mod evstat1;
#[doc = "Event Status 2 Register holds events 32 thru 47 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_TIMER2. - AUX_ANAIF. - AUX_TDC. - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evstat2](evstat2) module"]
pub type EVSTAT2 = crate::Reg<u32, _EVSTAT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVSTAT2;
#[doc = "`read()` method returns [evstat2::R](evstat2::R) reader structure"]
impl crate::Readable for EVSTAT2 {}
#[doc = "`write(|w| ..)` method takes [evstat2::W](evstat2::W) writer structure"]
impl crate::Writable for EVSTAT2 {}
#[doc = "Event Status 2 Register holds events 32 thru 47 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_TIMER2. - AUX_ANAIF. - AUX_TDC. - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG."]
pub mod evstat2;
#[doc = "Event Status 3 Register holds events 48 thru 63 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_TIMER2. - AUX_ANAIF. - AUX_TDC . - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evstat3](evstat3) module"]
pub type EVSTAT3 = crate::Reg<u32, _EVSTAT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVSTAT3;
#[doc = "`read()` method returns [evstat3::R](evstat3::R) reader structure"]
impl crate::Readable for EVSTAT3 {}
#[doc = "`write(|w| ..)` method takes [evstat3::W](evstat3::W) writer structure"]
impl crate::Writable for EVSTAT3 {}
#[doc = "Event Status 3 Register holds events 48 thru 63 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_TIMER2. - AUX_ANAIF. - AUX_TDC . - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG."]
pub mod evstat3;
#[doc = "Sensor Controller Engine Wait Event Configuration 0 Configuration of this register and SCEWEVCFG1 controls bit index 7 in AUX_SCE:WUSTAT.EV_SIGNALS. This bit can be used by AUX_SCE WEV0, WEV1, BEV0 and BEV1 instructions. When COMB_EV_EN = 0: AUX_SCE:WUSTAT.EV_SIGNALS (7) = EV0_SEL event When COMB_EV_EN = 1: AUX_SCE:WUSTAT.EV_SIGNALS (7) = ( EV0_SEL event ) OR ( SCEWEVCFG1.EV1_SEL event ) Bit fields SCEWEVCFG1.EV0_POL and SCEWEVCFG1.EV1_POL control the polarity of selected events. Event combination is useful when there is a need to wait for a certain condition with timeout.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scewevcfg0](scewevcfg0) module"]
pub type SCEWEVCFG0 = crate::Reg<u32, _SCEWEVCFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCEWEVCFG0;
#[doc = "`read()` method returns [scewevcfg0::R](scewevcfg0::R) reader structure"]
impl crate::Readable for SCEWEVCFG0 {}
#[doc = "`write(|w| ..)` method takes [scewevcfg0::W](scewevcfg0::W) writer structure"]
impl crate::Writable for SCEWEVCFG0 {}
#[doc = "Sensor Controller Engine Wait Event Configuration 0 Configuration of this register and SCEWEVCFG1 controls bit index 7 in AUX_SCE:WUSTAT.EV_SIGNALS. This bit can be used by AUX_SCE WEV0, WEV1, BEV0 and BEV1 instructions. When COMB_EV_EN = 0: AUX_SCE:WUSTAT.EV_SIGNALS (7) = EV0_SEL event When COMB_EV_EN = 1: AUX_SCE:WUSTAT.EV_SIGNALS (7) = ( EV0_SEL event ) OR ( SCEWEVCFG1.EV1_SEL event ) Bit fields SCEWEVCFG1.EV0_POL and SCEWEVCFG1.EV1_POL control the polarity of selected events. Event combination is useful when there is a need to wait for a certain condition with timeout."]
pub mod scewevcfg0;
#[doc = "Sensor Controller Engine Wait Event Configuration 1 See SCEWEVCFG0 for description.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scewevcfg1](scewevcfg1) module"]
pub type SCEWEVCFG1 = crate::Reg<u32, _SCEWEVCFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCEWEVCFG1;
#[doc = "`read()` method returns [scewevcfg1::R](scewevcfg1::R) reader structure"]
impl crate::Readable for SCEWEVCFG1 {}
#[doc = "`write(|w| ..)` method takes [scewevcfg1::W](scewevcfg1::W) writer structure"]
impl crate::Writable for SCEWEVCFG1 {}
#[doc = "Sensor Controller Engine Wait Event Configuration 1 See SCEWEVCFG0 for description."]
pub mod scewevcfg1;
#[doc = "Direct Memory Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl](dmactl) module"]
pub type DMACTL = crate::Reg<u32, _DMACTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACTL;
#[doc = "`read()` method returns [dmactl::R](dmactl::R) reader structure"]
impl crate::Readable for DMACTL {}
#[doc = "`write(|w| ..)` method takes [dmactl::W](dmactl::W) writer structure"]
impl crate::Writable for DMACTL {}
#[doc = "Direct Memory Access Control"]
pub mod dmactl;
#[doc = "Software Event Set Set software event flags from AUX domain to AON and MCU domains. CPUs in MCU domain can read the event flags from EVTOAONFLAGS and clear them in EVTOAONFLAGSCLR. Use of these event flags is software-defined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swevset](swevset) module"]
pub type SWEVSET = crate::Reg<u32, _SWEVSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWEVSET;
#[doc = "`read()` method returns [swevset::R](swevset::R) reader structure"]
impl crate::Readable for SWEVSET {}
#[doc = "`write(|w| ..)` method takes [swevset::W](swevset::W) writer structure"]
impl crate::Writable for SWEVSET {}
#[doc = "Software Event Set Set software event flags from AUX domain to AON and MCU domains. CPUs in MCU domain can read the event flags from EVTOAONFLAGS and clear them in EVTOAONFLAGSCLR. Use of these event flags is software-defined."]
pub mod swevset;
#[doc = "Events To AON Flags This register contains a collection of event flags routed to AON_EVENT. To clear an event flag, write to EVTOAONFLAGSCLR or write 0 to event flag in this register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtoaonflags](evtoaonflags) module"]
pub type EVTOAONFLAGS = crate::Reg<u32, _EVTOAONFLAGS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVTOAONFLAGS;
#[doc = "`read()` method returns [evtoaonflags::R](evtoaonflags::R) reader structure"]
impl crate::Readable for EVTOAONFLAGS {}
#[doc = "`write(|w| ..)` method takes [evtoaonflags::W](evtoaonflags::W) writer structure"]
impl crate::Writable for EVTOAONFLAGS {}
#[doc = "Events To AON Flags This register contains a collection of event flags routed to AON_EVENT. To clear an event flag, write to EVTOAONFLAGSCLR or write 0 to event flag in this register."]
pub mod evtoaonflags;
#[doc = "Events To AON Polarity Event source polarity configuration for EVTOAONFLAGS.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtoaonpol](evtoaonpol) module"]
pub type EVTOAONPOL = crate::Reg<u32, _EVTOAONPOL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVTOAONPOL;
#[doc = "`read()` method returns [evtoaonpol::R](evtoaonpol::R) reader structure"]
impl crate::Readable for EVTOAONPOL {}
#[doc = "`write(|w| ..)` method takes [evtoaonpol::W](evtoaonpol::W) writer structure"]
impl crate::Writable for EVTOAONPOL {}
#[doc = "Events To AON Polarity Event source polarity configuration for EVTOAONFLAGS."]
pub mod evtoaonpol;
#[doc = "Events To AON Clear Clear event flags in EVTOAONFLAGS. In order to clear a level sensitive event flag, the event must be deasserted.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtoaonflagsclr](evtoaonflagsclr) module"]
pub type EVTOAONFLAGSCLR = crate::Reg<u32, _EVTOAONFLAGSCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVTOAONFLAGSCLR;
#[doc = "`read()` method returns [evtoaonflagsclr::R](evtoaonflagsclr::R) reader structure"]
impl crate::Readable for EVTOAONFLAGSCLR {}
#[doc = "`write(|w| ..)` method takes [evtoaonflagsclr::W](evtoaonflagsclr::W) writer structure"]
impl crate::Writable for EVTOAONFLAGSCLR {}
#[doc = "Events To AON Clear Clear event flags in EVTOAONFLAGS. In order to clear a level sensitive event flag, the event must be deasserted."]
pub mod evtoaonflagsclr;
#[doc = "Events to MCU Flags This register contains a collection of event flags routed to MCU domain. To clear an event flag, write to EVTOMCUFLAGSCLR or write 0 to event flag in this register. Follow procedure described in AUX_SYSIF:WUCLR to clear AUX_WU_EV event flag.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtomcuflags](evtomcuflags) module"]
pub type EVTOMCUFLAGS = crate::Reg<u32, _EVTOMCUFLAGS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVTOMCUFLAGS;
#[doc = "`read()` method returns [evtomcuflags::R](evtomcuflags::R) reader structure"]
impl crate::Readable for EVTOMCUFLAGS {}
#[doc = "`write(|w| ..)` method takes [evtomcuflags::W](evtomcuflags::W) writer structure"]
impl crate::Writable for EVTOMCUFLAGS {}
#[doc = "Events to MCU Flags This register contains a collection of event flags routed to MCU domain. To clear an event flag, write to EVTOMCUFLAGSCLR or write 0 to event flag in this register. Follow procedure described in AUX_SYSIF:WUCLR to clear AUX_WU_EV event flag."]
pub mod evtomcuflags;
#[doc = "Event To MCU Polarity Event source polarity configuration for EVTOMCUFLAGS.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtomcupol](evtomcupol) module"]
pub type EVTOMCUPOL = crate::Reg<u32, _EVTOMCUPOL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVTOMCUPOL;
#[doc = "`read()` method returns [evtomcupol::R](evtomcupol::R) reader structure"]
impl crate::Readable for EVTOMCUPOL {}
#[doc = "`write(|w| ..)` method takes [evtomcupol::W](evtomcupol::W) writer structure"]
impl crate::Writable for EVTOMCUPOL {}
#[doc = "Event To MCU Polarity Event source polarity configuration for EVTOMCUFLAGS."]
pub mod evtomcupol;
#[doc = "Events To MCU Flags Clear Clear event flags in EVTOMCUFLAGS. In order to clear a level sensitive event flag, the event must be deasserted.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtomcuflagsclr](evtomcuflagsclr) module"]
pub type EVTOMCUFLAGSCLR = crate::Reg<u32, _EVTOMCUFLAGSCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVTOMCUFLAGSCLR;
#[doc = "`read()` method returns [evtomcuflagsclr::R](evtomcuflagsclr::R) reader structure"]
impl crate::Readable for EVTOMCUFLAGSCLR {}
#[doc = "`write(|w| ..)` method takes [evtomcuflagsclr::W](evtomcuflagsclr::W) writer structure"]
impl crate::Writable for EVTOMCUFLAGSCLR {}
#[doc = "Events To MCU Flags Clear Clear event flags in EVTOMCUFLAGS. In order to clear a level sensitive event flag, the event must be deasserted."]
pub mod evtomcuflagsclr;
#[doc = "Combined Event To MCU Mask Select event flags in EVTOMCUFLAGS that contribute to the AUX_COMB event to EVENT and system CPU. The AUX_COMB event is high as long as one or more of the included event flags are set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [combevtomcumask](combevtomcumask) module"]
pub type COMBEVTOMCUMASK = crate::Reg<u32, _COMBEVTOMCUMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMBEVTOMCUMASK;
#[doc = "`read()` method returns [combevtomcumask::R](combevtomcumask::R) reader structure"]
impl crate::Readable for COMBEVTOMCUMASK {}
#[doc = "`write(|w| ..)` method takes [combevtomcumask::W](combevtomcumask::W) writer structure"]
impl crate::Writable for COMBEVTOMCUMASK {}
#[doc = "Combined Event To MCU Mask Select event flags in EVTOMCUFLAGS that contribute to the AUX_COMB event to EVENT and system CPU. The AUX_COMB event is high as long as one or more of the included event flags are set."]
pub mod combevtomcumask;
#[doc = "Event Observation Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evobscfg](evobscfg) module"]
pub type EVOBSCFG = crate::Reg<u32, _EVOBSCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVOBSCFG;
#[doc = "`read()` method returns [evobscfg::R](evobscfg::R) reader structure"]
impl crate::Readable for EVOBSCFG {}
#[doc = "`write(|w| ..)` method takes [evobscfg::W](evobscfg::W) writer structure"]
impl crate::Writable for EVOBSCFG {}
#[doc = "Event Observation Configuration"]
pub mod evobscfg;
#[doc = "Programmable Delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [progdly](progdly) module"]
pub type PROGDLY = crate::Reg<u32, _PROGDLY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROGDLY;
#[doc = "`read()` method returns [progdly::R](progdly::R) reader structure"]
impl crate::Readable for PROGDLY {}
#[doc = "`write(|w| ..)` method takes [progdly::W](progdly::W) writer structure"]
impl crate::Writable for PROGDLY {}
#[doc = "Programmable Delay"]
pub mod progdly;
#[doc = "Manual Programmable event.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [manual](manual) module"]
pub type MANUAL = crate::Reg<u32, _MANUAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MANUAL;
#[doc = "`read()` method returns [manual::R](manual::R) reader structure"]
impl crate::Readable for MANUAL {}
#[doc = "`write(|w| ..)` method takes [manual::W](manual::W) writer structure"]
impl crate::Writable for MANUAL {}
#[doc = "Manual Programmable event."]
pub mod manual;
#[doc = "Event Status 0 Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evstat0l](evstat0l) module"]
pub type EVSTAT0L = crate::Reg<u32, _EVSTAT0L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVSTAT0L;
#[doc = "`read()` method returns [evstat0l::R](evstat0l::R) reader structure"]
impl crate::Readable for EVSTAT0L {}
#[doc = "`write(|w| ..)` method takes [evstat0l::W](evstat0l::W) writer structure"]
impl crate::Writable for EVSTAT0L {}
#[doc = "Event Status 0 Low"]
pub mod evstat0l;
#[doc = "Event Status 0 High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evstat0h](evstat0h) module"]
pub type EVSTAT0H = crate::Reg<u32, _EVSTAT0H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVSTAT0H;
#[doc = "`read()` method returns [evstat0h::R](evstat0h::R) reader structure"]
impl crate::Readable for EVSTAT0H {}
#[doc = "`write(|w| ..)` method takes [evstat0h::W](evstat0h::W) writer structure"]
impl crate::Writable for EVSTAT0H {}
#[doc = "Event Status 0 High"]
pub mod evstat0h;
#[doc = "Event Status 1 Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evstat1l](evstat1l) module"]
pub type EVSTAT1L = crate::Reg<u32, _EVSTAT1L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVSTAT1L;
#[doc = "`read()` method returns [evstat1l::R](evstat1l::R) reader structure"]
impl crate::Readable for EVSTAT1L {}
#[doc = "`write(|w| ..)` method takes [evstat1l::W](evstat1l::W) writer structure"]
impl crate::Writable for EVSTAT1L {}
#[doc = "Event Status 1 Low"]
pub mod evstat1l;
#[doc = "Event Status 1 High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evstat1h](evstat1h) module"]
pub type EVSTAT1H = crate::Reg<u32, _EVSTAT1H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVSTAT1H;
#[doc = "`read()` method returns [evstat1h::R](evstat1h::R) reader structure"]
impl crate::Readable for EVSTAT1H {}
#[doc = "`write(|w| ..)` method takes [evstat1h::W](evstat1h::W) writer structure"]
impl crate::Writable for EVSTAT1H {}
#[doc = "Event Status 1 High"]
pub mod evstat1h;
#[doc = "Event Status 2 Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evstat2l](evstat2l) module"]
pub type EVSTAT2L = crate::Reg<u32, _EVSTAT2L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVSTAT2L;
#[doc = "`read()` method returns [evstat2l::R](evstat2l::R) reader structure"]
impl crate::Readable for EVSTAT2L {}
#[doc = "`write(|w| ..)` method takes [evstat2l::W](evstat2l::W) writer structure"]
impl crate::Writable for EVSTAT2L {}
#[doc = "Event Status 2 Low"]
pub mod evstat2l;
#[doc = "Event Status 2 High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evstat2h](evstat2h) module"]
pub type EVSTAT2H = crate::Reg<u32, _EVSTAT2H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVSTAT2H;
#[doc = "`read()` method returns [evstat2h::R](evstat2h::R) reader structure"]
impl crate::Readable for EVSTAT2H {}
#[doc = "`write(|w| ..)` method takes [evstat2h::W](evstat2h::W) writer structure"]
impl crate::Writable for EVSTAT2H {}
#[doc = "Event Status 2 High"]
pub mod evstat2h;
#[doc = "Event Status 3 Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evstat3l](evstat3l) module"]
pub type EVSTAT3L = crate::Reg<u32, _EVSTAT3L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVSTAT3L;
#[doc = "`read()` method returns [evstat3l::R](evstat3l::R) reader structure"]
impl crate::Readable for EVSTAT3L {}
#[doc = "`write(|w| ..)` method takes [evstat3l::W](evstat3l::W) writer structure"]
impl crate::Writable for EVSTAT3L {}
#[doc = "Event Status 3 Low"]
pub mod evstat3l;
#[doc = "Event Status 3 High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evstat3h](evstat3h) module"]
pub type EVSTAT3H = crate::Reg<u32, _EVSTAT3H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVSTAT3H;
#[doc = "`read()` method returns [evstat3h::R](evstat3h::R) reader structure"]
impl crate::Readable for EVSTAT3H {}
#[doc = "`write(|w| ..)` method takes [evstat3h::W](evstat3h::W) writer structure"]
impl crate::Writable for EVSTAT3H {}
#[doc = "Event Status 3 High"]
pub mod evstat3h;
