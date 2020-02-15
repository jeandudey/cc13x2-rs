#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Signed Operand 0"]
    pub op0s: OP0S,
    #[doc = "0x04 - Unsigned Operand 0"]
    pub op0u: OP0U,
    #[doc = "0x08 - Signed Operand 1 and Multiply"]
    pub op1smul: OP1SMUL,
    #[doc = "0x0c - Unsigned Operand 1 and Multiply"]
    pub op1umul: OP1UMUL,
    #[doc = "0x10 - Signed Operand 1 and Multiply-Accumulate"]
    pub op1smac: OP1SMAC,
    #[doc = "0x14 - Unsigned Operand 1 and Multiply-Accumulate"]
    pub op1umac: OP1UMAC,
    #[doc = "0x18 - Signed Operand 1 and 16-bit Addition"]
    pub op1sadd16: OP1SADD16,
    #[doc = "0x1c - Unsigned Operand 1 and 16-bit Addition"]
    pub op1uadd16: OP1UADD16,
    #[doc = "0x20 - Signed Operand 1 and 32-bit Addition"]
    pub op1sadd32: OP1SADD32,
    #[doc = "0x24 - Unsigned Operand 1 and 32-bit Addition"]
    pub op1uadd32: OP1UADD32,
    #[doc = "0x28 - Count Leading Zero"]
    pub clz: CLZ,
    #[doc = "0x2c - Count Leading Sign"]
    pub cls: CLS,
    #[doc = "0x30 - Accumulator Shift Only one shift operation can be triggered per register write."]
    pub accshift: ACCSHIFT,
    #[doc = "0x34 - Accumulator Reset"]
    pub accreset: ACCRESET,
    #[doc = "0x38 - Accumulator Bits 15:0"]
    pub acc15_0: ACC15_0,
    #[doc = "0x3c - Accumulator Bits 16:1"]
    pub acc16_1: ACC16_1,
    #[doc = "0x40 - Accumulator Bits 17:2"]
    pub acc17_2: ACC17_2,
    #[doc = "0x44 - Accumulator Bits 18:3"]
    pub acc18_3: ACC18_3,
    #[doc = "0x48 - Accumulator Bits 19:4"]
    pub acc19_4: ACC19_4,
    #[doc = "0x4c - Accumulator Bits 20:5"]
    pub acc20_5: ACC20_5,
    #[doc = "0x50 - Accumulator Bits 21:6"]
    pub acc21_6: ACC21_6,
    #[doc = "0x54 - Accumulator Bits 22:7"]
    pub acc22_7: ACC22_7,
    #[doc = "0x58 - Accumulator Bits 23:8"]
    pub acc23_8: ACC23_8,
    #[doc = "0x5c - Accumulator Bits 24:9"]
    pub acc24_9: ACC24_9,
    #[doc = "0x60 - Accumulator Bits 25:10"]
    pub acc25_10: ACC25_10,
    #[doc = "0x64 - Accumulator Bits 26:11"]
    pub acc26_11: ACC26_11,
    #[doc = "0x68 - Accumulator Bits 27:12"]
    pub acc27_12: ACC27_12,
    #[doc = "0x6c - Accumulator Bits 28:13"]
    pub acc28_13: ACC28_13,
    #[doc = "0x70 - Accumulator Bits 29:14"]
    pub acc29_14: ACC29_14,
    #[doc = "0x74 - Accumulator Bits 30:15"]
    pub acc30_15: ACC30_15,
    #[doc = "0x78 - Accumulator Bits 31:16"]
    pub acc31_16: ACC31_16,
    #[doc = "0x7c - Accumulator Bits 32:17"]
    pub acc32_17: ACC32_17,
    #[doc = "0x80 - Accumulator Bits 33:18"]
    pub acc33_18: ACC33_18,
    #[doc = "0x84 - Accumulator Bits 34:19"]
    pub acc34_19: ACC34_19,
    #[doc = "0x88 - Accumulator Bits 35:20"]
    pub acc35_20: ACC35_20,
    #[doc = "0x8c - Accumulator Bits 36:21"]
    pub acc36_21: ACC36_21,
    #[doc = "0x90 - Accumulator Bits 37:22"]
    pub acc37_22: ACC37_22,
    #[doc = "0x94 - Accumulator Bits 38:23"]
    pub acc38_23: ACC38_23,
    #[doc = "0x98 - Accumulator Bits 39:24"]
    pub acc39_24: ACC39_24,
    #[doc = "0x9c - Accumulator Bits 39:32"]
    pub acc39_32: ACC39_32,
}
#[doc = "Signed Operand 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [op0s](op0s) module"]
pub type OP0S = crate::Reg<u32, _OP0S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OP0S;
#[doc = "`read()` method returns [op0s::R](op0s::R) reader structure"]
impl crate::Readable for OP0S {}
#[doc = "`write(|w| ..)` method takes [op0s::W](op0s::W) writer structure"]
impl crate::Writable for OP0S {}
#[doc = "Signed Operand 0"]
pub mod op0s;
#[doc = "Unsigned Operand 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [op0u](op0u) module"]
pub type OP0U = crate::Reg<u32, _OP0U>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OP0U;
#[doc = "`read()` method returns [op0u::R](op0u::R) reader structure"]
impl crate::Readable for OP0U {}
#[doc = "`write(|w| ..)` method takes [op0u::W](op0u::W) writer structure"]
impl crate::Writable for OP0U {}
#[doc = "Unsigned Operand 0"]
pub mod op0u;
#[doc = "Signed Operand 1 and Multiply\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [op1smul](op1smul) module"]
pub type OP1SMUL = crate::Reg<u32, _OP1SMUL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OP1SMUL;
#[doc = "`read()` method returns [op1smul::R](op1smul::R) reader structure"]
impl crate::Readable for OP1SMUL {}
#[doc = "`write(|w| ..)` method takes [op1smul::W](op1smul::W) writer structure"]
impl crate::Writable for OP1SMUL {}
#[doc = "Signed Operand 1 and Multiply"]
pub mod op1smul;
#[doc = "Unsigned Operand 1 and Multiply\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [op1umul](op1umul) module"]
pub type OP1UMUL = crate::Reg<u32, _OP1UMUL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OP1UMUL;
#[doc = "`read()` method returns [op1umul::R](op1umul::R) reader structure"]
impl crate::Readable for OP1UMUL {}
#[doc = "`write(|w| ..)` method takes [op1umul::W](op1umul::W) writer structure"]
impl crate::Writable for OP1UMUL {}
#[doc = "Unsigned Operand 1 and Multiply"]
pub mod op1umul;
#[doc = "Signed Operand 1 and Multiply-Accumulate\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [op1smac](op1smac) module"]
pub type OP1SMAC = crate::Reg<u32, _OP1SMAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OP1SMAC;
#[doc = "`read()` method returns [op1smac::R](op1smac::R) reader structure"]
impl crate::Readable for OP1SMAC {}
#[doc = "`write(|w| ..)` method takes [op1smac::W](op1smac::W) writer structure"]
impl crate::Writable for OP1SMAC {}
#[doc = "Signed Operand 1 and Multiply-Accumulate"]
pub mod op1smac;
#[doc = "Unsigned Operand 1 and Multiply-Accumulate\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [op1umac](op1umac) module"]
pub type OP1UMAC = crate::Reg<u32, _OP1UMAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OP1UMAC;
#[doc = "`read()` method returns [op1umac::R](op1umac::R) reader structure"]
impl crate::Readable for OP1UMAC {}
#[doc = "`write(|w| ..)` method takes [op1umac::W](op1umac::W) writer structure"]
impl crate::Writable for OP1UMAC {}
#[doc = "Unsigned Operand 1 and Multiply-Accumulate"]
pub mod op1umac;
#[doc = "Signed Operand 1 and 16-bit Addition\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [op1sadd16](op1sadd16) module"]
pub type OP1SADD16 = crate::Reg<u32, _OP1SADD16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OP1SADD16;
#[doc = "`read()` method returns [op1sadd16::R](op1sadd16::R) reader structure"]
impl crate::Readable for OP1SADD16 {}
#[doc = "`write(|w| ..)` method takes [op1sadd16::W](op1sadd16::W) writer structure"]
impl crate::Writable for OP1SADD16 {}
#[doc = "Signed Operand 1 and 16-bit Addition"]
pub mod op1sadd16;
#[doc = "Unsigned Operand 1 and 16-bit Addition\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [op1uadd16](op1uadd16) module"]
pub type OP1UADD16 = crate::Reg<u32, _OP1UADD16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OP1UADD16;
#[doc = "`read()` method returns [op1uadd16::R](op1uadd16::R) reader structure"]
impl crate::Readable for OP1UADD16 {}
#[doc = "`write(|w| ..)` method takes [op1uadd16::W](op1uadd16::W) writer structure"]
impl crate::Writable for OP1UADD16 {}
#[doc = "Unsigned Operand 1 and 16-bit Addition"]
pub mod op1uadd16;
#[doc = "Signed Operand 1 and 32-bit Addition\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [op1sadd32](op1sadd32) module"]
pub type OP1SADD32 = crate::Reg<u32, _OP1SADD32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OP1SADD32;
#[doc = "`read()` method returns [op1sadd32::R](op1sadd32::R) reader structure"]
impl crate::Readable for OP1SADD32 {}
#[doc = "`write(|w| ..)` method takes [op1sadd32::W](op1sadd32::W) writer structure"]
impl crate::Writable for OP1SADD32 {}
#[doc = "Signed Operand 1 and 32-bit Addition"]
pub mod op1sadd32;
#[doc = "Unsigned Operand 1 and 32-bit Addition\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [op1uadd32](op1uadd32) module"]
pub type OP1UADD32 = crate::Reg<u32, _OP1UADD32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OP1UADD32;
#[doc = "`read()` method returns [op1uadd32::R](op1uadd32::R) reader structure"]
impl crate::Readable for OP1UADD32 {}
#[doc = "`write(|w| ..)` method takes [op1uadd32::W](op1uadd32::W) writer structure"]
impl crate::Writable for OP1UADD32 {}
#[doc = "Unsigned Operand 1 and 32-bit Addition"]
pub mod op1uadd32;
#[doc = "Count Leading Zero\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clz](clz) module"]
pub type CLZ = crate::Reg<u32, _CLZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLZ;
#[doc = "`read()` method returns [clz::R](clz::R) reader structure"]
impl crate::Readable for CLZ {}
#[doc = "`write(|w| ..)` method takes [clz::W](clz::W) writer structure"]
impl crate::Writable for CLZ {}
#[doc = "Count Leading Zero"]
pub mod clz;
#[doc = "Count Leading Sign\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cls](cls) module"]
pub type CLS = crate::Reg<u32, _CLS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLS;
#[doc = "`read()` method returns [cls::R](cls::R) reader structure"]
impl crate::Readable for CLS {}
#[doc = "`write(|w| ..)` method takes [cls::W](cls::W) writer structure"]
impl crate::Writable for CLS {}
#[doc = "Count Leading Sign"]
pub mod cls;
#[doc = "Accumulator Shift Only one shift operation can be triggered per register write.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [accshift](accshift) module"]
pub type ACCSHIFT = crate::Reg<u32, _ACCSHIFT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACCSHIFT;
#[doc = "`read()` method returns [accshift::R](accshift::R) reader structure"]
impl crate::Readable for ACCSHIFT {}
#[doc = "`write(|w| ..)` method takes [accshift::W](accshift::W) writer structure"]
impl crate::Writable for ACCSHIFT {}
#[doc = "Accumulator Shift Only one shift operation can be triggered per register write."]
pub mod accshift;
#[doc = "Accumulator Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [accreset](accreset) module"]
pub type ACCRESET = crate::Reg<u32, _ACCRESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACCRESET;
#[doc = "`read()` method returns [accreset::R](accreset::R) reader structure"]
impl crate::Readable for ACCRESET {}
#[doc = "`write(|w| ..)` method takes [accreset::W](accreset::W) writer structure"]
impl crate::Writable for ACCRESET {}
#[doc = "Accumulator Reset"]
pub mod accreset;
#[doc = "Accumulator Bits 15:0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc15_0](acc15_0) module"]
pub type ACC15_0 = crate::Reg<u32, _ACC15_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC15_0;
#[doc = "`read()` method returns [acc15_0::R](acc15_0::R) reader structure"]
impl crate::Readable for ACC15_0 {}
#[doc = "`write(|w| ..)` method takes [acc15_0::W](acc15_0::W) writer structure"]
impl crate::Writable for ACC15_0 {}
#[doc = "Accumulator Bits 15:0"]
pub mod acc15_0;
#[doc = "Accumulator Bits 16:1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc16_1](acc16_1) module"]
pub type ACC16_1 = crate::Reg<u32, _ACC16_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC16_1;
#[doc = "`read()` method returns [acc16_1::R](acc16_1::R) reader structure"]
impl crate::Readable for ACC16_1 {}
#[doc = "`write(|w| ..)` method takes [acc16_1::W](acc16_1::W) writer structure"]
impl crate::Writable for ACC16_1 {}
#[doc = "Accumulator Bits 16:1"]
pub mod acc16_1;
#[doc = "Accumulator Bits 17:2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc17_2](acc17_2) module"]
pub type ACC17_2 = crate::Reg<u32, _ACC17_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC17_2;
#[doc = "`read()` method returns [acc17_2::R](acc17_2::R) reader structure"]
impl crate::Readable for ACC17_2 {}
#[doc = "`write(|w| ..)` method takes [acc17_2::W](acc17_2::W) writer structure"]
impl crate::Writable for ACC17_2 {}
#[doc = "Accumulator Bits 17:2"]
pub mod acc17_2;
#[doc = "Accumulator Bits 18:3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc18_3](acc18_3) module"]
pub type ACC18_3 = crate::Reg<u32, _ACC18_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC18_3;
#[doc = "`read()` method returns [acc18_3::R](acc18_3::R) reader structure"]
impl crate::Readable for ACC18_3 {}
#[doc = "`write(|w| ..)` method takes [acc18_3::W](acc18_3::W) writer structure"]
impl crate::Writable for ACC18_3 {}
#[doc = "Accumulator Bits 18:3"]
pub mod acc18_3;
#[doc = "Accumulator Bits 19:4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc19_4](acc19_4) module"]
pub type ACC19_4 = crate::Reg<u32, _ACC19_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC19_4;
#[doc = "`read()` method returns [acc19_4::R](acc19_4::R) reader structure"]
impl crate::Readable for ACC19_4 {}
#[doc = "`write(|w| ..)` method takes [acc19_4::W](acc19_4::W) writer structure"]
impl crate::Writable for ACC19_4 {}
#[doc = "Accumulator Bits 19:4"]
pub mod acc19_4;
#[doc = "Accumulator Bits 20:5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc20_5](acc20_5) module"]
pub type ACC20_5 = crate::Reg<u32, _ACC20_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC20_5;
#[doc = "`read()` method returns [acc20_5::R](acc20_5::R) reader structure"]
impl crate::Readable for ACC20_5 {}
#[doc = "`write(|w| ..)` method takes [acc20_5::W](acc20_5::W) writer structure"]
impl crate::Writable for ACC20_5 {}
#[doc = "Accumulator Bits 20:5"]
pub mod acc20_5;
#[doc = "Accumulator Bits 21:6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc21_6](acc21_6) module"]
pub type ACC21_6 = crate::Reg<u32, _ACC21_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC21_6;
#[doc = "`read()` method returns [acc21_6::R](acc21_6::R) reader structure"]
impl crate::Readable for ACC21_6 {}
#[doc = "`write(|w| ..)` method takes [acc21_6::W](acc21_6::W) writer structure"]
impl crate::Writable for ACC21_6 {}
#[doc = "Accumulator Bits 21:6"]
pub mod acc21_6;
#[doc = "Accumulator Bits 22:7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc22_7](acc22_7) module"]
pub type ACC22_7 = crate::Reg<u32, _ACC22_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC22_7;
#[doc = "`read()` method returns [acc22_7::R](acc22_7::R) reader structure"]
impl crate::Readable for ACC22_7 {}
#[doc = "`write(|w| ..)` method takes [acc22_7::W](acc22_7::W) writer structure"]
impl crate::Writable for ACC22_7 {}
#[doc = "Accumulator Bits 22:7"]
pub mod acc22_7;
#[doc = "Accumulator Bits 23:8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc23_8](acc23_8) module"]
pub type ACC23_8 = crate::Reg<u32, _ACC23_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC23_8;
#[doc = "`read()` method returns [acc23_8::R](acc23_8::R) reader structure"]
impl crate::Readable for ACC23_8 {}
#[doc = "`write(|w| ..)` method takes [acc23_8::W](acc23_8::W) writer structure"]
impl crate::Writable for ACC23_8 {}
#[doc = "Accumulator Bits 23:8"]
pub mod acc23_8;
#[doc = "Accumulator Bits 24:9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc24_9](acc24_9) module"]
pub type ACC24_9 = crate::Reg<u32, _ACC24_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC24_9;
#[doc = "`read()` method returns [acc24_9::R](acc24_9::R) reader structure"]
impl crate::Readable for ACC24_9 {}
#[doc = "`write(|w| ..)` method takes [acc24_9::W](acc24_9::W) writer structure"]
impl crate::Writable for ACC24_9 {}
#[doc = "Accumulator Bits 24:9"]
pub mod acc24_9;
#[doc = "Accumulator Bits 25:10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc25_10](acc25_10) module"]
pub type ACC25_10 = crate::Reg<u32, _ACC25_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC25_10;
#[doc = "`read()` method returns [acc25_10::R](acc25_10::R) reader structure"]
impl crate::Readable for ACC25_10 {}
#[doc = "`write(|w| ..)` method takes [acc25_10::W](acc25_10::W) writer structure"]
impl crate::Writable for ACC25_10 {}
#[doc = "Accumulator Bits 25:10"]
pub mod acc25_10;
#[doc = "Accumulator Bits 26:11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc26_11](acc26_11) module"]
pub type ACC26_11 = crate::Reg<u32, _ACC26_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC26_11;
#[doc = "`read()` method returns [acc26_11::R](acc26_11::R) reader structure"]
impl crate::Readable for ACC26_11 {}
#[doc = "`write(|w| ..)` method takes [acc26_11::W](acc26_11::W) writer structure"]
impl crate::Writable for ACC26_11 {}
#[doc = "Accumulator Bits 26:11"]
pub mod acc26_11;
#[doc = "Accumulator Bits 27:12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc27_12](acc27_12) module"]
pub type ACC27_12 = crate::Reg<u32, _ACC27_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC27_12;
#[doc = "`read()` method returns [acc27_12::R](acc27_12::R) reader structure"]
impl crate::Readable for ACC27_12 {}
#[doc = "`write(|w| ..)` method takes [acc27_12::W](acc27_12::W) writer structure"]
impl crate::Writable for ACC27_12 {}
#[doc = "Accumulator Bits 27:12"]
pub mod acc27_12;
#[doc = "Accumulator Bits 28:13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc28_13](acc28_13) module"]
pub type ACC28_13 = crate::Reg<u32, _ACC28_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC28_13;
#[doc = "`read()` method returns [acc28_13::R](acc28_13::R) reader structure"]
impl crate::Readable for ACC28_13 {}
#[doc = "`write(|w| ..)` method takes [acc28_13::W](acc28_13::W) writer structure"]
impl crate::Writable for ACC28_13 {}
#[doc = "Accumulator Bits 28:13"]
pub mod acc28_13;
#[doc = "Accumulator Bits 29:14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc29_14](acc29_14) module"]
pub type ACC29_14 = crate::Reg<u32, _ACC29_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC29_14;
#[doc = "`read()` method returns [acc29_14::R](acc29_14::R) reader structure"]
impl crate::Readable for ACC29_14 {}
#[doc = "`write(|w| ..)` method takes [acc29_14::W](acc29_14::W) writer structure"]
impl crate::Writable for ACC29_14 {}
#[doc = "Accumulator Bits 29:14"]
pub mod acc29_14;
#[doc = "Accumulator Bits 30:15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc30_15](acc30_15) module"]
pub type ACC30_15 = crate::Reg<u32, _ACC30_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC30_15;
#[doc = "`read()` method returns [acc30_15::R](acc30_15::R) reader structure"]
impl crate::Readable for ACC30_15 {}
#[doc = "`write(|w| ..)` method takes [acc30_15::W](acc30_15::W) writer structure"]
impl crate::Writable for ACC30_15 {}
#[doc = "Accumulator Bits 30:15"]
pub mod acc30_15;
#[doc = "Accumulator Bits 31:16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc31_16](acc31_16) module"]
pub type ACC31_16 = crate::Reg<u32, _ACC31_16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC31_16;
#[doc = "`read()` method returns [acc31_16::R](acc31_16::R) reader structure"]
impl crate::Readable for ACC31_16 {}
#[doc = "`write(|w| ..)` method takes [acc31_16::W](acc31_16::W) writer structure"]
impl crate::Writable for ACC31_16 {}
#[doc = "Accumulator Bits 31:16"]
pub mod acc31_16;
#[doc = "Accumulator Bits 32:17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc32_17](acc32_17) module"]
pub type ACC32_17 = crate::Reg<u32, _ACC32_17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC32_17;
#[doc = "`read()` method returns [acc32_17::R](acc32_17::R) reader structure"]
impl crate::Readable for ACC32_17 {}
#[doc = "`write(|w| ..)` method takes [acc32_17::W](acc32_17::W) writer structure"]
impl crate::Writable for ACC32_17 {}
#[doc = "Accumulator Bits 32:17"]
pub mod acc32_17;
#[doc = "Accumulator Bits 33:18\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc33_18](acc33_18) module"]
pub type ACC33_18 = crate::Reg<u32, _ACC33_18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC33_18;
#[doc = "`read()` method returns [acc33_18::R](acc33_18::R) reader structure"]
impl crate::Readable for ACC33_18 {}
#[doc = "`write(|w| ..)` method takes [acc33_18::W](acc33_18::W) writer structure"]
impl crate::Writable for ACC33_18 {}
#[doc = "Accumulator Bits 33:18"]
pub mod acc33_18;
#[doc = "Accumulator Bits 34:19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc34_19](acc34_19) module"]
pub type ACC34_19 = crate::Reg<u32, _ACC34_19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC34_19;
#[doc = "`read()` method returns [acc34_19::R](acc34_19::R) reader structure"]
impl crate::Readable for ACC34_19 {}
#[doc = "`write(|w| ..)` method takes [acc34_19::W](acc34_19::W) writer structure"]
impl crate::Writable for ACC34_19 {}
#[doc = "Accumulator Bits 34:19"]
pub mod acc34_19;
#[doc = "Accumulator Bits 35:20\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc35_20](acc35_20) module"]
pub type ACC35_20 = crate::Reg<u32, _ACC35_20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC35_20;
#[doc = "`read()` method returns [acc35_20::R](acc35_20::R) reader structure"]
impl crate::Readable for ACC35_20 {}
#[doc = "`write(|w| ..)` method takes [acc35_20::W](acc35_20::W) writer structure"]
impl crate::Writable for ACC35_20 {}
#[doc = "Accumulator Bits 35:20"]
pub mod acc35_20;
#[doc = "Accumulator Bits 36:21\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc36_21](acc36_21) module"]
pub type ACC36_21 = crate::Reg<u32, _ACC36_21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC36_21;
#[doc = "`read()` method returns [acc36_21::R](acc36_21::R) reader structure"]
impl crate::Readable for ACC36_21 {}
#[doc = "`write(|w| ..)` method takes [acc36_21::W](acc36_21::W) writer structure"]
impl crate::Writable for ACC36_21 {}
#[doc = "Accumulator Bits 36:21"]
pub mod acc36_21;
#[doc = "Accumulator Bits 37:22\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc37_22](acc37_22) module"]
pub type ACC37_22 = crate::Reg<u32, _ACC37_22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC37_22;
#[doc = "`read()` method returns [acc37_22::R](acc37_22::R) reader structure"]
impl crate::Readable for ACC37_22 {}
#[doc = "`write(|w| ..)` method takes [acc37_22::W](acc37_22::W) writer structure"]
impl crate::Writable for ACC37_22 {}
#[doc = "Accumulator Bits 37:22"]
pub mod acc37_22;
#[doc = "Accumulator Bits 38:23\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc38_23](acc38_23) module"]
pub type ACC38_23 = crate::Reg<u32, _ACC38_23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC38_23;
#[doc = "`read()` method returns [acc38_23::R](acc38_23::R) reader structure"]
impl crate::Readable for ACC38_23 {}
#[doc = "`write(|w| ..)` method takes [acc38_23::W](acc38_23::W) writer structure"]
impl crate::Writable for ACC38_23 {}
#[doc = "Accumulator Bits 38:23"]
pub mod acc38_23;
#[doc = "Accumulator Bits 39:24\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc39_24](acc39_24) module"]
pub type ACC39_24 = crate::Reg<u32, _ACC39_24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC39_24;
#[doc = "`read()` method returns [acc39_24::R](acc39_24::R) reader structure"]
impl crate::Readable for ACC39_24 {}
#[doc = "`write(|w| ..)` method takes [acc39_24::W](acc39_24::W) writer structure"]
impl crate::Writable for ACC39_24 {}
#[doc = "Accumulator Bits 39:24"]
pub mod acc39_24;
#[doc = "Accumulator Bits 39:32\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc39_32](acc39_32) module"]
pub type ACC39_32 = crate::Reg<u32, _ACC39_32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC39_32;
#[doc = "`read()` method returns [acc39_32::R](acc39_32::R) reader structure"]
impl crate::Readable for ACC39_32 {}
#[doc = "`write(|w| ..)` method takes [acc39_32::W](acc39_32::W) writer structure"]
impl crate::Writable for ACC39_32 {}
#[doc = "Accumulator Bits 39:32"]
pub mod acc39_32;
