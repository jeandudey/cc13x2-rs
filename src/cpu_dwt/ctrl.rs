#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0x4000_0000"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4000_0000
    }
}
#[doc = "Reader of field `RESERVED26`"]
pub type RESERVED26_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED26`"]
pub struct RESERVED26_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED26_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | (((value as u32) & 0x3f) << 26);
        self.w
    }
}
#[doc = "Reader of field `NOCYCCNT`"]
pub type NOCYCCNT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOCYCCNT`"]
pub struct NOCYCCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> NOCYCCNT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `NOPRFCNT`"]
pub type NOPRFCNT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOPRFCNT`"]
pub struct NOPRFCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> NOPRFCNT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `RESERVED23`"]
pub type RESERVED23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED23`"]
pub struct RESERVED23_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `CYCEVTENA`"]
pub type CYCEVTENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CYCEVTENA`"]
pub struct CYCEVTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CYCEVTENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `FOLDEVTENA`"]
pub type FOLDEVTENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FOLDEVTENA`"]
pub struct FOLDEVTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> FOLDEVTENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `LSUEVTENA`"]
pub type LSUEVTENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSUEVTENA`"]
pub struct LSUEVTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> LSUEVTENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `SLEEPEVTENA`"]
pub type SLEEPEVTENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLEEPEVTENA`"]
pub struct SLEEPEVTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEPEVTENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `EXCEVTENA`"]
pub type EXCEVTENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXCEVTENA`"]
pub struct EXCEVTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> EXCEVTENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `CPIEVTENA`"]
pub type CPIEVTENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPIEVTENA`"]
pub struct CPIEVTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CPIEVTENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `EXCTRCENA`"]
pub type EXCTRCENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXCTRCENA`"]
pub struct EXCTRCENA_W<'a> {
    w: &'a mut W,
}
impl<'a> EXCTRCENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `RESERVED13`"]
pub type RESERVED13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED13`"]
pub struct RESERVED13_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
#[doc = "Reader of field `PCSAMPLEENA`"]
pub type PCSAMPLEENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCSAMPLEENA`"]
pub struct PCSAMPLEENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSAMPLEENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "11:10\\]
Selects a synchronization packet rate. CYCCNTENA and CPU_ITM:TCR.SYNCENA must also be enabled for this feature. Synchronization packets (if enabled) are generated on tap transitions (0 to1 or 1 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYNCTAP_A {
    #[doc = "3: Tap at bit 28 of CYCCNT"]
    BIT28 = 3,
    #[doc = "2: Tap at bit 26 of CYCCNT"]
    BIT26 = 2,
    #[doc = "1: Tap at bit 24 of CYCCNT"]
    BIT24 = 1,
    #[doc = "0: Disabled. No synchronization packets"]
    DIS = 0,
}
impl From<SYNCTAP_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCTAP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYNCTAP`"]
pub type SYNCTAP_R = crate::R<u8, SYNCTAP_A>;
impl SYNCTAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCTAP_A {
        match self.bits {
            3 => SYNCTAP_A::BIT28,
            2 => SYNCTAP_A::BIT26,
            1 => SYNCTAP_A::BIT24,
            0 => SYNCTAP_A::DIS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BIT28`"]
    #[inline(always)]
    pub fn is_bit28(&self) -> bool {
        *self == SYNCTAP_A::BIT28
    }
    #[doc = "Checks if the value of the field is `BIT26`"]
    #[inline(always)]
    pub fn is_bit26(&self) -> bool {
        *self == SYNCTAP_A::BIT26
    }
    #[doc = "Checks if the value of the field is `BIT24`"]
    #[inline(always)]
    pub fn is_bit24(&self) -> bool {
        *self == SYNCTAP_A::BIT24
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SYNCTAP_A::DIS
    }
}
#[doc = "Write proxy for field `SYNCTAP`"]
pub struct SYNCTAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCTAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCTAP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Tap at bit 28 of CYCCNT"]
    #[inline(always)]
    pub fn bit28(self) -> &'a mut W {
        self.variant(SYNCTAP_A::BIT28)
    }
    #[doc = "Tap at bit 26 of CYCCNT"]
    #[inline(always)]
    pub fn bit26(self) -> &'a mut W {
        self.variant(SYNCTAP_A::BIT26)
    }
    #[doc = "Tap at bit 24 of CYCCNT"]
    #[inline(always)]
    pub fn bit24(self) -> &'a mut W {
        self.variant(SYNCTAP_A::BIT24)
    }
    #[doc = "Disabled. No synchronization packets"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SYNCTAP_A::DIS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "9:9\\]
Selects a tap on CYCCNT. These are spaced at bits \\[6\\]
and \\[10\\]. When the selected bit in CYCCNT changes from 0 to 1 or 1 to 0, it emits into the POSTCNT, post-scalar counter. That counter then counts down. On a bit change when post-scalar is 0, it triggers an event for PC sampling or cycle count event (see details in CYCEVTENA).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CYCTAP_A {
    #[doc = "1: Selects bit \\[10\\]
to tap"]
    BIT10 = 1,
    #[doc = "0: Selects bit \\[6\\]
to tap"]
    BIT6 = 0,
}
impl From<CYCTAP_A> for bool {
    #[inline(always)]
    fn from(variant: CYCTAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CYCTAP`"]
pub type CYCTAP_R = crate::R<bool, CYCTAP_A>;
impl CYCTAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CYCTAP_A {
        match self.bits {
            true => CYCTAP_A::BIT10,
            false => CYCTAP_A::BIT6,
        }
    }
    #[doc = "Checks if the value of the field is `BIT10`"]
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        *self == CYCTAP_A::BIT10
    }
    #[doc = "Checks if the value of the field is `BIT6`"]
    #[inline(always)]
    pub fn is_bit6(&self) -> bool {
        *self == CYCTAP_A::BIT6
    }
}
#[doc = "Write proxy for field `CYCTAP`"]
pub struct CYCTAP_W<'a> {
    w: &'a mut W,
}
impl<'a> CYCTAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CYCTAP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selects bit \\[10\\]
to tap"]
    #[inline(always)]
    pub fn bit10(self) -> &'a mut W {
        self.variant(CYCTAP_A::BIT10)
    }
    #[doc = "Selects bit \\[6\\]
