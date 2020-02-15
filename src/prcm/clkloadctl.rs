#[doc = "Reader of register CLKLOADCTL"]
pub type R = crate::R<u32, super::CLKLOADCTL>;
#[doc = "Writer for register CLKLOADCTL"]
pub type W = crate::W<u32, super::CLKLOADCTL>;
#[doc = "Register CLKLOADCTL `reset()`'s with value 0x02"]
impl crate::ResetValue for super::CLKLOADCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `RESERVED2`"]
pub type RESERVED2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED2`"]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
#[doc = "Reader of field `LOAD_DONE`"]
pub type LOAD_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOAD_DONE`"]
pub struct LOAD_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> LOAD_DONE_W<'a> {
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
#[doc = "Reader of field `LOAD`"]
pub type LOAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOAD`"]
pub struct LOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> LOAD_W<'a> {
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
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
    #[doc = "Bit 1 - 1:1\\]
Status of LOAD. Will be cleared to 0 when any of the registers requiring a LOAD is written to, and be set to 1 when a LOAD is done. Note that writing no change to a register will result in the LOAD_DONE being cleared. 0 : One or more registers have been write accessed after last LOAD 1 : No registers are write accessed after last LOAD"]
    #[inline(always)]
    pub fn load_done(&self) -> LOAD_DONE_R {
        LOAD_DONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
0: No action 1: Load settings to CLKCTRL. Bit is HW cleared. Multiple changes to settings may be done before LOAD is written once so all changes takes place at the same time. LOAD can also be done after single setting updates. Registers that needs to be followed by LOAD before settings being applied are: - SYSBUSCLKDIV - CPUCLKDIV - PERBUSCPUCLKDIV - PERDMACLKDIV - PERBUSCPUCLKG - RFCCLKG - VIMSCLKG - SECDMACLKGR - SECDMACLKGS - SECDMACLKGDS - GPIOCLKGR - GPIOCLKGS - GPIOCLKGDS - GPTCLKGR - GPTCLKGS - GPTCLKGDS - GPTCLKDIV - I2CCLKGR - I2CCLKGS - I2CCLKGDS - SSICLKGR - SSICLKGS - SSICLKGDS - UARTCLKGR - UARTCLKGS - UARTCLKGDS - I2SCLKGR - I2SCLKGS - I2SCLKGDS - I2SBCLKSEL - I2SCLKCTL - I2SMCLKDIV - I2SBCLKDIV - I2SWCLKDIV"]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Status of LOAD. Will be cleared to 0 when any of the registers requiring a LOAD is written to, and be set to 1 when a LOAD is done. Note that writing no change to a register will result in the LOAD_DONE being cleared. 0 : One or more registers have been write accessed after last LOAD 1 : No registers are write accessed after last LOAD"]
    #[inline(always)]
    pub fn load_done(&mut self) -> LOAD_DONE_W {
        LOAD_DONE_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
0: No action 1: Load settings to CLKCTRL. Bit is HW cleared. Multiple changes to settings may be done before LOAD is written once so all changes takes place at the same time. LOAD can also be done after single setting updates. Registers that needs to be followed by LOAD before settings being applied are: - SYSBUSCLKDIV - CPUCLKDIV - PERBUSCPUCLKDIV - PERDMACLKDIV - PERBUSCPUCLKG - RFCCLKG - VIMSCLKG - SECDMACLKGR - SECDMACLKGS - SECDMACLKGDS - GPIOCLKGR - GPIOCLKGS - GPIOCLKGDS - GPTCLKGR - GPTCLKGS - GPTCLKGDS - GPTCLKDIV - I2CCLKGR - I2CCLKGS - I2CCLKGDS - SSICLKGR - SSICLKGS - SSICLKGDS - UARTCLKGR - UARTCLKGS - UARTCLKGDS - I2SCLKGR - I2SCLKGS - I2SCLKGDS - I2SBCLKSEL - I2SCLKCTL - I2SMCLKDIV - I2SBCLKDIV - I2SWCLKDIV"]
    #[inline(always)]
    pub fn load(&mut self) -> LOAD_W {
        LOAD_W { w: self }
    }
}
