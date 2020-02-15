#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Wake-up Selector For MCU This register contains pointers to 4 of 8 events (events 0 to 3) which are routed to AON_PMCTRL as wakeup sources for MCU. AON_PMCTRL will start a wakeup sequence for the MCU domain when either of the 8 selected events are asserted. A wakeup sequence will guarantee that the MCU power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for MCU. Note: It is required to setup a wakeup event in AON_EVENT before MCU is requesting powerdown ( PRCM requests uLDO, see conditions in PRCM:VDCTL.ULDO )."]
    pub mcuwusel: MCUWUSEL,
    #[doc = "0x04 - Wake-up Selector For MCU This register contains pointers to 4 of 8 events (events 4 to 7) which are routed to AON_PMCTRL as wakeup sources for MCU. AON_PMCTRL will start a wakeup sequence for the MCU domain when either of the 8 selected events are asserted. A wakeup sequence will guarantee that the MCU power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for MCU. Note: It is required to setup a wakeup event in AON_EVENT before MCU is requesting powerdown ( PRCM requests uLDO, see conditions in PRCM:VDCTL.ULDO )."]
    pub mcuwusel1: MCUWUSEL1,
    #[doc = "0x08 - Event Selector For MCU Event Fabric This register contains pointers for 3 AON events that are routed to the MCU Event Fabric EVENT"]
    pub evtomcusel: EVTOMCUSEL,
    #[doc = "0x0c - RTC Capture Event Selector For AON_RTC This register contains a pointer to select an AON event for RTC capture. Please refer to AON_RTC:CH1CAPT"]
    pub rtcsel: RTCSEL,
}
#[doc = "Wake-up Selector For MCU This register contains pointers to 4 of 8 events (events 0 to 3) which are routed to AON_PMCTRL as wakeup sources for MCU. AON_PMCTRL will start a wakeup sequence for the MCU domain when either of the 8 selected events are asserted. A wakeup sequence will guarantee that the MCU power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for MCU. Note: It is required to setup a wakeup event in AON_EVENT before MCU is requesting powerdown ( PRCM requests uLDO, see conditions in PRCM:VDCTL.ULDO ).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcuwusel](mcuwusel) module"]
pub type MCUWUSEL = crate::Reg<u32, _MCUWUSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCUWUSEL;
#[doc = "`read()` method returns [mcuwusel::R](mcuwusel::R) reader structure"]
impl crate::Readable for MCUWUSEL {}
#[doc = "`write(|w| ..)` method takes [mcuwusel::W](mcuwusel::W) writer structure"]
impl crate::Writable for MCUWUSEL {}
#[doc = "Wake-up Selector For MCU This register contains pointers to 4 of 8 events (events 0 to 3) which are routed to AON_PMCTRL as wakeup sources for MCU. AON_PMCTRL will start a wakeup sequence for the MCU domain when either of the 8 selected events are asserted. A wakeup sequence will guarantee that the MCU power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for MCU. Note: It is required to setup a wakeup event in AON_EVENT before MCU is requesting powerdown ( PRCM requests uLDO, see conditions in PRCM:VDCTL.ULDO )."]
pub mod mcuwusel;
#[doc = "Wake-up Selector For MCU This register contains pointers to 4 of 8 events (events 4 to 7) which are routed to AON_PMCTRL as wakeup sources for MCU. AON_PMCTRL will start a wakeup sequence for the MCU domain when either of the 8 selected events are asserted. A wakeup sequence will guarantee that the MCU power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for MCU. Note: It is required to setup a wakeup event in AON_EVENT before MCU is requesting powerdown ( PRCM requests uLDO, see conditions in PRCM:VDCTL.ULDO ).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcuwusel1](mcuwusel1) module"]
pub type MCUWUSEL1 = crate::Reg<u32, _MCUWUSEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCUWUSEL1;
#[doc = "`read()` method returns [mcuwusel1::R](mcuwusel1::R) reader structure"]
impl crate::Readable for MCUWUSEL1 {}
#[doc = "`write(|w| ..)` method takes [mcuwusel1::W](mcuwusel1::W) writer structure"]
impl crate::Writable for MCUWUSEL1 {}
#[doc = "Wake-up Selector For MCU This register contains pointers to 4 of 8 events (events 4 to 7) which are routed to AON_PMCTRL as wakeup sources for MCU. AON_PMCTRL will start a wakeup sequence for the MCU domain when either of the 8 selected events are asserted. A wakeup sequence will guarantee that the MCU power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for MCU. Note: It is required to setup a wakeup event in AON_EVENT before MCU is requesting powerdown ( PRCM requests uLDO, see conditions in PRCM:VDCTL.ULDO )."]
pub mod mcuwusel1;
#[doc = "Event Selector For MCU Event Fabric This register contains pointers for 3 AON events that are routed to the MCU Event Fabric EVENT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtomcusel](evtomcusel) module"]
pub type EVTOMCUSEL = crate::Reg<u32, _EVTOMCUSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVTOMCUSEL;
#[doc = "`read()` method returns [evtomcusel::R](evtomcusel::R) reader structure"]
impl crate::Readable for EVTOMCUSEL {}
#[doc = "`write(|w| ..)` method takes [evtomcusel::W](evtomcusel::W) writer structure"]
impl crate::Writable for EVTOMCUSEL {}
#[doc = "Event Selector For MCU Event Fabric This register contains pointers for 3 AON events that are routed to the MCU Event Fabric EVENT"]
pub mod evtomcusel;
#[doc = "RTC Capture Event Selector For AON_RTC This register contains a pointer to select an AON event for RTC capture. Please refer to AON_RTC:CH1CAPT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcsel](rtcsel) module"]
pub type RTCSEL = crate::Reg<u32, _RTCSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCSEL;
#[doc = "`read()` method returns [rtcsel::R](rtcsel::R) reader structure"]
impl crate::Readable for RTCSEL {}
#[doc = "`write(|w| ..)` method takes [rtcsel::W](rtcsel::W) writer structure"]
impl crate::Writable for RTCSEL {}
#[doc = "RTC Capture Event Selector For AON_RTC This register contains a pointer to select an AON event for RTC capture. Please refer to AON_RTC:CH1CAPT"]
pub mod rtcsel;
