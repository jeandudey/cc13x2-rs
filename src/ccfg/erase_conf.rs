#[doc = "Reader of register ERASE_CONF"]
pub type R = crate::R<u32, super::ERASE_CONF>;
#[doc = "Writer for register ERASE_CONF"]
pub type W = crate::W<u32, super::ERASE_CONF>;
#[doc = "Register ERASE_CONF `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::ERASE_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
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
        self.w.bits = (self.w.bits & !(0x007f_ffff << 9)) | (((value as u32) & 0x007f_ffff) << 9);
        self.w
    }
}
#[doc = "Reader of field `CHIP_ERASE_DIS_N`"]
pub type CHIP_ERASE_DIS_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHIP_ERASE_DIS_N`"]
pub struct CHIP_ERASE_DIS_N_W<'a> {
    w: &'a mut W,
}
impl<'a> CHIP_ERASE_DIS_N_W<'a> {
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
#[doc = "Reader of field `RESERVED1`"]
pub type RESERVED1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u32) & 0x7f) << 1);
        self.w
    }
}
#[doc = "Reader of field `BANK_ERASE_DIS_N`"]
pub type BANK_ERASE_DIS_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BANK_ERASE_DIS_N`"]
pub struct BANK_ERASE_DIS_N_W<'a> {
    w: &'a mut W,
}
impl<'a> BANK_ERASE_DIS_N_W<'a> {
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
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 9) & 0x007f_ffff) as u32)
    }
    #[doc = "Bit 8 - 8:8\\]
Chip erase. This bit controls if a chip erase requested through the JTAG WUC TAP will be ignored in a following boot caused by a reset of the MCU VD. A successful chip erase operation will force the content of the flash main bank back to the state as it was when delivered by TI. 0: Disable. Any chip erase request detected during boot will be ignored. 1: Enable. Any chip erase request detected during boot will be performed by the boot FW."]
    #[inline(always)]
    pub fn chip_erase_dis_n(&self) -> CHIP_ERASE_DIS_N_R {
        CHIP_ERASE_DIS_N_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 0 - 0:0\\]
Bank erase. This bit controls if the ROM serial boot loader will accept a received Bank Erase command (COMMAND_BANK_ERASE). A successful Bank Erase operation will erase all main bank sectors not protected by write protect configuration bits in CCFG. 0: Disable the boot loader bank erase function. 1: Enable the boot loader bank erase function."]
    #[inline(always)]
    pub fn bank_erase_dis_n(&self) -> BANK_ERASE_DIS_N_R {
        BANK_ERASE_DIS_N_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Chip erase. This bit controls if a chip erase requested through the JTAG WUC TAP will be ignored in a following boot caused by a reset of the MCU VD. A successful chip erase operation will force the content of the flash main bank back to the state as it was when delivered by TI. 0: Disable. Any chip erase request detected during boot will be ignored. 1: Enable. Any chip erase request detected during boot will be performed by the boot FW."]
    #[inline(always)]
    pub fn chip_erase_dis_n(&mut self) -> CHIP_ERASE_DIS_N_W {
        CHIP_ERASE_DIS_N_W { w: self }
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Bank erase. This bit controls if the ROM serial boot loader will accept a received Bank Erase command (COMMAND_BANK_ERASE). A successful Bank Erase operation will erase all main bank sectors not protected by write protect configuration bits in CCFG. 0: Disable the boot loader bank erase function. 1: Enable the boot loader bank erase function."]
    #[inline(always)]
    pub fn bank_erase_dis_n(&mut self) -> BANK_ERASE_DIS_N_W {
        BANK_ERASE_DIS_N_W { w: self }
    }
}
