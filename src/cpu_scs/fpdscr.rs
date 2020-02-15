#[doc = "Reader of register FPDSCR"]
pub type R = crate::R<u32, super::FPDSCR>;
#[doc = "Writer for register FPDSCR"]
pub type W = crate::W<u32, super::FPDSCR>;
#[doc = "Register FPDSCR `reset()`'s with value 0"]
impl crate::ResetValue for super::FPDSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED27`"]
pub type RESERVED27_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED27`"]
pub struct RESERVED27_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED27_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | (((value as u32) & 0x1f) << 27);
        self.w
    }
}
#[doc = "Reader of field `AHP`"]
pub type AHP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHP`"]
pub struct AHP_W<'a> {
    w: &'a mut W,
}
impl<'a> AHP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `DN`"]
pub type DN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DN`"]
pub struct DN_W<'a> {
    w: &'a mut W,
}
impl<'a> DN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `FZ`"]
pub type FZ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FZ`"]
pub struct FZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FZ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `RMODE`"]
pub type RMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RMODE`"]
pub struct RMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `RESERVED0`"]
pub type RESERVED0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED0`"]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | ((value as u32) & 0x003f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:31 - 31:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved27(&self) -> RESERVED27_R {
        RESERVED27_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
    #[doc = "Bit 26 - 26:26\\]
Default value for Alternative Half Precision bit. (If this bit is set to 1 then Alternative half-precision format is selected)."]
    #[inline(always)]
    pub fn ahp(&self) -> AHP_R {
        AHP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Default value for Default NaN mode bit. (If this bit is set to 1 then any operation involving one or more NaNs returns the Default NaN)."]
    #[inline(always)]
    pub fn dn(&self) -> DN_R {
        DN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Default value for Flush-to-Zero mode bit. (If this bit is set to 1 then Flush-to-zero mode is enabled)."]
    #[inline(always)]
    pub fn fz(&self) -> FZ_R {
        FZ_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Default value for Rounding Mode control field. (The encoding for this field is: 0b00 Round to Nearest (RN) mode 0b01 Round towards Plus Infinity (RP) mode 0b10 Round towards Minus Infinity (RM) mode 0b11 Round towards Zero (RZ) mode. The specified rounding mode is used by almost all floating-point instructions)."]
    #[inline(always)]
    pub fn rmode(&self) -> RMODE_R {
        RMODE_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 0:21 - 21:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 27:31 - 31:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved27(&mut self) -> RESERVED27_W {
        RESERVED27_W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\]
Default value for Alternative Half Precision bit. (If this bit is set to 1 then Alternative half-precision format is selected)."]
    #[inline(always)]
    pub fn ahp(&mut self) -> AHP_W {
        AHP_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
Default value for Default NaN mode bit. (If this bit is set to 1 then any operation involving one or more NaNs returns the Default NaN)."]
    #[inline(always)]
    pub fn dn(&mut self) -> DN_W {
        DN_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
Default value for Flush-to-Zero mode bit. (If this bit is set to 1 then Flush-to-zero mode is enabled)."]
    #[inline(always)]
    pub fn fz(&mut self) -> FZ_W {
        FZ_W { w: self }
    }
    #[doc = "Bits 22:23 - 23:22\\]
Default value for Rounding Mode control field. (The encoding for this field is: 0b00 Round to Nearest (RN) mode 0b01 Round towards Plus Infinity (RP) mode 0b10 Round towards Minus Infinity (RM) mode 0b11 Round towards Zero (RZ) mode. The specified rounding mode is used by almost all floating-point instructions)."]
    #[inline(always)]
    pub fn rmode(&mut self) -> RMODE_W {
        RMODE_W { w: self }
    }
    #[doc = "Bits 0:21 - 21:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
}
