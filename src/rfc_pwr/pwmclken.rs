#[doc = "Reader of register PWMCLKEN"]
pub type R = crate::R<u32, super::PWMCLKEN>;
#[doc = "Writer for register PWMCLKEN"]
pub type W = crate::W<u32, super::PWMCLKEN>;
#[doc = "Register PWMCLKEN `reset()`'s with value 0x01"]
impl crate::ResetValue for super::PWMCLKEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `RESERVED11`"]
pub type RESERVED11_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED11`"]
pub struct RESERVED11_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x001f_ffff << 11)) | (((value as u32) & 0x001f_ffff) << 11);
        self.w
    }
}
#[doc = "Reader of field `RFCTRC`"]
pub type RFCTRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFCTRC`"]
pub struct RFCTRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCTRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `FSCA`"]
pub type FSCA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSCA`"]
pub struct FSCA_W<'a> {
    w: &'a mut W,
}
impl<'a> FSCA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `PHA`"]
pub type PHA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PHA`"]
pub struct PHA_W<'a> {
    w: &'a mut W,
}
impl<'a> PHA_W<'a> {
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
#[doc = "Reader of field `RAT`"]
pub type RAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAT`"]
pub struct RAT_W<'a> {
    w: &'a mut W,
}
impl<'a> RAT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `RFERAM`"]
pub type RFERAM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFERAM`"]
pub struct RFERAM_W<'a> {
    w: &'a mut W,
}
impl<'a> RFERAM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `RFE`"]
pub type RFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFE`"]
pub struct RFE_W<'a> {
    w: &'a mut W,
}
impl<'a> RFE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `MDMRAM`"]
pub type MDMRAM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MDMRAM`"]
pub struct MDMRAM_W<'a> {
    w: &'a mut W,
}
impl<'a> MDMRAM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `MDM`"]
pub type MDM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MDM`"]
pub struct MDM_W<'a> {
    w: &'a mut W,
}
impl<'a> MDM_W<'a> {
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
#[doc = "Reader of field `CPERAM`"]
pub type CPERAM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPERAM`"]
pub struct CPERAM_W<'a> {
    w: &'a mut W,
}
impl<'a> CPERAM_W<'a> {
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
#[doc = "Reader of field `CPE`"]
pub type CPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPE`"]
pub struct CPE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPE_W<'a> {
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
#[doc = "Reader of field `RFC`"]
pub type RFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFC`"]
pub struct RFC_W<'a> {
    w: &'a mut W,
}
impl<'a> RFC_W<'a> {
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
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> RESERVED11_R {
        RESERVED11_R::new(((self.bits >> 11) & 0x001f_ffff) as u32)
    }
    #[doc = "Bit 10 - 10:10\\]
Enable clock to the RF Core Tracer (RFCTRC) module."]
    #[inline(always)]
    pub fn rfctrc(&self) -> RFCTRC_R {
        RFCTRC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable clock to the Frequency Synthesizer Calibration Accelerator (FSCA) module."]
    #[inline(always)]
    pub fn fsca(&self) -> FSCA_R {
        FSCA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable clock to the Packet Handling Accelerator (PHA) module."]
    #[inline(always)]
    pub fn pha(&self) -> PHA_R {
        PHA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Enable clock to the Radio Timer (RAT) module."]
    #[inline(always)]
    pub fn rat(&self) -> RAT_R {
        RAT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Enable clock to the RF Engine RAM module."]
    #[inline(always)]
    pub fn rferam(&self) -> RFERAM_R {
        RFERAM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Enable clock to the RF Engine (RFE) module."]
    #[inline(always)]
    pub fn rfe(&self) -> RFE_R {
        RFE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Enable clock to the Modem RAM module."]
    #[inline(always)]
    pub fn mdmram(&self) -> MDMRAM_R {
        MDMRAM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Enable clock to the Modem (MDM) module."]
    #[inline(always)]
    pub fn mdm(&self) -> MDM_R {
        MDM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable clock to the Command and Packet Engine (CPE) RAM module. As part of RF Core initialization, set this bit together with CPE bit to enable CPE to boot."]
    #[inline(always)]
    pub fn cperam(&self) -> CPERAM_R {
        CPERAM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable processor clock (hclk) to the Command and Packet Engine (CPE). As part of RF Core initialization, set this bit together with CPERAM bit to enable CPE to boot."]
    #[inline(always)]
    pub fn cpe(&self) -> CPE_R {
        CPE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Enable essential clocks for the RF Core interface. This includes the interconnect, the radio doorbell DBELL command interface, the power management (PWR) clock control module, and bus clock (sclk) for the CPE. To remove possibility of locking yourself out from the RF Core, this bit can not be cleared. If you need to disable all clocks to the RF Core, see the PRCM:RFCCLKG.CLK_EN register."]
    #[inline(always)]
    pub fn rfc(&self) -> RFC_R {
        RFC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&mut self) -> RESERVED11_W {
        RESERVED11_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Enable clock to the RF Core Tracer (RFCTRC) module."]
    #[inline(always)]
    pub fn rfctrc(&mut self) -> RFCTRC_W {
        RFCTRC_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Enable clock to the Frequency Synthesizer Calibration Accelerator (FSCA) module."]
    #[inline(always)]
    pub fn fsca(&mut self) -> FSCA_W {
        FSCA_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Enable clock to the Packet Handling Accelerator (PHA) module."]
    #[inline(always)]
    pub fn pha(&mut self) -> PHA_W {
        PHA_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Enable clock to the Radio Timer (RAT) module."]
    #[inline(always)]
    pub fn rat(&mut self) -> RAT_W {
        RAT_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Enable clock to the RF Engine RAM module."]
    #[inline(always)]
    pub fn rferam(&mut self) -> RFERAM_W {
        RFERAM_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Enable clock to the RF Engine (RFE) module."]
    #[inline(always)]
    pub fn rfe(&mut self) -> RFE_W {
        RFE_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Enable clock to the Modem RAM module."]
    #[inline(always)]
    pub fn mdmram(&mut self) -> MDMRAM_W {
        MDMRAM_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Enable clock to the Modem (MDM) module."]
    #[inline(always)]
    pub fn mdm(&mut self) -> MDM_W {
        MDM_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Enable clock to the Command and Packet Engine (CPE) RAM module. As part of RF Core initialization, set this bit together with CPE bit to enable CPE to boot."]
    #[inline(always)]
    pub fn cperam(&mut self) -> CPERAM_W {
        CPERAM_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Enable processor clock (hclk) to the Command and Packet Engine (CPE). As part of RF Core initialization, set this bit together with CPERAM bit to enable CPE to boot."]
    #[inline(always)]
    pub fn cpe(&mut self) -> CPE_W {
        CPE_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Enable essential clocks for the RF Core interface. This includes the interconnect, the radio doorbell DBELL command interface, the power management (PWR) clock control module, and bus clock (sclk) for the CPE. To remove possibility of locking yourself out from the RF Core, this bit can not be cleared. If you need to disable all clocks to the RF Core, see the PRCM:RFCCLKG.CLK_EN register."]
    #[inline(always)]
    pub fn rfc(&mut self) -> RFC_W {
        RFC_W { w: self }
    }
}
