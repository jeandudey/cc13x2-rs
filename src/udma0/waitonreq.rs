#[doc = "Reader of register WAITONREQ"]
pub type R = crate::R<u32, super::WAITONREQ>;
#[doc = "Writer for register WAITONREQ"]
pub type W = crate::W<u32, super::WAITONREQ>;
#[doc = "Register WAITONREQ `reset()`'s with value 0xffff_1eff"]
impl crate::ResetValue for super::WAITONREQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_1eff
    }
}
#[doc = "Reader of field `CHNLSTATUS`"]
pub type CHNLSTATUS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CHNLSTATUS`"]
pub struct CHNLSTATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> CHNLSTATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel wait on request status: Bit \\[Ch\\]
= 0: Once uDMA receives a single or burst request on channel Ch, this channel may come out of active state even if request is still present. Bit \\[Ch\\]
= 1: Once uDMA receives a single or burst request on channel Ch, it keeps channel Ch in active state until the requests are deasserted. This handshake is necessary for channels where the requester is in an asynchronous domain or can run at slower clock speed than uDMA"]
    #[inline(always)]
    pub fn chnlstatus(&self) -> CHNLSTATUS_R {
        CHNLSTATUS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel wait on request status: Bit \\[Ch\\]
= 0: Once uDMA receives a single or burst request on channel Ch, this channel may come out of active state even if request is still present. Bit \\[Ch\\]
= 1: Once uDMA receives a single or burst request on channel Ch, it keeps channel Ch in active state until the requests are deasserted. This handshake is necessary for channels where the requester is in an asynchronous domain or can run at slower clock speed than uDMA"]
    #[inline(always)]
    pub fn chnlstatus(&mut self) -> CHNLSTATUS_W {
        CHNLSTATUS_W { w: self }
    }
}
