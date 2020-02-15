#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer 0 Configuration"]
    pub t0cfg: T0CFG,
    #[doc = "0x04 - Timer 0 Control"]
    pub t0ctl: T0CTL,
    #[doc = "0x08 - Timer 0 Target"]
    pub t0target: T0TARGET,
    #[doc = "0x0c - Timer 0 Counter"]
    pub t0cntr: T0CNTR,
    #[doc = "0x10 - Timer 1 Configuration"]
    pub t1cfg: T1CFG,
    #[doc = "0x14 - Timer 1 Control"]
    pub t1ctl: T1CTL,
    #[doc = "0x18 - Timer 1 Target Timer 1 counter target value"]
    pub t1target: T1TARGET,
    #[doc = "0x1c - Timer 1 Counter"]
    pub t1cntr: T1CNTR,
}
#[doc = "Timer 0 Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0cfg](t0cfg) module"]
pub type T0CFG = crate::Reg<u32, _T0CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T0CFG;
#[doc = "`read()` method returns [t0cfg::R](t0cfg::R) reader structure"]
impl crate::Readable for T0CFG {}
#[doc = "`write(|w| ..)` method takes [t0cfg::W](t0cfg::W) writer structure"]
impl crate::Writable for T0CFG {}
#[doc = "Timer 0 Configuration"]
pub mod t0cfg;
#[doc = "Timer 0 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0ctl](t0ctl) module"]
pub type T0CTL = crate::Reg<u32, _T0CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T0CTL;
#[doc = "`read()` method returns [t0ctl::R](t0ctl::R) reader structure"]
impl crate::Readable for T0CTL {}
#[doc = "`write(|w| ..)` method takes [t0ctl::W](t0ctl::W) writer structure"]
impl crate::Writable for T0CTL {}
#[doc = "Timer 0 Control"]
pub mod t0ctl;
#[doc = "Timer 0 Target\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0target](t0target) module"]
pub type T0TARGET = crate::Reg<u32, _T0TARGET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T0TARGET;
#[doc = "`read()` method returns [t0target::R](t0target::R) reader structure"]
impl crate::Readable for T0TARGET {}
#[doc = "`write(|w| ..)` method takes [t0target::W](t0target::W) writer structure"]
impl crate::Writable for T0TARGET {}
#[doc = "Timer 0 Target"]
pub mod t0target;
#[doc = "Timer 0 Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0cntr](t0cntr) module"]
pub type T0CNTR = crate::Reg<u32, _T0CNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T0CNTR;
#[doc = "`read()` method returns [t0cntr::R](t0cntr::R) reader structure"]
impl crate::Readable for T0CNTR {}
#[doc = "`write(|w| ..)` method takes [t0cntr::W](t0cntr::W) writer structure"]
impl crate::Writable for T0CNTR {}
#[doc = "Timer 0 Counter"]
pub mod t0cntr;
#[doc = "Timer 1 Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t1cfg](t1cfg) module"]
pub type T1CFG = crate::Reg<u32, _T1CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T1CFG;
#[doc = "`read()` method returns [t1cfg::R](t1cfg::R) reader structure"]
impl crate::Readable for T1CFG {}
#[doc = "`write(|w| ..)` method takes [t1cfg::W](t1cfg::W) writer structure"]
impl crate::Writable for T1CFG {}
#[doc = "Timer 1 Configuration"]
pub mod t1cfg;
#[doc = "Timer 1 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t1ctl](t1ctl) module"]
pub type T1CTL = crate::Reg<u32, _T1CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T1CTL;
#[doc = "`read()` method returns [t1ctl::R](t1ctl::R) reader structure"]
impl crate::Readable for T1CTL {}
#[doc = "`write(|w| ..)` method takes [t1ctl::W](t1ctl::W) writer structure"]
impl crate::Writable for T1CTL {}
#[doc = "Timer 1 Control"]
pub mod t1ctl;
#[doc = "Timer 1 Target Timer 1 counter target value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t1target](t1target) module"]
pub type T1TARGET = crate::Reg<u32, _T1TARGET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T1TARGET;
#[doc = "`read()` method returns [t1target::R](t1target::R) reader structure"]
impl crate::Readable for T1TARGET {}
#[doc = "`write(|w| ..)` method takes [t1target::W](t1target::W) writer structure"]
impl crate::Writable for T1TARGET {}
#[doc = "Timer 1 Target Timer 1 counter target value"]
pub mod t1target;
#[doc = "Timer 1 Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t1cntr](t1cntr) module"]
pub type T1CNTR = crate::Reg<u32, _T1CNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T1CNTR;
#[doc = "`read()` method returns [t1cntr::R](t1cntr::R) reader structure"]
impl crate::Readable for T1CNTR {}
#[doc = "`write(|w| ..)` method takes [t1cntr::W](t1cntr::W) writer structure"]
impl crate::Writable for T1CNTR {}
#[doc = "Timer 1 Counter"]
pub mod t1cntr;
