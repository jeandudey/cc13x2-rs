#[doc = "Reader of register UDMACH4SSEL"]
pub type R = crate::R<u32, super::UDMACH4SSEL>;
#[doc = "Writer for register UDMACH4SSEL"]
pub type W = crate::W<u32, super::UDMACH4SSEL>;
#[doc = "Register UDMACH4SSEL `reset()`'s with value 0x2b"]
impl crate::ResetValue for super::UDMACH4SSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2b
    }
}
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 43"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EV_A {
    #[doc = "43: SSI0 TX DMA single request, controlled by SSI0:DMACR.TXDMAE"]
    SSI0_TX_DMASREQ = 43,
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
            43 => Val(EV_A::SSI0_TX_DMASREQ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SSI0_TX_DMASREQ`"]
    #[inline(always)]
    pub fn is_ssi0_tx_dmasreq(&self) -> bool {
        *self == EV_A::SSI0_TX_DMASREQ
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
    #[doc = "SSI0 TX DMA single request, controlled by SSI0:DMACR.TXDMAE"]
    #[inline(always)]
    pub fn ssi0_tx_dmasreq(self) -> &'a mut W {
        self.variant(EV_A::SSI0_TX_DMASREQ)
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
