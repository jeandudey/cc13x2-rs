#[doc = "Reader of register UDMACH2SSEL"]
pub type R = crate::R<u32, super::UDMACH2SSEL>;
#[doc = "Writer for register UDMACH2SSEL"]
pub type W = crate::W<u32, super::UDMACH2SSEL>;
#[doc = "Register UDMACH2SSEL `reset()`'s with value 0x33"]
impl crate::ResetValue for super::UDMACH2SSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x33
    }
}
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 51"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EV_A {
    #[doc = "51: UART0 TX DMA single request, controlled by UART0:DMACTL.TXDMAE"]
    UART0_TX_DMASREQ = 51,
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
            51 => Val(EV_A::UART0_TX_DMASREQ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `UART0_TX_DMASREQ`"]
    #[inline(always)]
    pub fn is_uart0_tx_dmasreq(&self) -> bool {
        *self == EV_A::UART0_TX_DMASREQ
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
    #[doc = "UART0 TX DMA single request, controlled by UART0:DMACTL.TXDMAE"]
    #[inline(always)]
    pub fn uart0_tx_dmasreq(self) -> &'a mut W {
        self.variant(EV_A::UART0_TX_DMASREQ)
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
