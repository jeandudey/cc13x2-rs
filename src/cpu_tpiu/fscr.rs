#[doc = "Reader of register FSCR"]
pub type R = crate::R<u32, super::FSCR>;
#[doc = "Writer for register FSCR"]
pub type W = crate::W<u32, super::FSCR>;
#[doc = "Register FSCR `reset()`'s with value 0"]
impl crate::ResetValue for super::FSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FSCR`"]
pub type FSCR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FSCR`"]
pub struct FSCR_W<'a> {
    w: &'a mut W,
}
impl<'a> FSCR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
The global synchronization trigger is generated by the Program Counter (PC) Sampler block. This means that there is no synchronization counter in the TPIU."]
    #[inline(always)]
    pub fn fscr(&self) -> FSCR_R {
        FSCR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
The global synchronization trigger is generated by the Program Counter (PC) Sampler block. This means that there is no synchronization counter in the TPIU."]
    #[inline(always)]
    pub fn fscr(&mut self) -> FSCR_W {
        FSCR_W { w: self }
    }
}
