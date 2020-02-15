#[doc = "Reader of register FEMU_ADDR"]
pub type R = crate::R<u32, super::FEMU_ADDR>;
#[doc = "Writer for register FEMU_ADDR"]
pub type W = crate::W<u32, super::FEMU_ADDR>;
#[doc = "Register FEMU_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::FEMU_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EMU_ADDR`"]
pub type EMU_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EMU_ADDR`"]
pub struct EMU_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> EMU_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn emu_addr(&self) -> EMU_ADDR_R {
        EMU_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn emu_addr(&mut self) -> EMU_ADDR_W {
        EMU_ADDR_W { w: self }
    }
}
