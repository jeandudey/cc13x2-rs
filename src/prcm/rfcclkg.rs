#[doc = "Reader of register RFCCLKG"]
pub type R = crate::R<u32, super::RFCCLKG>;
#[doc = "Writer for register RFCCLKG"]
pub type W = crate::W<u32, super::RFCCLKG>;
#[doc = "Register RFCCLKG `reset()`'s with value 0x01"]
impl crate::ResetValue for super::RFCCLKG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `RESERVED1`"]
pub type RESERVED1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff_ffff << 1)) | (((value as u32) & 0x7fff_ffff) << 1);
        self.w
    }
}
#[doc = "Reader of field `CLK_EN`"]
pub type CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK_EN`"]
pub struct CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_EN_W<'a> {
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
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 0 - 0:0\\]
0: Disable Clock 1: Enable clock if RFC power domain is on For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
0: Disable Clock 1: Enable clock if RFC power domain is on For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W { w: self }
    }
}
