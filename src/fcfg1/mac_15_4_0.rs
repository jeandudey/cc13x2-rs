#[doc = "Reader of register MAC_15_4_0"]
pub type R = crate::R<u32, super::MAC_15_4_0>;
#[doc = "Writer for register MAC_15_4_0"]
pub type W = crate::W<u32, super::MAC_15_4_0>;
#[doc = "Register MAC_15_4_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MAC_15_4_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDR_0_31`"]
pub type ADDR_0_31_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADDR_0_31`"]
pub struct ADDR_0_31_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_0_31_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
The first 32-bits of the 64-bit MAC 15.4 address"]
    #[inline(always)]
    pub fn addr_0_31(&self) -> ADDR_0_31_R {
        ADDR_0_31_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
The first 32-bits of the 64-bit MAC 15.4 address"]
    #[inline(always)]
    pub fn addr_0_31(&mut self) -> ADDR_0_31_W {
        ADDR_0_31_W { w: self }
    }
}
