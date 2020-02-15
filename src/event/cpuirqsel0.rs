#[doc = "Reader of register CPUIRQSEL0"]
pub type R = crate::R<u32, super::CPUIRQSEL0>;
#[doc = "Writer for register CPUIRQSEL0"]
pub type W = crate::W<u32, super::CPUIRQSEL0>;
#[doc = "Register CPUIRQSEL0 `reset()`'s with value 0x04"]
impl crate::ResetValue for super::CPUIRQSEL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EV_A {
    #[doc = "4: Edge detect event from IOC. Configureded by the IOC:IOCFGn.EDGE_IRQ_EN and  IOC:IOCFGn.EDGE_DET settings"]
    AON_GPIO_EDGE = 4,
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
            4 => Val(EV_A::AON_GPIO_EDGE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AON_GPIO_EDGE`"]
    #[inline(always)]
    pub fn is_aon_gpio_edge(&self) -> bool {
        *self == EV_A::AON_GPIO_EDGE
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
    #[doc = "Edge detect event from IOC. Configureded by the IOC:IOCFGn.EDGE_IRQ_EN and IOC:IOCFGn.EDGE_DET settings"]
    #[inline(always)]
    pub fn aon_gpio_edge(self) -> &'a mut W {
        self.variant(EV_A::AON_GPIO_EDGE)
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
