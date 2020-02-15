#[doc = "Reader of register COMPARE"]
pub type R = crate::R<u32, super::COMPARE>;
#[doc = "Writer for register COMPARE"]
pub type W = crate::W<u32, super::COMPARE>;
#[doc = "Register COMPARE `reset()`'s with value 0x01"]
impl crate::ResetValue for super::COMPARE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `RESERVED3`"]
pub type RESERVED3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED3`"]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | (((value as u32) & 0x1fff_ffff) << 3);
        self.w
    }
}
#[doc = "Reader of field `A_GREATER_THAN_B`"]
pub type A_GREATER_THAN_B_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `A_GREATER_THAN_B`"]
pub struct A_GREATER_THAN_B_W<'a> {
    w: &'a mut W,
}
impl<'a> A_GREATER_THAN_B_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `A_LESS_THAN_B`"]
pub type A_LESS_THAN_B_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `A_LESS_THAN_B`"]
pub struct A_LESS_THAN_B_W<'a> {
    w: &'a mut W,
}
impl<'a> A_LESS_THAN_B_W<'a> {
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
#[doc = "Reader of field `A_EQUALS_B`"]
pub type A_EQUALS_B_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `A_EQUALS_B`"]
pub struct A_EQUALS_B_W<'a> {
    w: &'a mut W,
}
impl<'a> A_EQUALS_B_W<'a> {
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
    #[doc = "Bits 3:31 - 31:3\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 2 - 2:2\\]
Vector_A is greater than Vector_B"]
    #[inline(always)]
    pub fn a_greater_than_b(&self) -> A_GREATER_THAN_B_R {
        A_GREATER_THAN_B_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Vector_A is less than Vector_B"]
    #[inline(always)]
    pub fn a_less_than_b(&self) -> A_LESS_THAN_B_R {
        A_LESS_THAN_B_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Vector_A is equal to Vector_B"]
    #[inline(always)]
    pub fn a_equals_b(&self) -> A_EQUALS_B_R {
        A_EQUALS_B_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Vector_A is greater than Vector_B"]
    #[inline(always)]
    pub fn a_greater_than_b(&mut self) -> A_GREATER_THAN_B_W {
        A_GREATER_THAN_B_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Vector_A is less than Vector_B"]
    #[inline(always)]
    pub fn a_less_than_b(&mut self) -> A_LESS_THAN_B_W {
        A_LESS_THAN_B_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Vector_A is equal to Vector_B"]
    #[inline(always)]
    pub fn a_equals_b(&mut self) -> A_EQUALS_B_W {
        A_EQUALS_B_W { w: self }
    }
}
