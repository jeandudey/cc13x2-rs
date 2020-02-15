#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved000: RESERVED000,
    _reserved1: [u8; 4084usize],
    #[doc = "0xff8 - Internal. Only to be used through TI provided API."]
    pub traceclkmux: TRACECLKMUX,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved000](reserved000) module"]
pub type RESERVED000 = crate::Reg<u32, _RESERVED000>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED000;
#[doc = "`read()` method returns [reserved000::R](reserved000::R) reader structure"]
impl crate::Readable for RESERVED000 {}
#[doc = "`write(|w| ..)` method takes [reserved000::W](reserved000::W) writer structure"]
impl crate::Writable for RESERVED000 {}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved000;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [traceclkmux](traceclkmux) module"]
pub type TRACECLKMUX = crate::Reg<u32, _TRACECLKMUX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRACECLKMUX;
#[doc = "`read()` method returns [traceclkmux::R](traceclkmux::R) reader structure"]
impl crate::Readable for TRACECLKMUX {}
#[doc = "`write(|w| ..)` method takes [traceclkmux::W](traceclkmux::W) writer structure"]
impl crate::Writable for TRACECLKMUX {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod traceclkmux;
