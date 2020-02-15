#[doc = "Reader of register IRQFLAGSTAT"]
pub type R = crate::R<u32, super::IRQFLAGSTAT>;
#[doc = "Writer for register IRQFLAGSTAT"]
pub type W = crate::W<u32, super::IRQFLAGSTAT>;
#[doc = "Register IRQFLAGSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::IRQFLAGSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NEED_CLOCK`"]
pub type NEED_CLOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NEED_CLOCK`"]
pub struct NEED_CLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> NEED_CLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `RESERVED2`"]
pub type RESERVED2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED2`"]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 2)) | (((value as u32) & 0x1fff_ffff) << 2);
        self.w
    }
}
#[doc = "Reader of field `SHUTDOWN_OVF`"]
pub type SHUTDOWN_OVF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SHUTDOWN_OVF`"]
pub struct SHUTDOWN_OVF_W<'a> {
    w: &'a mut W,
}
impl<'a> SHUTDOWN_OVF_W<'a> {
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
#[doc = "Reader of field `RDY`"]
pub type RDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RDY`"]
pub struct RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> RDY_W<'a> {
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
    #[doc = "Bit 31 - 31:31\\]
1: Indicates that the TRNG is busy generating entropy or is in one of its test modes - clocks may not be turned off and the power supply voltage must be kept stable. 0: TRNG is idle and can be shut down"]
    #[inline(always)]
    pub fn need_clock(&self) -> NEED_CLOCK_R {
        NEED_CLOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 2:30 - 30:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 1 - 1:1\\]
1: The number of FROs shut down (i.e. the number of '1' bits in the ALARMSTOP register) has exceeded the threshold set by ALARMCNT.SHUTDOWN_THR Writing '1' to IRQFLAGCLR.SHUTDOWN_OVF clears this bit to '0' again."]
    #[inline(always)]
    pub fn shutdown_ovf(&self) -> SHUTDOWN_OVF_R {
        SHUTDOWN_OVF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
1: Data are available in OUT0 and OUT1. Acknowledging this state by writing '1' to IRQFLAGCLR.RDY clears this bit to '0'. If a new number is already available in the internal register of the TRNG, the number is directly clocked into the result register. In this case the status bit is asserted again, after one clock cycle."]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
1: Indicates that the TRNG is busy generating entropy or is in one of its test modes - clocks may not be turned off and the power supply voltage must be kept stable. 0: TRNG is idle and can be shut down"]
    #[inline(always)]
    pub fn need_clock(&mut self) -> NEED_CLOCK_W {
        NEED_CLOCK_W { w: self }
    }
    #[doc = "Bits 2:30 - 30:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
1: The number of FROs shut down (i.e. the number of '1' bits in the ALARMSTOP register) has exceeded the threshold set by ALARMCNT.SHUTDOWN_THR Writing '1' to IRQFLAGCLR.SHUTDOWN_OVF clears this bit to '0' again."]
    #[inline(always)]
    pub fn shutdown_ovf(&mut self) -> SHUTDOWN_OVF_W {
        SHUTDOWN_OVF_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
1: Data are available in OUT0 and OUT1. Acknowledging this state by writing '1' to IRQFLAGCLR.RDY clears this bit to '0'. If a new number is already available in the internal register of the TRNG, the number is directly clocked into the result register. In this case the status bit is asserted again, after one clock cycle."]
    #[inline(always)]
    pub fn rdy(&mut self) -> RDY_W {
        RDY_W { w: self }
    }
}
