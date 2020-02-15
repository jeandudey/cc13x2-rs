#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Input Output Mode This register controls pull-up, pull-down, and output mode for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    pub iomode: IOMODE,
    #[doc = "0x04 - General Purpose Input Output Digital Input Enable This register controls input buffers for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    pub gpiodie: GPIODIE,
    #[doc = "0x08 - Input Output Peripheral Output Enable This register selects the output source for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    pub iopoe: IOPOE,
    #[doc = "0x0c - General Purpose Input Output Data Out The output data register is used to set data on AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    pub gpiodout: GPIODOUT,
    #[doc = "0x10 - General Purpose Input Output Data In This register provides synchronized input data for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth"]
    pub gpiodin: GPIODIN,
    #[doc = "0x14 - General Purpose Input Output Data Out Set Set bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    pub gpiodoutset: GPIODOUTSET,
    #[doc = "0x18 - General Purpose Input Output Data Out Clear Clear bits in GPIODOUT instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    pub gpiodoutclr: GPIODOUTCLR,
    #[doc = "0x1c - General Purpose Input Output Data Out Toggle Toggle bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    pub gpiodouttgl: GPIODOUTTGL,
    #[doc = "0x20 - Input Output 0 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+0\\]
when IOPOE bit 0 is 1. To avoid glitches on AUXIO\\[8i+0\\]
you must configure this register while IOPOE bit 0 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    pub io0psel: IO0PSEL,
    #[doc = "0x24 - Input Output 1 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+1\\]
when IOPOE bit 1 is 1. To avoid glitches on AUXIO\\[8i+1\\]
you must configure this register while IOPOE bit 1 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    pub io1psel: IO1PSEL,
    #[doc = "0x28 - Input Output 2 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+2\\]
when IOPOE bit 2 is 1. To avoid glitches on AUXIO\\[8i+2\\]
you must configure this register while IOPOE bit 2 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    pub io2psel: IO2PSEL,
    #[doc = "0x2c - Input Output 3 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+3\\]
when IOPOE bit 3 is 1. To avoid glitches on AUXIO\\[8i+3\\]
you must configure this register while IOPOE bit 3 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    pub io3psel: IO3PSEL,
    #[doc = "0x30 - Input Output 4 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+4\\]
when IOPOE bit 4 is 1. To avoid glitches on AUXIO\\[8i+4\\]
you must configure this register while IOPOE bit 4 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    pub io4psel: IO4PSEL,
    #[doc = "0x34 - Input Output 5 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+5\\]
when IOPOE bit 5 is 1. To avoid glitches on AUXIO\\[8i+5\\]
you must configure this register while IOPOE bit 5 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    pub io5psel: IO5PSEL,
    #[doc = "0x38 - Input Output 6 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+6\\]
when IOPOE bit 6 is 1. To avoid glitches on AUXIO\\[8i+6\\]
you must configure this register while IOPOE bit 6 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    pub io6psel: IO6PSEL,
    #[doc = "0x3c - Input Output 7 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+7\\]
