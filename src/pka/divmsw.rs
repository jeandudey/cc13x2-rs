#[doc = "Reader of register DIVMSW"]
pub type R = crate::R<u32, super::DIVMSW>;
#[doc = "Writer for register DIVMSW"]
pub type W = crate::W<u32, super::DIVMSW>;
#[doc = "Register DIVMSW `reset()`'s with value 0x8000"]
impl crate::ResetValue for super::DIVMSW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000
    }
}
#[doc = "Reader of field `RESERVED16`"]
pub type RESERVED16_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED16`"]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RESULT_IS_ZERO`"]
pub type RESULT_IS_ZERO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESULT_IS_ZERO`"]
pub struct RESULT_IS_ZERO_W<'a> {
    w: &'a mut W,
}
impl<'a> RESULT_IS_ZERO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `RESERVED11`"]
pub type RESERVED11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED11`"]
pub struct RESERVED11_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 11)) | (((value as u32) & 0x0f) << 11);
        self.w
    }
}
#[doc = "Reader of field `MSW_ADDRESS`"]
pub type MSW_ADDRESS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MSW_ADDRESS`"]
pub struct MSW_ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> MSW_ADDRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 15 - 15:15\\]
The result vector is all zeroes, ignore the address returned in bits \\[10:0\\]"]
    #[inline(always)]
    pub fn result_is_zero(&self) -> RESULT_IS_ZERO_R {
        RESULT_IS_ZERO_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 11:14 - 14:11\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved11(&self) -> RESERVED11_R {
        RESERVED11_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 0:10 - 10:0\\]
Address of the most significant nonzero 32-bit word of the remainder result vector in PKA RAM"]
    #[inline(always)]
    pub fn msw_address(&self) -> MSW_ADDRESS_R {
        MSW_ADDRESS_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
The result vector is all zeroes, ignore the address returned in bits \\[10:0\\]"]
    #[inline(always)]
    pub fn result_is_zero(&mut self) -> RESULT_IS_ZERO_W {
        RESULT_IS_ZERO_W { w: self }
    }
    #[doc = "Bits 11:14 - 14:11\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved11(&mut self) -> RESERVED11_W {
        RESERVED11_W { w: self }
    }
    #[doc = "Bits 0:10 - 10:0\\]
Address of the most significant nonzero 32-bit word of the remainder result vector in PKA RAM"]
    #[inline(always)]
    pub fn msw_address(&mut self) -> MSW_ADDRESS_W {
        MSW_ADDRESS_W { w: self }
    }
}
