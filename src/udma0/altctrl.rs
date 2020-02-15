#[doc = "Reader of register ALTCTRL"]
pub type R = crate::R<u32, super::ALTCTRL>;
#[doc = "Writer for register ALTCTRL"]
pub type W = crate::W<u32, super::ALTCTRL>;
#[doc = "Register ALTCTRL `reset()`'s with value 0x0200"]
impl crate::ResetValue for super::ALTCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200
    }
}
#[doc = "Reader of field `BASEPTR`"]
pub type BASEPTR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BASEPTR`"]
pub struct BASEPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> BASEPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register shows the base address for the alternate data structures and is calculated by module, thus read only"]
    #[inline(always)]
    pub fn baseptr(&self) -> BASEPTR_R {
        BASEPTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register shows the base address for the alternate data structures and is calculated by module, thus read only"]
    #[inline(always)]
    pub fn baseptr(&mut self) -> BASEPTR_W {
        BASEPTR_W { w: self }
    }
}
