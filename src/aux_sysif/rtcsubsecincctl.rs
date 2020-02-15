#[doc = "Reader of register RTCSUBSECINCCTL"]
pub type R = crate::R<u32, super::RTCSUBSECINCCTL>;
#[doc = "Writer for register RTCSUBSECINCCTL"]
pub type W = crate::W<u32, super::RTCSUBSECINCCTL>;
#[doc = "Register RTCSUBSECINCCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCSUBSECINCCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
#[doc = "Reader of field `UPD_ACK`"]
pub type UPD_ACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UPD_ACK`"]
pub struct UPD_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> UPD_ACK_W<'a> {
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
#[doc = "Reader of field `UPD_REQ`"]
pub type UPD_REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UPD_REQ`"]
pub struct UPD_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> UPD_REQ_W<'a> {
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
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
    #[doc = "Bit 1 - 1:1\\]
Update acknowledgement. 0: AON_RTC has not acknowledged UPD_REQ. 1: AON_RTC has acknowledged UPD_REQ."]
    #[inline(always)]
    pub fn upd_ack(&self) -> UPD_ACK_R {
        UPD_ACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Request AON_RTC to update AON_RTC:SUBSECINC. 0: Clear request to update. 1: Set request to update. Only change UPD_REQ when it equals UPD_ACK. Clear UPD_REQ after UPD_ACK is 1."]
    #[inline(always)]
    pub fn upd_req(&self) -> UPD_REQ_R {
        UPD_REQ_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Update acknowledgement. 0: AON_RTC has not acknowledged UPD_REQ. 1: AON_RTC has acknowledged UPD_REQ."]
    #[inline(always)]
    pub fn upd_ack(&mut self) -> UPD_ACK_W {
        UPD_ACK_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Request AON_RTC to update AON_RTC:SUBSECINC. 0: Clear request to update. 1: Set request to update. Only change UPD_REQ when it equals UPD_ACK. Clear UPD_REQ after UPD_ACK is 1."]
    #[inline(always)]
    pub fn upd_req(&mut self) -> UPD_REQ_W {
        UPD_REQ_W { w: self }
    }
}
