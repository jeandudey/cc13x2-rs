#[doc = "Reader of register SECDMACLKGR"]
pub type R = crate::R<u32, super::SECDMACLKGR>;
#[doc = "Writer for register SECDMACLKGR"]
pub type W = crate::W<u32, super::SECDMACLKGR>;
#[doc = "Register SECDMACLKGR `reset()`'s with value 0"]
impl crate::ResetValue for super::SECDMACLKGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED25`"]
pub type RESERVED25_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED25`"]
pub struct RESERVED25_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED25_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | (((value as u32) & 0x7f) << 25);
        self.w
    }
}
#[doc = "Reader of field `DMA_AM_CLK_EN`"]
pub type DMA_AM_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_AM_CLK_EN`"]
pub struct DMA_AM_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_AM_CLK_EN_W<'a> {
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
#[doc = "Reader of field `RESERVED20`"]
pub type RESERVED20_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED20`"]
pub struct RESERVED20_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `PKA_ZERIOZE_RESET_N`"]
pub type PKA_ZERIOZE_RESET_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PKA_ZERIOZE_RESET_N`"]
pub struct PKA_ZERIOZE_RESET_N_W<'a> {
    w: &'a mut W,
}
impl<'a> PKA_ZERIOZE_RESET_N_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `PKA_AM_CLK_EN`"]
pub type PKA_AM_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PKA_AM_CLK_EN`"]
pub struct PKA_AM_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PKA_AM_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `TRNG_AM_CLK_EN`"]
pub type TRNG_AM_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRNG_AM_CLK_EN`"]
pub struct TRNG_AM_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRNG_AM_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `CRYPTO_AM_CLK_EN`"]
pub type CRYPTO_AM_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRYPTO_AM_CLK_EN`"]
pub struct CRYPTO_AM_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTO_AM_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `RESERVED9`"]
pub type RESERVED9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED9`"]
pub struct RESERVED9_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 9)) | (((value as u32) & 0x7f) << 9);
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
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&self) -> RESERVED25_R {
        RESERVED25_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides DMA_CLK_EN, SECDMACLKGS.DMA_CLK_EN and SECDMACLKGDS.DMA_CLK_EN when enabled. SYSBUS clock will always run when enabled For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn dma_am_clk_en(&self) -> DMA_AM_CLK_EN_R {
        DMA_AM_CLK_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&self) -> RESERVED20_R {
        RESERVED20_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 19 - 19:19\\]
Zeroization logic hardware reset. 0: pka_zeroize logic inactive. 1: pka_zeroize of memory is enabled. This register must remain active until the memory are completely zeroized which requires 256 periods on systembus clock."]
    #[inline(always)]
    pub fn pka_zerioze_reset_n(&self) -> PKA_ZERIOZE_RESET_N_R {
        PKA_ZERIOZE_RESET_N_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides PKA_CLK_EN, SECDMACLKGS.PKA_CLK_EN and SECDMACLKGDS.PKA_CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn pka_am_clk_en(&self) -> PKA_AM_CLK_EN_R {
        PKA_AM_CLK_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides TRNG_CLK_EN, SECDMACLKGS.TRNG_CLK_EN and SECDMACLKGDS.TRNG_CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn trng_am_clk_en(&self) -> TRNG_AM_CLK_EN_R {
        TRNG_AM_CLK_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CRYPTO_CLK_EN, SECDMACLKGS.CRYPTO_CLK_EN and SECDMACLKGDS.CRYPTO_CLK_EN when enabled. SYSBUS clock will always run when enabled For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn crypto_am_clk_en(&self) -> CRYPTO_AM_CLK_EN_R {
        CRYPTO_AM_CLK_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
0: Disable clock 1: Enable clock Can be forced on by DMA_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
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
0: Disable clock 1: Enable clock Can be forced on by PKA_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn pka_clk_en(&self) -> PKA_CLK_EN_R {
        PKA_CLK_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Disable clock 1: Enable clock Can be forced on by TRNG_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn trng_clk_en(&self) -> TRNG_CLK_EN_R {
        TRNG_CLK_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
0: Disable clock 1: Enable clock Can be forced on by CRYPTO_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn crypto_clk_en(&self) -> CRYPTO_CLK_EN_R {
        CRYPTO_CLK_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&mut self) -> RESERVED25_W {
        RESERVED25_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides DMA_CLK_EN, SECDMACLKGS.DMA_CLK_EN and SECDMACLKGDS.DMA_CLK_EN when enabled. SYSBUS clock will always run when enabled For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn dma_am_clk_en(&mut self) -> DMA_AM_CLK_EN_W {
        DMA_AM_CLK_EN_W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&mut self) -> RESERVED20_W {
        RESERVED20_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\]
Zeroization logic hardware reset. 0: pka_zeroize logic inactive. 1: pka_zeroize of memory is enabled. This register must remain active until the memory are completely zeroized which requires 256 periods on systembus clock."]
    #[inline(always)]
    pub fn pka_zerioze_reset_n(&mut self) -> PKA_ZERIOZE_RESET_N_W {
        PKA_ZERIOZE_RESET_N_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides PKA_CLK_EN, SECDMACLKGS.PKA_CLK_EN and SECDMACLKGDS.PKA_CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn pka_am_clk_en(&mut self) -> PKA_AM_CLK_EN_W {
        PKA_AM_CLK_EN_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides TRNG_CLK_EN, SECDMACLKGS.TRNG_CLK_EN and SECDMACLKGDS.TRNG_CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn trng_am_clk_en(&mut self) -> TRNG_AM_CLK_EN_W {
        TRNG_AM_CLK_EN_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CRYPTO_CLK_EN, SECDMACLKGS.CRYPTO_CLK_EN and SECDMACLKGDS.CRYPTO_CLK_EN when enabled. SYSBUS clock will always run when enabled For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn crypto_am_clk_en(&mut self) -> CRYPTO_AM_CLK_EN_W {
        CRYPTO_AM_CLK_EN_W { w: self }
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&mut self) -> RESERVED9_W {
        RESERVED9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
0: Disable clock 1: Enable clock Can be forced on by DMA_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
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
0: Disable clock 1: Enable clock Can be forced on by PKA_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn pka_clk_en(&mut self) -> PKA_CLK_EN_W {
        PKA_CLK_EN_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
0: Disable clock 1: Enable clock Can be forced on by TRNG_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn trng_clk_en(&mut self) -> TRNG_CLK_EN_W {
        TRNG_CLK_EN_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
0: Disable clock 1: Enable clock Can be forced on by CRYPTO_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn crypto_clk_en(&mut self) -> CRYPTO_CLK_EN_W {
        CRYPTO_CLK_EN_W { w: self }
    }
}
