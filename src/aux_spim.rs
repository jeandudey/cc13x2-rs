#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI Master Configuration Write operation stalls until current transfer completes."]
    pub spimcfg: SPIMCFG,
    #[doc = "0x04 - MISO Configuration Write operation stalls until current transfer completes."]
    pub misocfg: MISOCFG,
    #[doc = "0x08 - MOSI Control Write operation stalls until current transfer completes."]
    pub mosictl: MOSICTL,
    #[doc = "0x0c - Transmit 8 Bit Write operation stalls until current transfer completes."]
    pub tx8: TX8,
    #[doc = "0x10 - Transmit 16 Bit Write operation stalls until current transfer completes."]
    pub tx16: TX16,
    #[doc = "0x14 - Receive 8 Bit Read operation stalls until current transfer completes."]
    pub rx8: RX8,
    #[doc = "0x18 - Receive 16 Bit Read operation stalls until current transfer completes."]
    pub rx16: RX16,
    #[doc = "0x1c - SCLK Idle Read operation stalls until SCLK is idle with no remaining clock edges."]
    pub sclkidle: SCLKIDLE,
    #[doc = "0x20 - Data Idle Read operation stalls until current transfer completes."]
    pub dataidle: DATAIDLE,
}
#[doc = "SPI Master Configuration Write operation stalls until current transfer completes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spimcfg](spimcfg) module"]
pub type SPIMCFG = crate::Reg<u32, _SPIMCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPIMCFG;
#[doc = "`read()` method returns [spimcfg::R](spimcfg::R) reader structure"]
impl crate::Readable for SPIMCFG {}
#[doc = "`write(|w| ..)` method takes [spimcfg::W](spimcfg::W) writer structure"]
impl crate::Writable for SPIMCFG {}
#[doc = "SPI Master Configuration Write operation stalls until current transfer completes."]
pub mod spimcfg;
#[doc = "MISO Configuration Write operation stalls until current transfer completes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misocfg](misocfg) module"]
pub type MISOCFG = crate::Reg<u32, _MISOCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISOCFG;
#[doc = "`read()` method returns [misocfg::R](misocfg::R) reader structure"]
impl crate::Readable for MISOCFG {}
#[doc = "`write(|w| ..)` method takes [misocfg::W](misocfg::W) writer structure"]
impl crate::Writable for MISOCFG {}
#[doc = "MISO Configuration Write operation stalls until current transfer completes."]
pub mod misocfg;
#[doc = "MOSI Control Write operation stalls until current transfer completes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mosictl](mosictl) module"]
pub type MOSICTL = crate::Reg<u32, _MOSICTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MOSICTL;
#[doc = "`read()` method returns [mosictl::R](mosictl::R) reader structure"]
impl crate::Readable for MOSICTL {}
#[doc = "`write(|w| ..)` method takes [mosictl::W](mosictl::W) writer structure"]
impl crate::Writable for MOSICTL {}
#[doc = "MOSI Control Write operation stalls until current transfer completes."]
pub mod mosictl;
#[doc = "Transmit 8 Bit Write operation stalls until current transfer completes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx8](tx8) module"]
pub type TX8 = crate::Reg<u32, _TX8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX8;
#[doc = "`read()` method returns [tx8::R](tx8::R) reader structure"]
impl crate::Readable for TX8 {}
#[doc = "`write(|w| ..)` method takes [tx8::W](tx8::W) writer structure"]
impl crate::Writable for TX8 {}
#[doc = "Transmit 8 Bit Write operation stalls until current transfer completes."]
pub mod tx8;
#[doc = "Transmit 16 Bit Write operation stalls until current transfer completes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx16](tx16) module"]
pub type TX16 = crate::Reg<u32, _TX16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX16;
#[doc = "`read()` method returns [tx16::R](tx16::R) reader structure"]
impl crate::Readable for TX16 {}
#[doc = "`write(|w| ..)` method takes [tx16::W](tx16::W) writer structure"]
impl crate::Writable for TX16 {}
#[doc = "Transmit 16 Bit Write operation stalls until current transfer completes."]
pub mod tx16;
#[doc = "Receive 8 Bit Read operation stalls until current transfer completes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx8](rx8) module"]
pub type RX8 = crate::Reg<u32, _RX8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX8;
#[doc = "`read()` method returns [rx8::R](rx8::R) reader structure"]
impl crate::Readable for RX8 {}
#[doc = "`write(|w| ..)` method takes [rx8::W](rx8::W) writer structure"]
impl crate::Writable for RX8 {}
#[doc = "Receive 8 Bit Read operation stalls until current transfer completes."]
pub mod rx8;
#[doc = "Receive 16 Bit Read operation stalls until current transfer completes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx16](rx16) module"]
pub type RX16 = crate::Reg<u32, _RX16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX16;
#[doc = "`read()` method returns [rx16::R](rx16::R) reader structure"]
impl crate::Readable for RX16 {}
#[doc = "`write(|w| ..)` method takes [rx16::W](rx16::W) writer structure"]
impl crate::Writable for RX16 {}
#[doc = "Receive 16 Bit Read operation stalls until current transfer completes."]
pub mod rx16;
#[doc = "SCLK Idle Read operation stalls until SCLK is idle with no remaining clock edges.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sclkidle](sclkidle) module"]
pub type SCLKIDLE = crate::Reg<u32, _SCLKIDLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCLKIDLE;
#[doc = "`read()` method returns [sclkidle::R](sclkidle::R) reader structure"]
impl crate::Readable for SCLKIDLE {}
#[doc = "`write(|w| ..)` method takes [sclkidle::W](sclkidle::W) writer structure"]
impl crate::Writable for SCLKIDLE {}
#[doc = "SCLK Idle Read operation stalls until SCLK is idle with no remaining clock edges."]
pub mod sclkidle;
#[doc = "Data Idle Read operation stalls until current transfer completes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dataidle](dataidle) module"]
pub type DATAIDLE = crate::Reg<u32, _DATAIDLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAIDLE;
#[doc = "`read()` method returns [dataidle::R](dataidle::R) reader structure"]
impl crate::Readable for DATAIDLE {}
#[doc = "`write(|w| ..)` method takes [dataidle::W](dataidle::W) writer structure"]
impl crate::Writable for DATAIDLE {}
#[doc = "Data Idle Read operation stalls until current transfer completes."]
pub mod dataidle;
