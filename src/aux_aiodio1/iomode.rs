#[doc = "Reader of register IOMODE"]
pub type R = crate::R<u32, super::IOMODE>;
#[doc = "Writer for register IOMODE"]
pub type W = crate::W<u32, super::IOMODE>;
#[doc = "Register IOMODE `reset()`'s with value 0"]
impl crate::ResetValue for super::IOMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED16`"]
pub type RESERVED16_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED16`"]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "15:14\\]
Selects mode for AUXIO\\[8i+7\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IO7_A {
    #[doc = "3: Open-Source Mode: \n\nWhen IOPOE bit 7 is 0: \n- If GPIODOUT bit 7 is 0: AUXIO\\[8i+7\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n- If GPIODOUT bit 7 is 1: AUXIO\\[8i+7\\]
is driven high.\n\nWhen IOPOE bit 7 is 1: \n- If signal selected by IO7PSEL.SRC is 0: AUXIO\\[8i+7\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n- If signal selected by IO7PSEL.SRC is 1: AUXIO\\[8i+7\\]
is driven high."]
    OPEN_SOURCE = 3,
    #[doc = "2: Open-Drain Mode: \n\nWhen IOPOE bit 7 is 0: \n- If GPIODOUT bit 7 is 0: AUXIO\\[8i+7\\]
is driven low.  \n- If GPIODOUT bit 7 is 1: AUXIO\\[8i+7\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n\nWhen IOPOE bit 7 is 1: \n- If signal selected by IO7PSEL.SRC is 0: AUXIO\\[8i+7\\]
is driven low. \n- If signal selected by IO7PSEL.SRC is 1: AUXIO\\[8i+7\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OPEN_DRAIN = 2,
    #[doc = "1: Input Mode:\n\nWhen GPIODIE bit 7 is 0: AUXIO\\[8i+7\\]
is enabled for analog signal transfer.\n\nWhen GPIODIE bit 7 is 1: AUXIO\\[8i+7\\]
is enabled for digital input."]
    IN = 1,
    #[doc = "0: Output Mode:\n\nWhen IOPOE bit 7 is 0: GPIODOUT bit 7 drives AUXIO\\[8i+7\\].\n\nWhen IOPOE bit 7 is 1: The signal selected by IO7PSEL.SRC drives AUXIO\\[8i+7\\]."]
    OUT = 0,
}
impl From<IO7_A> for u8 {
    #[inline(always)]
    fn from(variant: IO7_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IO7`"]
pub type IO7_R = crate::R<u8, IO7_A>;
impl IO7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO7_A {
        match self.bits {
            3 => IO7_A::OPEN_SOURCE,
            2 => IO7_A::OPEN_DRAIN,
            1 => IO7_A::IN,
            0 => IO7_A::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_SOURCE`"]
    #[inline(always)]
    pub fn is_open_source(&self) -> bool {
        *self == IO7_A::OPEN_SOURCE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == IO7_A::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in_(&self) -> bool {
        *self == IO7_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == IO7_A::OUT
    }
}
#[doc = "Write proxy for field `IO7`"]
pub struct IO7_W<'a> {
    w: &'a mut W,
}
impl<'a> IO7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO7_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Open-Source Mode: When IOPOE bit 7 is 0: - If GPIODOUT bit 7 is 0: AUXIO\\[8i+7\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 7 is 1: AUXIO\\[8i+7\\]
is driven high. When IOPOE bit 7 is 1: - If signal selected by IO7PSEL.SRC is 0: AUXIO\\[8i+7\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO7PSEL.SRC is 1: AUXIO\\[8i+7\\]
is driven high."]
    #[inline(always)]
    pub fn open_source(self) -> &'a mut W {
        self.variant(IO7_A::OPEN_SOURCE)
    }
    #[doc = "Open-Drain Mode: When IOPOE bit 7 is 0: - If GPIODOUT bit 7 is 0: AUXIO\\[8i+7\\]
is driven low. - If GPIODOUT bit 7 is 1: AUXIO\\[8i+7\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 7 is 1: - If signal selected by IO7PSEL.SRC is 0: AUXIO\\[8i+7\\]
is driven low. - If signal selected by IO7PSEL.SRC is 1: AUXIO\\[8i+7\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(IO7_A::OPEN_DRAIN)
    }
    #[doc = "Input Mode: When GPIODIE bit 7 is 0: AUXIO\\[8i+7\\]
is enabled for analog signal transfer. When GPIODIE bit 7 is 1: AUXIO\\[8i+7\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO7_A::IN)
    }
    #[doc = "Output Mode: When IOPOE bit 7 is 0: GPIODOUT bit 7 drives AUXIO\\[8i+7\\]. When IOPOE bit 7 is 1: The signal selected by IO7PSEL.SRC drives AUXIO\\[8i+7\\]."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(IO7_A::OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "13:12\\]
Selects mode for AUXIO\\[8i+6\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IO6_A {
    #[doc = "3: Open-Source Mode: \n\nWhen IOPOE bit 6 is 0: \n- If GPIODOUT bit 6 is 0: AUXIO\\[8i+6\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n- If GPIODOUT bit 6 is 1: AUXIO\\[8i+6\\]
is driven high.\n\nWhen IOPOE bit 6 is 1: \n- If signal selected by IO6PSEL.SRC is 0: AUXIO\\[8i+6\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n- If signal selected by IO6PSEL.SRC is 1: AUXIO\\[8i+6\\]
is driven high."]
    OPEN_SOURCE = 3,
    #[doc = "2: Open-Drain Mode: \n\nWhen IOPOE bit 6 is 0: \n- If GPIODOUT bit 6 is 0: AUXIO\\[8i+6\\]
is driven low.  \n- If GPIODOUT bit 6 is 1: AUXIO\\[8i+6\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n\nWhen IOPOE bit 6 is 1: \n- If signal selected by IO6PSEL.SRC is 0: AUXIO\\[8i+6\\]
is driven low. \n- If signal selected by IO6PSEL.SRC is 1: AUXIO\\[8i+6\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OPEN_DRAIN = 2,
    #[doc = "1: Input Mode:\n\nWhen GPIODIE bit 6 is 0: AUXIO\\[8i+6\\]
is enabled for analog signal transfer.\n\nWhen GPIODIE bit 6 is 1: AUXIO\\[8i+6\\]
is enabled for digital input."]
    IN = 1,
    #[doc = "0: Output Mode:\n\nWhen IOPOE bit 6 is 0: GPIODOUT bit 6 drives AUXIO\\[8i+6\\].\n\nWhen IOPOE bit 6 is 1: The signal selected by IO6PSEL.SRC drives AUXIO\\[8i+6\\]."]
    OUT = 0,
}
impl From<IO6_A> for u8 {
    #[inline(always)]
    fn from(variant: IO6_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IO6`"]
pub type IO6_R = crate::R<u8, IO6_A>;
impl IO6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO6_A {
        match self.bits {
            3 => IO6_A::OPEN_SOURCE,
            2 => IO6_A::OPEN_DRAIN,
            1 => IO6_A::IN,
            0 => IO6_A::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_SOURCE`"]
    #[inline(always)]
    pub fn is_open_source(&self) -> bool {
        *self == IO6_A::OPEN_SOURCE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == IO6_A::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in_(&self) -> bool {
        *self == IO6_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == IO6_A::OUT
    }
}
#[doc = "Write proxy for field `IO6`"]
pub struct IO6_W<'a> {
    w: &'a mut W,
}
impl<'a> IO6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO6_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Open-Source Mode: When IOPOE bit 6 is 0: - If GPIODOUT bit 6 is 0: AUXIO\\[8i+6\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 6 is 1: AUXIO\\[8i+6\\]
is driven high. When IOPOE bit 6 is 1: - If signal selected by IO6PSEL.SRC is 0: AUXIO\\[8i+6\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO6PSEL.SRC is 1: AUXIO\\[8i+6\\]
is driven high."]
    #[inline(always)]
    pub fn open_source(self) -> &'a mut W {
        self.variant(IO6_A::OPEN_SOURCE)
    }
    #[doc = "Open-Drain Mode: When IOPOE bit 6 is 0: - If GPIODOUT bit 6 is 0: AUXIO\\[8i+6\\]
is driven low. - If GPIODOUT bit 6 is 1: AUXIO\\[8i+6\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 6 is 1: - If signal selected by IO6PSEL.SRC is 0: AUXIO\\[8i+6\\]
is driven low. - If signal selected by IO6PSEL.SRC is 1: AUXIO\\[8i+6\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(IO6_A::OPEN_DRAIN)
    }
    #[doc = "Input Mode: When GPIODIE bit 6 is 0: AUXIO\\[8i+6\\]
is enabled for analog signal transfer. When GPIODIE bit 6 is 1: AUXIO\\[8i+6\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO6_A::IN)
    }
    #[doc = "Output Mode: When IOPOE bit 6 is 0: GPIODOUT bit 6 drives AUXIO\\[8i+6\\]. When IOPOE bit 6 is 1: The signal selected by IO6PSEL.SRC drives AUXIO\\[8i+6\\]."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(IO6_A::OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "11:10\\]
Selects mode for AUXIO\\[8i+5\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IO5_A {
    #[doc = "3: Open-Source Mode: \n\nWhen IOPOE bit 5 is 0: \n- If GPIODOUT bit 5 is 0: AUXIO\\[8i+5\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n- If GPIODOUT bit 5 is 1: AUXIO\\[8i+5\\]
is driven high.\n\nWhen IOPOE bit 5 is 1: \n- If signal selected by IO5PSEL.SRC is 0: AUXIO\\[8i+5\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n- If signal selected by IO5PSEL.SRC is 1: AUXIO\\[8i+5\\]
is driven high."]
    OPEN_SOURCE = 3,
    #[doc = "2: Open-Drain Mode: \n\nWhen IOPOE bit 5 is 0: \n- If GPIODOUT bit 5 is 0: AUXIO\\[8i+5\\]
is driven low.  \n- If GPIODOUT bit 5 is 1: AUXIO\\[8i+5\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n\nWhen IOPOE bit 5 is 1: \n- If signal selected by IO5PSEL.SRC is 0: AUXIO\\[8i+5\\]
is driven low. \n- If signal selected by IO5PSEL.SRC is 1: AUXIO\\[8i+5\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OPEN_DRAIN = 2,
    #[doc = "1: Input Mode:\n\nWhen GPIODIE bit 5 is 0: AUXIO\\[8i+5\\]
is enabled for analog signal transfer.\n\nWhen GPIODIE bit 5 is 1: AUXIO\\[8i+5\\]
is enabled for digital input."]
    IN = 1,
    #[doc = "0: Output Mode:\n\nWhen IOPOE bit 5 is 0: GPIODOUT bit 5 drives AUXIO\\[8i+5\\].\n\nWhen IOPOE bit 5 is 1: The signal selected by IO5PSEL.SRC drives AUXIO\\[8i+5\\]."]
    OUT = 0,
}
impl From<IO5_A> for u8 {
    #[inline(always)]
    fn from(variant: IO5_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IO5`"]
pub type IO5_R = crate::R<u8, IO5_A>;
impl IO5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO5_A {
        match self.bits {
            3 => IO5_A::OPEN_SOURCE,
            2 => IO5_A::OPEN_DRAIN,
            1 => IO5_A::IN,
            0 => IO5_A::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_SOURCE`"]
    #[inline(always)]
    pub fn is_open_source(&self) -> bool {
        *self == IO5_A::OPEN_SOURCE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == IO5_A::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in_(&self) -> bool {
        *self == IO5_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == IO5_A::OUT
    }
}
#[doc = "Write proxy for field `IO5`"]
pub struct IO5_W<'a> {
    w: &'a mut W,
}
impl<'a> IO5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO5_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Open-Source Mode: When IOPOE bit 5 is 0: - If GPIODOUT bit 5 is 0: AUXIO\\[8i+5\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 5 is 1: AUXIO\\[8i+5\\]
is driven high. When IOPOE bit 5 is 1: - If signal selected by IO5PSEL.SRC is 0: AUXIO\\[8i+5\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO5PSEL.SRC is 1: AUXIO\\[8i+5\\]
is driven high."]
    #[inline(always)]
    pub fn open_source(self) -> &'a mut W {
        self.variant(IO5_A::OPEN_SOURCE)
    }
    #[doc = "Open-Drain Mode: When IOPOE bit 5 is 0: - If GPIODOUT bit 5 is 0: AUXIO\\[8i+5\\]
is driven low. - If GPIODOUT bit 5 is 1: AUXIO\\[8i+5\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 5 is 1: - If signal selected by IO5PSEL.SRC is 0: AUXIO\\[8i+5\\]
is driven low. - If signal selected by IO5PSEL.SRC is 1: AUXIO\\[8i+5\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(IO5_A::OPEN_DRAIN)
    }
    #[doc = "Input Mode: When GPIODIE bit 5 is 0: AUXIO\\[8i+5\\]
is enabled for analog signal transfer. When GPIODIE bit 5 is 1: AUXIO\\[8i+5\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO5_A::IN)
    }
    #[doc = "Output Mode: When IOPOE bit 5 is 0: GPIODOUT bit 5 drives AUXIO\\[8i+5\\]. When IOPOE bit 5 is 1: The signal selected by IO5PSEL.SRC drives AUXIO\\[8i+5\\]."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(IO5_A::OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "9:8\\]
Selects mode for AUXIO\\[8i+4\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IO4_A {
    #[doc = "3: Open-Source Mode: \n\nWhen IOPOE bit 4 is 0: \n- If GPIODOUT bit 4 is 0: AUXIO\\[8i+4\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n- If GPIODOUT bit 4 is 1: AUXIO\\[8i+4\\]
is driven high.\n\nWhen IOPOE bit 4 is 1: \n- If signal selected by IO4PSEL.SRC is 0: AUXIO\\[8i+4\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n- If signal selected by IO4PSEL.SRC is 1: AUXIO\\[8i+4\\]
is driven high."]
    OPEN_SOURCE = 3,
    #[doc = "2: Open-Drain Mode: \n\nWhen IOPOE bit 4 is 0: \n- If GPIODOUT bit 4 is 0: AUXIO\\[8i+4\\]
is driven low.  \n- If GPIODOUT bit 4 is 1: AUXIO\\[8i+4\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n\nWhen IOPOE bit 4 is 1: \n- If signal selected by IO4PSEL.SRC is 0: AUXIO\\[8i+4\\]
is driven low. \n- If signal selected by IO4PSEL.SRC is 1: AUXIO\\[8i+4\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OPEN_DRAIN = 2,
    #[doc = "1: Input Mode:\n\nWhen GPIODIE bit 4 is 0: AUXIO\\[8i+4\\]
is enabled for analog signal transfer.\n\nWhen GPIODIE bit 4 is 1: AUXIO\\[8i+4\\]
is enabled for digital input."]
    IN = 1,
    #[doc = "0: Output Mode:\n\nWhen IOPOE bit 4 is 0: GPIODOUT bit 4 drives AUXIO\\[8i+4\\].\n\nWhen IOPOE bit 4 is 1: The signal selected by IO4PSEL.SRC drives AUXIO\\[8i+4\\]."]
    OUT = 0,
}
impl From<IO4_A> for u8 {
    #[inline(always)]
    fn from(variant: IO4_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IO4`"]
pub type IO4_R = crate::R<u8, IO4_A>;
impl IO4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO4_A {
        match self.bits {
            3 => IO4_A::OPEN_SOURCE,
            2 => IO4_A::OPEN_DRAIN,
            1 => IO4_A::IN,
            0 => IO4_A::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_SOURCE`"]
    #[inline(always)]
    pub fn is_open_source(&self) -> bool {
        *self == IO4_A::OPEN_SOURCE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == IO4_A::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in_(&self) -> bool {
        *self == IO4_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == IO4_A::OUT
    }
}
#[doc = "Write proxy for field `IO4`"]
pub struct IO4_W<'a> {
    w: &'a mut W,
}
impl<'a> IO4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO4_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Open-Source Mode: When IOPOE bit 4 is 0: - If GPIODOUT bit 4 is 0: AUXIO\\[8i+4\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 4 is 1: AUXIO\\[8i+4\\]
is driven high. When IOPOE bit 4 is 1: - If signal selected by IO4PSEL.SRC is 0: AUXIO\\[8i+4\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO4PSEL.SRC is 1: AUXIO\\[8i+4\\]
is driven high."]
    #[inline(always)]
    pub fn open_source(self) -> &'a mut W {
        self.variant(IO4_A::OPEN_SOURCE)
    }
    #[doc = "Open-Drain Mode: When IOPOE bit 4 is 0: - If GPIODOUT bit 4 is 0: AUXIO\\[8i+4\\]
is driven low. - If GPIODOUT bit 4 is 1: AUXIO\\[8i+4\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 4 is 1: - If signal selected by IO4PSEL.SRC is 0: AUXIO\\[8i+4\\]
is driven low. - If signal selected by IO4PSEL.SRC is 1: AUXIO\\[8i+4\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(IO4_A::OPEN_DRAIN)
    }
    #[doc = "Input Mode: When GPIODIE bit 4 is 0: AUXIO\\[8i+4\\]
is enabled for analog signal transfer. When GPIODIE bit 4 is 1: AUXIO\\[8i+4\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO4_A::IN)
    }
    #[doc = "Output Mode: When IOPOE bit 4 is 0: GPIODOUT bit 4 drives AUXIO\\[8i+4\\]. When IOPOE bit 4 is 1: The signal selected by IO4PSEL.SRC drives AUXIO\\[8i+4\\]."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(IO4_A::OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "7:6\\]
Selects mode for AUXIO\\[8i+3\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IO3_A {
    #[doc = "3: Open-Source Mode: \n\nWhen IOPOE bit 3 is 0: \n- If GPIODOUT bit 3 is 0: AUXIO\\[8i+3\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n- If GPIODOUT bit 3 is 1: AUXIO\\[8i+3\\]
is driven high.\n\nWhen IOPOE bit 3 is 1: \n- If signal selected by IO3PSEL.SRC is 0: AUXIO\\[8i+3\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n- If signal selected by IO3PSEL.SRC is 1: AUXIO\\[8i+3\\]
is driven high."]
    OPEN_SOURCE = 3,
    #[doc = "2: Open-Drain Mode: \n\nWhen IOPOE bit 3 is 0: \n- If GPIODOUT bit 3 is 0: AUXIO\\[8i+3\\]
is driven low.  \n- If GPIODOUT bit 3 is 1: AUXIO\\[8i+3\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n\nWhen IOPOE bit 3 is 1: \n- If signal selected by IO3PSEL.SRC is 0: AUXIO\\[8i+3\\]
is driven low. \n- If signal selected by IO3PSEL.SRC is 1: AUXIO\\[8i+3\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OPEN_DRAIN = 2,
    #[doc = "1: Input Mode:\n\nWhen GPIODIE bit 3 is 0: AUXIO\\[8i+3\\]
is enabled for analog signal transfer.\n\nWhen GPIODIE bit 3 is 1: AUXIO\\[8i+3\\]
is enabled for digital input."]
    IN = 1,
    #[doc = "0: Output Mode:\n\nWhen IOPOE bit 3 is 0: GPIODOUT bit 3 drives AUXIO\\[8i+3\\].\n\nWhen IOPOE bit 3 is 1: The signal selected by IO3PSEL.SRC drives AUXIO\\[8i+3\\]."]
    OUT = 0,
}
impl From<IO3_A> for u8 {
    #[inline(always)]
    fn from(variant: IO3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IO3`"]
pub type IO3_R = crate::R<u8, IO3_A>;
impl IO3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO3_A {
        match self.bits {
            3 => IO3_A::OPEN_SOURCE,
            2 => IO3_A::OPEN_DRAIN,
            1 => IO3_A::IN,
            0 => IO3_A::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_SOURCE`"]
    #[inline(always)]
    pub fn is_open_source(&self) -> bool {
        *self == IO3_A::OPEN_SOURCE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == IO3_A::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in_(&self) -> bool {
        *self == IO3_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == IO3_A::OUT
    }
}
#[doc = "Write proxy for field `IO3`"]
pub struct IO3_W<'a> {
    w: &'a mut W,
}
impl<'a> IO3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO3_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Open-Source Mode: When IOPOE bit 3 is 0: - If GPIODOUT bit 3 is 0: AUXIO\\[8i+3\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 3 is 1: AUXIO\\[8i+3\\]
is driven high. When IOPOE bit 3 is 1: - If signal selected by IO3PSEL.SRC is 0: AUXIO\\[8i+3\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO3PSEL.SRC is 1: AUXIO\\[8i+3\\]
is driven high."]
    #[inline(always)]
    pub fn open_source(self) -> &'a mut W {
        self.variant(IO3_A::OPEN_SOURCE)
    }
    #[doc = "Open-Drain Mode: When IOPOE bit 3 is 0: - If GPIODOUT bit 3 is 0: AUXIO\\[8i+3\\]
is driven low. - If GPIODOUT bit 3 is 1: AUXIO\\[8i+3\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 3 is 1: - If signal selected by IO3PSEL.SRC is 0: AUXIO\\[8i+3\\]
is driven low. - If signal selected by IO3PSEL.SRC is 1: AUXIO\\[8i+3\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(IO3_A::OPEN_DRAIN)
    }
    #[doc = "Input Mode: When GPIODIE bit 3 is 0: AUXIO\\[8i+3\\]
is enabled for analog signal transfer. When GPIODIE bit 3 is 1: AUXIO\\[8i+3\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO3_A::IN)
    }
    #[doc = "Output Mode: When IOPOE bit 3 is 0: GPIODOUT bit 3 drives AUXIO\\[8i+3\\]. When IOPOE bit 3 is 1: The signal selected by IO3PSEL.SRC drives AUXIO\\[8i+3\\]."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(IO3_A::OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "5:4\\]
Select mode for AUXIO\\[8i+2\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IO2_A {
    #[doc = "3: Open-Source Mode: \n\nWhen IOPOE bit 2 is 0: \n- If GPIODOUT bit 2 is 0: AUXIO\\[8i+2\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n- If GPIODOUT bit 2 is 1: AUXIO\\[8i+2\\]
is driven high.\n\nWhen IOPOE bit 2 is 1: \n- If signal selected by IO2PSEL.SRC is 0: AUXIO\\[8i+2\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n- If signal selected by IO2PSEL.SRC is 1: AUXIO\\[8i+2\\]
is driven high."]
    OPEN_SOURCE = 3,
    #[doc = "2: Open-Drain Mode: \n\nWhen IOPOE bit 2 is 0: \n- If GPIODOUT bit 2 is 0: AUXIO\\[8i+2\\]
is driven low.  \n- If GPIODOUT bit 2 is 1: AUXIO\\[8i+2\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n\nWhen IOPOE bit 2 is 1: \n- If signal selected by IO2PSEL.SRC is 0: AUXIO\\[8i+2\\]
is driven low. \n- If signal selected by IO2PSEL.SRC is 1: AUXIO\\[8i+2\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OPEN_DRAIN = 2,
    #[doc = "1: Input Mode:\n\nWhen GPIODIE bit 2 is 0: AUXIO\\[8i+2\\]
is enabled for analog signal transfer.\n\nWhen GPIODIE bit 2 is 1: AUXIO\\[8i+2\\]
is enabled for digital input."]
    IN = 1,
    #[doc = "0: Output Mode:\n\nWhen IOPOE bit 2 is 0: GPIODOUT bit 2 drives AUXIO\\[8i+2\\].\n\nWhen IOPOE bit 2 is 1: The signal selected by IO2PSEL.SRC drives AUXIO\\[8i+2\\]."]
    OUT = 0,
}
impl From<IO2_A> for u8 {
    #[inline(always)]
    fn from(variant: IO2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IO2`"]
pub type IO2_R = crate::R<u8, IO2_A>;
impl IO2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO2_A {
        match self.bits {
            3 => IO2_A::OPEN_SOURCE,
            2 => IO2_A::OPEN_DRAIN,
            1 => IO2_A::IN,
            0 => IO2_A::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_SOURCE`"]
    #[inline(always)]
    pub fn is_open_source(&self) -> bool {
        *self == IO2_A::OPEN_SOURCE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == IO2_A::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in_(&self) -> bool {
        *self == IO2_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == IO2_A::OUT
    }
}
#[doc = "Write proxy for field `IO2`"]
pub struct IO2_W<'a> {
    w: &'a mut W,
}
impl<'a> IO2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Open-Source Mode: When IOPOE bit 2 is 0: - If GPIODOUT bit 2 is 0: AUXIO\\[8i+2\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 2 is 1: AUXIO\\[8i+2\\]
is driven high. When IOPOE bit 2 is 1: - If signal selected by IO2PSEL.SRC is 0: AUXIO\\[8i+2\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO2PSEL.SRC is 1: AUXIO\\[8i+2\\]
is driven high."]
    #[inline(always)]
    pub fn open_source(self) -> &'a mut W {
        self.variant(IO2_A::OPEN_SOURCE)
    }
    #[doc = "Open-Drain Mode: When IOPOE bit 2 is 0: - If GPIODOUT bit 2 is 0: AUXIO\\[8i+2\\]
is driven low. - If GPIODOUT bit 2 is 1: AUXIO\\[8i+2\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 2 is 1: - If signal selected by IO2PSEL.SRC is 0: AUXIO\\[8i+2\\]
is driven low. - If signal selected by IO2PSEL.SRC is 1: AUXIO\\[8i+2\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(IO2_A::OPEN_DRAIN)
    }
    #[doc = "Input Mode: When GPIODIE bit 2 is 0: AUXIO\\[8i+2\\]
is enabled for analog signal transfer. When GPIODIE bit 2 is 1: AUXIO\\[8i+2\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO2_A::IN)
    }
    #[doc = "Output Mode: When IOPOE bit 2 is 0: GPIODOUT bit 2 drives AUXIO\\[8i+2\\]. When IOPOE bit 2 is 1: The signal selected by IO2PSEL.SRC drives AUXIO\\[8i+2\\]."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(IO2_A::OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "3:2\\]
Select mode for AUXIO\\[8i+1\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IO1_A {
    #[doc = "3: Open-Source Mode: \n\nWhen IOPOE bit 1 is 0: \n- If GPIODOUT bit 1 is 0: AUXIO\\[8i+1\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n- If GPIODOUT bit 1 is 1: AUXIO\\[8i+1\\]
is driven high.\n\nWhen IOPOE bit 1 is 1: \n- If signal selected by IO1PSEL.SRC is 0: AUXIO\\[8i+1\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n- If signal selected by IO1PSEL.SRC is 1: AUXIO\\[8i+1\\]
is driven high."]
    OPEN_SOURCE = 3,
    #[doc = "2: Open-Drain Mode: \n\nWhen IOPOE bit 1 is 0: \n- If GPIODOUT bit 1 is 0: AUXIO\\[8i+1\\]
is driven low.  \n- If GPIODOUT bit 1 is 1: AUXIO\\[8i+1\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n\nWhen IOPOE bit 1 is 1: \n- If signal selected by IO1PSEL.SRC is 0: AUXIO\\[8i+1\\]
is driven low. \n- If signal selected by IO1PSEL.SRC is 1: AUXIO\\[8i+1\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OPEN_DRAIN = 2,
    #[doc = "1: Input Mode:\n\nWhen GPIODIE bit 1 is 0: AUXIO\\[8i+1\\]
is enabled for analog signal transfer.\n\nWhen GPIODIE bit 1 is 1: AUXIO\\[8i+1\\]
is enabled for digital input."]
    IN = 1,
    #[doc = "0: Output Mode:\n\nWhen IOPOE bit 1 is 0: GPIODOUT bit 1 drives AUXIO\\[8i+1\\].\n\nWhen IOPOE bit 1 is 1: The signal selected by IO1PSEL.SRC drives AUXIO\\[8i+1\\]."]
    OUT = 0,
}
impl From<IO1_A> for u8 {
    #[inline(always)]
    fn from(variant: IO1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IO1`"]
pub type IO1_R = crate::R<u8, IO1_A>;
impl IO1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO1_A {
        match self.bits {
            3 => IO1_A::OPEN_SOURCE,
            2 => IO1_A::OPEN_DRAIN,
            1 => IO1_A::IN,
            0 => IO1_A::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_SOURCE`"]
    #[inline(always)]
    pub fn is_open_source(&self) -> bool {
        *self == IO1_A::OPEN_SOURCE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == IO1_A::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in_(&self) -> bool {
        *self == IO1_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == IO1_A::OUT
    }
}
#[doc = "Write proxy for field `IO1`"]
pub struct IO1_W<'a> {
    w: &'a mut W,
}
impl<'a> IO1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Open-Source Mode: When IOPOE bit 1 is 0: - If GPIODOUT bit 1 is 0: AUXIO\\[8i+1\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 1 is 1: AUXIO\\[8i+1\\]
is driven high. When IOPOE bit 1 is 1: - If signal selected by IO1PSEL.SRC is 0: AUXIO\\[8i+1\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO1PSEL.SRC is 1: AUXIO\\[8i+1\\]
is driven high."]
    #[inline(always)]
    pub fn open_source(self) -> &'a mut W {
        self.variant(IO1_A::OPEN_SOURCE)
    }
    #[doc = "Open-Drain Mode: When IOPOE bit 1 is 0: - If GPIODOUT bit 1 is 0: AUXIO\\[8i+1\\]
is driven low. - If GPIODOUT bit 1 is 1: AUXIO\\[8i+1\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 1 is 1: - If signal selected by IO1PSEL.SRC is 0: AUXIO\\[8i+1\\]
is driven low. - If signal selected by IO1PSEL.SRC is 1: AUXIO\\[8i+1\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(IO1_A::OPEN_DRAIN)
    }
    #[doc = "Input Mode: When GPIODIE bit 1 is 0: AUXIO\\[8i+1\\]
is enabled for analog signal transfer. When GPIODIE bit 1 is 1: AUXIO\\[8i+1\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO1_A::IN)
    }
    #[doc = "Output Mode: When IOPOE bit 1 is 0: GPIODOUT bit 1 drives AUXIO\\[8i+1\\]. When IOPOE bit 1 is 1: The signal selected by IO1PSEL.SRC drives AUXIO\\[8i+1\\]."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(IO1_A::OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "1:0\\]
Select mode for AUXIO\\[8i+0\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IO0_A {
    #[doc = "3: Open-Source Mode: \n\nWhen IOPOE bit 0 is 0: \n- If GPIODOUT bit 0 is 0: AUXIO\\[8i+0\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n- If GPIODOUT bit 0 is 1: AUXIO\\[8i+0\\]
is driven high.\n\nWhen IOPOE bit 0 is 1: \n- If signal selected by IO0PSEL.SRC is 0: AUXIO\\[8i+0\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n- If signal selected by IO0PSEL.SRC is 1: AUXIO\\[8i+0\\]
is driven high."]
    OPEN_SOURCE = 3,
    #[doc = "2: Open-Drain Mode: \n\nWhen IOPOE bit 0 is 0: \n- If GPIODOUT bit 0 is 0: AUXIO\\[8i+0\\]
is driven low.  \n- If GPIODOUT bit 0 is 1: AUXIO\\[8i+0\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n\nWhen IOPOE bit 0 is 1: \n- If signal selected by IO0PSEL.SRC is 0: AUXIO\\[8i+0\\]
is driven low. \n- If signal selected by IO0PSEL.SRC is 1: AUXIO\\[8i+0\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OPEN_DRAIN = 2,
    #[doc = "1: Input Mode:\n\nWhen GPIODIE bit 0 is 0: AUXIO\\[8i+0\\]
is enabled for analog signal transfer.\n\nWhen GPIODIE bit 0 is 1: AUXIO\\[8i+0\\]
is enabled for digital input."]
    IN = 1,
    #[doc = "0: Output Mode:\n\nWhen IOPOE bit 0 is 0: GPIODOUT bit 0 drives AUXIO\\[8i+0\\].\n\nWhen IOPOE bit 0 is 1: The signal selected by IO0PSEL.SRC drives AUXIO\\[8i+0\\]."]
    OUT = 0,
}
impl From<IO0_A> for u8 {
    #[inline(always)]
    fn from(variant: IO0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IO0`"]
pub type IO0_R = crate::R<u8, IO0_A>;
impl IO0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO0_A {
        match self.bits {
            3 => IO0_A::OPEN_SOURCE,
            2 => IO0_A::OPEN_DRAIN,
            1 => IO0_A::IN,
            0 => IO0_A::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_SOURCE`"]
    #[inline(always)]
    pub fn is_open_source(&self) -> bool {
        *self == IO0_A::OPEN_SOURCE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == IO0_A::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in_(&self) -> bool {
        *self == IO0_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == IO0_A::OUT
    }
}
#[doc = "Write proxy for field `IO0`"]
pub struct IO0_W<'a> {
    w: &'a mut W,
}
impl<'a> IO0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Open-Source Mode: When IOPOE bit 0 is 0: - If GPIODOUT bit 0 is 0: AUXIO\\[8i+0\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If GPIODOUT bit 0 is 1: AUXIO\\[8i+0\\]
is driven high. When IOPOE bit 0 is 1: - If signal selected by IO0PSEL.SRC is 0: AUXIO\\[8i+0\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. - If signal selected by IO0PSEL.SRC is 1: AUXIO\\[8i+0\\]
is driven high."]
    #[inline(always)]
    pub fn open_source(self) -> &'a mut W {
        self.variant(IO0_A::OPEN_SOURCE)
    }
    #[doc = "Open-Drain Mode: When IOPOE bit 0 is 0: - If GPIODOUT bit 0 is 0: AUXIO\\[8i+0\\]
is driven low. - If GPIODOUT bit 0 is 1: AUXIO\\[8i+0\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When IOPOE bit 0 is 1: - If signal selected by IO0PSEL.SRC is 0: AUXIO\\[8i+0\\]
is driven low. - If signal selected by IO0PSEL.SRC is 1: AUXIO\\[8i+0\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(IO0_A::OPEN_DRAIN)
    }
    #[doc = "Input Mode: When GPIODIE bit 0 is 0: AUXIO\\[8i+0\\]
is enabled for analog signal transfer. When GPIODIE bit 0 is 1: AUXIO\\[8i+0\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO0_A::IN)
    }
    #[doc = "Output Mode: When IOPOE bit 0 is 0: GPIODOUT bit 0 drives AUXIO\\[8i+0\\]. When IOPOE bit 0 is 1: The signal selected by IO0PSEL.SRC drives AUXIO\\[8i+0\\]."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(IO0_A::OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Selects mode for AUXIO\\[8i+7\\]."]
    #[inline(always)]
    pub fn io7(&self) -> IO7_R {
        IO7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Selects mode for AUXIO\\[8i+6\\]."]
    #[inline(always)]
    pub fn io6(&self) -> IO6_R {
        IO6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Selects mode for AUXIO\\[8i+5\\]."]
    #[inline(always)]
    pub fn io5(&self) -> IO5_R {
        IO5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Selects mode for AUXIO\\[8i+4\\]."]
    #[inline(always)]
    pub fn io4(&self) -> IO4_R {
        IO4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Selects mode for AUXIO\\[8i+3\\]."]
    #[inline(always)]
    pub fn io3(&self) -> IO3_R {
        IO3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Select mode for AUXIO\\[8i+2\\]."]
    #[inline(always)]
    pub fn io2(&self) -> IO2_R {
        IO2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Select mode for AUXIO\\[8i+1\\]."]
    #[inline(always)]
    pub fn io1(&self) -> IO1_R {
        IO1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - 1:0\\]
Select mode for AUXIO\\[8i+0\\]."]
    #[inline(always)]
    pub fn io0(&self) -> IO0_R {
        IO0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bits 14:15 - 15:14\\]
Selects mode for AUXIO\\[8i+7\\]."]
    #[inline(always)]
    pub fn io7(&mut self) -> IO7_W {
        IO7_W { w: self }
    }
    #[doc = "Bits 12:13 - 13:12\\]
Selects mode for AUXIO\\[8i+6\\]."]
    #[inline(always)]
    pub fn io6(&mut self) -> IO6_W {
        IO6_W { w: self }
    }
    #[doc = "Bits 10:11 - 11:10\\]
Selects mode for AUXIO\\[8i+5\\]."]
    #[inline(always)]
    pub fn io5(&mut self) -> IO5_W {
        IO5_W { w: self }
    }
    #[doc = "Bits 8:9 - 9:8\\]
Selects mode for AUXIO\\[8i+4\\]."]
    #[inline(always)]
    pub fn io4(&mut self) -> IO4_W {
        IO4_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\]
Selects mode for AUXIO\\[8i+3\\]."]
    #[inline(always)]
    pub fn io3(&mut self) -> IO3_W {
        IO3_W { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\]
Select mode for AUXIO\\[8i+2\\]."]
    #[inline(always)]
    pub fn io2(&mut self) -> IO2_W {
        IO2_W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\]
Select mode for AUXIO\\[8i+1\\]."]
    #[inline(always)]
    pub fn io1(&mut self) -> IO1_W {
        IO1_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
Select mode for AUXIO\\[8i+0\\]."]
    #[inline(always)]
    pub fn io0(&mut self) -> IO0_W {
        IO0_W { w: self }
    }
}
