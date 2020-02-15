#[doc = "Reader of register AUXSEL0"]
pub type R = crate::R<u32, super::AUXSEL0>;
#[doc = "Writer for register AUXSEL0"]
pub type W = crate::W<u32, super::AUXSEL0>;
#[doc = "Register AUXSEL0 `reset()`'s with value 0x10"]
impl crate::ResetValue for super::AUXSEL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x10
    }
}
#[doc = "6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior.\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EV_A {
    #[doc = "121: Always asserted"]
    ALWAYS_ACTIVE = 121,
    #[doc = "68: GPT3B compare event. Configured by GPT3:TBMR.TCACT"]
    GPT3B_CMP = 68,
    #[doc = "67: GPT3A compare event. Configured by GPT3:TAMR.TCACT"]
    GPT3A_CMP = 67,
    #[doc = "66: GPT2B compare event. Configured by GPT2:TBMR.TCACT"]
    GPT2B_CMP = 66,
    #[doc = "65: GPT2A compare event. Configured by GPT2:TAMR.TCACT"]
    GPT2A_CMP = 65,
    #[doc = "64: GPT1B compare event. Configured by GPT1:TBMR.TCACT"]
    GPT1B_CMP = 64,
    #[doc = "63: GPT1A compare event. Configured by GPT1:TAMR.TCACT"]
    GPT1A_CMP = 63,
    #[doc = "62: GPT0B compare event. Configured by GPT0:TBMR.TCACT"]
    GPT0B_CMP = 62,
    #[doc = "61: GPT0A compare event. Configured by GPT0:TAMR.TCACT"]
    GPT0A_CMP = 61,
    #[doc = "19: GPT1B interrupt event, controlled by GPT1:TBMR"]
    GPT1B = 19,
    #[doc = "18: GPT1A interrupt event, controlled by GPT1:TAMR"]
    GPT1A = 18,
    #[doc = "17: GPT0B interrupt event, controlled by GPT0:TBMR"]
    GPT0B = 17,
    #[doc = "16: GPT0A interrupt event, controlled by GPT0:TAMR"]
    GPT0A = 16,
    #[doc = "15: GPT3B interrupt event, controlled by GPT3:TBMR"]
    GPT3B = 15,
    #[doc = "14: GPT3A interrupt event, controlled by GPT3:TAMR"]
    GPT3A = 14,
    #[doc = "13: GPT2B interrupt event, controlled by GPT2:TBMR"]
    GPT2B = 13,
    #[doc = "12: GPT2A interrupt event, controlled by GPT2:TAMR"]
    GPT2A = 12,
    #[doc = "0: Always inactive"]
    NONE = 0,
}
impl From<EV_A> for u8 {
    #[inline(always)]
    fn from(variant: EV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EV`"]
pub type EV_R = crate::R<u8, EV_A>;
impl EV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EV_A> {
        use crate::Variant::*;
        match self.bits {
            121 => Val(EV_A::ALWAYS_ACTIVE),
            68 => Val(EV_A::GPT3B_CMP),
            67 => Val(EV_A::GPT3A_CMP),
            66 => Val(EV_A::GPT2B_CMP),
            65 => Val(EV_A::GPT2A_CMP),
            64 => Val(EV_A::GPT1B_CMP),
            63 => Val(EV_A::GPT1A_CMP),
            62 => Val(EV_A::GPT0B_CMP),
            61 => Val(EV_A::GPT0A_CMP),
            19 => Val(EV_A::GPT1B),
            18 => Val(EV_A::GPT1A),
            17 => Val(EV_A::GPT0B),
            16 => Val(EV_A::GPT0A),
            15 => Val(EV_A::GPT3B),
            14 => Val(EV_A::GPT3A),
            13 => Val(EV_A::GPT2B),
            12 => Val(EV_A::GPT2A),
            0 => Val(EV_A::NONE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALWAYS_ACTIVE`"]
    #[inline(always)]
    pub fn is_always_active(&self) -> bool {
        *self == EV_A::ALWAYS_ACTIVE
    }
    #[doc = "Checks if the value of the field is `GPT3B_CMP`"]
    #[inline(always)]
    pub fn is_gpt3b_cmp(&self) -> bool {
        *self == EV_A::GPT3B_CMP
    }
    #[doc = "Checks if the value of the field is `GPT3A_CMP`"]
    #[inline(always)]
    pub fn is_gpt3a_cmp(&self) -> bool {
        *self == EV_A::GPT3A_CMP
    }
    #[doc = "Checks if the value of the field is `GPT2B_CMP`"]
    #[inline(always)]
    pub fn is_gpt2b_cmp(&self) -> bool {
        *self == EV_A::GPT2B_CMP
    }
    #[doc = "Checks if the value of the field is `GPT2A_CMP`"]
    #[inline(always)]
    pub fn is_gpt2a_cmp(&self) -> bool {
        *self == EV_A::GPT2A_CMP
    }
    #[doc = "Checks if the value of the field is `GPT1B_CMP`"]
    #[inline(always)]
    pub fn is_gpt1b_cmp(&self) -> bool {
        *self == EV_A::GPT1B_CMP
    }
    #[doc = "Checks if the value of the field is `GPT1A_CMP`"]
    #[inline(always)]
    pub fn is_gpt1a_cmp(&self) -> bool {
        *self == EV_A::GPT1A_CMP
    }
    #[doc = "Checks if the value of the field is `GPT0B_CMP`"]
    #[inline(always)]
    pub fn is_gpt0b_cmp(&self) -> bool {
        *self == EV_A::GPT0B_CMP
    }
    #[doc = "Checks if the value of the field is `GPT0A_CMP`"]
    #[inline(always)]
    pub fn is_gpt0a_cmp(&self) -> bool {
        *self == EV_A::GPT0A_CMP
    }
    #[doc = "Checks if the value of the field is `GPT1B`"]
    #[inline(always)]
    pub fn is_gpt1b(&self) -> bool {
        *self == EV_A::GPT1B
    }
    #[doc = "Checks if the value of the field is `GPT1A`"]
    #[inline(always)]
    pub fn is_gpt1a(&self) -> bool {
        *self == EV_A::GPT1A
    }
    #[doc = "Checks if the value of the field is `GPT0B`"]
    #[inline(always)]
    pub fn is_gpt0b(&self) -> bool {
        *self == EV_A::GPT0B
    }
    #[doc = "Checks if the value of the field is `GPT0A`"]
    #[inline(always)]
    pub fn is_gpt0a(&self) -> bool {
        *self == EV_A::GPT0A
    }
    #[doc = "Checks if the value of the field is `GPT3B`"]
    #[inline(always)]
    pub fn is_gpt3b(&self) -> bool {
        *self == EV_A::GPT3B
    }
    #[doc = "Checks if the value of the field is `GPT3A`"]
    #[inline(always)]
    pub fn is_gpt3a(&self) -> bool {
        *self == EV_A::GPT3A
    }
    #[doc = "Checks if the value of the field is `GPT2B`"]
    #[inline(always)]
    pub fn is_gpt2b(&self) -> bool {
        *self == EV_A::GPT2B
    }
    #[doc = "Checks if the value of the field is `GPT2A`"]
    #[inline(always)]
    pub fn is_gpt2a(&self) -> bool {
        *self == EV_A::GPT2A
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == EV_A::NONE
    }
}
#[doc = "Write proxy for field `EV`"]
pub struct EV_W<'a> {
    w: &'a mut W,
}
impl<'a> EV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Always asserted"]
    #[inline(always)]
    pub fn always_active(self) -> &'a mut W {
        self.variant(EV_A::ALWAYS_ACTIVE)
    }
    #[doc = "GPT3B compare event. Configured by GPT3:TBMR.TCACT"]
    #[inline(always)]
    pub fn gpt3b_cmp(self) -> &'a mut W {
        self.variant(EV_A::GPT3B_CMP)
    }
    #[doc = "GPT3A compare event. Configured by GPT3:TAMR.TCACT"]
    #[inline(always)]
    pub fn gpt3a_cmp(self) -> &'a mut W {
        self.variant(EV_A::GPT3A_CMP)
    }
    #[doc = "GPT2B compare event. Configured by GPT2:TBMR.TCACT"]
    #[inline(always)]
    pub fn gpt2b_cmp(self) -> &'a mut W {
        self.variant(EV_A::GPT2B_CMP)
    }
    #[doc = "GPT2A compare event. Configured by GPT2:TAMR.TCACT"]
    #[inline(always)]
    pub fn gpt2a_cmp(self) -> &'a mut W {
        self.variant(EV_A::GPT2A_CMP)
    }
    #[doc = "GPT1B compare event. Configured by GPT1:TBMR.TCACT"]
    #[inline(always)]
    pub fn gpt1b_cmp(self) -> &'a mut W {
        self.variant(EV_A::GPT1B_CMP)
    }
    #[doc = "GPT1A compare event. Configured by GPT1:TAMR.TCACT"]
    #[inline(always)]
    pub fn gpt1a_cmp(self) -> &'a mut W {
        self.variant(EV_A::GPT1A_CMP)
    }
    #[doc = "GPT0B compare event. Configured by GPT0:TBMR.TCACT"]
    #[inline(always)]
    pub fn gpt0b_cmp(self) -> &'a mut W {
        self.variant(EV_A::GPT0B_CMP)
    }
    #[doc = "GPT0A compare event. Configured by GPT0:TAMR.TCACT"]
    #[inline(always)]
    pub fn gpt0a_cmp(self) -> &'a mut W {
        self.variant(EV_A::GPT0A_CMP)
    }
    #[doc = "GPT1B interrupt event, controlled by GPT1:TBMR"]
    #[inline(always)]
    pub fn gpt1b(self) -> &'a mut W {
        self.variant(EV_A::GPT1B)
    }
    #[doc = "GPT1A interrupt event, controlled by GPT1:TAMR"]
    #[inline(always)]
    pub fn gpt1a(self) -> &'a mut W {
        self.variant(EV_A::GPT1A)
    }
    #[doc = "GPT0B interrupt event, controlled by GPT0:TBMR"]
    #[inline(always)]
    pub fn gpt0b(self) -> &'a mut W {
        self.variant(EV_A::GPT0B)
    }
    #[doc = "GPT0A interrupt event, controlled by GPT0:TAMR"]
    #[inline(always)]
    pub fn gpt0a(self) -> &'a mut W {
        self.variant(EV_A::GPT0A)
    }
    #[doc = "GPT3B interrupt event, controlled by GPT3:TBMR"]
    #[inline(always)]
    pub fn gpt3b(self) -> &'a mut W {
        self.variant(EV_A::GPT3B)
    }
    #[doc = "GPT3A interrupt event, controlled by GPT3:TAMR"]
    #[inline(always)]
    pub fn gpt3a(self) -> &'a mut W {
        self.variant(EV_A::GPT3A)
    }
    #[doc = "GPT2B interrupt event, controlled by GPT2:TBMR"]
    #[inline(always)]
    pub fn gpt2b(self) -> &'a mut W {
        self.variant(EV_A::GPT2B)
    }
    #[doc = "GPT2A interrupt event, controlled by GPT2:TAMR"]
    #[inline(always)]
    pub fn gpt2a(self) -> &'a mut W {
        self.variant(EV_A::GPT2A)
    }
    #[doc = "Always inactive"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(EV_A::NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub fn ev(&self) -> EV_R {
        EV_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub fn ev(&mut self) -> EV_W {
        EV_W { w: self }
    }
}
