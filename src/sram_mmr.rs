#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Parity Error Control Parity error check controls"]
    pub per_ctl: PER_CTL,
    #[doc = "0x04 - Parity Error Check Parity error check results"]
    pub per_chk: PER_CHK,
    #[doc = "0x08 - Parity Error Debug Parity error check debug address setting"]
    pub per_dbg: PER_DBG,
    #[doc = "0x0c - Memory Control Controls memory initialization"]
    pub mem_ctl: MEM_CTL,
}
#[doc = "Parity Error Control Parity error check controls\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [per_ctl](per_ctl) module"]
pub type PER_CTL = crate::Reg<u32, _PER_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PER_CTL;
#[doc = "`read()` method returns [per_ctl::R](per_ctl::R) reader structure"]
impl crate::Readable for PER_CTL {}
#[doc = "`write(|w| ..)` method takes [per_ctl::W](per_ctl::W) writer structure"]
impl crate::Writable for PER_CTL {}
#[doc = "Parity Error Control Parity error check controls"]
pub mod per_ctl;
#[doc = "Parity Error Check Parity error check results\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [per_chk](per_chk) module"]
pub type PER_CHK = crate::Reg<u32, _PER_CHK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PER_CHK;
#[doc = "`read()` method returns [per_chk::R](per_chk::R) reader structure"]
impl crate::Readable for PER_CHK {}
#[doc = "`write(|w| ..)` method takes [per_chk::W](per_chk::W) writer structure"]
impl crate::Writable for PER_CHK {}
#[doc = "Parity Error Check Parity error check results"]
pub mod per_chk;
#[doc = "Parity Error Debug Parity error check debug address setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [per_dbg](per_dbg) module"]
pub type PER_DBG = crate::Reg<u32, _PER_DBG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PER_DBG;
#[doc = "`read()` method returns [per_dbg::R](per_dbg::R) reader structure"]
impl crate::Readable for PER_DBG {}
#[doc = "`write(|w| ..)` method takes [per_dbg::W](per_dbg::W) writer structure"]
impl crate::Writable for PER_DBG {}
#[doc = "Parity Error Debug Parity error check debug address setting"]
pub mod per_dbg;
#[doc = "Memory Control Controls memory initialization\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_ctl](mem_ctl) module"]
pub type MEM_CTL = crate::Reg<u32, _MEM_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_CTL;
#[doc = "`read()` method returns [mem_ctl::R](mem_ctl::R) reader structure"]
impl crate::Readable for MEM_CTL {}
#[doc = "`write(|w| ..)` method takes [mem_ctl::W](mem_ctl::W) writer structure"]
impl crate::Writable for MEM_CTL {}
#[doc = "Memory Control Controls memory initialization"]
pub mod mem_ctl;
