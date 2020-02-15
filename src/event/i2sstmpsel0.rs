#[doc = "Reader of register I2SSTMPSEL0"]
pub type R = crate::R<u32, super::I2SSTMPSEL0>;
#[doc = "Writer for register I2SSTMPSEL0"]
pub type W = crate::W<u32, super::I2SSTMPSEL0>;
#[doc = "Register I2SSTMPSEL0 `reset()`'s with value 0x5f"]
impl crate::ResetValue for super::I2SSTMPSEL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x5f
    }
}
#[doc = "6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior.\n\nValue on reset: 95"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EV_A {
    #[doc = "121: Always asserted"]
    ALWAYS_ACTIVE = 121,
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
            0 => Val(EV_A::NONE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALWAYS_ACTIVE`"]
    #[inline(always)]
    pub fn is_always_active(&self) -> bool {
        *self == EV_A::ALWAYS_ACTIVE
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
