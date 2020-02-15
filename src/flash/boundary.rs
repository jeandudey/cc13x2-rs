#[doc = "Reader of register BOUNDARY"]
pub type R = crate::R<u32, super::BOUNDARY>;
#[doc = "Writer for register BOUNDARY"]
pub type W = crate::W<u32, super::BOUNDARY>;
#[doc = "Register BOUNDARY `reset()`'s with value 0"]
impl crate::ResetValue for super::BOUNDARY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED24`"]
pub type RESERVED24_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED24`"]
pub struct RESERVED24_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `DISROW0`"]
pub type DISROW0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISROW0`"]
pub struct DISROW0_W<'a> {
    w: &'a mut W,
}
impl<'a> DISROW0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `SPARE`"]
pub type SPARE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPARE`"]
pub struct SPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `EFC_SELF_TEST_ERROR`"]
pub type EFC_SELF_TEST_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFC_SELF_TEST_ERROR`"]
pub struct EFC_SELF_TEST_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> EFC_SELF_TEST_ERROR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `EFC_INSTRUCTION_INFO`"]
pub type EFC_INSTRUCTION_INFO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFC_INSTRUCTION_INFO`"]
pub struct EFC_INSTRUCTION_INFO_W<'a> {
    w: &'a mut W,
}
impl<'a> EFC_INSTRUCTION_INFO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `EFC_INSTRUCTION_ERROR`"]
pub type EFC_INSTRUCTION_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFC_INSTRUCTION_ERROR`"]
pub struct EFC_INSTRUCTION_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> EFC_INSTRUCTION_ERROR_W<'a> {
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
#[doc = "Reader of field `EFC_AUTOLOAD_ERROR`"]
pub type EFC_AUTOLOAD_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFC_AUTOLOAD_ERROR`"]
pub struct EFC_AUTOLOAD_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> EFC_AUTOLOAD_ERROR_W<'a> {
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
#[doc = "Reader of field `OUTPUTENABLE`"]
pub type OUTPUTENABLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUTPUTENABLE`"]
pub struct OUTPUTENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTPUTENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 14)) | (((value as u32) & 0x0f) << 14);
        self.w
    }
}
#[doc = "Reader of field `SYS_ECC_SELF_TEST_EN`"]
pub type SYS_ECC_SELF_TEST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYS_ECC_SELF_TEST_EN`"]
pub struct SYS_ECC_SELF_TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_ECC_SELF_TEST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `SYS_ECC_OVERRIDE_EN`"]
pub type SYS_ECC_OVERRIDE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYS_ECC_OVERRIDE_EN`"]
pub struct SYS_ECC_OVERRIDE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_ECC_OVERRIDE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `EFC_FDI`"]
pub type EFC_FDI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFC_FDI`"]
pub struct EFC_FDI_W<'a> {
    w: &'a mut W,
}
impl<'a> EFC_FDI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `SYS_DIEID_AUTOLOAD_EN`"]
pub type SYS_DIEID_AUTOLOAD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYS_DIEID_AUTOLOAD_EN`"]
pub struct SYS_DIEID_AUTOLOAD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_DIEID_AUTOLOAD_EN_W<'a> {
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
#[doc = "Reader of field `SYS_REPAIR_EN`"]
pub type SYS_REPAIR_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYS_REPAIR_EN`"]
pub struct SYS_REPAIR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_REPAIR_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `SYS_WS_READ_STATES`"]
pub type SYS_WS_READ_STATES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYS_WS_READ_STATES`"]
pub struct SYS_WS_READ_STATES_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_WS_READ_STATES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `INPUTENABLE`"]
pub type INPUTENABLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INPUTENABLE`"]
pub struct INPUTENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUTENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn disrow0(&self) -> DISROW0_R {
        DISROW0_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_self_test_error(&self) -> EFC_SELF_TEST_ERROR_R {
        EFC_SELF_TEST_ERROR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_instruction_info(&self) -> EFC_INSTRUCTION_INFO_R {
        EFC_INSTRUCTION_INFO_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_instruction_error(&self) -> EFC_INSTRUCTION_ERROR_R {
        EFC_INSTRUCTION_ERROR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_autoload_error(&self) -> EFC_AUTOLOAD_ERROR_R {
        EFC_AUTOLOAD_ERROR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 14:17 - 17:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn outputenable(&self) -> OUTPUTENABLE_R {
        OUTPUTENABLE_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_ecc_self_test_en(&self) -> SYS_ECC_SELF_TEST_EN_R {
        SYS_ECC_SELF_TEST_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_ecc_override_en(&self) -> SYS_ECC_OVERRIDE_EN_R {
        SYS_ECC_OVERRIDE_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_fdi(&self) -> EFC_FDI_R {
        EFC_FDI_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_dieid_autoload_en(&self) -> SYS_DIEID_AUTOLOAD_EN_R {
        SYS_DIEID_AUTOLOAD_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_repair_en(&self) -> SYS_REPAIR_EN_R {
        SYS_REPAIR_EN_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_ws_read_states(&self) -> SYS_WS_READ_STATES_R {
        SYS_WS_READ_STATES_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn inputenable(&self) -> INPUTENABLE_R {
        INPUTENABLE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved24(&mut self) -> RESERVED24_W {
        RESERVED24_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn disrow0(&mut self) -> DISROW0_W {
        DISROW0_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn spare(&mut self) -> SPARE_W {
        SPARE_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_self_test_error(&mut self) -> EFC_SELF_TEST_ERROR_W {
        EFC_SELF_TEST_ERROR_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_instruction_info(&mut self) -> EFC_INSTRUCTION_INFO_W {
        EFC_INSTRUCTION_INFO_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_instruction_error(&mut self) -> EFC_INSTRUCTION_ERROR_W {
        EFC_INSTRUCTION_ERROR_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_autoload_error(&mut self) -> EFC_AUTOLOAD_ERROR_W {
        EFC_AUTOLOAD_ERROR_W { w: self }
    }
    #[doc = "Bits 14:17 - 17:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn outputenable(&mut self) -> OUTPUTENABLE_W {
        OUTPUTENABLE_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_ecc_self_test_en(&mut self) -> SYS_ECC_SELF_TEST_EN_W {
        SYS_ECC_SELF_TEST_EN_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_ecc_override_en(&mut self) -> SYS_ECC_OVERRIDE_EN_W {
        SYS_ECC_OVERRIDE_EN_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_fdi(&mut self) -> EFC_FDI_W {
        EFC_FDI_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_dieid_autoload_en(&mut self) -> SYS_DIEID_AUTOLOAD_EN_W {
        SYS_DIEID_AUTOLOAD_EN_W { w: self }
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_repair_en(&mut self) -> SYS_REPAIR_EN_W {
        SYS_REPAIR_EN_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_ws_read_states(&mut self) -> SYS_WS_READ_STATES_W {
        SYS_WS_READ_STATES_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn inputenable(&mut self) -> INPUTENABLE_W {
        INPUTENABLE_W { w: self }
    }
}
