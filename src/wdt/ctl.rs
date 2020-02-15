#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED3`"]
pub type RESERVED3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED3`"]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | (((value as u32) & 0x1fff_ffff) << 3);
        self.w
    }
}
#[doc = "2:2\\]
WDT Interrupt Type 0: WDT interrupt is a standard interrupt. 1: WDT interrupt is a non-maskable interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTTYPE_A {
    #[doc = "1: Non-maskable interrupt"]
    NONMASKABLE = 1,
    #[doc = "0: Maskable interrupt"]
    MASKABLE = 0,
}
impl From<INTTYPE_A> for bool {
    #[inline(always)]
    fn from(variant: INTTYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INTTYPE`"]
pub type INTTYPE_R = crate::R<bool, INTTYPE_A>;
impl INTTYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTTYPE_A {
        match self.bits {
            true => INTTYPE_A::NONMASKABLE,
            false => INTTYPE_A::MASKABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NONMASKABLE`"]
    #[inline(always)]
    pub fn is_nonmaskable(&self) -> bool {
        *self == INTTYPE_A::NONMASKABLE
    }
    #[doc = "Checks if the value of the field is `MASKABLE`"]
    #[inline(always)]
    pub fn is_maskable(&self) -> bool {
        *self == INTTYPE_A::MASKABLE
    }
}
#[doc = "Write proxy for field `INTTYPE`"]
pub struct INTTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> INTTYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTTYPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Non-maskable interrupt"]
    #[inline(always)]
    pub fn nonmaskable(self) -> &'a mut W {
        self.variant(INTTYPE_A::NONMASKABLE)
    }
    #[doc = "Maskable interrupt"]
    #[inline(always)]
    pub fn maskable(self) -> &'a mut W {
        self.variant(INTTYPE_A::MASKABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "1:1\\]
WDT Reset Enable. Defines the function of the WDT reset source (see PRCM:WARMRESET.WDT_STAT if enabled) 0: Disabled. 1: Enable the Watchdog reset output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESEN_A {
    #[doc = "1: Reset output Enabled"]
    EN = 1,
    #[doc = "0: Reset output Disabled"]
    DIS = 0,
}
impl From<RESEN_A> for bool {
    #[inline(always)]
    fn from(variant: RESEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RESEN`"]
pub type RESEN_R = crate::R<bool, RESEN_A>;
impl RESEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESEN_A {
        match self.bits {
            true => RESEN_A::EN,
            false => RESEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RESEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RESEN_A::DIS
    }
}
#[doc = "Write proxy for field `RESEN`"]
pub struct RESEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RESEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset output Enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RESEN_A::EN)
    }
    #[doc = "Reset output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RESEN_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "0:0\\]
WDT Interrupt Enable 0: Interrupt event disabled. 1: Interrupt event enabled. Once set, this bit can only be cleared by a hardware reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTEN_A {
    #[doc = "1: Interrupt Enabled"]
    EN = 1,
    #[doc = "0: Interrupt Disabled"]
    DIS = 0,
}
impl From<INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INTEN`"]
pub type INTEN_R = crate::R<bool, INTEN_A>;
impl INTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN_A {
        match self.bits {
            true => INTEN_A::EN,
            false => INTEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == INTEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == INTEN_A::DIS
    }
}
#[doc = "Write proxy for field `INTEN`"]
pub struct INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(INTEN_A::EN)
    }
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(INTEN_A::DIS)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 2 - 2:2\\]
WDT Interrupt Type 0: WDT interrupt is a standard interrupt. 1: WDT interrupt is a non-maskable interrupt."]
    #[inline(always)]
    pub fn inttype(&self) -> INTTYPE_R {
        INTTYPE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
WDT Reset Enable. Defines the function of the WDT reset source (see PRCM:WARMRESET.WDT_STAT if enabled) 0: Disabled. 1: Enable the Watchdog reset output."]
    #[inline(always)]
    pub fn resen(&self) -> RESEN_R {
        RESEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
WDT Interrupt Enable 0: Interrupt event disabled. 1: Interrupt event enabled. Once set, this bit can only be cleared by a hardware reset."]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
WDT Interrupt Type 0: WDT interrupt is a standard interrupt. 1: WDT interrupt is a non-maskable interrupt."]
    #[inline(always)]
    pub fn inttype(&mut self) -> INTTYPE_W {
        INTTYPE_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
WDT Reset Enable. Defines the function of the WDT reset source (see PRCM:WARMRESET.WDT_STAT if enabled) 0: Disabled. 1: Enable the Watchdog reset output."]
    #[inline(always)]
    pub fn resen(&mut self) -> RESEN_W {
        RESEN_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
WDT Interrupt Enable 0: Interrupt event disabled. 1: Interrupt event enabled. Once set, this bit can only be cleared by a hardware reset."]
    #[inline(always)]
    pub fn inten(&mut self) -> INTEN_W {
        INTEN_W { w: self }
    }
}
