#[doc = "Reader of register REQDONE"]
pub type R = crate::R<u32, super::REQDONE>;
#[doc = "Writer for register REQDONE"]
pub type W = crate::W<u32, super::REQDONE>;
#[doc = "Register REQDONE `reset()`'s with value 0"]
impl crate::ResetValue for super::REQDONE {
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
Reflects the uDMA done status for the given channel, channel \\[Ch\\]. It's a sticky done bit. Unless cleared by writing a 1, it holds the value of 1. Read as: Bit \\[Ch\\]
= 0: Request has not completed for channel Ch Bit \\[Ch\\]
= 1: Request has completed for the channel Ch Writing a 1 to individual bits would clear the corresponding bit. Write as: Bit \\[Ch\\]
= 0: No effect. Bit \\[Ch\\]
= 1: The corresponding \\[Ch\\]
bit is cleared and is set to 0"]
    #[inline(always)]
    pub fn chnls(&self) -> CHNLS_R {
        CHNLS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reflects the uDMA done status for the given channel, channel \\[Ch\\]. It's a sticky done bit. Unless cleared by writing a 1, it holds the value of 1. Read as: Bit \\[Ch\\]
= 0: Request has not completed for channel Ch Bit \\[Ch\\]
= 1: Request has completed for the channel Ch Writing a 1 to individual bits would clear the corresponding bit. Write as: Bit \\[Ch\\]
= 0: No effect. Bit \\[Ch\\]
= 1: The corresponding \\[Ch\\]
bit is cleared and is set to 0"]
    #[inline(always)]
    pub fn chnls(&mut self) -> CHNLS_W {
        CHNLS_W { w: self }
    }
}
