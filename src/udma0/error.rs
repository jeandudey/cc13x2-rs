#[doc = "Reader of register ERROR"]
pub type R = crate::R<u32, super::ERROR>;
#[doc = "Writer for register ERROR"]
pub type W = crate::W<u32, super::ERROR>;
#[doc = "Register ERROR `reset()`'s with value 0"]
impl crate::ResetValue for super::ERROR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STATUS`"]
pub type STATUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATUS`"]
pub struct STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
Returns the status of bus error flag in uDMA, or clears this bit Read as: 0: No bus error detected 1: Bus error detected Write as: 0: No effect, status of bus error flag is unchanged. 1: Clears the bus error flag."]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Returns the status of bus error flag in uDMA, or clears this bit Read as: 0: No bus error detected 1: Bus error detected Write as: 0: No effect, status of bus error flag is unchanged. 1: Clears the bus error flag."]
    #[inline(always)]
    pub fn status(&mut self) -> STATUS_W {
        STATUS_W { w: self }
    }
}
