#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer Control"]
    pub ctl: CTL,
    #[doc = "0x04 - Target User defined counter target."]
    pub target: TARGET,
    #[doc = "0x08 - Shadow Target"]
    pub shdwtarget: SHDWTARGET,
    #[doc = "0x0c - Counter"]
    pub cntr: CNTR,
    #[doc = "0x10 - Clock Prescaler Configuration"]
    pub precfg: PRECFG,
    #[doc = "0x14 - Event Control Set and clear individual events manually. Manual update of an event takes priority over automatic channel updates to the same event. You cannot set and clear an event at the same time, such requests will be neglected. An event can be automatically cleared, set, toggled, or pulsed by each channel, listed in decreasing order of priority. The action with highest priority happens when multiple channels want to update an event at the same time. The four events connect to the asynchronous AUX event bus: - Event 0 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0. - Event 1 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1. - Event 2 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2. - Event 3 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3."]
    pub evctl: EVCTL,
    #[doc = "0x18 - Pulse Trigger"]
    pub pulsetrig: PULSETRIG,
    _reserved7: [u8; 100usize],
    #[doc = "0x80 - Channel 0 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
    pub ch0evcfg: CH0EVCFG,
    #[doc = "0x84 - Channel 0 Capture Configuration"]
    pub ch0ccfg: CH0CCFG,
    #[doc = "0x88 - Channel 0 Pipeline Capture Compare"]
    pub ch0pcc: CH0PCC,
    #[doc = "0x8c - Channel 0 Capture Compare"]
    pub ch0cc: CH0CC,
    #[doc = "0x90 - Channel 1 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
    pub ch1evcfg: CH1EVCFG,
    #[doc = "0x94 - Channel 1 Capture Configuration"]
    pub ch1ccfg: CH1CCFG,
    #[doc = "0x98 - Channel 1 Pipeline Capture Compare"]
    pub ch1pcc: CH1PCC,
    #[doc = "0x9c - Channel 1 Capture Compare"]
    pub ch1cc: CH1CC,
    #[doc = "0xa0 - Channel 2 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
    pub ch2evcfg: CH2EVCFG,
    #[doc = "0xa4 - Channel 2 Capture Configuration"]
    pub ch2ccfg: CH2CCFG,
    #[doc = "0xa8 - Channel 2 Pipeline Capture Compare"]
    pub ch2pcc: CH2PCC,
    #[doc = "0xac - Channel 2 Capture Compare"]
    pub ch2cc: CH2CC,
    #[doc = "0xb0 - Channel 3 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
    pub ch3evcfg: CH3EVCFG,
    #[doc = "0xb4 - Channel 3 Capture Configuration"]
    pub ch3ccfg: CH3CCFG,
    #[doc = "0xb8 - Channel 3 Pipeline Capture Compare"]
    pub ch3pcc: CH3PCC,
    #[doc = "0xbc - Channel 3 Capture Compare"]
    pub ch3cc: CH3CC,
}
#[doc = "Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "Timer Control"]
pub mod ctl;
#[doc = "Target User defined counter target.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [target](target) module"]
pub type TARGET = crate::Reg<u32, _TARGET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TARGET;
#[doc = "`read()` method returns [target::R](target::R) reader structure"]
impl crate::Readable for TARGET {}
#[doc = "`write(|w| ..)` method takes [target::W](target::W) writer structure"]
impl crate::Writable for TARGET {}
#[doc = "Target User defined counter target."]
pub mod target;
#[doc = "Shadow Target\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shdwtarget](shdwtarget) module"]
pub type SHDWTARGET = crate::Reg<u32, _SHDWTARGET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHDWTARGET;
#[doc = "`read()` method returns [shdwtarget::R](shdwtarget::R) reader structure"]
impl crate::Readable for SHDWTARGET {}
#[doc = "`write(|w| ..)` method takes [shdwtarget::W](shdwtarget::W) writer structure"]
impl crate::Writable for SHDWTARGET {}
#[doc = "Shadow Target"]
pub mod shdwtarget;
#[doc = "Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntr](cntr) module"]
pub type CNTR = crate::Reg<u32, _CNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTR;
#[doc = "`read()` method returns [cntr::R](cntr::R) reader structure"]
impl crate::Readable for CNTR {}
#[doc = "`write(|w| ..)` method takes [cntr::W](cntr::W) writer structure"]
impl crate::Writable for CNTR {}
#[doc = "Counter"]
pub mod cntr;
#[doc = "Clock Prescaler Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [precfg](precfg) module"]
pub type PRECFG = crate::Reg<u32, _PRECFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRECFG;
#[doc = "`read()` method returns [precfg::R](precfg::R) reader structure"]
impl crate::Readable for PRECFG {}
#[doc = "`write(|w| ..)` method takes [precfg::W](precfg::W) writer structure"]
impl crate::Writable for PRECFG {}
#[doc = "Clock Prescaler Configuration"]
pub mod precfg;
#[doc = "Event Control Set and clear individual events manually. Manual update of an event takes priority over automatic channel updates to the same event. You cannot set and clear an event at the same time, such requests will be neglected. An event can be automatically cleared, set, toggled, or pulsed by each channel, listed in decreasing order of priority. The action with highest priority happens when multiple channels want to update an event at the same time. The four events connect to the asynchronous AUX event bus: - Event 0 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0. - Event 1 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1. - Event 2 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2. - Event 3 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evctl](evctl) module"]
pub type EVCTL = crate::Reg<u32, _EVCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVCTL;
#[doc = "`read()` method returns [evctl::R](evctl::R) reader structure"]
impl crate::Readable for EVCTL {}
#[doc = "`write(|w| ..)` method takes [evctl::W](evctl::W) writer structure"]
impl crate::Writable for EVCTL {}
#[doc = "Event Control Set and clear individual events manually. Manual update of an event takes priority over automatic channel updates to the same event. You cannot set and clear an event at the same time, such requests will be neglected. An event can be automatically cleared, set, toggled, or pulsed by each channel, listed in decreasing order of priority. The action with highest priority happens when multiple channels want to update an event at the same time. The four events connect to the asynchronous AUX event bus: - Event 0 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0. - Event 1 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1. - Event 2 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2. - Event 3 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3."]
pub mod evctl;
#[doc = "Pulse Trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pulsetrig](pulsetrig) module"]
pub type PULSETRIG = crate::Reg<u32, _PULSETRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PULSETRIG;
#[doc = "`read()` method returns [pulsetrig::R](pulsetrig::R) reader structure"]
impl crate::Readable for PULSETRIG {}
#[doc = "`write(|w| ..)` method takes [pulsetrig::W](pulsetrig::W) writer structure"]
impl crate::Writable for PULSETRIG {}
#[doc = "Pulse Trigger"]
pub mod pulsetrig;
#[doc = "Channel 0 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0evcfg](ch0evcfg) module"]
pub type CH0EVCFG = crate::Reg<u32, _CH0EVCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0EVCFG;
#[doc = "`read()` method returns [ch0evcfg::R](ch0evcfg::R) reader structure"]
impl crate::Readable for CH0EVCFG {}
#[doc = "`write(|w| ..)` method takes [ch0evcfg::W](ch0evcfg::W) writer structure"]
impl crate::Writable for CH0EVCFG {}
#[doc = "Channel 0 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
pub mod ch0evcfg;
#[doc = "Channel 0 Capture Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0ccfg](ch0ccfg) module"]
pub type CH0CCFG = crate::Reg<u32, _CH0CCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0CCFG;
#[doc = "`read()` method returns [ch0ccfg::R](ch0ccfg::R) reader structure"]
impl crate::Readable for CH0CCFG {}
#[doc = "`write(|w| ..)` method takes [ch0ccfg::W](ch0ccfg::W) writer structure"]
impl crate::Writable for CH0CCFG {}
#[doc = "Channel 0 Capture Configuration"]
pub mod ch0ccfg;
#[doc = "Channel 0 Pipeline Capture Compare\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0pcc](ch0pcc) module"]
pub type CH0PCC = crate::Reg<u32, _CH0PCC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0PCC;
#[doc = "`read()` method returns [ch0pcc::R](ch0pcc::R) reader structure"]
impl crate::Readable for CH0PCC {}
#[doc = "`write(|w| ..)` method takes [ch0pcc::W](ch0pcc::W) writer structure"]
impl crate::Writable for CH0PCC {}
#[doc = "Channel 0 Pipeline Capture Compare"]
pub mod ch0pcc;
#[doc = "Channel 0 Capture Compare\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0cc](ch0cc) module"]
pub type CH0CC = crate::Reg<u32, _CH0CC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0CC;
#[doc = "`read()` method returns [ch0cc::R](ch0cc::R) reader structure"]
impl crate::Readable for CH0CC {}
#[doc = "`write(|w| ..)` method takes [ch0cc::W](ch0cc::W) writer structure"]
impl crate::Writable for CH0CC {}
#[doc = "Channel 0 Capture Compare"]
pub mod ch0cc;
#[doc = "Channel 1 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1evcfg](ch1evcfg) module"]
pub type CH1EVCFG = crate::Reg<u32, _CH1EVCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1EVCFG;
#[doc = "`read()` method returns [ch1evcfg::R](ch1evcfg::R) reader structure"]
impl crate::Readable for CH1EVCFG {}
#[doc = "`write(|w| ..)` method takes [ch1evcfg::W](ch1evcfg::W) writer structure"]
impl crate::Writable for CH1EVCFG {}
#[doc = "Channel 1 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
pub mod ch1evcfg;
#[doc = "Channel 1 Capture Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1ccfg](ch1ccfg) module"]
pub type CH1CCFG = crate::Reg<u32, _CH1CCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1CCFG;
#[doc = "`read()` method returns [ch1ccfg::R](ch1ccfg::R) reader structure"]
impl crate::Readable for CH1CCFG {}
#[doc = "`write(|w| ..)` method takes [ch1ccfg::W](ch1ccfg::W) writer structure"]
impl crate::Writable for CH1CCFG {}
#[doc = "Channel 1 Capture Configuration"]
pub mod ch1ccfg;
#[doc = "Channel 1 Pipeline Capture Compare\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1pcc](ch1pcc) module"]
pub type CH1PCC = crate::Reg<u32, _CH1PCC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1PCC;
#[doc = "`read()` method returns [ch1pcc::R](ch1pcc::R) reader structure"]
impl crate::Readable for CH1PCC {}
#[doc = "`write(|w| ..)` method takes [ch1pcc::W](ch1pcc::W) writer structure"]
impl crate::Writable for CH1PCC {}
#[doc = "Channel 1 Pipeline Capture Compare"]
pub mod ch1pcc;
#[doc = "Channel 1 Capture Compare\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1cc](ch1cc) module"]
pub type CH1CC = crate::Reg<u32, _CH1CC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1CC;
#[doc = "`read()` method returns [ch1cc::R](ch1cc::R) reader structure"]
impl crate::Readable for CH1CC {}
#[doc = "`write(|w| ..)` method takes [ch1cc::W](ch1cc::W) writer structure"]
impl crate::Writable for CH1CC {}
#[doc = "Channel 1 Capture Compare"]
pub mod ch1cc;
#[doc = "Channel 2 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2evcfg](ch2evcfg) module"]
pub type CH2EVCFG = crate::Reg<u32, _CH2EVCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2EVCFG;
#[doc = "`read()` method returns [ch2evcfg::R](ch2evcfg::R) reader structure"]
impl crate::Readable for CH2EVCFG {}
#[doc = "`write(|w| ..)` method takes [ch2evcfg::W](ch2evcfg::W) writer structure"]
impl crate::Writable for CH2EVCFG {}
#[doc = "Channel 2 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
pub mod ch2evcfg;
#[doc = "Channel 2 Capture Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2ccfg](ch2ccfg) module"]
pub type CH2CCFG = crate::Reg<u32, _CH2CCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2CCFG;
#[doc = "`read()` method returns [ch2ccfg::R](ch2ccfg::R) reader structure"]
impl crate::Readable for CH2CCFG {}
#[doc = "`write(|w| ..)` method takes [ch2ccfg::W](ch2ccfg::W) writer structure"]
impl crate::Writable for CH2CCFG {}
#[doc = "Channel 2 Capture Configuration"]
pub mod ch2ccfg;
#[doc = "Channel 2 Pipeline Capture Compare\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2pcc](ch2pcc) module"]
pub type CH2PCC = crate::Reg<u32, _CH2PCC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2PCC;
#[doc = "`read()` method returns [ch2pcc::R](ch2pcc::R) reader structure"]
impl crate::Readable for CH2PCC {}
#[doc = "`write(|w| ..)` method takes [ch2pcc::W](ch2pcc::W) writer structure"]
impl crate::Writable for CH2PCC {}
#[doc = "Channel 2 Pipeline Capture Compare"]
pub mod ch2pcc;
#[doc = "Channel 2 Capture Compare\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2cc](ch2cc) module"]
pub type CH2CC = crate::Reg<u32, _CH2CC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2CC;
#[doc = "`read()` method returns [ch2cc::R](ch2cc::R) reader structure"]
impl crate::Readable for CH2CC {}
#[doc = "`write(|w| ..)` method takes [ch2cc::W](ch2cc::W) writer structure"]
impl crate::Writable for CH2CC {}
#[doc = "Channel 2 Capture Compare"]
pub mod ch2cc;
#[doc = "Channel 3 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3evcfg](ch3evcfg) module"]
pub type CH3EVCFG = crate::Reg<u32, _CH3EVCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3EVCFG;
#[doc = "`read()` method returns [ch3evcfg::R](ch3evcfg::R) reader structure"]
impl crate::Readable for CH3EVCFG {}
#[doc = "`write(|w| ..)` method takes [ch3evcfg::W](ch3evcfg::W) writer structure"]
impl crate::Writable for CH3EVCFG {}
#[doc = "Channel 3 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
pub mod ch3evcfg;
#[doc = "Channel 3 Capture Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3ccfg](ch3ccfg) module"]
pub type CH3CCFG = crate::Reg<u32, _CH3CCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3CCFG;
#[doc = "`read()` method returns [ch3ccfg::R](ch3ccfg::R) reader structure"]
impl crate::Readable for CH3CCFG {}
#[doc = "`write(|w| ..)` method takes [ch3ccfg::W](ch3ccfg::W) writer structure"]
impl crate::Writable for CH3CCFG {}
#[doc = "Channel 3 Capture Configuration"]
pub mod ch3ccfg;
#[doc = "Channel 3 Pipeline Capture Compare\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3pcc](ch3pcc) module"]
pub type CH3PCC = crate::Reg<u32, _CH3PCC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3PCC;
#[doc = "`read()` method returns [ch3pcc::R](ch3pcc::R) reader structure"]
impl crate::Readable for CH3PCC {}
#[doc = "`write(|w| ..)` method takes [ch3pcc::W](ch3pcc::W) writer structure"]
impl crate::Writable for CH3PCC {}
#[doc = "Channel 3 Pipeline Capture Compare"]
pub mod ch3pcc;
#[doc = "Channel 3 Capture Compare\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3cc](ch3cc) module"]
pub type CH3CC = crate::Reg<u32, _CH3CC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3CC;
#[doc = "`read()` method returns [ch3cc::R](ch3cc::R) reader structure"]
impl crate::Readable for CH3CC {}
#[doc = "`write(|w| ..)` method takes [ch3cc::W](ch3cc::W) writer structure"]
impl crate::Writable for CH3CC {}
#[doc = "Channel 3 Capture Compare"]
pub mod ch3cc;
