#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - ADC Control Configuration of ADI_4_AUX:ADC0.SMPL_MODE decides if the ADC trigger starts sampling or conversion."]
    pub adcctl: ADCCTL,
    #[doc = "0x14 - ADC FIFO Status FIFO can hold up to four ADC samples."]
    pub adcfifostat: ADCFIFOSTAT,
    #[doc = "0x18 - ADC FIFO"]
    pub adcfifo: ADCFIFO,
    #[doc = "0x1c - ADC Trigger"]
    pub adctrig: ADCTRIG,
    #[doc = "0x20 - Current Source Control"]
    pub isrcctl: ISRCCTL,
    _reserved5: [u8; 12usize],
    #[doc = "0x30 - DAC Control This register controls the analog part of the DAC."]
    pub dacctl: DACCTL,
    #[doc = "0x34 - Low Power Mode Bias Control The low power mode bias module provides bias current to DAC and Comparator A when AUX_SYSIF:OPMODEREQ.REQ differers from A."]
    pub lpmbiasctl: LPMBIASCTL,
    #[doc = "0x38 - DAC Sample Control The DAC sample clock maintains the DAC voltage stored in the sample-and-hold capacitor. The DAC sample clock waveform consists of a setup phase followed by a hold phase. In the setup phase the sample-and-hold capacitor charges to the programmed voltage. The hold phase maintains the voltage with minimal power. DACSMPLCFG0 and DACSMPLCFG1 configure the DAC sample clock waveform."]
    pub dacsmplctl: DACSMPLCTL,
    #[doc = "0x3c - DAC Sample Configuration 0"]
    pub dacsmplcfg0: DACSMPLCFG0,
    #[doc = "0x40 - DAC Sample Configuration 1 The sample clock period equals (high time + low time) * base period. DACSMPLCFG0.CLKDIV determines the base period. Timing requirements (DAC Buffer On / DAC Buffer Off): - (high time + low time) * base period > (4 us / 1 us) - (high time * base period) > (2 us / 0.5 us) - (low time * base period) > (2 us / 0.5 us) - (low time * base period + HOLD_INTERVAL * sample clock period) < 32 us If AUX_SYSIF:OPMODEREQ.REQ equals PDLP, you must set: - H_PER = L_PER = HOLD_INTERVAL = 0."]
    pub dacsmplcfg1: DACSMPLCFG1,
    #[doc = "0x44 - DAC Value"]
    pub dacvalue: DACVALUE,
    #[doc = "0x48 - DAC Status"]
    pub dacstat: DACSTAT,
}
#[doc = "ADC Control Configuration of ADI_4_AUX:ADC0.SMPL_MODE decides if the ADC trigger starts sampling or conversion.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcctl](adcctl) module"]
pub type ADCCTL = crate::Reg<u32, _ADCCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCCTL;
#[doc = "`read()` method returns [adcctl::R](adcctl::R) reader structure"]
impl crate::Readable for ADCCTL {}
#[doc = "`write(|w| ..)` method takes [adcctl::W](adcctl::W) writer structure"]
impl crate::Writable for ADCCTL {}
#[doc = "ADC Control Configuration of ADI_4_AUX:ADC0.SMPL_MODE decides if the ADC trigger starts sampling or conversion."]
pub mod adcctl;
#[doc = "ADC FIFO Status FIFO can hold up to four ADC samples.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcfifostat](adcfifostat) module"]
pub type ADCFIFOSTAT = crate::Reg<u32, _ADCFIFOSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCFIFOSTAT;
#[doc = "`read()` method returns [adcfifostat::R](adcfifostat::R) reader structure"]
impl crate::Readable for ADCFIFOSTAT {}
#[doc = "`write(|w| ..)` method takes [adcfifostat::W](adcfifostat::W) writer structure"]
impl crate::Writable for ADCFIFOSTAT {}
#[doc = "ADC FIFO Status FIFO can hold up to four ADC samples."]
pub mod adcfifostat;
#[doc = "ADC FIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcfifo](adcfifo) module"]
pub type ADCFIFO = crate::Reg<u32, _ADCFIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCFIFO;
#[doc = "`read()` method returns [adcfifo::R](adcfifo::R) reader structure"]
impl crate::Readable for ADCFIFO {}
#[doc = "`write(|w| ..)` method takes [adcfifo::W](adcfifo::W) writer structure"]
impl crate::Writable for ADCFIFO {}
#[doc = "ADC FIFO"]
pub mod adcfifo;
#[doc = "ADC Trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adctrig](adctrig) module"]
pub type ADCTRIG = crate::Reg<u32, _ADCTRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCTRIG;
#[doc = "`read()` method returns [adctrig::R](adctrig::R) reader structure"]
impl crate::Readable for ADCTRIG {}
#[doc = "`write(|w| ..)` method takes [adctrig::W](adctrig::W) writer structure"]
impl crate::Writable for ADCTRIG {}
#[doc = "ADC Trigger"]
pub mod adctrig;
#[doc = "Current Source Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isrcctl](isrcctl) module"]
pub type ISRCCTL = crate::Reg<u32, _ISRCCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISRCCTL;
#[doc = "`read()` method returns [isrcctl::R](isrcctl::R) reader structure"]
impl crate::Readable for ISRCCTL {}
#[doc = "`write(|w| ..)` method takes [isrcctl::W](isrcctl::W) writer structure"]
impl crate::Writable for ISRCCTL {}
#[doc = "Current Source Control"]
pub mod isrcctl;
#[doc = "DAC Control This register controls the analog part of the DAC.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacctl](dacctl) module"]
pub type DACCTL = crate::Reg<u32, _DACCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DACCTL;
#[doc = "`read()` method returns [dacctl::R](dacctl::R) reader structure"]
impl crate::Readable for DACCTL {}
#[doc = "`write(|w| ..)` method takes [dacctl::W](dacctl::W) writer structure"]
impl crate::Writable for DACCTL {}
#[doc = "DAC Control This register controls the analog part of the DAC."]
pub mod dacctl;
#[doc = "Low Power Mode Bias Control The low power mode bias module provides bias current to DAC and Comparator A when AUX_SYSIF:OPMODEREQ.REQ differers from A.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmbiasctl](lpmbiasctl) module"]
pub type LPMBIASCTL = crate::Reg<u32, _LPMBIASCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPMBIASCTL;
#[doc = "`read()` method returns [lpmbiasctl::R](lpmbiasctl::R) reader structure"]
impl crate::Readable for LPMBIASCTL {}
#[doc = "`write(|w| ..)` method takes [lpmbiasctl::W](lpmbiasctl::W) writer structure"]
impl crate::Writable for LPMBIASCTL {}
#[doc = "Low Power Mode Bias Control The low power mode bias module provides bias current to DAC and Comparator A when AUX_SYSIF:OPMODEREQ.REQ differers from A."]
pub mod lpmbiasctl;
#[doc = "DAC Sample Control The DAC sample clock maintains the DAC voltage stored in the sample-and-hold capacitor. The DAC sample clock waveform consists of a setup phase followed by a hold phase. In the setup phase the sample-and-hold capacitor charges to the programmed voltage. The hold phase maintains the voltage with minimal power. DACSMPLCFG0 and DACSMPLCFG1 configure the DAC sample clock waveform.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacsmplctl](dacsmplctl) module"]
pub type DACSMPLCTL = crate::Reg<u32, _DACSMPLCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DACSMPLCTL;
#[doc = "`read()` method returns [dacsmplctl::R](dacsmplctl::R) reader structure"]
impl crate::Readable for DACSMPLCTL {}
#[doc = "`write(|w| ..)` method takes [dacsmplctl::W](dacsmplctl::W) writer structure"]
impl crate::Writable for DACSMPLCTL {}
#[doc = "DAC Sample Control The DAC sample clock maintains the DAC voltage stored in the sample-and-hold capacitor. The DAC sample clock waveform consists of a setup phase followed by a hold phase. In the setup phase the sample-and-hold capacitor charges to the programmed voltage. The hold phase maintains the voltage with minimal power. DACSMPLCFG0 and DACSMPLCFG1 configure the DAC sample clock waveform."]
pub mod dacsmplctl;
#[doc = "DAC Sample Configuration 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacsmplcfg0](dacsmplcfg0) module"]
pub type DACSMPLCFG0 = crate::Reg<u32, _DACSMPLCFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DACSMPLCFG0;
#[doc = "`read()` method returns [dacsmplcfg0::R](dacsmplcfg0::R) reader structure"]
impl crate::Readable for DACSMPLCFG0 {}
#[doc = "`write(|w| ..)` method takes [dacsmplcfg0::W](dacsmplcfg0::W) writer structure"]
impl crate::Writable for DACSMPLCFG0 {}
#[doc = "DAC Sample Configuration 0"]
pub mod dacsmplcfg0;
#[doc = "DAC Sample Configuration 1 The sample clock period equals (high time + low time) * base period. DACSMPLCFG0.CLKDIV determines the base period. Timing requirements (DAC Buffer On / DAC Buffer Off): - (high time + low time) * base period > (4 us / 1 us) - (high time * base period) > (2 us / 0.5 us) - (low time * base period) > (2 us / 0.5 us) - (low time * base period + HOLD_INTERVAL * sample clock period) < 32 us If AUX_SYSIF:OPMODEREQ.REQ equals PDLP, you must set: - H_PER = L_PER = HOLD_INTERVAL = 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacsmplcfg1](dacsmplcfg1) module"]
pub type DACSMPLCFG1 = crate::Reg<u32, _DACSMPLCFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DACSMPLCFG1;
#[doc = "`read()` method returns [dacsmplcfg1::R](dacsmplcfg1::R) reader structure"]
impl crate::Readable for DACSMPLCFG1 {}
#[doc = "`write(|w| ..)` method takes [dacsmplcfg1::W](dacsmplcfg1::W) writer structure"]
impl crate::Writable for DACSMPLCFG1 {}
#[doc = "DAC Sample Configuration 1 The sample clock period equals (high time + low time) * base period. DACSMPLCFG0.CLKDIV determines the base period. Timing requirements (DAC Buffer On / DAC Buffer Off): - (high time + low time) * base period > (4 us / 1 us) - (high time * base period) > (2 us / 0.5 us) - (low time * base period) > (2 us / 0.5 us) - (low time * base period + HOLD_INTERVAL * sample clock period) < 32 us If AUX_SYSIF:OPMODEREQ.REQ equals PDLP, you must set: - H_PER = L_PER = HOLD_INTERVAL = 0."]
pub mod dacsmplcfg1;
#[doc = "DAC Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacvalue](dacvalue) module"]
pub type DACVALUE = crate::Reg<u32, _DACVALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DACVALUE;
#[doc = "`read()` method returns [dacvalue::R](dacvalue::R) reader structure"]
impl crate::Readable for DACVALUE {}
#[doc = "`write(|w| ..)` method takes [dacvalue::W](dacvalue::W) writer structure"]
impl crate::Writable for DACVALUE {}
#[doc = "DAC Value"]
pub mod dacvalue;
#[doc = "DAC Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacstat](dacstat) module"]
pub type DACSTAT = crate::Reg<u32, _DACSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DACSTAT;
#[doc = "`read()` method returns [dacstat::R](dacstat::R) reader structure"]
impl crate::Readable for DACSTAT {}
#[doc = "`write(|w| ..)` method takes [dacstat::W](dacstat::W) writer structure"]
impl crate::Writable for DACSTAT {}
#[doc = "DAC Status"]
pub mod dacstat;
