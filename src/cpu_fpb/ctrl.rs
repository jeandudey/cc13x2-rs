#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0x0260"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0260
    }
}
#[doc = "Reader of field `RESERVED14`"]
pub type RESERVED14_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED14`"]
pub struct RESERVED14_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 14)) | (((value as u32) & 0x0003_ffff) << 14);
        self.w
    }
}
#[doc = "Reader of field `NUM_CODE2`"]
pub type NUM_CODE2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NUM_CODE2`"]
pub struct NUM_CODE2_W<'a> {
    w: &'a mut W,
}
impl<'a> NUM_CODE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `NUM_LIT`"]
pub type NUM_LIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NUM_LIT`"]
pub struct NUM_LIT_W<'a> {
    w: &'a mut W,
}
impl<'a> NUM_LIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `NUM_CODE1`"]
pub type NUM_CODE1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NUM_CODE1`"]
pub struct NUM_CODE1_W<'a> {
    w: &'a mut W,
}
impl<'a> NUM_CODE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `RESERVED2`"]
pub type RESERVED2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED2`"]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `KEY`"]
pub type KEY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `KEY`"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
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
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> RESERVED14_R {
        RESERVED14_R::new(((self.bits >> 14) & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Number of full banks of code comparators, sixteen comparators per bank. Where less than sixteen code comparators are provided, the bank count is zero, and the number present indicated by NUM_CODE1. This read only field contains 3'b000 to indicate 0 banks for Cortex-M processor."]
    #[inline(always)]
    pub fn num_code2(&self) -> NUM_CODE2_R {
        NUM_CODE2_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Number of literal slots field. 0x0: No literal slots 0x2: Two literal slots"]
    #[inline(always)]
    pub fn num_lit(&self) -> NUM_LIT_R {
        NUM_LIT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Number of code slots field. 0x0: No code slots 0x2: Two code slots 0x6: Six code slots"]
    #[inline(always)]
    pub fn num_code1(&self) -> NUM_CODE1_R {
        NUM_CODE1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - 1:1\\]
Key field. In order to write to this register, this bit-field must be written to '1'. This bit always reads 0."]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Flash patch unit enable bit 0x0: Flash patch unit disabled 0x1: Flash patch unit enabled"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&mut self) -> RESERVED14_W {
        RESERVED14_W { w: self }
    }
    #[doc = "Bits 12:13 - 13:12\\]
Number of full banks of code comparators, sixteen comparators per bank. Where less than sixteen code comparators are provided, the bank count is zero, and the number present indicated by NUM_CODE1. This read only field contains 3'b000 to indicate 0 banks for Cortex-M processor."]
    #[inline(always)]
    pub fn num_code2(&mut self) -> NUM_CODE2_W {
        NUM_CODE2_W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\]
Number of literal slots field. 0x0: No literal slots 0x2: Two literal slots"]
    #[inline(always)]
    pub fn num_lit(&mut self) -> NUM_LIT_W {
        NUM_LIT_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\]
Number of code slots field. 0x0: No code slots 0x2: Two code slots 0x6: Six code slots"]
    #[inline(always)]
    pub fn num_code1(&mut self) -> NUM_CODE1_W {
        NUM_CODE1_W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Key field. In order to write to this register, this bit-field must be written to '1'. This bit always reads 0."]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Flash patch unit enable bit 0x0: Flash patch unit disabled 0x1: Flash patch unit enabled"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
