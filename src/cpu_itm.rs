#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Stimulus Port 0"]
    pub stim0: STIM0,
    #[doc = "0x04 - Stimulus Port 1"]
    pub stim1: STIM1,
    #[doc = "0x08 - Stimulus Port 2"]
    pub stim2: STIM2,
    #[doc = "0x0c - Stimulus Port 3"]
    pub stim3: STIM3,
    #[doc = "0x10 - Stimulus Port 4"]
    pub stim4: STIM4,
    #[doc = "0x14 - Stimulus Port 5"]
    pub stim5: STIM5,
    #[doc = "0x18 - Stimulus Port 6"]
    pub stim6: STIM6,
    #[doc = "0x1c - Stimulus Port 7"]
    pub stim7: STIM7,
    #[doc = "0x20 - Stimulus Port 8"]
    pub stim8: STIM8,
    #[doc = "0x24 - Stimulus Port 9"]
    pub stim9: STIM9,
    #[doc = "0x28 - Stimulus Port 10"]
    pub stim10: STIM10,
    #[doc = "0x2c - Stimulus Port 11"]
    pub stim11: STIM11,
    #[doc = "0x30 - Stimulus Port 12"]
    pub stim12: STIM12,
    #[doc = "0x34 - Stimulus Port 13"]
    pub stim13: STIM13,
    #[doc = "0x38 - Stimulus Port 14"]
    pub stim14: STIM14,
    #[doc = "0x3c - Stimulus Port 15"]
    pub stim15: STIM15,
    #[doc = "0x40 - Stimulus Port 16"]
    pub stim16: STIM16,
    #[doc = "0x44 - Stimulus Port 17"]
    pub stim17: STIM17,
    #[doc = "0x48 - Stimulus Port 18"]
    pub stim18: STIM18,
    #[doc = "0x4c - Stimulus Port 19"]
    pub stim19: STIM19,
    #[doc = "0x50 - Stimulus Port 20"]
    pub stim20: STIM20,
    #[doc = "0x54 - Stimulus Port 21"]
    pub stim21: STIM21,
    #[doc = "0x58 - Stimulus Port 22"]
    pub stim22: STIM22,
    #[doc = "0x5c - Stimulus Port 23"]
    pub stim23: STIM23,
    #[doc = "0x60 - Stimulus Port 24"]
    pub stim24: STIM24,
    #[doc = "0x64 - Stimulus Port 25"]
    pub stim25: STIM25,
    #[doc = "0x68 - Stimulus Port 26"]
    pub stim26: STIM26,
    #[doc = "0x6c - Stimulus Port 27"]
    pub stim27: STIM27,
    #[doc = "0x70 - Stimulus Port 28"]
    pub stim28: STIM28,
    #[doc = "0x74 - Stimulus Port 29"]
    pub stim29: STIM29,
    #[doc = "0x78 - Stimulus Port 30"]
    pub stim30: STIM30,
    #[doc = "0x7c - Stimulus Port 31"]
    pub stim31: STIM31,
    _reserved32: [u8; 3456usize],
    #[doc = "0xe00 - Trace Enable Use the Trace Enable Register to generate trace data by writing to the corresponding stimulus port. Note: Privileged writes are accepted to this register if TCR.ITMENA is set. User writes are accepted to this register if TCR.ITMENA is set and the appropriate privilege mask is cleared. Privileged access to the stimulus ports enables an RTOS kernel to guarantee instrumentation slots or bandwidth as required."]
    pub ter: TER,
    _reserved33: [u8; 60usize],
    #[doc = "0xe40 - Trace Privilege This register is used to enable an operating system to control which stimulus ports are accessible by user code. This register can only be used in privileged mode."]
    pub tpr: TPR,
    _reserved34: [u8; 60usize],
    #[doc = "0xe80 - Trace Control Use this register to configure and control ITM transfers. This register can only be written in privilege mode. DWT is not enabled in the ITM block. However, DWT stimulus entry into the FIFO is controlled by DWTENA. If DWT requires timestamping, the TSENA bit must be set."]
    pub tcr: TCR,
    _reserved35: [u8; 300usize],
    #[doc = "0xfb0 - Lock Access This register is used to prevent write accesses to the Control Registers: TER, TPR and TCR."]
    pub lar: LAR,
    #[doc = "0xfb4 - Lock Status Use this register to enable write accesses to the Control Register."]
    pub lsr: LSR,
}
#[doc = "Stimulus Port 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim0](stim0) module"]
pub type STIM0 = crate::Reg<u32, _STIM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIM0;
#[doc = "`read()` method returns [stim0::R](stim0::R) reader structure"]
impl crate::Readable for STIM0 {}
#[doc = "`write(|w| ..)` method takes [stim0::W](stim0::W) writer structure"]
impl crate::Writable for STIM0 {}
#[doc = "Stimulus Port 0"]
pub mod stim0;
#[doc = "Stimulus Port 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim1](stim1) module"]
pub type STIM1 = crate::Reg<u32, _STIM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIM1;
#[doc = "`read()` method returns [stim1::R](stim1::R) reader structure"]
impl crate::Readable for STIM1 {}
#[doc = "`write(|w| ..)` method takes [stim1::W](stim1::W) writer structure"]
impl crate::Writable for STIM1 {}
#[doc = "Stimulus Port 1"]
pub mod stim1;
#[doc = "Stimulus Port 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim2](stim2) module"]
pub type STIM2 = crate::Reg<u32, _STIM2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIM2;
#[doc = "`read()` method returns [stim2::R](stim2::R) reader structure"]
impl crate::Readable for STIM2 {}
#[doc = "`write(|w| ..)` method takes [stim2::W](stim2::W) writer structure"]
impl crate::Writable for STIM2 {}
#[doc = "Stimulus Port 2"]
pub mod stim2;
#[doc = "Stimulus Port 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim3](stim3) module"]
pub type STIM3 = crate::Reg<u32, _STIM3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIM3;
#[doc = "`read()` method returns [stim3::R](stim3::R) reader structure"]
impl crate::Readable for STIM3 {}
#[doc = "`write(|w| ..)` method takes [stim3::W](stim3::W) writer structure"]
impl crate::Writable for STIM3 {}
#[doc = "Stimulus Port 3"]
pub mod stim3;
#[doc = "Stimulus Port 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim4](stim4) module"]
pub type STIM4 = crate::Reg<u32, _STIM4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIM4;
#[doc = "`read()` method returns [stim4::R](stim4::R) reader structure"]
impl crate::Readable for STIM4 {}
#[doc = "`write(|w| ..)` method takes [stim4::W](stim4::W) writer structure"]
impl crate::Writable for STIM4 {}
#[doc = "Stimulus Port 4"]
pub mod stim4;
#[doc = "Stimulus Port 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim5](stim5) module"]
pub type STIM5 = crate::Reg<u32, _STIM5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIM5;
#[doc = "`read()` method returns [stim5::R](stim5::R) reader structure"]
impl crate::Readable for STIM5 {}
#[doc = "`write(|w| ..)` method takes [stim5::W](stim5::W) writer structure"]
impl crate::Writable for STIM5 {}
#[doc = "Stimulus Port 5"]
pub mod stim5;
#[doc = "Stimulus Port 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim6](stim6) module"]
pub type STIM6 = crate::Reg<u32, _STIM6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIM6;
#[doc = "`read()` method returns [stim6::R](stim6::R) reader structure"]
impl crate::Readable for STIM6 {}
#[doc = "`write(|w| ..)` method takes [stim6::W](stim6::W) writer structure"]
impl crate::Writable for STIM6 {}
#[doc = "Stimulus Port 6"]
pub mod stim6;
#[doc = "Stimulus Port 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim7](stim7) module"]
pub type STIM7 = crate::Reg<u32, _STIM7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIM7;
#[doc = "`read()` method returns [stim7::R](stim7::R) reader structure"]
impl crate::Readable for STIM7 {}
#[doc = "`write(|w| ..)` method takes [stim7::W](stim7::W) writer structure"]
impl crate::Writable for STIM7 {}
#[doc = "Stimulus Port 7"]
pub mod stim7;
#[doc = "Stimulus Port 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim8](stim8) module"]
pub type STIM8 = crate::Reg<u32, _STIM8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIM8;
#[doc = "`read()` method returns [stim8::R](stim8::R) reader structure"]
impl crate::Readable for STIM8 {}
#[doc = "`write(|w| ..)` method takes [stim8::W](stim8::W) writer structure"]
impl crate::Writable for STIM8 {}
#[doc = "Stimulus Port 8"]
pub mod stim8;
#[doc = "Stimulus Port 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim9](stim9) module"]
pub type STIM9 = crate::Reg<u32, _STIM9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIM9;
#[doc = "`read()` method returns [stim9::R](stim9::R) reader structure"]
impl crate::Readable for STIM9 {}
#[doc = "`write(|w| ..)` method takes [stim9::W](stim9::W) writer structure"]
impl crate::Writable for STIM9 {}
#[doc = "Stimulus Port 9"]
pub mod stim9;
#[doc = "Stimulus Port 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim10](stim10) module"]
pub type STIM10 = crate::Reg<u32, _STIM10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIM10;
#[doc = "`read()` method returns [stim10::R](stim10::R) reader structure"]
impl crate::Readable for STIM10 {}
#[doc = "`write(|w| ..)` method takes [stim10::W](stim10::W) writer structure"]
impl crate::Writable for STIM10 {}
#[doc = "Stimulus Port 10"]
pub mod stim10;
#[doc = "Stimulus Port 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim11](stim11) module"]
pub type STIM11 = crate::Reg<u32, _STIM11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIM11;
#[doc = "`read()` method returns [stim11::R](stim11::R) reader structure"]
impl crate::Readable for STIM11 {}
#[doc = "`write(|w| ..)` method takes [stim11::W](stim11::W) writer structure"]
impl crate::Writable for STIM11 {}
#[doc = "Stimulus Port 11"]
pub mod stim11;
#[doc = "Stimulus Port 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim12](stim12) module"]
pub type STIM12 = crate::Reg<u32, _STIM12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIM12;
#[doc = "`read()` method returns [stim12::R](stim12::R) reader structure"]
impl crate::Readable for STIM12 {}
#[doc = "`write(|w| ..)` method takes [stim12::W](stim12::W) writer structure"]
impl crate::Writable for STIM12 {}
#[doc = "Stimulus Port 12"]
pub mod stim12;
#[doc = "Stimulus Port 13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim13](stim13) module"]
pub type STIM13 = crate::Reg<u32, _STIM13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIM13;
#[doc = "`read()` method returns [stim13::R](stim13::R) reader structure"]
impl crate::Readable for STIM13 {}
#[doc = "`write(|w| ..)` method takes [stim13::W](stim13::W) writer structure"]
impl crate::Writable for STIM13 {}
#[doc = "Stimulus Port 13"]
pub mod stim13;
#[doc = "Stimulus Port 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim14](stim14) module"]
pub type STIM14 = crate::Reg<u32, _STIM14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIM14;
#[doc = "`read()` method returns [stim14::R](stim14::R) reader structure"]
impl crate::Readable for STIM14 {}
#[doc = "`write(|w| ..)` method takes [stim14::W](stim14::W) writer structure"]
impl crate::Writable for STIM14 {}
#[doc = "Stimulus Port 14"]
pub mod stim14;
#[doc = "Stimulus Port 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim15](stim15) module"]
pub type STIM15 = crate::Reg<u32, _STIM15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIM15;
#[doc = "`read()` method returns [stim15::R](stim15::R) reader structure"]
impl crate::Readable for STIM15 {}
#[doc = "`write(|w| ..)` method takes [stim15::W](stim15::W) writer structure"]
impl crate::Writable for STIM15 {}
#[doc = "Stimulus Port 15"]
pub mod stim15;
#[doc = "Stimulus Port 16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim16](stim16) module"]
pub type STIM16 = crate::Reg<u32, _STIM16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIM16;
#[doc = "`read()` method returns [stim16::R](stim16::R) reader structure"]
impl crate::Readable for STIM16 {}
#[doc = "`write(|w| ..)` method takes [stim16::W](stim16::W) writer structure"]
impl crate::Writable for STIM16 {}
#[doc = "Stimulus Port 16"]
pub mod stim16;
#[doc = "Stimulus Port 17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim17](stim17) module"]
pub type STIM17 = crate::Reg<u32, _STIM17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIM17;
#[doc = "`read()` method returns [stim17::R](stim17::R) reader structure"]
impl crate::Readable for STIM17 {}
#[doc = "`write(|w| ..)` method takes [stim17::W](stim17::W) writer structure"]
impl crate::Writable for STIM17 {}
#[doc = "Stimulus Port 17"]
pub mod stim17;
#[doc = "Stimulus Port 18\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim18](stim18) module"]
pub type STIM18 = crate::Reg<u32, _STIM18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIM18;
#[doc = "`read()` method returns [stim18::R](stim18::R) reader structure"]
impl crate::Readable for STIM18 {}
#[doc = "`write(|w| ..)` method takes [stim18::W](stim18::W) writer structure"]
impl crate::Writable for STIM18 {}
#[doc = "Stimulus Port 18"]
pub mod stim18;
#[doc = "Stimulus Port 19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim19](stim19) module"]
pub type STIM19 = crate::Reg<u32, _STIM19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIM19;
#[doc = "`read()` method returns [stim19::R](stim19::R) reader structure"]
impl crate::Readable for STIM19 {}
#[doc = "`write(|w| ..)` method takes [stim19::W](stim19::W) writer structure"]
impl crate::Writable for STIM19 {}
#[doc = "Stimulus Port 19"]
pub mod stim19;
#[doc = "Stimulus Port 20\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim20](stim20) module"]
pub type STIM20 = crate::Reg<u32, _STIM20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIM20;
#[doc = "`read()` method returns [stim20::R](stim20::R) reader structure"]
impl crate::Readable for STIM20 {}
#[doc = "`write(|w| ..)` method takes [stim20::W](stim20::W) writer structure"]
impl crate::Writable for STIM20 {}
#[doc = "Stimulus Port 20"]
pub mod stim20;
#[doc = "Stimulus Port 21\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim21](stim21) module"]
pub type STIM21 = crate::Reg<u32, _STIM21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIM21;
#[doc = "`read()` method returns [stim21::R](stim21::R) reader structure"]
impl crate::Readable for STIM21 {}
#[doc = "`write(|w| ..)` method takes [stim21::W](stim21::W) writer structure"]
impl crate::Writable for STIM21 {}
#[doc = "Stimulus Port 21"]
pub mod stim21;
#[doc = "Stimulus Port 22\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim22](stim22) module"]
pub type STIM22 = crate::Reg<u32, _STIM22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIM22;
#[doc = "`read()` method returns [stim22::R](stim22::R) reader structure"]
impl crate::Readable for STIM22 {}
#[doc = "`write(|w| ..)` method takes [stim22::W](stim22::W) writer structure"]
impl crate::Writable for STIM22 {}
#[doc = "Stimulus Port 22"]
pub mod stim22;
#[doc = "Stimulus Port 23\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim23](stim23) module"]
pub type STIM23 = crate::Reg<u32, _STIM23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIM23;
#[doc = "`read()` method returns [stim23::R](stim23::R) reader structure"]
impl crate::Readable for STIM23 {}
#[doc = "`write(|w| ..)` method takes [stim23::W](stim23::W) writer structure"]
impl crate::Writable for STIM23 {}
#[doc = "Stimulus Port 23"]
pub mod stim23;
#[doc = "Stimulus Port 24\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim24](stim24) module"]
pub type STIM24 = crate::Reg<u32, _STIM24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIM24;
#[doc = "`read()` method returns [stim24::R](stim24::R) reader structure"]
impl crate::Readable for STIM24 {}
#[doc = "`write(|w| ..)` method takes [stim24::W](stim24::W) writer structure"]
impl crate::Writable for STIM24 {}
#[doc = "Stimulus Port 24"]
pub mod stim24;
#[doc = "Stimulus Port 25\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim25](stim25) module"]
pub type STIM25 = crate::Reg<u32, _STIM25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIM25;
#[doc = "`read()` method returns [stim25::R](stim25::R) reader structure"]
impl crate::Readable for STIM25 {}
#[doc = "`write(|w| ..)` method takes [stim25::W](stim25::W) writer structure"]
impl crate::Writable for STIM25 {}
#[doc = "Stimulus Port 25"]
pub mod stim25;
#[doc = "Stimulus Port 26\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim26](stim26) module"]
pub type STIM26 = crate::Reg<u32, _STIM26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIM26;
#[doc = "`read()` method returns [stim26::R](stim26::R) reader structure"]
impl crate::Readable for STIM26 {}
#[doc = "`write(|w| ..)` method takes [stim26::W](stim26::W) writer structure"]
impl crate::Writable for STIM26 {}
#[doc = "Stimulus Port 26"]
pub mod stim26;
#[doc = "Stimulus Port 27\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim27](stim27) module"]
pub type STIM27 = crate::Reg<u32, _STIM27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIM27;
#[doc = "`read()` method returns [stim27::R](stim27::R) reader structure"]
impl crate::Readable for STIM27 {}
#[doc = "`write(|w| ..)` method takes [stim27::W](stim27::W) writer structure"]
impl crate::Writable for STIM27 {}
#[doc = "Stimulus Port 27"]
pub mod stim27;
#[doc = "Stimulus Port 28\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim28](stim28) module"]
pub type STIM28 = crate::Reg<u32, _STIM28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIM28;
#[doc = "`read()` method returns [stim28::R](stim28::R) reader structure"]
impl crate::Readable for STIM28 {}
#[doc = "`write(|w| ..)` method takes [stim28::W](stim28::W) writer structure"]
impl crate::Writable for STIM28 {}
#[doc = "Stimulus Port 28"]
pub mod stim28;
#[doc = "Stimulus Port 29\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim29](stim29) module"]
pub type STIM29 = crate::Reg<u32, _STIM29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIM29;
#[doc = "`read()` method returns [stim29::R](stim29::R) reader structure"]
impl crate::Readable for STIM29 {}
#[doc = "`write(|w| ..)` method takes [stim29::W](stim29::W) writer structure"]
impl crate::Writable for STIM29 {}
#[doc = "Stimulus Port 29"]
pub mod stim29;
#[doc = "Stimulus Port 30\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim30](stim30) module"]
pub type STIM30 = crate::Reg<u32, _STIM30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIM30;
#[doc = "`read()` method returns [stim30::R](stim30::R) reader structure"]
impl crate::Readable for STIM30 {}
#[doc = "`write(|w| ..)` method takes [stim30::W](stim30::W) writer structure"]
impl crate::Writable for STIM30 {}
#[doc = "Stimulus Port 30"]
pub mod stim30;
#[doc = "Stimulus Port 31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim31](stim31) module"]
pub type STIM31 = crate::Reg<u32, _STIM31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIM31;
#[doc = "`read()` method returns [stim31::R](stim31::R) reader structure"]
impl crate::Readable for STIM31 {}
#[doc = "`write(|w| ..)` method takes [stim31::W](stim31::W) writer structure"]
impl crate::Writable for STIM31 {}
#[doc = "Stimulus Port 31"]
pub mod stim31;
#[doc = "Trace Enable Use the Trace Enable Register to generate trace data by writing to the corresponding stimulus port. Note: Privileged writes are accepted to this register if TCR.ITMENA is set. User writes are accepted to this register if TCR.ITMENA is set and the appropriate privilege mask is cleared. Privileged access to the stimulus ports enables an RTOS kernel to guarantee instrumentation slots or bandwidth as required.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ter](ter) module"]
pub type TER = crate::Reg<u32, _TER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TER;
#[doc = "`read()` method returns [ter::R](ter::R) reader structure"]
impl crate::Readable for TER {}
#[doc = "`write(|w| ..)` method takes [ter::W](ter::W) writer structure"]
impl crate::Writable for TER {}
#[doc = "Trace Enable Use the Trace Enable Register to generate trace data by writing to the corresponding stimulus port. Note: Privileged writes are accepted to this register if TCR.ITMENA is set. User writes are accepted to this register if TCR.ITMENA is set and the appropriate privilege mask is cleared. Privileged access to the stimulus ports enables an RTOS kernel to guarantee instrumentation slots or bandwidth as required."]
pub mod ter;
#[doc = "Trace Privilege This register is used to enable an operating system to control which stimulus ports are accessible by user code. This register can only be used in privileged mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpr](tpr) module"]
pub type TPR = crate::Reg<u32, _TPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPR;
#[doc = "`read()` method returns [tpr::R](tpr::R) reader structure"]
impl crate::Readable for TPR {}
#[doc = "`write(|w| ..)` method takes [tpr::W](tpr::W) writer structure"]
impl crate::Writable for TPR {}
#[doc = "Trace Privilege This register is used to enable an operating system to control which stimulus ports are accessible by user code. This register can only be used in privileged mode."]
pub mod tpr;
#[doc = "Trace Control Use this register to configure and control ITM transfers. This register can only be written in privilege mode. DWT is not enabled in the ITM block. However, DWT stimulus entry into the FIFO is controlled by DWTENA. If DWT requires timestamping, the TSENA bit must be set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr](tcr) module"]
pub type TCR = crate::Reg<u32, _TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCR;
#[doc = "`read()` method returns [tcr::R](tcr::R) reader structure"]
impl crate::Readable for TCR {}
#[doc = "`write(|w| ..)` method takes [tcr::W](tcr::W) writer structure"]
impl crate::Writable for TCR {}
#[doc = "Trace Control Use this register to configure and control ITM transfers. This register can only be written in privilege mode. DWT is not enabled in the ITM block. However, DWT stimulus entry into the FIFO is controlled by DWTENA. If DWT requires timestamping, the TSENA bit must be set."]
pub mod tcr;
#[doc = "Lock Access This register is used to prevent write accesses to the Control Registers: TER, TPR and TCR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lar](lar) module"]
pub type LAR = crate::Reg<u32, _LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LAR;
#[doc = "`read()` method returns [lar::R](lar::R) reader structure"]
impl crate::Readable for LAR {}
#[doc = "`write(|w| ..)` method takes [lar::W](lar::W) writer structure"]
impl crate::Writable for LAR {}
#[doc = "Lock Access This register is used to prevent write accesses to the Control Registers: TER, TPR and TCR."]
pub mod lar;
#[doc = "Lock Status Use this register to enable write accesses to the Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsr](lsr) module"]
pub type LSR = crate::Reg<u32, _LSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSR;
#[doc = "`read()` method returns [lsr::R](lsr::R) reader structure"]
impl crate::Readable for LSR {}
#[doc = "`write(|w| ..)` method takes [lsr::W](lsr::W) writer structure"]
impl crate::Writable for LSR {}
#[doc = "Lock Status Use this register to enable write accesses to the Control Register."]
pub mod lsr;
