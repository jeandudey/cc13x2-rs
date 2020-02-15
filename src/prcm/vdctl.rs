#[doc = "Reader of register VDCTL"]
pub type R = crate::R<u32, super::VDCTL>;
#[doc = "Writer for register VDCTL"]
pub type W = crate::W<u32, super::VDCTL>;
#[doc = "Register VDCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::VDCTL {
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
#[doc = "Reader of field `ULDO`"]
pub type ULDO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ULDO`"]
pub struct ULDO_W<'a> {
    w: &'a mut W,
}
impl<'a> ULDO_W<'a> {
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
Request PMCTL to switch to uLDO. 0: No request 1: Assert request when possible The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = x0 3. SECDMACLKGDS.DMA_CLK_EN = 0 and S.CRYPTO_CLK_EN\\]
= 0 and SECDMACLKGR.DMA_AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 and SECDMACLKGR.CRYPTO_AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 5. I2SCLKGDS.CLK_EN = 0 and I2SCLKGR.AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 6. RFC do no request access to BUS 7. System CPU in deepsleep"]
    #[inline(always)]
    pub fn uldo(&self) -> ULDO_R {
        ULDO_R::new((self.bits & 0x01) != 0)
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
Request PMCTL to switch to uLDO. 0: No request 1: Assert request when possible The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = x0 3. SECDMACLKGDS.DMA_CLK_EN = 0 and S.CRYPTO_CLK_EN\\]
= 0 and SECDMACLKGR.DMA_AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 and SECDMACLKGR.CRYPTO_AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 5. I2SCLKGDS.CLK_EN = 0 and I2SCLKGR.AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 6. RFC do no request access to BUS 7. System CPU in deepsleep"]
    #[inline(always)]
    pub fn uldo(&mut self) -> ULDO_W {
        ULDO_W { w: self }
    }
}
