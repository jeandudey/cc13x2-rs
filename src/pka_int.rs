#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved_0: RESERVED_0,
    _reserved1: [u8; 4084usize],
    #[doc = "0xff8 - PKA Options register"]
    pub options: OPTIONS,
    #[doc = "0xffc - PKA hardware revision register This register allows the host access to the hardware revision number of the PKA engine for software driver matching and diagnostic purposes. It is always located at the highest address in the access space of the module and contains an encoding of the EIP number (with its complement as signature) for recognition of the hardware module."]
    pub revision: REVISION,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved_0](reserved_0) module"]
pub type RESERVED_0 = crate::Reg<u32, _RESERVED_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED_0;
#[doc = "`read()` method returns [reserved_0::R](reserved_0::R) reader structure"]
impl crate::Readable for RESERVED_0 {}
#[doc = "`write(|w| ..)` method takes [reserved_0::W](reserved_0::W) writer structure"]
impl crate::Writable for RESERVED_0 {}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved_0;
#[doc = "PKA Options register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [options](options) module"]
pub type OPTIONS = crate::Reg<u32, _OPTIONS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPTIONS;
#[doc = "`read()` method returns [options::R](options::R) reader structure"]
impl crate::Readable for OPTIONS {}
#[doc = "`write(|w| ..)` method takes [options::W](options::W) writer structure"]
impl crate::Writable for OPTIONS {}
#[doc = "PKA Options register"]
pub mod options;
#[doc = "PKA hardware revision register This register allows the host access to the hardware revision number of the PKA engine for software driver matching and diagnostic purposes. It is always located at the highest address in the access space of the module and contains an encoding of the EIP number (with its complement as signature) for recognition of the hardware module.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [revision](revision) module"]
pub type REVISION = crate::Reg<u32, _REVISION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REVISION;
#[doc = "`read()` method returns [revision::R](revision::R) reader structure"]
impl crate::Readable for REVISION {}
#[doc = "`write(|w| ..)` method takes [revision::W](revision::W) writer structure"]
impl crate::Writable for REVISION {}
#[doc = "PKA hardware revision register This register allows the host access to the hardware revision number of the PKA engine for software driver matching and diagnostic purposes. It is always located at the highest address in the access space of the module and contains an encoding of the EIP number (with its complement as signature) for recognition of the hardware module."]
pub mod revision;
