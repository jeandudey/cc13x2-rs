#[doc = "Reader of register STAT"]
pub type R = crate::R<u8, super::STAT>;
#[doc = "Writer for register STAT"]
pub type W = crate::W<u8, super::STAT>;
#[doc = "Register STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::STAT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPARE0`"]
pub type SPARE0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPARE0`"]
pub struct SPARE0_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare0(&self) -> SPARE0_R {
        SPARE0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare0(&mut self) -> SPARE0_W {
        SPARE0_W { w: self }
    }
}
