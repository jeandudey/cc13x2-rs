#[doc = "Reader of register CR1"]
pub type R = crate::R<u32, super::CR1>;
#[doc = "Writer for register CR1"]
pub type W = crate::W<u32, super::CR1>;
#[doc = "Register CR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SOD`"]
pub type SOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOD`"]
pub struct SOD_W<'a> {
    w: &'a mut W,
}
impl<'a> SOD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "2:2\\]
Master or slave mode select. This bit can be modified only when SSI is disabled, SSE=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MS_A {
    #[doc = "1: Device configured as slave"]
    SLAVE = 1,
    #[doc = "0: Device configured as master"]
    MASTER = 0,
}
impl From<MS_A> for bool {
    #[inline(always)]
    fn from(variant: MS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MS`"]
pub type MS_R = crate::R<bool, MS_A>;
impl MS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MS_A {
        match self.bits {
            true => MS_A::SLAVE,
            false => MS_A::MASTER,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == MS_A::SLAVE
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == MS_A::MASTER
    }
}
#[doc = "Write proxy for field `MS`"]
pub struct MS_W<'a> {
    w: &'a mut W,
}
impl<'a> MS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Device configured as slave"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut W {
        self.variant(MS_A::SLAVE)
    }
    #[doc = "Device configured as master"]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(MS_A::MASTER)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "1:1\\]
Synchronous serial interface enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSE_A {
    #[doc = "1: Operation enabled"]
    SSI_ENABLED = 1,
    #[doc = "0: Operation disabled"]
    SSI_DISABLED = 0,
}
impl From<SSE_A> for bool {
    #[inline(always)]
    fn from(variant: SSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SSE`"]
pub type SSE_R = crate::R<bool, SSE_A>;
impl SSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSE_A {
        match self.bits {
            true => SSE_A::SSI_ENABLED,
            false => SSE_A::SSI_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `SSI_ENABLED`"]
    #[inline(always)]
    pub fn is_ssi_enabled(&self) -> bool {
        *self == SSE_A::SSI_ENABLED
    }
    #[doc = "Checks if the value of the field is `SSI_DISABLED`"]
    #[inline(always)]
    pub fn is_ssi_disabled(&self) -> bool {
        *self == SSE_A::SSI_DISABLED
    }
}
#[doc = "Write proxy for field `SSE`"]
pub struct SSE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Operation enabled"]
    #[inline(always)]
    pub fn ssi_enabled(self) -> &'a mut W {
        self.variant(SSE_A::SSI_ENABLED)
    }
    #[doc = "Operation disabled"]
    #[inline(always)]
    pub fn ssi_disabled(self) -> &'a mut W {
        self.variant(SSE_A::SSI_DISABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `LBM`"]
pub type LBM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LBM`"]
pub struct LBM_W<'a> {
    w: &'a mut W,
}
impl<'a> LBM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - 3:3\\]
Slave-mode output disabled This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an SSI master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, this bitfield can be set if the SSI slave is not supposed to drive the TXD line: 0: SSI can drive the TXD output in slave mode. 1: SSI cannot drive the TXD output in slave mode."]
    #[inline(always)]
    pub fn sod(&self) -> SOD_R {
        SOD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Master or slave mode select. This bit can be modified only when SSI is disabled, SSE=0."]
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Synchronous serial interface enable."]
    #[inline(always)]
    pub fn sse(&self) -> SSE_R {
        SSE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Loop back mode: 0: Normal serial port operation enabled. 1: Output of transmit serial shifter is connected to input of receive serial shifter internally."]
    #[inline(always)]
    pub fn lbm(&self) -> LBM_R {
        LBM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - 3:3\\]
Slave-mode output disabled This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an SSI master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, this bitfield can be set if the SSI slave is not supposed to drive the TXD line: 0: SSI can drive the TXD output in slave mode. 1: SSI cannot drive the TXD output in slave mode."]
    #[inline(always)]
    pub fn sod(&mut self) -> SOD_W {
        SOD_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Master or slave mode select. This bit can be modified only when SSI is disabled, SSE=0."]
    #[inline(always)]
    pub fn ms(&mut self) -> MS_W {
        MS_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Synchronous serial interface enable."]
    #[inline(always)]
    pub fn sse(&mut self) -> SSE_W {
        SSE_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Loop back mode: 0: Normal serial port operation enabled. 1: Output of transmit serial shifter is connected to input of receive serial shifter internally."]
    #[inline(always)]
    pub fn lbm(&mut self) -> LBM_W {
        LBM_W { w: self }
    }
}
