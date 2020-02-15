#[doc = "Reader of register ICR"]
pub type R = crate::R<u32, super::ICR>;
#[doc = "Writer for register ICR"]
pub type W = crate::W<u32, super::ICR>;
#[doc = "Register ICR `reset()`'s with value 0"]
impl crate::ResetValue for super::ICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTIC`"]
pub type RTIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTIC`"]
pub struct RTIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTIC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `RORIC`"]
pub type RORIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RORIC`"]
pub struct RORIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RORIC_W<'a> {
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
    #[doc = "Bit 1 - 1:1\\]
Clear the receive timeout interrupt: Writing 1 to this field clears the timeout interrupt (RIS.RTRIS). Writing 0 has no effect."]
    #[inline(always)]
    pub fn rtic(&self) -> RTIC_R {
        RTIC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Clear the receive overrun interrupt: Writing 1 to this field clears the overrun error interrupt (RIS.RORRIS). Writing 0 has no effect."]
    #[inline(always)]
    pub fn roric(&self) -> RORIC_R {
        RORIC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 1:1\\]
Clear the receive timeout interrupt: Writing 1 to this field clears the timeout interrupt (RIS.RTRIS). Writing 0 has no effect."]
    #[inline(always)]
    pub fn rtic(&mut self) -> RTIC_W {
        RTIC_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Clear the receive overrun interrupt: Writing 1 to this field clears the overrun error interrupt (RIS.RORRIS). Writing 0 has no effect."]
    #[inline(always)]
    pub fn roric(&mut self) -> RORIC_W {
        RORIC_W { w: self }
    }
}
