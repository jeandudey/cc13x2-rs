#[doc = "Reader of register I2SBCLKSEL"]
pub type R = crate::R<u32, super::I2SBCLKSEL>;
#[doc = "Writer for register I2SBCLKSEL"]
pub type W = crate::W<u32, super::I2SBCLKSEL>;
#[doc = "Register I2SBCLKSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::I2SBCLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPARE1`"]
pub type SPARE1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPARE1`"]
pub struct SPARE1_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff_ffff << 1)) | (((value as u32) & 0x7fff_ffff) << 1);
        self.w
    }
}
#[doc = "Reader of field `SRC`"]
pub type SRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRC`"]
pub struct SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_W<'a> {
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
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare1(&self) -> SPARE1_R {
        SPARE1_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 0 - 0:0\\]
BCLK source selector 0: Use external BCLK 1: Use internally generated clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare1(&mut self) -> SPARE1_W {
        SPARE1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
BCLK source selector 0: Use external BCLK 1: Use internally generated clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
}
