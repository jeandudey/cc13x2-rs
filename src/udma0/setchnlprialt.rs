#[doc = "Reader of register SETCHNLPRIALT"]
pub type R = crate::R<u32, super::SETCHNLPRIALT>;
#[doc = "Writer for register SETCHNLPRIALT"]
pub type W = crate::W<u32, super::SETCHNLPRIALT>;
#[doc = "Register SETCHNLPRIALT `reset()`'s with value 0"]
impl crate::ResetValue for super::SETCHNLPRIALT {
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
Returns the channel control data structure status, or selects the alternate data structure for the corresponding uDMA channel. Read as: Bit \\[Ch\\]
= 0: uDMA channel Ch is using the primary data structure. Bit \\[Ch\\]
= 1: uDMA channel Ch is using the alternate data structure. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARCHNLPRIALT.CHNLS to disable a channel Bit \\[Ch\\]
= 1: Selects the alternate data structure for channel Ch Writing to a bit where a uDMA channel is not implemented has no effect"]
    #[inline(always)]
    pub fn chnls(&self) -> CHNLS_R {
        CHNLS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Returns the channel control data structure status, or selects the alternate data structure for the corresponding uDMA channel. Read as: Bit \\[Ch\\]
= 0: uDMA channel Ch is using the primary data structure. Bit \\[Ch\\]
= 1: uDMA channel Ch is using the alternate data structure. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARCHNLPRIALT.CHNLS to disable a channel Bit \\[Ch\\]
= 1: Selects the alternate data structure for channel Ch Writing to a bit where a uDMA channel is not implemented has no effect"]
    #[inline(always)]
    pub fn chnls(&mut self) -> CHNLS_W {
        CHNLS_W { w: self }
    }
}
