#[doc = "Reader of register CPUIRQSEL13"]
pub type R = crate::R<u32, super::CPUIRQSEL13>;
#[doc = "Writer for register CPUIRQSEL13"]
pub type W = crate::W<u32, super::CPUIRQSEL13>;
#[doc = "Register CPUIRQSEL13 `reset()`'s with value 0x1d"]
impl crate::ResetValue for super::CPUIRQSEL13 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1d
    }
}
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 29"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EV_A {
    #[doc = "29: AUX software event 1, triggered by AUX_EVCTL:SWEVSET.SWEV1, also available as AUX_EVENT2 AON wake up event.\nMCU domain wakeup control AON_EVENT:MCUWUSEL"]
    AUX_SWEV1 = 29,
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
            29 => Val(EV_A::AUX_SWEV1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV1`"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == EV_A::AUX_SWEV1
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
    #[doc = "AUX software event 1, triggered by AUX_EVCTL:SWEVSET.SWEV1, also available as AUX_EVENT2 AON wake up event. MCU domain wakeup control AON_EVENT:MCUWUSEL"]
    #[inline(always)]
    pub fn aux_swev1(self) -> &'a mut W {
        self.variant(EV_A::AUX_SWEV1)
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
