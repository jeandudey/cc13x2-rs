#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control This register is used to enable the flash patch block."]
    pub ctrl: CTRL,
    #[doc = "0x04 - Remap This register provides the remap base address location where a matched addresses are remapped. The three most significant bits and the five least significant bits of the remap base address are hard-coded to 3'b001 and 5'b00000 respectively. The remap base address must be in system space and is it required to be 8-word aligned, with one word allocated to each of the eight FPB comparators."]
    pub remap: REMAP,
    #[doc = "0x08 - Comparator 0"]
    pub comp0: COMP0,
    #[doc = "0x0c - Comparator 1"]
    pub comp1: COMP1,
    #[doc = "0x10 - Comparator 2"]
    pub comp2: COMP2,
    #[doc = "0x14 - Comparator 3"]
    pub comp3: COMP3,
    #[doc = "0x18 - Comparator 4"]
    pub comp4: COMP4,
    #[doc = "0x1c - Comparator 5"]
    pub comp5: COMP5,
    #[doc = "0x20 - Comparator 6"]
    pub comp6: COMP6,
    #[doc = "0x24 - Comparator 7"]
    pub comp7: COMP7,
}
#[doc = "Control This register is used to enable the flash patch block.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control This register is used to enable the flash patch block."]
pub mod ctrl;
#[doc = "Remap This register provides the remap base address location where a matched addresses are remapped. The three most significant bits and the five least significant bits of the remap base address are hard-coded to 3'b001 and 5'b00000 respectively. The remap base address must be in system space and is it required to be 8-word aligned, with one word allocated to each of the eight FPB comparators.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [remap](remap) module"]
pub type REMAP = crate::Reg<u32, _REMAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REMAP;
#[doc = "`read()` method returns [remap::R](remap::R) reader structure"]
impl crate::Readable for REMAP {}
#[doc = "`write(|w| ..)` method takes [remap::W](remap::W) writer structure"]
impl crate::Writable for REMAP {}
#[doc = "Remap This register provides the remap base address location where a matched addresses are remapped. The three most significant bits and the five least significant bits of the remap base address are hard-coded to 3'b001 and 5'b00000 respectively. The remap base address must be in system space and is it required to be 8-word aligned, with one word allocated to each of the eight FPB comparators."]
pub mod remap;
#[doc = "Comparator 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp0](comp0) module"]
pub type COMP0 = crate::Reg<u32, _COMP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP0;
#[doc = "`read()` method returns [comp0::R](comp0::R) reader structure"]
impl crate::Readable for COMP0 {}
#[doc = "`write(|w| ..)` method takes [comp0::W](comp0::W) writer structure"]
impl crate::Writable for COMP0 {}
#[doc = "Comparator 0"]
pub mod comp0;
#[doc = "Comparator 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp1](comp1) module"]
pub type COMP1 = crate::Reg<u32, _COMP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP1;
#[doc = "`read()` method returns [comp1::R](comp1::R) reader structure"]
impl crate::Readable for COMP1 {}
#[doc = "`write(|w| ..)` method takes [comp1::W](comp1::W) writer structure"]
impl crate::Writable for COMP1 {}
#[doc = "Comparator 1"]
pub mod comp1;
#[doc = "Comparator 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp2](comp2) module"]
pub type COMP2 = crate::Reg<u32, _COMP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP2;
#[doc = "`read()` method returns [comp2::R](comp2::R) reader structure"]
impl crate::Readable for COMP2 {}
#[doc = "`write(|w| ..)` method takes [comp2::W](comp2::W) writer structure"]
impl crate::Writable for COMP2 {}
#[doc = "Comparator 2"]
pub mod comp2;
#[doc = "Comparator 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp3](comp3) module"]
pub type COMP3 = crate::Reg<u32, _COMP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP3;
#[doc = "`read()` method returns [comp3::R](comp3::R) reader structure"]
impl crate::Readable for COMP3 {}
#[doc = "`write(|w| ..)` method takes [comp3::W](comp3::W) writer structure"]
impl crate::Writable for COMP3 {}
#[doc = "Comparator 3"]
pub mod comp3;
#[doc = "Comparator 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp4](comp4) module"]
pub type COMP4 = crate::Reg<u32, _COMP4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP4;
#[doc = "`read()` method returns [comp4::R](comp4::R) reader structure"]
impl crate::Readable for COMP4 {}
#[doc = "`write(|w| ..)` method takes [comp4::W](comp4::W) writer structure"]
impl crate::Writable for COMP4 {}
#[doc = "Comparator 4"]
pub mod comp4;
#[doc = "Comparator 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp5](comp5) module"]
pub type COMP5 = crate::Reg<u32, _COMP5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP5;
#[doc = "`read()` method returns [comp5::R](comp5::R) reader structure"]
impl crate::Readable for COMP5 {}
#[doc = "`write(|w| ..)` method takes [comp5::W](comp5::W) writer structure"]
impl crate::Writable for COMP5 {}
#[doc = "Comparator 5"]
pub mod comp5;
#[doc = "Comparator 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp6](comp6) module"]
pub type COMP6 = crate::Reg<u32, _COMP6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP6;
#[doc = "`read()` method returns [comp6::R](comp6::R) reader structure"]
impl crate::Readable for COMP6 {}
#[doc = "`write(|w| ..)` method takes [comp6::W](comp6::W) writer structure"]
impl crate::Writable for COMP6 {}
#[doc = "Comparator 6"]
pub mod comp6;
#[doc = "Comparator 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp7](comp7) module"]
pub type COMP7 = crate::Reg<u32, _COMP7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP7;
#[doc = "`read()` method returns [comp7::R](comp7::R) reader structure"]
impl crate::Readable for COMP7 {}
#[doc = "`write(|w| ..)` method takes [comp7::W](comp7::W) writer structure"]
impl crate::Writable for COMP7 {}
#[doc = "Comparator 7"]
pub mod comp7;
