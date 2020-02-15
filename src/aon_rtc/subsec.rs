#[doc = "Reader of register SUBSEC"]
pub type R = crate::R<u32, super::SUBSEC>;
#[doc = "Writer for register SUBSEC"]
pub type W = crate::W<u32, super::SUBSEC>;
#[doc = "Register SUBSEC `reset()`'s with value 0"]
impl crate::ResetValue for super::SUBSEC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VALUE`"]
pub type VALUE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `VALUE`"]
pub struct VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Unsigned integer representing Real Time Clock in fractions of a second (VALUE/2^32 seconds) at the time when SEC register was read. Examples : - 0x0000_0000 = 0.0 sec - 0x4000_0000 = 0.25 sec - 0x8000_0000 = 0.5 sec - 0xC000_0000 = 0.75 sec"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Unsigned integer representing Real Time Clock in fractions of a second (VALUE/2^32 seconds) at the time when SEC register was read. Examples : - 0x0000_0000 = 0.0 sec - 0x4000_0000 = 0.25 sec - 0x8000_0000 = 0.5 sec - 0xC000_0000 = 0.75 sec"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
}
