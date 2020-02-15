#[doc = "Reader of register FFSR"]
pub type R = crate::R<u32, super::FFSR>;
#[doc = "Writer for register FFSR"]
pub type W = crate::W<u32, super::FFSR>;
#[doc = "Register FFSR `reset()`'s with value 0x08"]
impl crate::ResetValue for super::FFSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x08
    }
}
#[doc = "Reader of field `RESERVED4`"]
pub type RESERVED4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED4`"]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff_ffff << 4)) | (((value as u32) & 0x0fff_ffff) << 4);
        self.w
    }
}
#[doc = "Reader of field `FTNONSTOP`"]
pub type FTNONSTOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FTNONSTOP`"]
pub struct FTNONSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> FTNONSTOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
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
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
    #[doc = "Bit 3 - 3:3\\]
0: Formatter can be stopped 1: Formatter cannot be stopped"]
    #[inline(always)]
    pub fn ftnonstop(&self) -> FTNONSTOP_R {
        FTNONSTOP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - 2:0\\]
This field always reads as zero"]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
0: Formatter can be stopped 1: Formatter cannot be stopped"]
    #[inline(always)]
    pub fn ftnonstop(&mut self) -> FTNONSTOP_W {
        FTNONSTOP_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\]
This field always reads as zero"]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
}