when IOPOE bit 7 is 1. To avoid glitches on AUXIO\\[8i+7\\]
you must configure this register while IOPOE bit 7 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    pub io7psel: IO7PSEL,
    #[doc = "0x40 - Input Output Mode Low This is an alias register for IOMODE.IO0 thru IOMODE.IO3."]
    pub iomodel: IOMODEL,
    #[doc = "0x44 - Input Output Mode High This is an alias register for IOMODE.IO4 thru IOMODE.IO7."]
    pub iomodeh: IOMODEH,
}
#[doc = "Input Output Mode This register controls pull-up, pull-down, and output mode for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iomode](iomode) module"]
pub type IOMODE = crate::Reg<u32, _IOMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOMODE;
#[doc = "`read()` method returns [iomode::R](iomode::R) reader structure"]
impl crate::Readable for IOMODE {}
#[doc = "`write(|w| ..)` method takes [iomode::W](iomode::W) writer structure"]
impl crate::Writable for IOMODE {}
#[doc = "Input Output Mode This register controls pull-up, pull-down, and output mode for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod iomode;
#[doc = "General Purpose Input Output Digital Input Enable This register controls input buffers for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiodie](gpiodie) module"]
pub type GPIODIE = crate::Reg<u32, _GPIODIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODIE;
#[doc = "`read()` method returns [gpiodie::R](gpiodie::R) reader structure"]
impl crate::Readable for GPIODIE {}
#[doc = "`write(|w| ..)` method takes [gpiodie::W](gpiodie::W) writer structure"]
impl crate::Writable for GPIODIE {}
#[doc = "General Purpose Input Output Digital Input Enable This register controls input buffers for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod gpiodie;
#[doc = "Input Output Peripheral Output Enable This register selects the output source for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iopoe](iopoe) module"]
pub type IOPOE = crate::Reg<u32, _IOPOE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOPOE;
#[doc = "`read()` method returns [iopoe::R](iopoe::R) reader structure"]
impl crate::Readable for IOPOE {}
#[doc = "`write(|w| ..)` method takes [iopoe::W](iopoe::W) writer structure"]
impl crate::Writable for IOPOE {}
#[doc = "Input Output Peripheral Output Enable This register selects the output source for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod iopoe;
#[doc = "General Purpose Input Output Data Out The output data register is used to set data on AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiodout](gpiodout) module"]
pub type GPIODOUT = crate::Reg<u32, _GPIODOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODOUT;
#[doc = "`read()` method returns [gpiodout::R](gpiodout::R) reader structure"]
impl crate::Readable for GPIODOUT {}
#[doc = "`write(|w| ..)` method takes [gpiodout::W](gpiodout::W) writer structure"]
impl crate::Writable for GPIODOUT {}
#[doc = "General Purpose Input Output Data Out The output data register is used to set data on AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod gpiodout;
#[doc = "General Purpose Input Output Data In This register provides synchronized input data for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiodin](gpiodin) module"]
pub type GPIODIN = crate::Reg<u32, _GPIODIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODIN;
#[doc = "`read()` method returns [gpiodin::R](gpiodin::R) reader structure"]
impl crate::Readable for GPIODIN {}
#[doc = "`write(|w| ..)` method takes [gpiodin::W](gpiodin::W) writer structure"]
impl crate::Writable for GPIODIN {}
#[doc = "General Purpose Input Output Data In This register provides synchronized input data for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth"]
pub mod gpiodin;
#[doc = "General Purpose Input Output Data Out Set Set bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiodoutset](gpiodoutset) module"]
pub type GPIODOUTSET = crate::Reg<u32, _GPIODOUTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODOUTSET;
#[doc = "`read()` method returns [gpiodoutset::R](gpiodoutset::R) reader structure"]
impl crate::Readable for GPIODOUTSET {}
#[doc = "`write(|w| ..)` method takes [gpiodoutset::W](gpiodoutset::W) writer structure"]
impl crate::Writable for GPIODOUTSET {}
#[doc = "General Purpose Input Output Data Out Set Set bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod gpiodoutset;
#[doc = "General Purpose Input Output Data Out Clear Clear bits in GPIODOUT instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiodoutclr](gpiodoutclr) module"]
pub type GPIODOUTCLR = crate::Reg<u32, _GPIODOUTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODOUTCLR;
#[doc = "`read()` method returns [gpiodoutclr::R](gpiodoutclr::R) reader structure"]
impl crate::Readable for GPIODOUTCLR {}
#[doc = "`write(|w| ..)` method takes [gpiodoutclr::W](gpiodoutclr::W) writer structure"]
impl crate::Writable for GPIODOUTCLR {}
#[doc = "General Purpose Input Output Data Out Clear Clear bits in GPIODOUT instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod gpiodoutclr;
#[doc = "General Purpose Input Output Data Out Toggle Toggle bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiodouttgl](gpiodouttgl) module"]
pub type GPIODOUTTGL = crate::Reg<u32, _GPIODOUTTGL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODOUTTGL;
#[doc = "`read()` method returns [gpiodouttgl::R](gpiodouttgl::R) reader structure"]
impl crate::Readable for GPIODOUTTGL {}
#[doc = "`write(|w| ..)` method takes [gpiodouttgl::W](gpiodouttgl::W) writer structure"]
impl crate::Writable for GPIODOUTTGL {}
#[doc = "General Purpose Input Output Data Out Toggle Toggle bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod gpiodouttgl;
#[doc = "Input Output 0 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+0\\]
when IOPOE bit 0 is 1. To avoid glitches on AUXIO\\[8i+0\\]
you must configure this register while IOPOE bit 0 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [io0psel](io0psel) module"]
pub type IO0PSEL = crate::Reg<u32, _IO0PSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IO0PSEL;
#[doc = "`read()` method returns [io0psel::R](io0psel::R) reader structure"]
impl crate::Readable for IO0PSEL {}
#[doc = "`write(|w| ..)` method takes [io0psel::W](io0psel::W) writer structure"]
impl crate::Writable for IO0PSEL {}
#[doc = "Input Output 0 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+0\\]
when IOPOE bit 0 is 1. To avoid glitches on AUXIO\\[8i+0\\]
you must configure this register while IOPOE bit 0 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod io0psel;
#[doc = "Input Output 1 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+1\\]
when IOPOE bit 1 is 1. To avoid glitches on AUXIO\\[8i+1\\]
you must configure this register while IOPOE bit 1 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [io1psel](io1psel) module"]
pub type IO1PSEL = crate::Reg<u32, _IO1PSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IO1PSEL;
#[doc = "`read()` method returns [io1psel::R](io1psel::R) reader structure"]
impl crate::Readable for IO1PSEL {}
#[doc = "`write(|w| ..)` method takes [io1psel::W](io1psel::W) writer structure"]
impl crate::Writable for IO1PSEL {}
#[doc = "Input Output 1 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+1\\]
when IOPOE bit 1 is 1. To avoid glitches on AUXIO\\[8i+1\\]
you must configure this register while IOPOE bit 1 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod io1psel;
#[doc = "Input Output 2 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+2\\]
when IOPOE bit 2 is 1. To avoid glitches on AUXIO\\[8i+2\\]
you must configure this register while IOPOE bit 2 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [io2psel](io2psel) module"]
pub type IO2PSEL = crate::Reg<u32, _IO2PSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IO2PSEL;
#[doc = "`read()` method returns [io2psel::R](io2psel::R) reader structure"]
impl crate::Readable for IO2PSEL {}
#[doc = "`write(|w| ..)` method takes [io2psel::W](io2psel::W) writer structure"]
impl crate::Writable for IO2PSEL {}
#[doc = "Input Output 2 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+2\\]
when IOPOE bit 2 is 1. To avoid glitches on AUXIO\\[8i+2\\]
you must configure this register while IOPOE bit 2 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod io2psel;
#[doc = "Input Output 3 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+3\\]
when IOPOE bit 3 is 1. To avoid glitches on AUXIO\\[8i+3\\]
you must configure this register while IOPOE bit 3 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [io3psel](io3psel) module"]
pub type IO3PSEL = crate::Reg<u32, _IO3PSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IO3PSEL;
#[doc = "`read()` method returns [io3psel::R](io3psel::R) reader structure"]
impl crate::Readable for IO3PSEL {}
#[doc = "`write(|w| ..)` method takes [io3psel::W](io3psel::W) writer structure"]
impl crate::Writable for IO3PSEL {}
#[doc = "Input Output 3 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+3\\]
when IOPOE bit 3 is 1. To avoid glitches on AUXIO\\[8i+3\\]
you must configure this register while IOPOE bit 3 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod io3psel;
#[doc = "Input Output 4 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+4\\]
when IOPOE bit 4 is 1. To avoid glitches on AUXIO\\[8i+4\\]
you must configure this register while IOPOE bit 4 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [io4psel](io4psel) module"]
pub type IO4PSEL = crate::Reg<u32, _IO4PSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IO4PSEL;
#[doc = "`read()` method returns [io4psel::R](io4psel::R) reader structure"]
impl crate::Readable for IO4PSEL {}
#[doc = "`write(|w| ..)` method takes [io4psel::W](io4psel::W) writer structure"]
impl crate::Writable for IO4PSEL {}
#[doc = "Input Output 4 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+4\\]
when IOPOE bit 4 is 1. To avoid glitches on AUXIO\\[8i+4\\]
you must configure this register while IOPOE bit 4 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod io4psel;
#[doc = "Input Output 5 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+5\\]
when IOPOE bit 5 is 1. To avoid glitches on AUXIO\\[8i+5\\]
you must configure this register while IOPOE bit 5 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [io5psel](io5psel) module"]
pub type IO5PSEL = crate::Reg<u32, _IO5PSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IO5PSEL;
#[doc = "`read()` method returns [io5psel::R](io5psel::R) reader structure"]
impl crate::Readable for IO5PSEL {}
#[doc = "`write(|w| ..)` method takes [io5psel::W](io5psel::W) writer structure"]
impl crate::Writable for IO5PSEL {}
#[doc = "Input Output 5 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+5\\]
when IOPOE bit 5 is 1. To avoid glitches on AUXIO\\[8i+5\\]
you must configure this register while IOPOE bit 5 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod io5psel;
#[doc = "Input Output 6 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+6\\]
when IOPOE bit 6 is 1. To avoid glitches on AUXIO\\[8i+6\\]
you must configure this register while IOPOE bit 6 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [io6psel](io6psel) module"]
pub type IO6PSEL = crate::Reg<u32, _IO6PSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IO6PSEL;
#[doc = "`read()` method returns [io6psel::R](io6psel::R) reader structure"]
impl crate::Readable for IO6PSEL {}
#[doc = "`write(|w| ..)` method takes [io6psel::W](io6psel::W) writer structure"]
impl crate::Writable for IO6PSEL {}
#[doc = "Input Output 6 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+6\\]
when IOPOE bit 6 is 1. To avoid glitches on AUXIO\\[8i+6\\]
you must configure this register while IOPOE bit 6 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod io6psel;
#[doc = "Input Output 7 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+7\\]
when IOPOE bit 7 is 1. To avoid glitches on AUXIO\\[8i+7\\]
you must configure this register while IOPOE bit 7 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [io7psel](io7psel) module"]
pub type IO7PSEL = crate::Reg<u32, _IO7PSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IO7PSEL;
#[doc = "`read()` method returns [io7psel::R](io7psel::R) reader structure"]
impl crate::Readable for IO7PSEL {}
#[doc = "`write(|w| ..)` method takes [io7psel::W](io7psel::W) writer structure"]
impl crate::Writable for IO7PSEL {}
#[doc = "Input Output 7 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+7\\]
when IOPOE bit 7 is 1. To avoid glitches on AUXIO\\[8i+7\\]
you must configure this register while IOPOE bit 7 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod io7psel;
#[doc = "Input Output Mode Low This is an alias register for IOMODE.IO0 thru IOMODE.IO3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iomodel](iomodel) module"]
pub type IOMODEL = crate::Reg<u32, _IOMODEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOMODEL;
#[doc = "`read()` method returns [iomodel::R](iomodel::R) reader structure"]
impl crate::Readable for IOMODEL {}
#[doc = "`write(|w| ..)` method takes [iomodel::W](iomodel::W) writer structure"]
impl crate::Writable for IOMODEL {}
#[doc = "Input Output Mode Low This is an alias register for IOMODE.IO0 thru IOMODE.IO3."]
pub mod iomodel;
#[doc = "Input Output Mode High This is an alias register for IOMODE.IO4 thru IOMODE.IO7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iomodeh](iomodeh) module"]
pub type IOMODEH = crate::Reg<u32, _IOMODEH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOMODEH;
#[doc = "`read()` method returns [iomodeh::R](iomodeh::R) reader structure"]
impl crate::Readable for IOMODEH {}
#[doc = "`write(|w| ..)` method takes [iomodeh::W](iomodeh::W) writer structure"]
impl crate::Writable for IOMODEH {}
#[doc = "Input Output Mode High This is an alias register for IOMODE.IO4 thru IOMODE.IO7."]
pub mod iomodeh;
