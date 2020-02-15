#[doc = "Reader of register RFCSEL1"]
pub type R = crate::R<u32, super::RFCSEL1>;
#[doc = "Writer for register RFCSEL1"]
pub type W = crate::W<u32, super::RFCSEL1>;
#[doc = "Register RFCSEL1 `reset()`'s with value 0x3e"]
impl crate::ResetValue for super::RFCSEL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3e
    }
}
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 62"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EV_A {
    #[doc = "62: GPT0B compare event. Configured by GPT0:TBMR.TCACT"]
    GPT0B_CMP = 62,
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
            62 => Val(EV_A::GPT0B_CMP),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GPT0B_CMP`"]
    #[inline(always)]
    pub fn is_gpt0b_cmp(&self) -> bool {
        *self == EV_A::GPT0B_CMP
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
    #[doc = "GPT0B compare event. Configured by GPT0:TBMR.TCACT"]
    #[inline(always)]
    pub fn gpt0b_cmp(self) -> &'a mut W {
        self.variant(EV_A::GPT0B_CMP)
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
Read only selection value"]
    #[inline(always)]
    pub fn ev(&self) -> EV_R {
        EV_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Read only selection value"]
    #[inline(always)]
    pub fn ev(&mut self) -> EV_W {
        EV_W { w: self }
    }
}
