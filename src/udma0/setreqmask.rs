#[doc = "Reader of register SETREQMASK"]
pub type R = crate::R<u32, super::SETREQMASK>;
#[doc = "Writer for register SETREQMASK"]
pub type W = crate::W<u32, super::SETREQMASK>;
#[doc = "Register SETREQMASK `reset()`'s with value 0"]
impl crate::ResetValue for super::SETREQMASK {
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
Returns the burst and single request mask status, or disables the corresponding channel from generating uDMA requests. Read as: Bit \\[Ch\\]
= 0: External requests are enabled for channel Ch. Bit \\[Ch\\]
= 1: External requests are disabled for channel Ch. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARREQMASK.CHNLS to enable uDMA requests. Bit \\[Ch\\]
= 1: Disables uDMA burst request channel \\[C\\]
and uDMA single request channel \\[C\\]
input from generating uDMA requests. Writing to a bit where a uDMA channel is not implemented has no effect"]
    #[inline(always)]
    pub fn chnls(&self) -> CHNLS_R {
        CHNLS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Returns the burst and single request mask status, or disables the corresponding channel from generating uDMA requests. Read as: Bit \\[Ch\\]
= 0: External requests are enabled for channel Ch. Bit \\[Ch\\]
= 1: External requests are disabled for channel Ch. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARREQMASK.CHNLS to enable uDMA requests. Bit \\[Ch\\]
= 1: Disables uDMA burst request channel \\[C\\]
and uDMA single request channel \\[C\\]
input from generating uDMA requests. Writing to a bit where a uDMA channel is not implemented has no effect"]
    #[inline(always)]
    pub fn chnls(&mut self) -> CHNLS_W {
        CHNLS_W { w: self }
    }
}
