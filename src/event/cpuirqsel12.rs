#[doc = "Reader of register CPUIRQSEL12"]
pub type R = crate::R<u32, super::CPUIRQSEL12>;
#[doc = "Writer for register CPUIRQSEL12"]
pub type W = crate::W<u32, super::CPUIRQSEL12>;
#[doc = "Register CPUIRQSEL12 `reset()`'s with value 0x08"]
impl crate::ResetValue for super::CPUIRQSEL12 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x08
    }
}
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EV_A {
    #[doc = "8: Interrupt event from I2S"]
    I2S_IRQ = 8,
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
            8 => Val(EV_A::I2S_IRQ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `I2S_IRQ`"]
    #[inline(always)]
    pub fn is_i2s_irq(&self) -> bool {
        *self == EV_A::I2S_IRQ
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
    #[doc = "Interrupt event from I2S"]
    #[inline(always)]
    pub fn i2s_irq(self) -> &'a mut W {
        self.variant(EV_A::I2S_IRQ)
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
