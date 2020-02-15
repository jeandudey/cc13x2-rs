#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - AUX SCE Clock Management This register contains bitfields that are relevant for setting up the clock to the AUX domain."]
    pub auxsceclk: AUXSCECLK,
    #[doc = "0x08 - RAM Configuration This register contains power management related configuration for the SRAM in the MCU and AUX domain."]
    pub ramcfg: RAMCFG,
    _reserved2: [u8; 4usize],
    #[doc = "0x10 - Power Management Control This register controls bitfields for setting low level power management features such as selection of regulator for VDDR supply and control of IO ring where certain segments can be enabled / disabled."]
    pub pwrctl: PWRCTL,
    #[doc = "0x14 - AON Power and Reset Status This register is used to monitor various power management related signals in AON. All other signals than JTAG_PD_ON, AUX_BUS_RESET_DONE, and AUX_RESET_DONE are for test, calibration and debug purpose only."]
    pub pwrstat: PWRSTAT,
    #[doc = "0x18 - Shutdown Control This register contains bitfields required for entering shutdown mode"]
    pub shutdown: SHUTDOWN,
    #[doc = "0x1c - Recharge Controller Configuration This register sets all relevant parameters for controlling the recharge algorithm."]
    pub rechargecfg: RECHARGECFG,
    #[doc = "0x20 - Recharge Controller Status This register controls various status registers which are updated during recharge. The register is mostly intended for test and debug."]
    pub rechargestat: RECHARGESTAT,
    #[doc = "0x24 - Oscillator Configuration This register sets the period for Amplitude compensation requests sent to the oscillator control system. The amplitude compensations is only applicable when XOSC_HF is running in low power mode."]
    pub osccfg: OSCCFG,
    #[doc = "0x28 - Reset Management This register contains bitfields related to system reset such as reset source and reset request and control of brown out resets."]
    pub resetctl: RESETCTL,
    #[doc = "0x2c - Sleep Control This register is used to unfreeze the IO pad ring after waking up from SHUTDOWN"]
    pub sleepctl: SLEEPCTL,
    _reserved10: [u8; 4usize],
    #[doc = "0x34 - JTAG Configuration This register contains control for configuration of the JTAG domain. This includes permissions for each TAP."]
    pub jtagcfg: JTAGCFG,
    _reserved11: [u8; 4usize],
    #[doc = "0x3c - JTAG USERCODE Boot code copies the JTAG USERCODE to this register from where it is forwarded to the debug subsystem."]
    pub jtagusercode: JTAGUSERCODE,
}
#[doc = "AUX SCE Clock Management This register contains bitfields that are relevant for setting up the clock to the AUX domain.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auxsceclk](auxsceclk) module"]
pub type AUXSCECLK = crate::Reg<u32, _AUXSCECLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUXSCECLK;
#[doc = "`read()` method returns [auxsceclk::R](auxsceclk::R) reader structure"]
impl crate::Readable for AUXSCECLK {}
#[doc = "`write(|w| ..)` method takes [auxsceclk::W](auxsceclk::W) writer structure"]
impl crate::Writable for AUXSCECLK {}
#[doc = "AUX SCE Clock Management This register contains bitfields that are relevant for setting up the clock to the AUX domain."]
pub mod auxsceclk;
#[doc = "RAM Configuration This register contains power management related configuration for the SRAM in the MCU and AUX domain.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ramcfg](ramcfg) module"]
pub type RAMCFG = crate::Reg<u32, _RAMCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAMCFG;
#[doc = "`read()` method returns [ramcfg::R](ramcfg::R) reader structure"]
impl crate::Readable for RAMCFG {}
#[doc = "`write(|w| ..)` method takes [ramcfg::W](ramcfg::W) writer structure"]
impl crate::Writable for RAMCFG {}
#[doc = "RAM Configuration This register contains power management related configuration for the SRAM in the MCU and AUX domain."]
pub mod ramcfg;
#[doc = "Power Management Control This register controls bitfields for setting low level power management features such as selection of regulator for VDDR supply and control of IO ring where certain segments can be enabled / disabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrctl](pwrctl) module"]
pub type PWRCTL = crate::Reg<u32, _PWRCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRCTL;
#[doc = "`read()` method returns [pwrctl::R](pwrctl::R) reader structure"]
impl crate::Readable for PWRCTL {}
#[doc = "`write(|w| ..)` method takes [pwrctl::W](pwrctl::W) writer structure"]
impl crate::Writable for PWRCTL {}
#[doc = "Power Management Control This register controls bitfields for setting low level power management features such as selection of regulator for VDDR supply and control of IO ring where certain segments can be enabled / disabled."]
pub mod pwrctl;
#[doc = "AON Power and Reset Status This register is used to monitor various power management related signals in AON. All other signals than JTAG_PD_ON, AUX_BUS_RESET_DONE, and AUX_RESET_DONE are for test, calibration and debug purpose only.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrstat](pwrstat) module"]
pub type PWRSTAT = crate::Reg<u32, _PWRSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRSTAT;
#[doc = "`read()` method returns [pwrstat::R](pwrstat::R) reader structure"]
impl crate::Readable for PWRSTAT {}
#[doc = "`write(|w| ..)` method takes [pwrstat::W](pwrstat::W) writer structure"]
impl crate::Writable for PWRSTAT {}
#[doc = "AON Power and Reset Status This register is used to monitor various power management related signals in AON. All other signals than JTAG_PD_ON, AUX_BUS_RESET_DONE, and AUX_RESET_DONE are for test, calibration and debug purpose only."]
pub mod pwrstat;
#[doc = "Shutdown Control This register contains bitfields required for entering shutdown mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shutdown](shutdown) module"]
pub type SHUTDOWN = crate::Reg<u32, _SHUTDOWN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHUTDOWN;
#[doc = "`read()` method returns [shutdown::R](shutdown::R) reader structure"]
impl crate::Readable for SHUTDOWN {}
#[doc = "`write(|w| ..)` method takes [shutdown::W](shutdown::W) writer structure"]
impl crate::Writable for SHUTDOWN {}
#[doc = "Shutdown Control This register contains bitfields required for entering shutdown mode"]
pub mod shutdown;
#[doc = "Recharge Controller Configuration This register sets all relevant parameters for controlling the recharge algorithm.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rechargecfg](rechargecfg) module"]
pub type RECHARGECFG = crate::Reg<u32, _RECHARGECFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RECHARGECFG;
#[doc = "`read()` method returns [rechargecfg::R](rechargecfg::R) reader structure"]
impl crate::Readable for RECHARGECFG {}
#[doc = "`write(|w| ..)` method takes [rechargecfg::W](rechargecfg::W) writer structure"]
impl crate::Writable for RECHARGECFG {}
#[doc = "Recharge Controller Configuration This register sets all relevant parameters for controlling the recharge algorithm."]
pub mod rechargecfg;
#[doc = "Recharge Controller Status This register controls various status registers which are updated during recharge. The register is mostly intended for test and debug.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rechargestat](rechargestat) module"]
pub type RECHARGESTAT = crate::Reg<u32, _RECHARGESTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RECHARGESTAT;
#[doc = "`read()` method returns [rechargestat::R](rechargestat::R) reader structure"]
impl crate::Readable for RECHARGESTAT {}
#[doc = "`write(|w| ..)` method takes [rechargestat::W](rechargestat::W) writer structure"]
impl crate::Writable for RECHARGESTAT {}
#[doc = "Recharge Controller Status This register controls various status registers which are updated during recharge. The register is mostly intended for test and debug."]
pub mod rechargestat;
#[doc = "Oscillator Configuration This register sets the period for Amplitude compensation requests sent to the oscillator control system. The amplitude compensations is only applicable when XOSC_HF is running in low power mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osccfg](osccfg) module"]
pub type OSCCFG = crate::Reg<u32, _OSCCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCCFG;
#[doc = "`read()` method returns [osccfg::R](osccfg::R) reader structure"]
impl crate::Readable for OSCCFG {}
#[doc = "`write(|w| ..)` method takes [osccfg::W](osccfg::W) writer structure"]
impl crate::Writable for OSCCFG {}
#[doc = "Oscillator Configuration This register sets the period for Amplitude compensation requests sent to the oscillator control system. The amplitude compensations is only applicable when XOSC_HF is running in low power mode."]
pub mod osccfg;
#[doc = "Reset Management This register contains bitfields related to system reset such as reset source and reset request and control of brown out resets.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resetctl](resetctl) module"]
pub type RESETCTL = crate::Reg<u32, _RESETCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESETCTL;
#[doc = "`read()` method returns [resetctl::R](resetctl::R) reader structure"]
impl crate::Readable for RESETCTL {}
#[doc = "`write(|w| ..)` method takes [resetctl::W](resetctl::W) writer structure"]
impl crate::Writable for RESETCTL {}
#[doc = "Reset Management This register contains bitfields related to system reset such as reset source and reset request and control of brown out resets."]
pub mod resetctl;
#[doc = "Sleep Control This register is used to unfreeze the IO pad ring after waking up from SHUTDOWN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sleepctl](sleepctl) module"]
pub type SLEEPCTL = crate::Reg<u32, _SLEEPCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLEEPCTL;
#[doc = "`read()` method returns [sleepctl::R](sleepctl::R) reader structure"]
impl crate::Readable for SLEEPCTL {}
#[doc = "`write(|w| ..)` method takes [sleepctl::W](sleepctl::W) writer structure"]
impl crate::Writable for SLEEPCTL {}
#[doc = "Sleep Control This register is used to unfreeze the IO pad ring after waking up from SHUTDOWN"]
pub mod sleepctl;
#[doc = "JTAG Configuration This register contains control for configuration of the JTAG domain. This includes permissions for each TAP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jtagcfg](jtagcfg) module"]
pub type JTAGCFG = crate::Reg<u32, _JTAGCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JTAGCFG;
#[doc = "`read()` method returns [jtagcfg::R](jtagcfg::R) reader structure"]
impl crate::Readable for JTAGCFG {}
#[doc = "`write(|w| ..)` method takes [jtagcfg::W](jtagcfg::W) writer structure"]
impl crate::Writable for JTAGCFG {}
#[doc = "JTAG Configuration This register contains control for configuration of the JTAG domain. This includes permissions for each TAP."]
pub mod jtagcfg;
#[doc = "JTAG USERCODE Boot code copies the JTAG USERCODE to this register from where it is forwarded to the debug subsystem.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jtagusercode](jtagusercode) module"]
pub type JTAGUSERCODE = crate::Reg<u32, _JTAGUSERCODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JTAGUSERCODE;
#[doc = "`read()` method returns [jtagusercode::R](jtagusercode::R) reader structure"]
impl crate::Readable for JTAGUSERCODE {}
#[doc = "`write(|w| ..)` method takes [jtagusercode::W](jtagusercode::W) writer structure"]
impl crate::Writable for JTAGUSERCODE {}
#[doc = "JTAG USERCODE Boot code copies the JTAG USERCODE to this register from where it is forwarded to the debug subsystem."]
pub mod jtagusercode;
