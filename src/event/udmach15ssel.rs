#[doc = "Reader of register UDMACH15SSEL"]
pub type R = crate::R<u32, super::UDMACH15SSEL>;
#[doc = "Writer for register UDMACH15SSEL"]
pub type W = crate::W<u32, super::UDMACH15SSEL>;
#[doc = "Register UDMACH15SSEL `reset()`'s with value 0x07"]
impl crate::ResetValue for super::UDMACH15SSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07
    }
}
#[doc = "31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum EV_A {
    #[doc = "7: Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting"]
    AON_RTC_COMB = 7,
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
            7 => Val(EV_A::AON_RTC_COMB),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AON_RTC_COMB`"]
    #[inline(always)]
    pub fn is_aon_rtc_comb(&self) -> bool {
        *self == EV_A::AON_RTC_COMB
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
    #[doc = "Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting"]
    #[inline(always)]
    pub fn aon_rtc_comb(self) -> &'a mut W {
        self.variant(EV_A::AON_RTC_COMB)
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
