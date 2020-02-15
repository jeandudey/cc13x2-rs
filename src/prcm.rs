#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Infrastructure Clock Division Factor For Run Mode"]
    pub infrclkdivr: INFRCLKDIVR,
    #[doc = "0x04 - Infrastructure Clock Division Factor For Sleep Mode"]
    pub infrclkdivs: INFRCLKDIVS,
    #[doc = "0x08 - Infrastructure Clock Division Factor For DeepSleep Mode"]
    pub infrclkdivds: INFRCLKDIVDS,
    #[doc = "0x0c - MCU Voltage Domain Control"]
    pub vdctl: VDCTL,
    _reserved4: [u8; 24usize],
    #[doc = "0x28 - Load PRCM Settings To CLKCTRL Power Domain"]
    pub clkloadctl: CLKLOADCTL,
    #[doc = "0x2c - RFC Clock Gate"]
    pub rfcclkg: RFCCLKG,
    #[doc = "0x30 - VIMS Clock Gate"]
    pub vimsclkg: VIMSCLKG,
    _reserved7: [u8; 8usize],
    #[doc = "0x3c - SEC (PKA And TRNG And CRYPTO) And UDMA Clock Gate For Run And All Modes"]
    pub secdmaclkgr: SECDMACLKGR,
    #[doc = "0x40 - SEC (PKA And TRNG And CRYPTO) And UDMA Clock Gate For Sleep Mode"]
    pub secdmaclkgs: SECDMACLKGS,
    #[doc = "0x44 - SEC (PKA And TRNG and CRYPTO) And UDMA Clock Gate For Deep Sleep Mode"]
    pub secdmaclkgds: SECDMACLKGDS,
    #[doc = "0x48 - GPIO Clock Gate For Run And All Modes"]
    pub gpioclkgr: GPIOCLKGR,
    #[doc = "0x4c - GPIO Clock Gate For Sleep Mode"]
    pub gpioclkgs: GPIOCLKGS,
    #[doc = "0x50 - GPIO Clock Gate For Deep Sleep Mode"]
    pub gpioclkgds: GPIOCLKGDS,
    #[doc = "0x54 - GPT Clock Gate For Run And All Modes"]
    pub gptclkgr: GPTCLKGR,
    #[doc = "0x58 - GPT Clock Gate For Sleep Mode"]
    pub gptclkgs: GPTCLKGS,
    #[doc = "0x5c - GPT Clock Gate For Deep Sleep Mode"]
    pub gptclkgds: GPTCLKGDS,
    #[doc = "0x60 - I2C Clock Gate For Run And All Modes"]
    pub i2cclkgr: I2CCLKGR,
    #[doc = "0x64 - I2C Clock Gate For Sleep Mode"]
    pub i2cclkgs: I2CCLKGS,
    #[doc = "0x68 - I2C Clock Gate For Deep Sleep Mode"]
    pub i2cclkgds: I2CCLKGDS,
    #[doc = "0x6c - UART Clock Gate For Run And All Modes"]
    pub uartclkgr: UARTCLKGR,
    #[doc = "0x70 - UART Clock Gate For Sleep Mode"]
    pub uartclkgs: UARTCLKGS,
    #[doc = "0x74 - UART Clock Gate For Deep Sleep Mode"]
    pub uartclkgds: UARTCLKGDS,
    #[doc = "0x78 - SSI Clock Gate For Run And All Modes"]
    pub ssiclkgr: SSICLKGR,
    #[doc = "0x7c - SSI Clock Gate For Sleep Mode"]
    pub ssiclkgs: SSICLKGS,
    #[doc = "0x80 - SSI Clock Gate For Deep Sleep Mode"]
    pub ssiclkgds: SSICLKGDS,
    #[doc = "0x84 - I2S Clock Gate For Run And All Modes"]
    pub i2sclkgr: I2SCLKGR,
    #[doc = "0x88 - I2S Clock Gate For Sleep Mode"]
    pub i2sclkgs: I2SCLKGS,
    #[doc = "0x8c - I2S Clock Gate For Deep Sleep Mode"]
    pub i2sclkgds: I2SCLKGDS,
    _reserved28: [u8; 36usize],
    #[doc = "0xb4 - Internal. Only to be used through TI provided API."]
    pub sysbusclkdiv: SYSBUSCLKDIV,
    #[doc = "0xb8 - Internal. Only to be used through TI provided API."]
    pub cpuclkdiv: CPUCLKDIV,
    #[doc = "0xbc - Internal. Only to be used through TI provided API."]
    pub perbuscpuclkdiv: PERBUSCPUCLKDIV,
    #[doc = "0xc0 - Internal. Only to be used through TI provided API."]
    pub perbusdmaclkdiv: PERBUSDMACLKDIV,
    #[doc = "0xc4 - Internal. Only to be used through TI provided API."]
    pub perdmaclkdiv: PERDMACLKDIV,
    #[doc = "0xc8 - I2S Clock Control"]
    pub i2sbclksel: I2SBCLKSEL,
    #[doc = "0xcc - GPT Scalar"]
    pub gptclkdiv: GPTCLKDIV,
    #[doc = "0xd0 - I2S Clock Control"]
    pub i2sclkctl: I2SCLKCTL,
    #[doc = "0xd4 - MCLK Division Ratio"]
    pub i2smclkdiv: I2SMCLKDIV,
    #[doc = "0xd8 - BCLK Division Ratio"]
    pub i2sbclkdiv: I2SBCLKDIV,
    #[doc = "0xdc - WCLK Division Ratio"]
    pub i2swclkdiv: I2SWCLKDIV,
    _reserved39: [u8; 16usize],
    #[doc = "0xf0 - RESET For SEC (PKA And TRNG And CRYPTO) And UDMA"]
    pub resetsecdma: RESETSECDMA,
    #[doc = "0xf4 - RESET For GPIO IPs"]
    pub resetgpio: RESETGPIO,
    #[doc = "0xf8 - RESET For GPT Ips"]
    pub resetgpt: RESETGPT,
    #[doc = "0xfc - RESET For I2C IPs"]
    pub reseti2c: RESETI2C,
    #[doc = "0x100 - RESET For UART IPs"]
    pub resetuart: RESETUART,
    #[doc = "0x104 - RESET For SSI IPs"]
    pub resetssi: RESETSSI,
    #[doc = "0x108 - RESET For I2S IP"]
    pub reseti2s: RESETI2S,
    _reserved46: [u8; 32usize],
    #[doc = "0x12c - Power Domain Control"]
    pub pdctl0: PDCTL0,
    #[doc = "0x130 - RFC Power Domain Control"]
    pub pdctl0rfc: PDCTL0RFC,
    #[doc = "0x134 - SERIAL Power Domain Control"]
    pub pdctl0serial: PDCTL0SERIAL,
    #[doc = "0x138 - PERIPH Power Domain Control"]
    pub pdctl0periph: PDCTL0PERIPH,
    _reserved50: [u8; 4usize],
    #[doc = "0x140 - Power Domain Status"]
    pub pdstat0: PDSTAT0,
    #[doc = "0x144 - RFC Power Domain Status"]
    pub pdstat0rfc: PDSTAT0RFC,
    #[doc = "0x148 - SERIAL Power Domain Status"]
    pub pdstat0serial: PDSTAT0SERIAL,
    #[doc = "0x14c - PERIPH Power Domain Status"]
    pub pdstat0periph: PDSTAT0PERIPH,
    _reserved54: [u8; 44usize],
    #[doc = "0x17c - Power Domain Control"]
    pub pdctl1: PDCTL1,
    _reserved55: [u8; 4usize],
    #[doc = "0x184 - CPU Power Domain Direct Control"]
    pub pdctl1cpu: PDCTL1CPU,
    #[doc = "0x188 - RFC Power Domain Direct Control"]
    pub pdctl1rfc: PDCTL1RFC,
    #[doc = "0x18c - VIMS Mode Direct Control"]
    pub pdctl1vims: PDCTL1VIMS,
    _reserved58: [u8; 4usize],
    #[doc = "0x194 - Power Manager Status"]
    pub pdstat1: PDSTAT1,
    #[doc = "0x198 - BUS Power Domain Direct Read Status"]
    pub pdstat1bus: PDSTAT1BUS,
    #[doc = "0x19c - RFC Power Domain Direct Read Status"]
    pub pdstat1rfc: PDSTAT1RFC,
    #[doc = "0x1a0 - CPU Power Domain Direct Read Status"]
    pub pdstat1cpu: PDSTAT1CPU,
    #[doc = "0x1a4 - VIMS Mode Direct Read Status"]
    pub pdstat1vims: PDSTAT1VIMS,
    _reserved63: [u8; 36usize],
    #[doc = "0x1cc - Control To RFC"]
    pub rfcbits: RFCBITS,
    #[doc = "0x1d0 - Selected RFC Mode"]
    pub rfcmodesel: RFCMODESEL,
    #[doc = "0x1d4 - Allowed RFC Modes"]
    pub rfcmodehwopt: RFCMODEHWOPT,
    _reserved66: [u8; 8usize],
    #[doc = "0x1e0 - Power Profiler Register"]
    pub pwrprofstat: PWRPROFSTAT,
    _reserved67: [u8; 56usize],
    #[doc = "0x21c - MCU SRAM configuration"]
    pub mcusramcfg: MCUSRAMCFG,
    _reserved68: [u8; 4usize],
    #[doc = "0x224 - Memory Retention Control"]
    pub ramreten: RAMRETEN,
    _reserved69: [u8; 104usize],
    #[doc = "0x290 - Oscillator Interrupt Mask"]
    pub oscimsc: OSCIMSC,
    #[doc = "0x294 - Oscillator Raw Interrupt Status"]
    pub oscris: OSCRIS,
    #[doc = "0x298 - Oscillator Raw Interrupt Clear"]
    pub oscicr: OSCICR,
}
#[doc = "Infrastructure Clock Division Factor For Run Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [infrclkdivr](infrclkdivr) module"]
pub type INFRCLKDIVR = crate::Reg<u32, _INFRCLKDIVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INFRCLKDIVR;
#[doc = "`read()` method returns [infrclkdivr::R](infrclkdivr::R) reader structure"]
impl crate::Readable for INFRCLKDIVR {}
#[doc = "`write(|w| ..)` method takes [infrclkdivr::W](infrclkdivr::W) writer structure"]
impl crate::Writable for INFRCLKDIVR {}
#[doc = "Infrastructure Clock Division Factor For Run Mode"]
pub mod infrclkdivr;
#[doc = "Infrastructure Clock Division Factor For Sleep Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [infrclkdivs](infrclkdivs) module"]
pub type INFRCLKDIVS = crate::Reg<u32, _INFRCLKDIVS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INFRCLKDIVS;
#[doc = "`read()` method returns [infrclkdivs::R](infrclkdivs::R) reader structure"]
impl crate::Readable for INFRCLKDIVS {}
#[doc = "`write(|w| ..)` method takes [infrclkdivs::W](infrclkdivs::W) writer structure"]
impl crate::Writable for INFRCLKDIVS {}
#[doc = "Infrastructure Clock Division Factor For Sleep Mode"]
pub mod infrclkdivs;
#[doc = "Infrastructure Clock Division Factor For DeepSleep Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [infrclkdivds](infrclkdivds) module"]
pub type INFRCLKDIVDS = crate::Reg<u32, _INFRCLKDIVDS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INFRCLKDIVDS;
#[doc = "`read()` method returns [infrclkdivds::R](infrclkdivds::R) reader structure"]
impl crate::Readable for INFRCLKDIVDS {}
#[doc = "`write(|w| ..)` method takes [infrclkdivds::W](infrclkdivds::W) writer structure"]
impl crate::Writable for INFRCLKDIVDS {}
#[doc = "Infrastructure Clock Division Factor For DeepSleep Mode"]
pub mod infrclkdivds;
#[doc = "MCU Voltage Domain Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdctl](vdctl) module"]
pub type VDCTL = crate::Reg<u32, _VDCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VDCTL;
#[doc = "`read()` method returns [vdctl::R](vdctl::R) reader structure"]
impl crate::Readable for VDCTL {}
#[doc = "`write(|w| ..)` method takes [vdctl::W](vdctl::W) writer structure"]
impl crate::Writable for VDCTL {}
#[doc = "MCU Voltage Domain Control"]
pub mod vdctl;
#[doc = "Load PRCM Settings To CLKCTRL Power Domain\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkloadctl](clkloadctl) module"]
pub type CLKLOADCTL = crate::Reg<u32, _CLKLOADCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKLOADCTL;
#[doc = "`read()` method returns [clkloadctl::R](clkloadctl::R) reader structure"]
impl crate::Readable for CLKLOADCTL {}
#[doc = "`write(|w| ..)` method takes [clkloadctl::W](clkloadctl::W) writer structure"]
impl crate::Writable for CLKLOADCTL {}
#[doc = "Load PRCM Settings To CLKCTRL Power Domain"]
pub mod clkloadctl;
#[doc = "RFC Clock Gate\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcclkg](rfcclkg) module"]
pub type RFCCLKG = crate::Reg<u32, _RFCCLKG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFCCLKG;
#[doc = "`read()` method returns [rfcclkg::R](rfcclkg::R) reader structure"]
impl crate::Readable for RFCCLKG {}
#[doc = "`write(|w| ..)` method takes [rfcclkg::W](rfcclkg::W) writer structure"]
impl crate::Writable for RFCCLKG {}
#[doc = "RFC Clock Gate"]
pub mod rfcclkg;
#[doc = "VIMS Clock Gate\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vimsclkg](vimsclkg) module"]
pub type VIMSCLKG = crate::Reg<u32, _VIMSCLKG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VIMSCLKG;
#[doc = "`read()` method returns [vimsclkg::R](vimsclkg::R) reader structure"]
impl crate::Readable for VIMSCLKG {}
#[doc = "`write(|w| ..)` method takes [vimsclkg::W](vimsclkg::W) writer structure"]
impl crate::Writable for VIMSCLKG {}
#[doc = "VIMS Clock Gate"]
pub mod vimsclkg;
#[doc = "SEC (PKA And TRNG And CRYPTO) And UDMA Clock Gate For Run And All Modes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secdmaclkgr](secdmaclkgr) module"]
pub type SECDMACLKGR = crate::Reg<u32, _SECDMACLKGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECDMACLKGR;
#[doc = "`read()` method returns [secdmaclkgr::R](secdmaclkgr::R) reader structure"]
impl crate::Readable for SECDMACLKGR {}
#[doc = "`write(|w| ..)` method takes [secdmaclkgr::W](secdmaclkgr::W) writer structure"]
impl crate::Writable for SECDMACLKGR {}
#[doc = "SEC (PKA And TRNG And CRYPTO) And UDMA Clock Gate For Run And All Modes"]
pub mod secdmaclkgr;
#[doc = "SEC (PKA And TRNG And CRYPTO) And UDMA Clock Gate For Sleep Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secdmaclkgs](secdmaclkgs) module"]
pub type SECDMACLKGS = crate::Reg<u32, _SECDMACLKGS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECDMACLKGS;
#[doc = "`read()` method returns [secdmaclkgs::R](secdmaclkgs::R) reader structure"]
impl crate::Readable for SECDMACLKGS {}
#[doc = "`write(|w| ..)` method takes [secdmaclkgs::W](secdmaclkgs::W) writer structure"]
impl crate::Writable for SECDMACLKGS {}
#[doc = "SEC (PKA And TRNG And CRYPTO) And UDMA Clock Gate For Sleep Mode"]
pub mod secdmaclkgs;
#[doc = "SEC (PKA And TRNG and CRYPTO) And UDMA Clock Gate For Deep Sleep Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secdmaclkgds](secdmaclkgds) module"]
pub type SECDMACLKGDS = crate::Reg<u32, _SECDMACLKGDS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECDMACLKGDS;
#[doc = "`read()` method returns [secdmaclkgds::R](secdmaclkgds::R) reader structure"]
impl crate::Readable for SECDMACLKGDS {}
#[doc = "`write(|w| ..)` method takes [secdmaclkgds::W](secdmaclkgds::W) writer structure"]
impl crate::Writable for SECDMACLKGDS {}
#[doc = "SEC (PKA And TRNG and CRYPTO) And UDMA Clock Gate For Deep Sleep Mode"]
pub mod secdmaclkgds;
#[doc = "GPIO Clock Gate For Run And All Modes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioclkgr](gpioclkgr) module"]
pub type GPIOCLKGR = crate::Reg<u32, _GPIOCLKGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOCLKGR;
#[doc = "`read()` method returns [gpioclkgr::R](gpioclkgr::R) reader structure"]
impl crate::Readable for GPIOCLKGR {}
#[doc = "`write(|w| ..)` method takes [gpioclkgr::W](gpioclkgr::W) writer structure"]
impl crate::Writable for GPIOCLKGR {}
#[doc = "GPIO Clock Gate For Run And All Modes"]
pub mod gpioclkgr;
#[doc = "GPIO Clock Gate For Sleep Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioclkgs](gpioclkgs) module"]
pub type GPIOCLKGS = crate::Reg<u32, _GPIOCLKGS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOCLKGS;
#[doc = "`read()` method returns [gpioclkgs::R](gpioclkgs::R) reader structure"]
impl crate::Readable for GPIOCLKGS {}
#[doc = "`write(|w| ..)` method takes [gpioclkgs::W](gpioclkgs::W) writer structure"]
impl crate::Writable for GPIOCLKGS {}
#[doc = "GPIO Clock Gate For Sleep Mode"]
pub mod gpioclkgs;
#[doc = "GPIO Clock Gate For Deep Sleep Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioclkgds](gpioclkgds) module"]
pub type GPIOCLKGDS = crate::Reg<u32, _GPIOCLKGDS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOCLKGDS;
#[doc = "`read()` method returns [gpioclkgds::R](gpioclkgds::R) reader structure"]
impl crate::Readable for GPIOCLKGDS {}
#[doc = "`write(|w| ..)` method takes [gpioclkgds::W](gpioclkgds::W) writer structure"]
impl crate::Writable for GPIOCLKGDS {}
#[doc = "GPIO Clock Gate For Deep Sleep Mode"]
pub mod gpioclkgds;
#[doc = "GPT Clock Gate For Run And All Modes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptclkgr](gptclkgr) module"]
pub type GPTCLKGR = crate::Reg<u32, _GPTCLKGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTCLKGR;
#[doc = "`read()` method returns [gptclkgr::R](gptclkgr::R) reader structure"]
impl crate::Readable for GPTCLKGR {}
#[doc = "`write(|w| ..)` method takes [gptclkgr::W](gptclkgr::W) writer structure"]
impl crate::Writable for GPTCLKGR {}
#[doc = "GPT Clock Gate For Run And All Modes"]
pub mod gptclkgr;
#[doc = "GPT Clock Gate For Sleep Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptclkgs](gptclkgs) module"]
pub type GPTCLKGS = crate::Reg<u32, _GPTCLKGS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTCLKGS;
#[doc = "`read()` method returns [gptclkgs::R](gptclkgs::R) reader structure"]
impl crate::Readable for GPTCLKGS {}
#[doc = "`write(|w| ..)` method takes [gptclkgs::W](gptclkgs::W) writer structure"]
impl crate::Writable for GPTCLKGS {}
#[doc = "GPT Clock Gate For Sleep Mode"]
pub mod gptclkgs;
#[doc = "GPT Clock Gate For Deep Sleep Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptclkgds](gptclkgds) module"]
pub type GPTCLKGDS = crate::Reg<u32, _GPTCLKGDS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTCLKGDS;
#[doc = "`read()` method returns [gptclkgds::R](gptclkgds::R) reader structure"]
impl crate::Readable for GPTCLKGDS {}
#[doc = "`write(|w| ..)` method takes [gptclkgds::W](gptclkgds::W) writer structure"]
impl crate::Writable for GPTCLKGDS {}
#[doc = "GPT Clock Gate For Deep Sleep Mode"]
pub mod gptclkgds;
#[doc = "I2C Clock Gate For Run And All Modes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cclkgr](i2cclkgr) module"]
pub type I2CCLKGR = crate::Reg<u32, _I2CCLKGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CCLKGR;
#[doc = "`read()` method returns [i2cclkgr::R](i2cclkgr::R) reader structure"]
impl crate::Readable for I2CCLKGR {}
#[doc = "`write(|w| ..)` method takes [i2cclkgr::W](i2cclkgr::W) writer structure"]
impl crate::Writable for I2CCLKGR {}
#[doc = "I2C Clock Gate For Run And All Modes"]
pub mod i2cclkgr;
#[doc = "I2C Clock Gate For Sleep Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cclkgs](i2cclkgs) module"]
pub type I2CCLKGS = crate::Reg<u32, _I2CCLKGS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CCLKGS;
#[doc = "`read()` method returns [i2cclkgs::R](i2cclkgs::R) reader structure"]
impl crate::Readable for I2CCLKGS {}
#[doc = "`write(|w| ..)` method takes [i2cclkgs::W](i2cclkgs::W) writer structure"]
impl crate::Writable for I2CCLKGS {}
#[doc = "I2C Clock Gate For Sleep Mode"]
pub mod i2cclkgs;
#[doc = "I2C Clock Gate For Deep Sleep Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cclkgds](i2cclkgds) module"]
pub type I2CCLKGDS = crate::Reg<u32, _I2CCLKGDS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CCLKGDS;
#[doc = "`read()` method returns [i2cclkgds::R](i2cclkgds::R) reader structure"]
impl crate::Readable for I2CCLKGDS {}
#[doc = "`write(|w| ..)` method takes [i2cclkgds::W](i2cclkgds::W) writer structure"]
impl crate::Writable for I2CCLKGDS {}
#[doc = "I2C Clock Gate For Deep Sleep Mode"]
pub mod i2cclkgds;
#[doc = "UART Clock Gate For Run And All Modes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartclkgr](uartclkgr) module"]
pub type UARTCLKGR = crate::Reg<u32, _UARTCLKGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTCLKGR;
#[doc = "`read()` method returns [uartclkgr::R](uartclkgr::R) reader structure"]
impl crate::Readable for UARTCLKGR {}
#[doc = "`write(|w| ..)` method takes [uartclkgr::W](uartclkgr::W) writer structure"]
impl crate::Writable for UARTCLKGR {}
#[doc = "UART Clock Gate For Run And All Modes"]
pub mod uartclkgr;
#[doc = "UART Clock Gate For Sleep Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartclkgs](uartclkgs) module"]
pub type UARTCLKGS = crate::Reg<u32, _UARTCLKGS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTCLKGS;
#[doc = "`read()` method returns [uartclkgs::R](uartclkgs::R) reader structure"]
impl crate::Readable for UARTCLKGS {}
#[doc = "`write(|w| ..)` method takes [uartclkgs::W](uartclkgs::W) writer structure"]
impl crate::Writable for UARTCLKGS {}
#[doc = "UART Clock Gate For Sleep Mode"]
pub mod uartclkgs;
#[doc = "UART Clock Gate For Deep Sleep Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartclkgds](uartclkgds) module"]
pub type UARTCLKGDS = crate::Reg<u32, _UARTCLKGDS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTCLKGDS;
#[doc = "`read()` method returns [uartclkgds::R](uartclkgds::R) reader structure"]
impl crate::Readable for UARTCLKGDS {}
#[doc = "`write(|w| ..)` method takes [uartclkgds::W](uartclkgds::W) writer structure"]
impl crate::Writable for UARTCLKGDS {}
#[doc = "UART Clock Gate For Deep Sleep Mode"]
pub mod uartclkgds;
#[doc = "SSI Clock Gate For Run And All Modes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssiclkgr](ssiclkgr) module"]
pub type SSICLKGR = crate::Reg<u32, _SSICLKGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSICLKGR;
#[doc = "`read()` method returns [ssiclkgr::R](ssiclkgr::R) reader structure"]
impl crate::Readable for SSICLKGR {}
#[doc = "`write(|w| ..)` method takes [ssiclkgr::W](ssiclkgr::W) writer structure"]
impl crate::Writable for SSICLKGR {}
#[doc = "SSI Clock Gate For Run And All Modes"]
pub mod ssiclkgr;
#[doc = "SSI Clock Gate For Sleep Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssiclkgs](ssiclkgs) module"]
pub type SSICLKGS = crate::Reg<u32, _SSICLKGS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSICLKGS;
#[doc = "`read()` method returns [ssiclkgs::R](ssiclkgs::R) reader structure"]
impl crate::Readable for SSICLKGS {}
#[doc = "`write(|w| ..)` method takes [ssiclkgs::W](ssiclkgs::W) writer structure"]
impl crate::Writable for SSICLKGS {}
#[doc = "SSI Clock Gate For Sleep Mode"]
pub mod ssiclkgs;
#[doc = "SSI Clock Gate For Deep Sleep Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssiclkgds](ssiclkgds) module"]
pub type SSICLKGDS = crate::Reg<u32, _SSICLKGDS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSICLKGDS;
#[doc = "`read()` method returns [ssiclkgds::R](ssiclkgds::R) reader structure"]
impl crate::Readable for SSICLKGDS {}
#[doc = "`write(|w| ..)` method takes [ssiclkgds::W](ssiclkgds::W) writer structure"]
impl crate::Writable for SSICLKGDS {}
#[doc = "SSI Clock Gate For Deep Sleep Mode"]
pub mod ssiclkgds;
#[doc = "I2S Clock Gate For Run And All Modes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sclkgr](i2sclkgr) module"]
pub type I2SCLKGR = crate::Reg<u32, _I2SCLKGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2SCLKGR;
#[doc = "`read()` method returns [i2sclkgr::R](i2sclkgr::R) reader structure"]
impl crate::Readable for I2SCLKGR {}
#[doc = "`write(|w| ..)` method takes [i2sclkgr::W](i2sclkgr::W) writer structure"]
impl crate::Writable for I2SCLKGR {}
#[doc = "I2S Clock Gate For Run And All Modes"]
pub mod i2sclkgr;
#[doc = "I2S Clock Gate For Sleep Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sclkgs](i2sclkgs) module"]
pub type I2SCLKGS = crate::Reg<u32, _I2SCLKGS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2SCLKGS;
#[doc = "`read()` method returns [i2sclkgs::R](i2sclkgs::R) reader structure"]
impl crate::Readable for I2SCLKGS {}
#[doc = "`write(|w| ..)` method takes [i2sclkgs::W](i2sclkgs::W) writer structure"]
impl crate::Writable for I2SCLKGS {}
#[doc = "I2S Clock Gate For Sleep Mode"]
pub mod i2sclkgs;
#[doc = "I2S Clock Gate For Deep Sleep Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sclkgds](i2sclkgds) module"]
pub type I2SCLKGDS = crate::Reg<u32, _I2SCLKGDS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2SCLKGDS;
#[doc = "`read()` method returns [i2sclkgds::R](i2sclkgds::R) reader structure"]
impl crate::Readable for I2SCLKGDS {}
#[doc = "`write(|w| ..)` method takes [i2sclkgds::W](i2sclkgds::W) writer structure"]
impl crate::Writable for I2SCLKGDS {}
#[doc = "I2S Clock Gate For Deep Sleep Mode"]
pub mod i2sclkgds;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysbusclkdiv](sysbusclkdiv) module"]
pub type SYSBUSCLKDIV = crate::Reg<u32, _SYSBUSCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSBUSCLKDIV;
#[doc = "`read()` method returns [sysbusclkdiv::R](sysbusclkdiv::R) reader structure"]
impl crate::Readable for SYSBUSCLKDIV {}
#[doc = "`write(|w| ..)` method takes [sysbusclkdiv::W](sysbusclkdiv::W) writer structure"]
impl crate::Writable for SYSBUSCLKDIV {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod sysbusclkdiv;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuclkdiv](cpuclkdiv) module"]
pub type CPUCLKDIV = crate::Reg<u32, _CPUCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUCLKDIV;
#[doc = "`read()` method returns [cpuclkdiv::R](cpuclkdiv::R) reader structure"]
impl crate::Readable for CPUCLKDIV {}
#[doc = "`write(|w| ..)` method takes [cpuclkdiv::W](cpuclkdiv::W) writer structure"]
impl crate::Writable for CPUCLKDIV {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod cpuclkdiv;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perbuscpuclkdiv](perbuscpuclkdiv) module"]
pub type PERBUSCPUCLKDIV = crate::Reg<u32, _PERBUSCPUCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERBUSCPUCLKDIV;
#[doc = "`read()` method returns [perbuscpuclkdiv::R](perbuscpuclkdiv::R) reader structure"]
impl crate::Readable for PERBUSCPUCLKDIV {}
#[doc = "`write(|w| ..)` method takes [perbuscpuclkdiv::W](perbuscpuclkdiv::W) writer structure"]
impl crate::Writable for PERBUSCPUCLKDIV {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod perbuscpuclkdiv;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perbusdmaclkdiv](perbusdmaclkdiv) module"]
pub type PERBUSDMACLKDIV = crate::Reg<u32, _PERBUSDMACLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERBUSDMACLKDIV;
#[doc = "`read()` method returns [perbusdmaclkdiv::R](perbusdmaclkdiv::R) reader structure"]
impl crate::Readable for PERBUSDMACLKDIV {}
#[doc = "`write(|w| ..)` method takes [perbusdmaclkdiv::W](perbusdmaclkdiv::W) writer structure"]
impl crate::Writable for PERBUSDMACLKDIV {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod perbusdmaclkdiv;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perdmaclkdiv](perdmaclkdiv) module"]
pub type PERDMACLKDIV = crate::Reg<u32, _PERDMACLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERDMACLKDIV;
#[doc = "`read()` method returns [perdmaclkdiv::R](perdmaclkdiv::R) reader structure"]
impl crate::Readable for PERDMACLKDIV {}
#[doc = "`write(|w| ..)` method takes [perdmaclkdiv::W](perdmaclkdiv::W) writer structure"]
impl crate::Writable for PERDMACLKDIV {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod perdmaclkdiv;
#[doc = "I2S Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sbclksel](i2sbclksel) module"]
pub type I2SBCLKSEL = crate::Reg<u32, _I2SBCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2SBCLKSEL;
#[doc = "`read()` method returns [i2sbclksel::R](i2sbclksel::R) reader structure"]
impl crate::Readable for I2SBCLKSEL {}
#[doc = "`write(|w| ..)` method takes [i2sbclksel::W](i2sbclksel::W) writer structure"]
impl crate::Writable for I2SBCLKSEL {}
#[doc = "I2S Clock Control"]
pub mod i2sbclksel;
#[doc = "GPT Scalar\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptclkdiv](gptclkdiv) module"]
pub type GPTCLKDIV = crate::Reg<u32, _GPTCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTCLKDIV;
#[doc = "`read()` method returns [gptclkdiv::R](gptclkdiv::R) reader structure"]
impl crate::Readable for GPTCLKDIV {}
#[doc = "`write(|w| ..)` method takes [gptclkdiv::W](gptclkdiv::W) writer structure"]
impl crate::Writable for GPTCLKDIV {}
#[doc = "GPT Scalar"]
pub mod gptclkdiv;
#[doc = "I2S Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sclkctl](i2sclkctl) module"]
pub type I2SCLKCTL = crate::Reg<u32, _I2SCLKCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2SCLKCTL;
#[doc = "`read()` method returns [i2sclkctl::R](i2sclkctl::R) reader structure"]
impl crate::Readable for I2SCLKCTL {}
#[doc = "`write(|w| ..)` method takes [i2sclkctl::W](i2sclkctl::W) writer structure"]
impl crate::Writable for I2SCLKCTL {}
#[doc = "I2S Clock Control"]
pub mod i2sclkctl;
#[doc = "MCLK Division Ratio\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2smclkdiv](i2smclkdiv) module"]
pub type I2SMCLKDIV = crate::Reg<u32, _I2SMCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2SMCLKDIV;
#[doc = "`read()` method returns [i2smclkdiv::R](i2smclkdiv::R) reader structure"]
impl crate::Readable for I2SMCLKDIV {}
#[doc = "`write(|w| ..)` method takes [i2smclkdiv::W](i2smclkdiv::W) writer structure"]
impl crate::Writable for I2SMCLKDIV {}
#[doc = "MCLK Division Ratio"]
pub mod i2smclkdiv;
#[doc = "BCLK Division Ratio\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sbclkdiv](i2sbclkdiv) module"]
pub type I2SBCLKDIV = crate::Reg<u32, _I2SBCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2SBCLKDIV;
#[doc = "`read()` method returns [i2sbclkdiv::R](i2sbclkdiv::R) reader structure"]
impl crate::Readable for I2SBCLKDIV {}
#[doc = "`write(|w| ..)` method takes [i2sbclkdiv::W](i2sbclkdiv::W) writer structure"]
impl crate::Writable for I2SBCLKDIV {}
#[doc = "BCLK Division Ratio"]
pub mod i2sbclkdiv;
#[doc = "WCLK Division Ratio\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2swclkdiv](i2swclkdiv) module"]
pub type I2SWCLKDIV = crate::Reg<u32, _I2SWCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2SWCLKDIV;
#[doc = "`read()` method returns [i2swclkdiv::R](i2swclkdiv::R) reader structure"]
impl crate::Readable for I2SWCLKDIV {}
#[doc = "`write(|w| ..)` method takes [i2swclkdiv::W](i2swclkdiv::W) writer structure"]
impl crate::Writable for I2SWCLKDIV {}
#[doc = "WCLK Division Ratio"]
pub mod i2swclkdiv;
#[doc = "RESET For SEC (PKA And TRNG And CRYPTO) And UDMA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resetsecdma](resetsecdma) module"]
pub type RESETSECDMA = crate::Reg<u32, _RESETSECDMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESETSECDMA;
#[doc = "`read()` method returns [resetsecdma::R](resetsecdma::R) reader structure"]
impl crate::Readable for RESETSECDMA {}
#[doc = "`write(|w| ..)` method takes [resetsecdma::W](resetsecdma::W) writer structure"]
impl crate::Writable for RESETSECDMA {}
#[doc = "RESET For SEC (PKA And TRNG And CRYPTO) And UDMA"]
pub mod resetsecdma;
#[doc = "RESET For GPIO IPs\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resetgpio](resetgpio) module"]
pub type RESETGPIO = crate::Reg<u32, _RESETGPIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESETGPIO;
#[doc = "`read()` method returns [resetgpio::R](resetgpio::R) reader structure"]
impl crate::Readable for RESETGPIO {}
#[doc = "`write(|w| ..)` method takes [resetgpio::W](resetgpio::W) writer structure"]
impl crate::Writable for RESETGPIO {}
#[doc = "RESET For GPIO IPs"]
pub mod resetgpio;
#[doc = "RESET For GPT Ips\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resetgpt](resetgpt) module"]
pub type RESETGPT = crate::Reg<u32, _RESETGPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESETGPT;
#[doc = "`read()` method returns [resetgpt::R](resetgpt::R) reader structure"]
impl crate::Readable for RESETGPT {}
#[doc = "`write(|w| ..)` method takes [resetgpt::W](resetgpt::W) writer structure"]
impl crate::Writable for RESETGPT {}
#[doc = "RESET For GPT Ips"]
pub mod resetgpt;
#[doc = "RESET For I2C IPs\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reseti2c](reseti2c) module"]
pub type RESETI2C = crate::Reg<u32, _RESETI2C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESETI2C;
#[doc = "`read()` method returns [reseti2c::R](reseti2c::R) reader structure"]
impl crate::Readable for RESETI2C {}
#[doc = "`write(|w| ..)` method takes [reseti2c::W](reseti2c::W) writer structure"]
impl crate::Writable for RESETI2C {}
#[doc = "RESET For I2C IPs"]
pub mod reseti2c;
#[doc = "RESET For UART IPs\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resetuart](resetuart) module"]
pub type RESETUART = crate::Reg<u32, _RESETUART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESETUART;
#[doc = "`read()` method returns [resetuart::R](resetuart::R) reader structure"]
impl crate::Readable for RESETUART {}
#[doc = "`write(|w| ..)` method takes [resetuart::W](resetuart::W) writer structure"]
impl crate::Writable for RESETUART {}
#[doc = "RESET For UART IPs"]
pub mod resetuart;
#[doc = "RESET For SSI IPs\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resetssi](resetssi) module"]
pub type RESETSSI = crate::Reg<u32, _RESETSSI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESETSSI;
#[doc = "`read()` method returns [resetssi::R](resetssi::R) reader structure"]
impl crate::Readable for RESETSSI {}
#[doc = "`write(|w| ..)` method takes [resetssi::W](resetssi::W) writer structure"]
impl crate::Writable for RESETSSI {}
#[doc = "RESET For SSI IPs"]
pub mod resetssi;
#[doc = "RESET For I2S IP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reseti2s](reseti2s) module"]
pub type RESETI2S = crate::Reg<u32, _RESETI2S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESETI2S;
#[doc = "`read()` method returns [reseti2s::R](reseti2s::R) reader structure"]
impl crate::Readable for RESETI2S {}
#[doc = "`write(|w| ..)` method takes [reseti2s::W](reseti2s::W) writer structure"]
impl crate::Writable for RESETI2S {}
#[doc = "RESET For I2S IP"]
pub mod reseti2s;
#[doc = "Power Domain Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdctl0](pdctl0) module"]
pub type PDCTL0 = crate::Reg<u32, _PDCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDCTL0;
#[doc = "`read()` method returns [pdctl0::R](pdctl0::R) reader structure"]
impl crate::Readable for PDCTL0 {}
#[doc = "`write(|w| ..)` method takes [pdctl0::W](pdctl0::W) writer structure"]
impl crate::Writable for PDCTL0 {}
#[doc = "Power Domain Control"]
pub mod pdctl0;
#[doc = "RFC Power Domain Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdctl0rfc](pdctl0rfc) module"]
pub type PDCTL0RFC = crate::Reg<u32, _PDCTL0RFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDCTL0RFC;
#[doc = "`read()` method returns [pdctl0rfc::R](pdctl0rfc::R) reader structure"]
impl crate::Readable for PDCTL0RFC {}
#[doc = "`write(|w| ..)` method takes [pdctl0rfc::W](pdctl0rfc::W) writer structure"]
impl crate::Writable for PDCTL0RFC {}
#[doc = "RFC Power Domain Control"]
pub mod pdctl0rfc;
#[doc = "SERIAL Power Domain Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdctl0serial](pdctl0serial) module"]
pub type PDCTL0SERIAL = crate::Reg<u32, _PDCTL0SERIAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDCTL0SERIAL;
#[doc = "`read()` method returns [pdctl0serial::R](pdctl0serial::R) reader structure"]
impl crate::Readable for PDCTL0SERIAL {}
#[doc = "`write(|w| ..)` method takes [pdctl0serial::W](pdctl0serial::W) writer structure"]
impl crate::Writable for PDCTL0SERIAL {}
#[doc = "SERIAL Power Domain Control"]
pub mod pdctl0serial;
#[doc = "PERIPH Power Domain Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdctl0periph](pdctl0periph) module"]
pub type PDCTL0PERIPH = crate::Reg<u32, _PDCTL0PERIPH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDCTL0PERIPH;
#[doc = "`read()` method returns [pdctl0periph::R](pdctl0periph::R) reader structure"]
impl crate::Readable for PDCTL0PERIPH {}
#[doc = "`write(|w| ..)` method takes [pdctl0periph::W](pdctl0periph::W) writer structure"]
impl crate::Writable for PDCTL0PERIPH {}
#[doc = "PERIPH Power Domain Control"]
pub mod pdctl0periph;
#[doc = "Power Domain Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdstat0](pdstat0) module"]
pub type PDSTAT0 = crate::Reg<u32, _PDSTAT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDSTAT0;
#[doc = "`read()` method returns [pdstat0::R](pdstat0::R) reader structure"]
impl crate::Readable for PDSTAT0 {}
#[doc = "`write(|w| ..)` method takes [pdstat0::W](pdstat0::W) writer structure"]
impl crate::Writable for PDSTAT0 {}
#[doc = "Power Domain Status"]
pub mod pdstat0;
#[doc = "RFC Power Domain Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdstat0rfc](pdstat0rfc) module"]
pub type PDSTAT0RFC = crate::Reg<u32, _PDSTAT0RFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDSTAT0RFC;
#[doc = "`read()` method returns [pdstat0rfc::R](pdstat0rfc::R) reader structure"]
impl crate::Readable for PDSTAT0RFC {}
#[doc = "`write(|w| ..)` method takes [pdstat0rfc::W](pdstat0rfc::W) writer structure"]
impl crate::Writable for PDSTAT0RFC {}
#[doc = "RFC Power Domain Status"]
pub mod pdstat0rfc;
#[doc = "SERIAL Power Domain Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdstat0serial](pdstat0serial) module"]
pub type PDSTAT0SERIAL = crate::Reg<u32, _PDSTAT0SERIAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDSTAT0SERIAL;
#[doc = "`read()` method returns [pdstat0serial::R](pdstat0serial::R) reader structure"]
impl crate::Readable for PDSTAT0SERIAL {}
#[doc = "`write(|w| ..)` method takes [pdstat0serial::W](pdstat0serial::W) writer structure"]
impl crate::Writable for PDSTAT0SERIAL {}
#[doc = "SERIAL Power Domain Status"]
pub mod pdstat0serial;
#[doc = "PERIPH Power Domain Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdstat0periph](pdstat0periph) module"]
pub type PDSTAT0PERIPH = crate::Reg<u32, _PDSTAT0PERIPH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDSTAT0PERIPH;
#[doc = "`read()` method returns [pdstat0periph::R](pdstat0periph::R) reader structure"]
impl crate::Readable for PDSTAT0PERIPH {}
#[doc = "`write(|w| ..)` method takes [pdstat0periph::W](pdstat0periph::W) writer structure"]
impl crate::Writable for PDSTAT0PERIPH {}
#[doc = "PERIPH Power Domain Status"]
pub mod pdstat0periph;
#[doc = "Power Domain Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdctl1](pdctl1) module"]
pub type PDCTL1 = crate::Reg<u32, _PDCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDCTL1;
#[doc = "`read()` method returns [pdctl1::R](pdctl1::R) reader structure"]
impl crate::Readable for PDCTL1 {}
#[doc = "`write(|w| ..)` method takes [pdctl1::W](pdctl1::W) writer structure"]
impl crate::Writable for PDCTL1 {}
#[doc = "Power Domain Control"]
pub mod pdctl1;
#[doc = "CPU Power Domain Direct Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdctl1cpu](pdctl1cpu) module"]
pub type PDCTL1CPU = crate::Reg<u32, _PDCTL1CPU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDCTL1CPU;
#[doc = "`read()` method returns [pdctl1cpu::R](pdctl1cpu::R) reader structure"]
impl crate::Readable for PDCTL1CPU {}
#[doc = "`write(|w| ..)` method takes [pdctl1cpu::W](pdctl1cpu::W) writer structure"]
impl crate::Writable for PDCTL1CPU {}
#[doc = "CPU Power Domain Direct Control"]
pub mod pdctl1cpu;
#[doc = "RFC Power Domain Direct Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdctl1rfc](pdctl1rfc) module"]
pub type PDCTL1RFC = crate::Reg<u32, _PDCTL1RFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDCTL1RFC;
#[doc = "`read()` method returns [pdctl1rfc::R](pdctl1rfc::R) reader structure"]
impl crate::Readable for PDCTL1RFC {}
#[doc = "`write(|w| ..)` method takes [pdctl1rfc::W](pdctl1rfc::W) writer structure"]
impl crate::Writable for PDCTL1RFC {}
#[doc = "RFC Power Domain Direct Control"]
pub mod pdctl1rfc;
#[doc = "VIMS Mode Direct Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdctl1vims](pdctl1vims) module"]
pub type PDCTL1VIMS = crate::Reg<u32, _PDCTL1VIMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDCTL1VIMS;
#[doc = "`read()` method returns [pdctl1vims::R](pdctl1vims::R) reader structure"]
impl crate::Readable for PDCTL1VIMS {}
#[doc = "`write(|w| ..)` method takes [pdctl1vims::W](pdctl1vims::W) writer structure"]
impl crate::Writable for PDCTL1VIMS {}
#[doc = "VIMS Mode Direct Control"]
pub mod pdctl1vims;
#[doc = "Power Manager Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdstat1](pdstat1) module"]
pub type PDSTAT1 = crate::Reg<u32, _PDSTAT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDSTAT1;
#[doc = "`read()` method returns [pdstat1::R](pdstat1::R) reader structure"]
impl crate::Readable for PDSTAT1 {}
#[doc = "`write(|w| ..)` method takes [pdstat1::W](pdstat1::W) writer structure"]
impl crate::Writable for PDSTAT1 {}
#[doc = "Power Manager Status"]
pub mod pdstat1;
#[doc = "BUS Power Domain Direct Read Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdstat1bus](pdstat1bus) module"]
pub type PDSTAT1BUS = crate::Reg<u32, _PDSTAT1BUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDSTAT1BUS;
#[doc = "`read()` method returns [pdstat1bus::R](pdstat1bus::R) reader structure"]
impl crate::Readable for PDSTAT1BUS {}
#[doc = "`write(|w| ..)` method takes [pdstat1bus::W](pdstat1bus::W) writer structure"]
impl crate::Writable for PDSTAT1BUS {}
#[doc = "BUS Power Domain Direct Read Status"]
pub mod pdstat1bus;
#[doc = "RFC Power Domain Direct Read Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdstat1rfc](pdstat1rfc) module"]
pub type PDSTAT1RFC = crate::Reg<u32, _PDSTAT1RFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDSTAT1RFC;
#[doc = "`read()` method returns [pdstat1rfc::R](pdstat1rfc::R) reader structure"]
impl crate::Readable for PDSTAT1RFC {}
#[doc = "`write(|w| ..)` method takes [pdstat1rfc::W](pdstat1rfc::W) writer structure"]
impl crate::Writable for PDSTAT1RFC {}
#[doc = "RFC Power Domain Direct Read Status"]
pub mod pdstat1rfc;
#[doc = "CPU Power Domain Direct Read Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdstat1cpu](pdstat1cpu) module"]
pub type PDSTAT1CPU = crate::Reg<u32, _PDSTAT1CPU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDSTAT1CPU;
#[doc = "`read()` method returns [pdstat1cpu::R](pdstat1cpu::R) reader structure"]
impl crate::Readable for PDSTAT1CPU {}
#[doc = "`write(|w| ..)` method takes [pdstat1cpu::W](pdstat1cpu::W) writer structure"]
impl crate::Writable for PDSTAT1CPU {}
#[doc = "CPU Power Domain Direct Read Status"]
pub mod pdstat1cpu;
#[doc = "VIMS Mode Direct Read Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdstat1vims](pdstat1vims) module"]
pub type PDSTAT1VIMS = crate::Reg<u32, _PDSTAT1VIMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDSTAT1VIMS;
#[doc = "`read()` method returns [pdstat1vims::R](pdstat1vims::R) reader structure"]
impl crate::Readable for PDSTAT1VIMS {}
#[doc = "`write(|w| ..)` method takes [pdstat1vims::W](pdstat1vims::W) writer structure"]
impl crate::Writable for PDSTAT1VIMS {}
#[doc = "VIMS Mode Direct Read Status"]
pub mod pdstat1vims;
#[doc = "Control To RFC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcbits](rfcbits) module"]
pub type RFCBITS = crate::Reg<u32, _RFCBITS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFCBITS;
#[doc = "`read()` method returns [rfcbits::R](rfcbits::R) reader structure"]
impl crate::Readable for RFCBITS {}
#[doc = "`write(|w| ..)` method takes [rfcbits::W](rfcbits::W) writer structure"]
impl crate::Writable for RFCBITS {}
#[doc = "Control To RFC"]
pub mod rfcbits;
#[doc = "Selected RFC Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcmodesel](rfcmodesel) module"]
pub type RFCMODESEL = crate::Reg<u32, _RFCMODESEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFCMODESEL;
#[doc = "`read()` method returns [rfcmodesel::R](rfcmodesel::R) reader structure"]
impl crate::Readable for RFCMODESEL {}
#[doc = "`write(|w| ..)` method takes [rfcmodesel::W](rfcmodesel::W) writer structure"]
impl crate::Writable for RFCMODESEL {}
#[doc = "Selected RFC Mode"]
pub mod rfcmodesel;
#[doc = "Allowed RFC Modes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcmodehwopt](rfcmodehwopt) module"]
pub type RFCMODEHWOPT = crate::Reg<u32, _RFCMODEHWOPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFCMODEHWOPT;
#[doc = "`read()` method returns [rfcmodehwopt::R](rfcmodehwopt::R) reader structure"]
impl crate::Readable for RFCMODEHWOPT {}
#[doc = "`write(|w| ..)` method takes [rfcmodehwopt::W](rfcmodehwopt::W) writer structure"]
impl crate::Writable for RFCMODEHWOPT {}
#[doc = "Allowed RFC Modes"]
pub mod rfcmodehwopt;
#[doc = "Power Profiler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrprofstat](pwrprofstat) module"]
pub type PWRPROFSTAT = crate::Reg<u32, _PWRPROFSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRPROFSTAT;
#[doc = "`read()` method returns [pwrprofstat::R](pwrprofstat::R) reader structure"]
impl crate::Readable for PWRPROFSTAT {}
#[doc = "`write(|w| ..)` method takes [pwrprofstat::W](pwrprofstat::W) writer structure"]
impl crate::Writable for PWRPROFSTAT {}
#[doc = "Power Profiler Register"]
pub mod pwrprofstat;
#[doc = "MCU SRAM configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcusramcfg](mcusramcfg) module"]
pub type MCUSRAMCFG = crate::Reg<u32, _MCUSRAMCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCUSRAMCFG;
#[doc = "`read()` method returns [mcusramcfg::R](mcusramcfg::R) reader structure"]
impl crate::Readable for MCUSRAMCFG {}
#[doc = "`write(|w| ..)` method takes [mcusramcfg::W](mcusramcfg::W) writer structure"]
impl crate::Writable for MCUSRAMCFG {}
#[doc = "MCU SRAM configuration"]
pub mod mcusramcfg;
#[doc = "Memory Retention Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ramreten](ramreten) module"]
pub type RAMRETEN = crate::Reg<u32, _RAMRETEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAMRETEN;
#[doc = "`read()` method returns [ramreten::R](ramreten::R) reader structure"]
impl crate::Readable for RAMRETEN {}
#[doc = "`write(|w| ..)` method takes [ramreten::W](ramreten::W) writer structure"]
impl crate::Writable for RAMRETEN {}
#[doc = "Memory Retention Control"]
pub mod ramreten;
#[doc = "Oscillator Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscimsc](oscimsc) module"]
pub type OSCIMSC = crate::Reg<u32, _OSCIMSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCIMSC;
#[doc = "`read()` method returns [oscimsc::R](oscimsc::R) reader structure"]
impl crate::Readable for OSCIMSC {}
#[doc = "`write(|w| ..)` method takes [oscimsc::W](oscimsc::W) writer structure"]
impl crate::Writable for OSCIMSC {}
#[doc = "Oscillator Interrupt Mask"]
pub mod oscimsc;
#[doc = "Oscillator Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscris](oscris) module"]
pub type OSCRIS = crate::Reg<u32, _OSCRIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCRIS;
#[doc = "`read()` method returns [oscris::R](oscris::R) reader structure"]
impl crate::Readable for OSCRIS {}
#[doc = "`write(|w| ..)` method takes [oscris::W](oscris::W) writer structure"]
impl crate::Writable for OSCRIS {}
#[doc = "Oscillator Raw Interrupt Status"]
pub mod oscris;
#[doc = "Oscillator Raw Interrupt Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscicr](oscicr) module"]
pub type OSCICR = crate::Reg<u32, _OSCICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCICR;
#[doc = "`read()` method returns [oscicr::R](oscicr::R) reader structure"]
impl crate::Readable for OSCICR {}
#[doc = "`write(|w| ..)` method takes [oscicr::W](oscicr::W) writer structure"]
impl crate::Writable for OSCICR {}
#[doc = "Oscillator Raw Interrupt Clear"]
pub mod oscicr;
