#[doc = "Reader of register UDMACH13SSEL"]
pub type R = crate::R<u32, super::UDMACH13SSEL>;
#[doc = "Writer for register UDMACH13SSEL"]
pub type W = crate::W<u32, super::UDMACH13SSEL>;
#[doc = "Register UDMACH13SSEL `reset()`'s with value 0x03"]
impl crate::ResetValue for super::UDMACH13SSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum EV_A {
    #[doc = "3: AON programmable event 2. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG2_EV"]
    AON_PROG2 = 3,
}
impl From<EV_A> for u32 {
    #[inline(always)]
    fn from(variant: EV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EV`"]
pub type EV_R = crate::R<u32, EV_A>;
impl EV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, EV_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(EV_A::AON_PROG2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AON_PROG2`"]
    #[inline(always)]
    pub fn is_aon_prog2(&self) -> bool {
        *self == EV_A::AON_PROG2
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
    #[doc = "AON programmable event 2. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG2_EV"]
    #[inline(always)]
    pub fn aon_prog2(self) -> &'a mut W {
        self.variant(EV_A::AON_PROG2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn ev(&self) -> EV_R {
        EV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn ev(&mut self) -> EV_W {
        EV_W { w: self }
    }
}
