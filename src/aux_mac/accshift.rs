#[doc = "Reader of register ACCSHIFT"]
pub type R = crate::R<u32, super::ACCSHIFT>;
#[doc = "Writer for register ACCSHIFT"]
pub type W = crate::W<u32, super::ACCSHIFT>;
#[doc = "Register ACCSHIFT `reset()`'s with value 0"]
impl crate::ResetValue for super::ACCSHIFT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `LSL1`"]
pub type LSL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSL1`"]
pub struct LSL1_W<'a> {
    w: &'a mut W,
}
impl<'a> LSL1_W<'a> {
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
#[doc = "Reader of field `LSR1`"]
pub type LSR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSR1`"]
pub struct LSR1_W<'a> {
    w: &'a mut W,
}
impl<'a> LSR1_W<'a> {
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
#[doc = "Reader of field `ASR1`"]
pub type ASR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASR1`"]
pub struct ASR1_W<'a> {
    w: &'a mut W,
}
impl<'a> ASR1_W<'a> {
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
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 2 - 2:2\\]
Logic shift left by 1 bit. Write 1 to shift the accumulator one bit to the left, 0 inserted at bit 0."]
    #[inline(always)]
    pub fn lsl1(&self) -> LSL1_R {
        LSL1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Logic shift right by 1 bit. Write 1 to shift the accumulator one bit to the right, 0 inserted at bit 39."]
    #[inline(always)]
    pub fn lsr1(&self) -> LSR1_R {
        LSR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Arithmetic shift right by 1 bit. Write 1 to shift the accumulator one bit to the right, previous sign bit inserted at bit 39."]
    #[inline(always)]
    pub fn asr1(&self) -> ASR1_R {
        ASR1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Logic shift left by 1 bit. Write 1 to shift the accumulator one bit to the left, 0 inserted at bit 0."]
    #[inline(always)]
    pub fn lsl1(&mut self) -> LSL1_W {
        LSL1_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Logic shift right by 1 bit. Write 1 to shift the accumulator one bit to the right, 0 inserted at bit 39."]
    #[inline(always)]
    pub fn lsr1(&mut self) -> LSR1_W {
        LSR1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Arithmetic shift right by 1 bit. Write 1 to shift the accumulator one bit to the right, previous sign bit inserted at bit 39."]
    #[inline(always)]
    pub fn asr1(&mut self) -> ASR1_W {
        ASR1_W { w: self }
    }
}
