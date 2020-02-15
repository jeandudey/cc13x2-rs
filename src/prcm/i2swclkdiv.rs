#[doc = "Reader of register I2SWCLKDIV"]
pub type R = crate::R<u32, super::I2SWCLKDIV>;
#[doc = "Writer for register I2SWCLKDIV"]
pub type W = crate::W<u32, super::I2SWCLKDIV>;
#[doc = "Register I2SWCLKDIV `reset()`'s with value 0"]
impl crate::ResetValue for super::I2SWCLKDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `WDIV`"]
pub type WDIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WDIV`"]
pub struct WDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> WDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - 15:0\\]
If I2SCLKCTL.WCLK_PHASE = 0, Single phase. WCLK is high one BCLK period and low WDIV\\[9:0\\]
(unsigned, \\[1-1023\\]) BCLK periods. WCLK = MCUCLK / BDIV*(WDIV\\[9:0\\]
+ 1) \\[Hz\\]
MCUCLK is 48MHz. If I2SCLKCTL.WCLK_PHASE = 1, Dual phase. Each phase on WCLK (50% duty cycle) is WDIV\\[9:0\\]
(unsigned, \\[1-1023\\]) BCLK periods. WCLK = MCUCLK / BDIV*(2*WDIV\\[9:0\\]) \\[Hz\\]
If I2SCLKCTL.WCLK_PHASE = 2, User defined. WCLK is high WDIV\\[7:0\\]
(unsigned, \\[1-255\\]) BCLK periods and low WDIV\\[15:8\\]
(unsigned, \\[1-255\\]) BCLK periods. WCLK = MCUCLK / (BDIV*(WDIV\\[7:0\\]
+ WDIV\\[15:8\\]) \\[Hz\\]
For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn wdiv(&self) -> WDIV_R {
        WDIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
If I2SCLKCTL.WCLK_PHASE = 0, Single phase. WCLK is high one BCLK period and low WDIV\\[9:0\\]
(unsigned, \\[1-1023\\]) BCLK periods. WCLK = MCUCLK / BDIV*(WDIV\\[9:0\\]
+ 1) \\[Hz\\]
MCUCLK is 48MHz. If I2SCLKCTL.WCLK_PHASE = 1, Dual phase. Each phase on WCLK (50% duty cycle) is WDIV\\[9:0\\]
(unsigned, \\[1-1023\\]) BCLK periods. WCLK = MCUCLK / BDIV*(2*WDIV\\[9:0\\]) \\[Hz\\]
If I2SCLKCTL.WCLK_PHASE = 2, User defined. WCLK is high WDIV\\[7:0\\]
(unsigned, \\[1-255\\]) BCLK periods and low WDIV\\[15:8\\]
(unsigned, \\[1-255\\]) BCLK periods. WCLK = MCUCLK / (BDIV*(WDIV\\[7:0\\]
+ WDIV\\[15:8\\]) \\[Hz\\]
For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn wdiv(&mut self) -> WDIV_W {
        WDIV_W { w: self }
    }
}
