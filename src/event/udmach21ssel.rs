#[doc = "Reader of register UDMACH21SSEL"]
pub type R = crate::R<u32, super::UDMACH21SSEL>;
#[doc = "Writer for register UDMACH21SSEL"]
pub type W = crate::W<u32, super::UDMACH21SSEL>;
#[doc = "Register UDMACH21SSEL `reset()`'s with value 0x64"]
impl crate::ResetValue for super::UDMACH21SSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x64
    }
}
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 100"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EV_A {
    #[doc = "100: Software event 0, triggered by SWEV.SWEV0"]
    SWEV0 = 100,
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
            100 => Val(EV_A::SWEV0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SWEV0`"]
    #[inline(always)]
    pub fn is_swev0(&self) -> bool {
        *self == EV_A::SWEV0
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
    #[doc = "Software event 0, triggered by SWEV.SWEV0"]
    #[inline(always)]
    pub fn swev0(self) -> &'a mut W {
        self.variant(EV_A::SWEV0)
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
