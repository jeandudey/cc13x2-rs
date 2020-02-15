#[doc = "Reader of register I2SMCLKDIV"]
pub type R = crate::R<u32, super::I2SMCLKDIV>;
#[doc = "Writer for register I2SMCLKDIV"]
pub type W = crate::W<u32, super::I2SMCLKDIV>;
#[doc = "Register I2SMCLKDIV `reset()`'s with value 0"]
impl crate::ResetValue for super::I2SMCLKDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED10`"]
pub type RESERVED10_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED10`"]
pub struct RESERVED10_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x003f_ffff << 10)) | (((value as u32) & 0x003f_ffff) << 10);
        self.w
    }
}
#[doc = "Reader of field `MDIV`"]
pub type MDIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MDIV`"]
pub struct MDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:31 - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> RESERVED10_R {
        RESERVED10_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
    #[doc = "Bits 0:9 - 9:0\\]
An unsigned factor of the division ratio used to generate MCLK \\[2-1024\\]: MCLK = MCUCLK/MDIV\\[Hz\\]
MCUCLK is 48MHz. A value of 0 is interpreted as 1024. A value of 1 is invalid. If MDIV is odd the low phase of the clock is one MCUCLK period longer than the high phase. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn mdiv(&self) -> MDIV_R {
        MDIV_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 10:31 - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&mut self) -> RESERVED10_W {
        RESERVED10_W { w: self }
    }
    #[doc = "Bits 0:9 - 9:0\\]
An unsigned factor of the division ratio used to generate MCLK \\[2-1024\\]: MCLK = MCUCLK/MDIV\\[Hz\\]
MCUCLK is 48MHz. A value of 0 is interpreted as 1024. A value of 1 is invalid. If MDIV is odd the low phase of the clock is one MCUCLK period longer than the high phase. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn mdiv(&mut self) -> MDIV_W {
        MDIV_W { w: self }
    }
}
