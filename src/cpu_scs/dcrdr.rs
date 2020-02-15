#[doc = "Reader of register DCRDR"]
pub type R = crate::R<u32, super::DCRDR>;
#[doc = "Writer for register DCRDR"]
pub type W = crate::W<u32, super::DCRDR>;
#[doc = "Register DCRDR `reset()`'s with value 0"]
impl crate::ResetValue for super::DCRDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DCRDR`"]
pub type DCRDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DCRDR`"]
pub struct DCRDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DCRDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register holds data for reading and writing registers to and from the processor. This is the data value written to the register selected by DCRSR. When the processor receives a request from DCRSR, this register is read or written by the processor using a normal load-store unit operation. If core register transfers are not being performed, software-based debug monitors can use this register for communication in non-halting debug. This enables flags and bits to acknowledge state and indicate if commands have been accepted to, replied to, or accepted and replied to."]
    #[inline(always)]
    pub fn dcrdr(&self) -> DCRDR_R {
        DCRDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register holds data for reading and writing registers to and from the processor. This is the data value written to the register selected by DCRSR. When the processor receives a request from DCRSR, this register is read or written by the processor using a normal load-store unit operation. If core register transfers are not being performed, software-based debug monitors can use this register for communication in non-halting debug. This enables flags and bits to acknowledge state and indicate if commands have been accepted to, replied to, or accepted and replied to."]
    #[inline(always)]
    pub fn dcrdr(&mut self) -> DCRDR_W {
        DCRDR_W { w: self }
    }
}