to tap"]
    #[inline(always)]
    pub fn bit6(self) -> &'a mut W {
        self.variant(CYCTAP_A::BIT6)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `POSTCNT`"]
pub type POSTCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `POSTCNT`"]
pub struct POSTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> POSTCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | (((value as u32) & 0x0f) << 5);
        self.w
    }
}
#[doc = "Reader of field `POSTPRESET`"]
pub type POSTPRESET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `POSTPRESET`"]
pub struct POSTPRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> POSTPRESET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | (((value as u32) & 0x0f) << 1);
        self.w
    }
}
#[doc = "Reader of field `CYCCNTENA`"]
pub type CYCCNTENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CYCCNTENA`"]
pub struct CYCCNTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CYCCNTENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 26:31 - 31:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved26(&self) -> RESERVED26_R {
        RESERVED26_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
    #[doc = "Bit 25 - 25:25\\]
When set, CYCCNT is not supported."]
    #[inline(always)]
    pub fn nocyccnt(&self) -> NOCYCCNT_R {
        NOCYCCNT_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
When set, FOLDCNT, LSUCNT, SLEEPCNT, EXCCNT, and CPICNT are not supported."]
    #[inline(always)]
    pub fn noprfcnt(&self) -> NOPRFCNT_R {
        NOPRFCNT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved23(&self) -> RESERVED23_R {
        RESERVED23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Enables Cycle count event. Emits an event when the POSTCNT counter triggers it. See CYCTAP and POSTPRESET for details. This event is only emitted if PCSAMPLEENA is disabled. PCSAMPLEENA overrides the setting of this bit. 0: Cycle count events disabled 1: Cycle count events enabled"]
    #[inline(always)]
    pub fn cycevtena(&self) -> CYCEVTENA_R {
        CYCEVTENA_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Enables Folded instruction count event. Emits an event when FOLDCNT overflows (every 256 cycles of folded instructions). A folded instruction is one that does not incur even one cycle to execute. For example, an IT instruction is folded away and so does not use up one cycle. 0: Folded instruction count events disabled. 1: Folded instruction count events enabled."]
    #[inline(always)]
    pub fn foldevtena(&self) -> FOLDEVTENA_R {
        FOLDEVTENA_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Enables LSU count event. Emits an event when LSUCNT overflows (every 256 cycles of LSU operation). LSU counts include all LSU costs after the initial cycle for the instruction. 0: LSU count events disabled. 1: LSU count events enabled."]
    #[inline(always)]
    pub fn lsuevtena(&self) -> LSUEVTENA_R {
        LSUEVTENA_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Enables Sleep count event. Emits an event when SLEEPCNT overflows (every 256 cycles that the processor is sleeping). 0: Sleep count events disabled. 1: Sleep count events enabled."]
    #[inline(always)]
    pub fn sleepevtena(&self) -> SLEEPEVTENA_R {
        SLEEPEVTENA_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Enables Interrupt overhead event. Emits an event when EXCCNT overflows (every 256 cycles of interrupt overhead). 0x0: Interrupt overhead event disabled. 0x1: Interrupt overhead event enabled."]
    #[inline(always)]
    pub fn excevtena(&self) -> EXCEVTENA_R {
        EXCEVTENA_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Enables CPI count event. Emits an event when CPICNT overflows (every 256 cycles of multi-cycle instructions). 0: CPI counter events disabled. 1: CPI counter events enabled."]
    #[inline(always)]
    pub fn cpievtena(&self) -> CPIEVTENA_R {
        CPIEVTENA_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Enables Interrupt event tracing. 0: Interrupt event trace disabled. 1: Interrupt event trace enabled."]
    #[inline(always)]
    pub fn exctrcena(&self) -> EXCTRCENA_R {
        EXCTRCENA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&self) -> RESERVED13_R {
        RESERVED13_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
Enables PC Sampling event. A PC sample event is emitted when the POSTCNT counter triggers it. See CYCTAP and POSTPRESET for details. Enabling this bit overrides CYCEVTENA. 0: PC Sampling event disabled. 1: Sampling event enabled."]
    #[inline(always)]
    pub fn pcsampleena(&self) -> PCSAMPLEENA_R {
        PCSAMPLEENA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Selects a synchronization packet rate. CYCCNTENA and CPU_ITM:TCR.SYNCENA must also be enabled for this feature. Synchronization packets (if enabled) are generated on tap transitions (0 to1 or 1 to 0)."]
    #[inline(always)]
    pub fn synctap(&self) -> SYNCTAP_R {
        SYNCTAP_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 9 - 9:9\\]
Selects a tap on CYCCNT. These are spaced at bits \\[6\\]
and \\[10\\]. When the selected bit in CYCCNT changes from 0 to 1 or 1 to 0, it emits into the POSTCNT, post-scalar counter. That counter then counts down. On a bit change when post-scalar is 0, it triggers an event for PC sampling or cycle count event (see details in CYCEVTENA)."]
    #[inline(always)]
    pub fn cyctap(&self) -> CYCTAP_R {
        CYCTAP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 5:8 - 8:5\\]
Post-scalar counter for CYCTAP. When the selected tapped bit changes from 0 to 1 or 1 to 0, the post scalar counter is down-counted when not 0. If 0, it triggers an event for PCSAMPLEENA or CYCEVTENA use. It also reloads with the value from POSTPRESET."]
    #[inline(always)]
    pub fn postcnt(&self) -> POSTCNT_R {
        POSTCNT_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 1:4 - 4:1\\]
Reload value for post-scalar counter POSTCNT. When 0, events are triggered on each tap change (a power of 2). If this field has a non-0 value, it forms a count-down value, to be reloaded into POSTCNT each time it reaches 0. For example, a value 1 in this register means an event is formed every other tap change."]
    #[inline(always)]
    pub fn postpreset(&self) -> POSTPRESET_R {
        POSTPRESET_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 0 - 0:0\\]
Enable CYCCNT, allowing it to increment and generate synchronization and count events. If NOCYCCNT = 1, this bit reads zero and ignore writes."]
    #[inline(always)]
    pub fn cyccntena(&self) -> CYCCNTENA_R {
        CYCCNTENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 26:31 - 31:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved26(&mut self) -> RESERVED26_W {
        RESERVED26_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
When set, CYCCNT is not supported."]
    #[inline(always)]
    pub fn nocyccnt(&mut self) -> NOCYCCNT_W {
        NOCYCCNT_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
When set, FOLDCNT, LSUCNT, SLEEPCNT, EXCCNT, and CPICNT are not supported."]
    #[inline(always)]
    pub fn noprfcnt(&mut self) -> NOPRFCNT_W {
        NOPRFCNT_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved23(&mut self) -> RESERVED23_W {
        RESERVED23_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\]
Enables Cycle count event. Emits an event when the POSTCNT counter triggers it. See CYCTAP and POSTPRESET for details. This event is only emitted if PCSAMPLEENA is disabled. PCSAMPLEENA overrides the setting of this bit. 0: Cycle count events disabled 1: Cycle count events enabled"]
    #[inline(always)]
    pub fn cycevtena(&mut self) -> CYCEVTENA_W {
        CYCEVTENA_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\]
Enables Folded instruction count event. Emits an event when FOLDCNT overflows (every 256 cycles of folded instructions). A folded instruction is one that does not incur even one cycle to execute. For example, an IT instruction is folded away and so does not use up one cycle. 0: Folded instruction count events disabled. 1: Folded instruction count events enabled."]
    #[inline(always)]
    pub fn foldevtena(&mut self) -> FOLDEVTENA_W {
        FOLDEVTENA_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\]
Enables LSU count event. Emits an event when LSUCNT overflows (every 256 cycles of LSU operation). LSU counts include all LSU costs after the initial cycle for the instruction. 0: LSU count events disabled. 1: LSU count events enabled."]
    #[inline(always)]
    pub fn lsuevtena(&mut self) -> LSUEVTENA_W {
        LSUEVTENA_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\]
Enables Sleep count event. Emits an event when SLEEPCNT overflows (every 256 cycles that the processor is sleeping). 0: Sleep count events disabled. 1: Sleep count events enabled."]
    #[inline(always)]
    pub fn sleepevtena(&mut self) -> SLEEPEVTENA_W {
        SLEEPEVTENA_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
Enables Interrupt overhead event. Emits an event when EXCCNT overflows (every 256 cycles of interrupt overhead). 0x0: Interrupt overhead event disabled. 0x1: Interrupt overhead event enabled."]
    #[inline(always)]
    pub fn excevtena(&mut self) -> EXCEVTENA_W {
        EXCEVTENA_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Enables CPI count event. Emits an event when CPICNT overflows (every 256 cycles of multi-cycle instructions). 0: CPI counter events disabled. 1: CPI counter events enabled."]
    #[inline(always)]
    pub fn cpievtena(&mut self) -> CPIEVTENA_W {
        CPIEVTENA_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Enables Interrupt event tracing. 0: Interrupt event trace disabled. 1: Interrupt event trace enabled."]
    #[inline(always)]
    pub fn exctrcena(&mut self) -> EXCTRCENA_W {
        EXCTRCENA_W { w: self }
    }
    #[doc = "Bits 13:15 - 15:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&mut self) -> RESERVED13_W {
        RESERVED13_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Enables PC Sampling event. A PC sample event is emitted when the POSTCNT counter triggers it. See CYCTAP and POSTPRESET for details. Enabling this bit overrides CYCEVTENA. 0: PC Sampling event disabled. 1: Sampling event enabled."]
    #[inline(always)]
    pub fn pcsampleena(&mut self) -> PCSAMPLEENA_W {
        PCSAMPLEENA_W { w: self }
    }
    #[doc = "Bits 10:11 - 11:10\\]
Selects a synchronization packet rate. CYCCNTENA and CPU_ITM:TCR.SYNCENA must also be enabled for this feature. Synchronization packets (if enabled) are generated on tap transitions (0 to1 or 1 to 0)."]
    #[inline(always)]
    pub fn synctap(&mut self) -> SYNCTAP_W {
        SYNCTAP_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Selects a tap on CYCCNT. These are spaced at bits \\[6\\]
and \\[10\\]. When the selected bit in CYCCNT changes from 0 to 1 or 1 to 0, it emits into the POSTCNT, post-scalar counter. That counter then counts down. On a bit change when post-scalar is 0, it triggers an event for PC sampling or cycle count event (see details in CYCEVTENA)."]
    #[inline(always)]
    pub fn cyctap(&mut self) -> CYCTAP_W {
        CYCTAP_W { w: self }
    }
    #[doc = "Bits 5:8 - 8:5\\]
Post-scalar counter for CYCTAP. When the selected tapped bit changes from 0 to 1 or 1 to 0, the post scalar counter is down-counted when not 0. If 0, it triggers an event for PCSAMPLEENA or CYCEVTENA use. It also reloads with the value from POSTPRESET."]
    #[inline(always)]
    pub fn postcnt(&mut self) -> POSTCNT_W {
        POSTCNT_W { w: self }
    }
    #[doc = "Bits 1:4 - 4:1\\]
Reload value for post-scalar counter POSTCNT. When 0, events are triggered on each tap change (a power of 2). If this field has a non-0 value, it forms a count-down value, to be reloaded into POSTCNT each time it reaches 0. For example, a value 1 in this register means an event is formed every other tap change."]
    #[inline(always)]
    pub fn postpreset(&mut self) -> POSTPRESET_W {
        POSTPRESET_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Enable CYCCNT, allowing it to increment and generate synchronization and count events. If NOCYCCNT = 1, this bit reads zero and ignore writes."]
    #[inline(always)]
    pub fn cyccntena(&mut self) -> CYCCNTENA_W {
        CYCCNTENA_W { w: self }
    }
}
