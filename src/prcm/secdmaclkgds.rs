#[doc = "Reader of register SECDMACLKGDS"]
pub type R = crate::R<u32, super::SECDMACLKGDS>;
#[doc = "Writer for register SECDMACLKGDS"]
pub type W = crate::W<u32, super::SECDMACLKGDS>;
#[doc = "Register SECDMACLKGDS `reset()`'s with value 0"]
impl crate::ResetValue for super::SECDMACLKGDS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED9`"]
pub type RESERVED9_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED9`"]
pub struct RESERVED9_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x007f_ffff << 9)) | (((value as u32) & 0x007f_ffff) << 9);
        self.w
    }
}
#[doc = "Reader of field `DMA_CLK_EN`"]
pub type DMA_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_CLK_EN`"]
pub struct DMA_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `RESERVED3`"]
pub type RESERVED3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED3`"]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Reader of field `PKA_CLK_EN`"]
pub type PKA_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PKA_CLK_EN`"]
pub struct PKA_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PKA_CLK_EN_W<'a> {
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
#[doc = "Reader of field `TRNG_CLK_EN`"]
pub type TRNG_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRNG_CLK_EN`"]
pub struct TRNG_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRNG_CLK_EN_W<'a> {
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
#[doc = "Reader of field `CRYPTO_CLK_EN`"]
pub type CRYPTO_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRYPTO_CLK_EN`"]
pub struct CRYPTO_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTO_CLK_EN_W<'a> {
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
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x007f_ffff) as u32)
    }
    #[doc = "Bit 8 - 8:8\\]
0: Disable clock 1: Enable clock Can be forced on by SECDMACLKGR.DMA_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn dma_clk_en(&self) -> DMA_CLK_EN_R {
        DMA_CLK_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Disable clock 1: Enable clock Can be forced on by SECDMACLKGR.PKA_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn pka_clk_en(&self) -> PKA_CLK_EN_R {
        PKA_CLK_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Disable clock 1: Enable clock SYSBUS clock will always run when enabled Can be forced on by SECDMACLKGR.TRNG_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn trng_clk_en(&self) -> TRNG_CLK_EN_R {
        TRNG_CLK_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
0: Disable clock 1: Enable clock SYSBUS clock will always run when enabled Can be forced on by SECDMACLKGR.CRYPTO_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn crypto_clk_en(&self) -> CRYPTO_CLK_EN_R {
        CRYPTO_CLK_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&mut self) -> RESERVED9_W {
        RESERVED9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
0: Disable clock 1: Enable clock Can be forced on by SECDMACLKGR.DMA_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn dma_clk_en(&mut self) -> DMA_CLK_EN_W {
        DMA_CLK_EN_W { w: self }
    }
    #[doc = "Bits 3:7 - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
0: Disable clock 1: Enable clock Can be forced on by SECDMACLKGR.PKA_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn pka_clk_en(&mut self) -> PKA_CLK_EN_W {
        PKA_CLK_EN_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
0: Disable clock 1: Enable clock SYSBUS clock will always run when enabled Can be forced on by SECDMACLKGR.TRNG_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn trng_clk_en(&mut self) -> TRNG_CLK_EN_W {
        TRNG_CLK_EN_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
0: Disable clock 1: Enable clock SYSBUS clock will always run when enabled Can be forced on by SECDMACLKGR.CRYPTO_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn crypto_clk_en(&mut self) -> CRYPTO_CLK_EN_W {
        CRYPTO_CLK_EN_W { w: self }
    }
}
