#[doc = "Reader of register SOFTREQ"]
pub type R = crate::R<u32, super::SOFTREQ>;
#[doc = "Writer for register SOFTREQ"]
pub type W = crate::W<u32, super::SOFTREQ>;
#[doc = "Register SOFTREQ `reset()`'s with value 0"]
impl crate::ResetValue for super::SOFTREQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHNLS`"]
pub type CHNLS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CHNLS`"]
pub struct CHNLS_W<'a> {
    w: &'a mut W,
}
impl<'a> CHNLS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Set the appropriate bit to generate a software uDMA request on the corresponding uDMA channel Bit \\[Ch\\]
= 0: Does not create a uDMA request for channel Ch Bit \\[Ch\\]
= 1: Creates a uDMA request for channel Ch Writing to a bit where a uDMA channel is not implemented does not create a uDMA request for that channel"]
    #[inline(always)]
    pub fn chnls(&self) -> CHNLS_R {
        CHNLS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Set the appropriate bit to generate a software uDMA request on the corresponding uDMA channel Bit \\[Ch\\]
= 0: Does not create a uDMA request for channel Ch Bit \\[Ch\\]
= 1: Creates a uDMA request for channel Ch Writing to a bit where a uDMA channel is not implemented does not create a uDMA request for that channel"]
    #[inline(always)]
    pub fn chnls(&mut self) -> CHNLS_W {
        CHNLS_W { w: self }
    }
}
