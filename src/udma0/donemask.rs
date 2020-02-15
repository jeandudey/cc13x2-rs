#[doc = "Reader of register DONEMASK"]
pub type R = crate::R<u32, super::DONEMASK>;
#[doc = "Writer for register DONEMASK"]
pub type W = crate::W<u32, super::DONEMASK>;
#[doc = "Register DONEMASK `reset()`'s with value 0"]
impl crate::ResetValue for super::DONEMASK {
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
Controls the propagation of the uDMA done and active state to the assigned peripheral. Specifically used for software channels. Read as: Bit \\[Ch\\]
= 0: uDMA done and active state for channel Ch is not blocked from reaching to the peripherals. Note that the uDMA done state for channel \\[Ch\\]
is blocked from contributing to generation of combined uDMA done signal Bit \\[Ch\\]
= 1: uDMA done and active state for channel Ch is blocked from reaching to the peripherals. Note that the uDMA done state for channel \\[Ch\\]
is not blocked from contributing to generation of combined uDMA done signal Write as: Bit \\[Ch\\]
= 0: Allows uDMA done and active stat to propagate to the peripherals. Note that this disables uDMA done state for channel \\[Ch\\]
from contributing to generation of combined uDMA done signal Bit \\[Ch\\]
= 1: Blocks uDMA done and active state to propagate to the peripherals. Note that this enables uDMA done for channel \\[Ch\\]
to contribute to generation of combined uDMA done signal."]
    #[inline(always)]
    pub fn chnls(&self) -> CHNLS_R {
        CHNLS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Controls the propagation of the uDMA done and active state to the assigned peripheral. Specifically used for software channels. Read as: Bit \\[Ch\\]
= 0: uDMA done and active state for channel Ch is not blocked from reaching to the peripherals. Note that the uDMA done state for channel \\[Ch\\]
is blocked from contributing to generation of combined uDMA done signal Bit \\[Ch\\]
= 1: uDMA done and active state for channel Ch is blocked from reaching to the peripherals. Note that the uDMA done state for channel \\[Ch\\]
is not blocked from contributing to generation of combined uDMA done signal Write as: Bit \\[Ch\\]
= 0: Allows uDMA done and active stat to propagate to the peripherals. Note that this disables uDMA done state for channel \\[Ch\\]
from contributing to generation of combined uDMA done signal Bit \\[Ch\\]
= 1: Blocks uDMA done and active state to propagate to the peripherals. Note that this enables uDMA done for channel \\[Ch\\]
to contribute to generation of combined uDMA done signal."]
    #[inline(always)]
    pub fn chnls(&mut self) -> CHNLS_W {
        CHNLS_W { w: self }
    }
}
