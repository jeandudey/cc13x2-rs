#[doc = "Reader of register SEC"]
pub type R = crate::R<u32, super::SEC>;
#[doc = "Writer for register SEC"]
pub type W = crate::W<u32, super::SEC>;
#[doc = "Register SEC `reset()`'s with value 0"]
impl crate::ResetValue for super::SEC {
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
Unsigned integer representing Real Time Clock in seconds. When reading this register the content of SUBSEC.VALUE is simultaneously latched. A consistent reading of the combined Real Time Clock can be obtained by first reading this register, then reading SUBSEC register."]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Unsigned integer representing Real Time Clock in seconds. When reading this register the content of SUBSEC.VALUE is simultaneously latched. A consistent reading of the combined Real Time Clock can be obtained by first reading this register, then reading SUBSEC register."]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
}
