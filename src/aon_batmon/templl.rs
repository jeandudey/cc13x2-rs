#[doc = "Reader of register TEMPLL"]
pub type R = crate::R<u32, super::TEMPLL>;
#[doc = "Writer for register TEMPLL"]
pub type W = crate::W<u32, super::TEMPLL>;
#[doc = "Register TEMPLL `reset()`'s with value 0x0001_0000"]
impl crate::ResetValue for super::TEMPLL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_0000
    }
}
#[doc = "Reader of field `RESERVED17`"]
pub type RESERVED17_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED17`"]
pub struct RESERVED17_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 17)) | (((value as u32) & 0x7fff) << 17);
        self.w
    }
}
#[doc = "Reader of field `INT`"]
pub type INT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INT`"]
pub struct INT_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 8)) | (((value as u32) & 0x01ff) << 8);
        self.w
    }
}
#[doc = "Reader of field `FRAC`"]
pub type FRAC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRAC`"]
pub struct FRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `RESERVED0`"]
pub type RESERVED0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED0`"]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> RESERVED17_R {
        RESERVED17_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
    #[doc = "Bits 8:16 - 16:8\\]
Integer part (signed) of temperature lower limit. Total value = INTEGER + FRACTIONAL 2's complement encoding 0x100: Min value 0x1D8: -40C 0x1FF: -1C 0x00: 0C 0x1B: 27C 0x55: 85C 0xFF: Max value"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Fractional part of temperature lower limit. Total value = INTEGER + FRACTIONAL The encoding is an extension of the 2's complement encoding. 00: 0.0C 01: 0.25C 10: 0.5C 11: 0.75C For example: 000000001,00 = ( 1+0,00) = 1,00 000000000,11 = ( 0+0,75) = 0,75 000000000,10 = ( 0+0,50) = 0,50 000000000,01 = ( 0+0,25) = 0,25 000000000,00 = ( 0+0,00) = 0,00 111111111,11 = (-1+0,75) = -0,25 111111111,10 = (-1+0,50) = -0,50 111111111,01 = (-1+0,25) = -0,75 111111111,00 = (-1+0,00) = -1,00 111111110,11 = (-2+0,75) = -1,25"]
    #[inline(always)]
    pub fn frac(&self) -> FRAC_R {
        FRAC_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 0:5 - 5:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&mut self) -> RESERVED17_W {
        RESERVED17_W { w: self }
    }
    #[doc = "Bits 8:16 - 16:8\\]
Integer part (signed) of temperature lower limit. Total value = INTEGER + FRACTIONAL 2's complement encoding 0x100: Min value 0x1D8: -40C 0x1FF: -1C 0x00: 0C 0x1B: 27C 0x55: 85C 0xFF: Max value"]
    #[inline(always)]
    pub fn int(&mut self) -> INT_W {
        INT_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\]
Fractional part of temperature lower limit. Total value = INTEGER + FRACTIONAL The encoding is an extension of the 2's complement encoding. 00: 0.0C 01: 0.25C 10: 0.5C 11: 0.75C For example: 000000001,00 = ( 1+0,00) = 1,00 000000000,11 = ( 0+0,75) = 0,75 000000000,10 = ( 0+0,50) = 0,50 000000000,01 = ( 0+0,25) = 0,25 000000000,00 = ( 0+0,00) = 0,00 111111111,11 = (-1+0,75) = -0,25 111111111,10 = (-1+0,50) = -0,50 111111111,01 = (-1+0,25) = -0,75 111111111,00 = (-1+0,00) = -1,00 111111110,11 = (-2+0,75) = -1,25"]
    #[inline(always)]
    pub fn frac(&mut self) -> FRAC_W {
        FRAC_W { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
}
