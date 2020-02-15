#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Supported Sync Port Sizes This register represents a single port size that is supported on the device, that is, 4, 2 or 1. This is to ensure that tools do not attempt to select a port width that an attached TPA cannot capture."]
    pub sspsr: SSPSR,
    #[doc = "0x04 - Current Sync Port Size This register has the same format as SSPSR but only one bit can be set, and all others must be zero. Writing values with more than one bit set, or setting a bit that is not indicated as supported can cause Unpredictable behavior. On reset this defaults to the smallest possible port size, 1 bit."]
    pub cspsr: CSPSR,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - Async Clock Prescaler This register scales the baud rate of the asynchronous output."]
    pub acpr: ACPR,
    _reserved3: [u8; 220usize],
    #[doc = "0xf0 - Selected Pin Protocol This register selects the protocol to be used for trace output. Note: If this register is changed while trace data is being output, data corruption occurs."]
    pub sppr: SPPR,
    _reserved4: [u8; 524usize],
    #[doc = "0x300 - Formatter and Flush Status"]
    pub ffsr: FFSR,
    #[doc = "0x304 - Formatter and Flush Control When one of the two single wire output (SWO) modes is selected, ENFCONT enables the formatter to be bypassed. If the formatter is bypassed, only the ITM/DWT trace source (ATDATA2) passes through. The TPIU accepts and discards data that is presented on the ETM port (ATDATA1). This function is intended to be used when it is necessary to connect a device containing an ETM to a trace capture device that is only able to capture Serial Wire Output (SWO) data. Enabling or disabling the formatter causes momentary data corruption. Note: If the selected pin protocol register (SPPR.PROTOCOL) is set to 0x00 (TracePort mode), this register always reads 0x102, because the formatter is automatically enabled. If one of the serial wire modes is then selected, the register reverts to its previously programmed value."]
    pub ffcr: FFCR,
    #[doc = "0x308 - Formatter Synchronization Counter"]
    pub fscr: FSCR,
    _reserved7: [u8; 3220usize],
    _reserved_7_claimset: [u8; 4usize],
    _reserved_8_claimclr: [u8; 4usize],
    _reserved9: [u8; 32usize],
    #[doc = "0xfc8 - Device ID"]
    pub devid: DEVID,
}
impl RegisterBlock {
    #[doc = "0xfa0 - Claim Tag Set"]
    #[inline(always)]
    pub fn claimset(&self) -> &CLAIMSET {
        unsafe { &*(((self as *const Self) as *const u8).add(4000usize) as *const CLAIMSET) }
    }
    #[doc = "0xfa0 - Claim Tag Set"]
    #[inline(always)]
    pub fn claimset_mut(&self) -> &mut CLAIMSET {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4000usize) as *mut CLAIMSET) }
    }
    #[doc = "0xfa0 - Claim Tag Mask"]
    #[inline(always)]
    pub fn claimmask(&self) -> &CLAIMMASK {
        unsafe { &*(((self as *const Self) as *const u8).add(4000usize) as *const CLAIMMASK) }
    }
    #[doc = "0xfa0 - Claim Tag Mask"]
    #[inline(always)]
    pub fn claimmask_mut(&self) -> &mut CLAIMMASK {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4000usize) as *mut CLAIMMASK) }
    }
    #[doc = "0xfa4 - Claim Tag Clear"]
    #[inline(always)]
    pub fn claimclr(&self) -> &CLAIMCLR {
        unsafe { &*(((self as *const Self) as *const u8).add(4004usize) as *const CLAIMCLR) }
    }
    #[doc = "0xfa4 - Claim Tag Clear"]
    #[inline(always)]
    pub fn claimclr_mut(&self) -> &mut CLAIMCLR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4004usize) as *mut CLAIMCLR) }
    }
    #[doc = "0xfa4 - Current Claim Tag"]
    #[inline(always)]
    pub fn claimtag(&self) -> &CLAIMTAG {
        unsafe { &*(((self as *const Self) as *const u8).add(4004usize) as *const CLAIMTAG) }
    }
    #[doc = "0xfa4 - Current Claim Tag"]
    #[inline(always)]
    pub fn claimtag_mut(&self) -> &mut CLAIMTAG {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4004usize) as *mut CLAIMTAG) }
    }
}
#[doc = "Supported Sync Port Sizes This register represents a single port size that is supported on the device, that is, 4, 2 or 1. This is to ensure that tools do not attempt to select a port width that an attached TPA cannot capture.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sspsr](sspsr) module"]
pub type SSPSR = crate::Reg<u32, _SSPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSPSR;
#[doc = "`read()` method returns [sspsr::R](sspsr::R) reader structure"]
impl crate::Readable for SSPSR {}
#[doc = "`write(|w| ..)` method takes [sspsr::W](sspsr::W) writer structure"]
impl crate::Writable for SSPSR {}
#[doc = "Supported Sync Port Sizes This register represents a single port size that is supported on the device, that is, 4, 2 or 1. This is to ensure that tools do not attempt to select a port width that an attached TPA cannot capture."]
pub mod sspsr;
#[doc = "Current Sync Port Size This register has the same format as SSPSR but only one bit can be set, and all others must be zero. Writing values with more than one bit set, or setting a bit that is not indicated as supported can cause Unpredictable behavior. On reset this defaults to the smallest possible port size, 1 bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cspsr](cspsr) module"]
pub type CSPSR = crate::Reg<u32, _CSPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPSR;
#[doc = "`read()` method returns [cspsr::R](cspsr::R) reader structure"]
impl crate::Readable for CSPSR {}
#[doc = "`write(|w| ..)` method takes [cspsr::W](cspsr::W) writer structure"]
impl crate::Writable for CSPSR {}
#[doc = "Current Sync Port Size This register has the same format as SSPSR but only one bit can be set, and all others must be zero. Writing values with more than one bit set, or setting a bit that is not indicated as supported can cause Unpredictable behavior. On reset this defaults to the smallest possible port size, 1 bit."]
pub mod cspsr;
#[doc = "Async Clock Prescaler This register scales the baud rate of the asynchronous output.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acpr](acpr) module"]
pub type ACPR = crate::Reg<u32, _ACPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACPR;
#[doc = "`read()` method returns [acpr::R](acpr::R) reader structure"]
impl crate::Readable for ACPR {}
#[doc = "`write(|w| ..)` method takes [acpr::W](acpr::W) writer structure"]
impl crate::Writable for ACPR {}
#[doc = "Async Clock Prescaler This register scales the baud rate of the asynchronous output."]
pub mod acpr;
#[doc = "Selected Pin Protocol This register selects the protocol to be used for trace output. Note: If this register is changed while trace data is being output, data corruption occurs.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sppr](sppr) module"]
pub type SPPR = crate::Reg<u32, _SPPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPPR;
#[doc = "`read()` method returns [sppr::R](sppr::R) reader structure"]
impl crate::Readable for SPPR {}
#[doc = "`write(|w| ..)` method takes [sppr::W](sppr::W) writer structure"]
impl crate::Writable for SPPR {}
#[doc = "Selected Pin Protocol This register selects the protocol to be used for trace output. Note: If this register is changed while trace data is being output, data corruption occurs."]
pub mod sppr;
#[doc = "Formatter and Flush Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffsr](ffsr) module"]
pub type FFSR = crate::Reg<u32, _FFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FFSR;
#[doc = "`read()` method returns [ffsr::R](ffsr::R) reader structure"]
impl crate::Readable for FFSR {}
#[doc = "`write(|w| ..)` method takes [ffsr::W](ffsr::W) writer structure"]
impl crate::Writable for FFSR {}
#[doc = "Formatter and Flush Status"]
pub mod ffsr;
#[doc = "Formatter and Flush Control When one of the two single wire output (SWO) modes is selected, ENFCONT enables the formatter to be bypassed. If the formatter is bypassed, only the ITM/DWT trace source (ATDATA2) passes through. The TPIU accepts and discards data that is presented on the ETM port (ATDATA1). This function is intended to be used when it is necessary to connect a device containing an ETM to a trace capture device that is only able to capture Serial Wire Output (SWO) data. Enabling or disabling the formatter causes momentary data corruption. Note: If the selected pin protocol register (SPPR.PROTOCOL) is set to 0x00 (TracePort mode), this register always reads 0x102, because the formatter is automatically enabled. If one of the serial wire modes is then selected, the register reverts to its previously programmed value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffcr](ffcr) module"]
pub type FFCR = crate::Reg<u32, _FFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FFCR;
#[doc = "`read()` method returns [ffcr::R](ffcr::R) reader structure"]
impl crate::Readable for FFCR {}
#[doc = "`write(|w| ..)` method takes [ffcr::W](ffcr::W) writer structure"]
impl crate::Writable for FFCR {}
#[doc = "Formatter and Flush Control When one of the two single wire output (SWO) modes is selected, ENFCONT enables the formatter to be bypassed. If the formatter is bypassed, only the ITM/DWT trace source (ATDATA2) passes through. The TPIU accepts and discards data that is presented on the ETM port (ATDATA1). This function is intended to be used when it is necessary to connect a device containing an ETM to a trace capture device that is only able to capture Serial Wire Output (SWO) data. Enabling or disabling the formatter causes momentary data corruption. Note: If the selected pin protocol register (SPPR.PROTOCOL) is set to 0x00 (TracePort mode), this register always reads 0x102, because the formatter is automatically enabled. If one of the serial wire modes is then selected, the register reverts to its previously programmed value."]
pub mod ffcr;
#[doc = "Formatter Synchronization Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fscr](fscr) module"]
pub type FSCR = crate::Reg<u32, _FSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSCR;
#[doc = "`read()` method returns [fscr::R](fscr::R) reader structure"]
impl crate::Readable for FSCR {}
#[doc = "`write(|w| ..)` method takes [fscr::W](fscr::W) writer structure"]
impl crate::Writable for FSCR {}
#[doc = "Formatter Synchronization Counter"]
pub mod fscr;
#[doc = "Claim Tag Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [claimmask](claimmask) module"]
pub type CLAIMMASK = crate::Reg<u32, _CLAIMMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLAIMMASK;
#[doc = "`read()` method returns [claimmask::R](claimmask::R) reader structure"]
impl crate::Readable for CLAIMMASK {}
#[doc = "`write(|w| ..)` method takes [claimmask::W](claimmask::W) writer structure"]
impl crate::Writable for CLAIMMASK {}
#[doc = "Claim Tag Mask"]
pub mod claimmask;
#[doc = "Claim Tag Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [claimset](claimset) module"]
pub type CLAIMSET = crate::Reg<u32, _CLAIMSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLAIMSET;
#[doc = "`read()` method returns [claimset::R](claimset::R) reader structure"]
impl crate::Readable for CLAIMSET {}
#[doc = "`write(|w| ..)` method takes [claimset::W](claimset::W) writer structure"]
impl crate::Writable for CLAIMSET {}
#[doc = "Claim Tag Set"]
pub mod claimset;
#[doc = "Current Claim Tag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [claimtag](claimtag) module"]
pub type CLAIMTAG = crate::Reg<u32, _CLAIMTAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLAIMTAG;
#[doc = "`read()` method returns [claimtag::R](claimtag::R) reader structure"]
impl crate::Readable for CLAIMTAG {}
#[doc = "`write(|w| ..)` method takes [claimtag::W](claimtag::W) writer structure"]
impl crate::Writable for CLAIMTAG {}
#[doc = "Current Claim Tag"]
pub mod claimtag;
#[doc = "Claim Tag Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [claimclr](claimclr) module"]
pub type CLAIMCLR = crate::Reg<u32, _CLAIMCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLAIMCLR;
#[doc = "`read()` method returns [claimclr::R](claimclr::R) reader structure"]
impl crate::Readable for CLAIMCLR {}
#[doc = "`write(|w| ..)` method takes [claimclr::W](claimclr::W) writer structure"]
impl crate::Writable for CLAIMCLR {}
#[doc = "Claim Tag Clear"]
pub mod claimclr;
#[doc = "Device ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devid](devid) module"]
pub type DEVID = crate::Reg<u32, _DEVID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVID;
#[doc = "`read()` method returns [devid::R](devid::R) reader structure"]
impl crate::Readable for DEVID {}
#[doc = "`write(|w| ..)` method takes [devid::W](devid::W) writer structure"]
impl crate::Writable for DEVID {}
#[doc = "Device ID"]
pub mod devid;
