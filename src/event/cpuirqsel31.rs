#[doc = "Reader of register CPUIRQSEL31"]
pub type R = crate::R<u32, super::CPUIRQSEL31>;
#[doc = "Writer for register CPUIRQSEL31"]
pub type W = crate::W<u32, super::CPUIRQSEL31>;
#[doc = "Register CPUIRQSEL31 `reset()`'s with value 0x6a"]
impl crate::ResetValue for super::CPUIRQSEL31 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x6a
    }
}
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 106"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EV_A {
    #[doc = "106: AUX Compare A event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPA"]
    AUX_COMPA = 106,
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
            106 => Val(EV_A::AUX_COMPA),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == EV_A::AUX_COMPA
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
    #[doc = "AUX Compare A event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPA"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(EV_A::AUX_COMPA)
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
