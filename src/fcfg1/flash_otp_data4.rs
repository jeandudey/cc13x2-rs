#[doc = "Reader of register FLASH_OTP_DATA4"]
pub type R = crate::R<u32, super::FLASH_OTP_DATA4>;
#[doc = "Writer for register FLASH_OTP_DATA4"]
pub type W = crate::W<u32, super::FLASH_OTP_DATA4>;
#[doc = "Register FLASH_OTP_DATA4 `reset()`'s with value 0x9898_9f9f"]
impl crate::ResetValue for super::FLASH_OTP_DATA4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x9898_9f9f
    }
}
#[doc = "Reader of field `STANDBY_MODE_SEL_INT_WRT`"]
pub type STANDBY_MODE_SEL_INT_WRT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STANDBY_MODE_SEL_INT_WRT`"]
pub struct STANDBY_MODE_SEL_INT_WRT_W<'a> {
    w: &'a mut W,
}
impl<'a> STANDBY_MODE_SEL_INT_WRT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `STANDBY_PW_SEL_INT_WRT`"]
pub type STANDBY_PW_SEL_INT_WRT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STANDBY_PW_SEL_INT_WRT`"]
pub struct STANDBY_PW_SEL_INT_WRT_W<'a> {
    w: &'a mut W,
}
impl<'a> STANDBY_PW_SEL_INT_WRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
#[doc = "Reader of field `DIS_STANDBY_INT_WRT`"]
pub type DIS_STANDBY_INT_WRT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_STANDBY_INT_WRT`"]
pub struct DIS_STANDBY_INT_WRT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_STANDBY_INT_WRT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `DIS_IDLE_INT_WRT`"]
pub type DIS_IDLE_INT_WRT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_IDLE_INT_WRT`"]
pub struct DIS_IDLE_INT_WRT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_IDLE_INT_WRT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `VIN_AT_X_INT_WRT`"]
pub type VIN_AT_X_INT_WRT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VIN_AT_X_INT_WRT`"]
pub struct VIN_AT_X_INT_WRT_W<'a> {
    w: &'a mut W,
}
impl<'a> VIN_AT_X_INT_WRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `STANDBY_MODE_SEL_EXT_WRT`"]
pub type STANDBY_MODE_SEL_EXT_WRT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STANDBY_MODE_SEL_EXT_WRT`"]
pub struct STANDBY_MODE_SEL_EXT_WRT_W<'a> {
    w: &'a mut W,
}
impl<'a> STANDBY_MODE_SEL_EXT_WRT_W<'a> {
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
#[doc = "Reader of field `STANDBY_PW_SEL_EXT_WRT`"]
pub type STANDBY_PW_SEL_EXT_WRT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STANDBY_PW_SEL_EXT_WRT`"]
pub struct STANDBY_PW_SEL_EXT_WRT_W<'a> {
    w: &'a mut W,
}
impl<'a> STANDBY_PW_SEL_EXT_WRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "Reader of field `DIS_STANDBY_EXT_WRT`"]
pub type DIS_STANDBY_EXT_WRT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_STANDBY_EXT_WRT`"]
pub struct DIS_STANDBY_EXT_WRT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_STANDBY_EXT_WRT_W<'a> {
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
#[doc = "Reader of field `DIS_IDLE_EXT_WRT`"]
pub type DIS_IDLE_EXT_WRT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_IDLE_EXT_WRT`"]
pub struct DIS_IDLE_EXT_WRT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_IDLE_EXT_WRT_W<'a> {
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
#[doc = "Reader of field `VIN_AT_X_EXT_WRT`"]
pub type VIN_AT_X_EXT_WRT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VIN_AT_X_EXT_WRT`"]
pub struct VIN_AT_X_EXT_WRT_W<'a> {
    w: &'a mut W,
}
impl<'a> VIN_AT_X_EXT_WRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `STANDBY_MODE_SEL_INT_RD`"]
pub type STANDBY_MODE_SEL_INT_RD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STANDBY_MODE_SEL_INT_RD`"]
pub struct STANDBY_MODE_SEL_INT_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> STANDBY_MODE_SEL_INT_RD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `STANDBY_PW_SEL_INT_RD`"]
pub type STANDBY_PW_SEL_INT_RD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STANDBY_PW_SEL_INT_RD`"]
pub struct STANDBY_PW_SEL_INT_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> STANDBY_PW_SEL_INT_RD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `DIS_STANDBY_INT_RD`"]
pub type DIS_STANDBY_INT_RD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_STANDBY_INT_RD`"]
pub struct DIS_STANDBY_INT_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_STANDBY_INT_RD_W<'a> {
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
#[doc = "Reader of field `DIS_IDLE_INT_RD`"]
pub type DIS_IDLE_INT_RD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_IDLE_INT_RD`"]
pub struct DIS_IDLE_INT_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_IDLE_INT_RD_W<'a> {
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
#[doc = "Reader of field `VIN_AT_X_INT_RD`"]
pub type VIN_AT_X_INT_RD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VIN_AT_X_INT_RD`"]
pub struct VIN_AT_X_INT_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> VIN_AT_X_INT_RD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `STANDBY_MODE_SEL_EXT_RD`"]
pub type STANDBY_MODE_SEL_EXT_RD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STANDBY_MODE_SEL_EXT_RD`"]
pub struct STANDBY_MODE_SEL_EXT_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> STANDBY_MODE_SEL_EXT_RD_W<'a> {
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
#[doc = "Reader of field `STANDBY_PW_SEL_EXT_RD`"]
pub type STANDBY_PW_SEL_EXT_RD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STANDBY_PW_SEL_EXT_RD`"]
pub struct STANDBY_PW_SEL_EXT_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> STANDBY_PW_SEL_EXT_RD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `DIS_STANDBY_EXT_RD`"]
pub type DIS_STANDBY_EXT_RD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_STANDBY_EXT_RD`"]
pub struct DIS_STANDBY_EXT_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_STANDBY_EXT_RD_W<'a> {
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
#[doc = "Reader of field `DIS_IDLE_EXT_RD`"]
pub type DIS_IDLE_EXT_RD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_IDLE_EXT_RD`"]
pub struct DIS_IDLE_EXT_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_IDLE_EXT_RD_W<'a> {
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
#[doc = "Reader of field `VIN_AT_X_EXT_RD`"]
pub type VIN_AT_X_EXT_RD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VIN_AT_X_EXT_RD`"]
pub struct VIN_AT_X_EXT_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> VIN_AT_X_EXT_RD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_mode_sel_int_wrt(&self) -> STANDBY_MODE_SEL_INT_WRT_R {
        STANDBY_MODE_SEL_INT_WRT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - 30:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_pw_sel_int_wrt(&self) -> STANDBY_PW_SEL_INT_WRT_R {
        STANDBY_PW_SEL_INT_WRT_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bit 28 - 28:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_standby_int_wrt(&self) -> DIS_STANDBY_INT_WRT_R {
        DIS_STANDBY_INT_WRT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_idle_int_wrt(&self) -> DIS_IDLE_INT_WRT_R {
        DIS_IDLE_INT_WRT_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_at_x_int_wrt(&self) -> VIN_AT_X_INT_WRT_R {
        VIN_AT_X_INT_WRT_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_mode_sel_ext_wrt(&self) -> STANDBY_MODE_SEL_EXT_WRT_R {
        STANDBY_MODE_SEL_EXT_WRT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - 22:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_pw_sel_ext_wrt(&self) -> STANDBY_PW_SEL_EXT_WRT_R {
        STANDBY_PW_SEL_EXT_WRT_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_standby_ext_wrt(&self) -> DIS_STANDBY_EXT_WRT_R {
        DIS_STANDBY_EXT_WRT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_idle_ext_wrt(&self) -> DIS_IDLE_EXT_WRT_R {
        DIS_IDLE_EXT_WRT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_at_x_ext_wrt(&self) -> VIN_AT_X_EXT_WRT_R {
        VIN_AT_X_EXT_WRT_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_mode_sel_int_rd(&self) -> STANDBY_MODE_SEL_INT_RD_R {
        STANDBY_MODE_SEL_INT_RD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - 14:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_pw_sel_int_rd(&self) -> STANDBY_PW_SEL_INT_RD_R {
        STANDBY_PW_SEL_INT_RD_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_standby_int_rd(&self) -> DIS_STANDBY_INT_RD_R {
        DIS_STANDBY_INT_RD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_idle_int_rd(&self) -> DIS_IDLE_INT_RD_R {
        DIS_IDLE_INT_RD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_at_x_int_rd(&self) -> VIN_AT_X_INT_RD_R {
        VIN_AT_X_INT_RD_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_mode_sel_ext_rd(&self) -> STANDBY_MODE_SEL_EXT_RD_R {
        STANDBY_MODE_SEL_EXT_RD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_pw_sel_ext_rd(&self) -> STANDBY_PW_SEL_EXT_RD_R {
        STANDBY_PW_SEL_EXT_RD_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_standby_ext_rd(&self) -> DIS_STANDBY_EXT_RD_R {
        DIS_STANDBY_EXT_RD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_idle_ext_rd(&self) -> DIS_IDLE_EXT_RD_R {
        DIS_IDLE_EXT_RD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_at_x_ext_rd(&self) -> VIN_AT_X_EXT_RD_R {
        VIN_AT_X_EXT_RD_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_mode_sel_int_wrt(&mut self) -> STANDBY_MODE_SEL_INT_WRT_W {
        STANDBY_MODE_SEL_INT_WRT_W { w: self }
    }
    #[doc = "Bits 29:30 - 30:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_pw_sel_int_wrt(&mut self) -> STANDBY_PW_SEL_INT_WRT_W {
        STANDBY_PW_SEL_INT_WRT_W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_standby_int_wrt(&mut self) -> DIS_STANDBY_INT_WRT_W {
        DIS_STANDBY_INT_WRT_W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_idle_int_wrt(&mut self) -> DIS_IDLE_INT_WRT_W {
        DIS_IDLE_INT_WRT_W { w: self }
    }
    #[doc = "Bits 24:26 - 26:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_at_x_int_wrt(&mut self) -> VIN_AT_X_INT_WRT_W {
        VIN_AT_X_INT_WRT_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_mode_sel_ext_wrt(&mut self) -> STANDBY_MODE_SEL_EXT_WRT_W {
        STANDBY_MODE_SEL_EXT_WRT_W { w: self }
    }
    #[doc = "Bits 21:22 - 22:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_pw_sel_ext_wrt(&mut self) -> STANDBY_PW_SEL_EXT_WRT_W {
        STANDBY_PW_SEL_EXT_WRT_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_standby_ext_wrt(&mut self) -> DIS_STANDBY_EXT_WRT_W {
        DIS_STANDBY_EXT_WRT_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_idle_ext_wrt(&mut self) -> DIS_IDLE_EXT_WRT_W {
        DIS_IDLE_EXT_WRT_W { w: self }
    }
    #[doc = "Bits 16:18 - 18:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_at_x_ext_wrt(&mut self) -> VIN_AT_X_EXT_WRT_W {
        VIN_AT_X_EXT_WRT_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_mode_sel_int_rd(&mut self) -> STANDBY_MODE_SEL_INT_RD_W {
        STANDBY_MODE_SEL_INT_RD_W { w: self }
    }
    #[doc = "Bits 13:14 - 14:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_pw_sel_int_rd(&mut self) -> STANDBY_PW_SEL_INT_RD_W {
        STANDBY_PW_SEL_INT_RD_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_standby_int_rd(&mut self) -> DIS_STANDBY_INT_RD_W {
        DIS_STANDBY_INT_RD_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_idle_int_rd(&mut self) -> DIS_IDLE_INT_RD_W {
        DIS_IDLE_INT_RD_W { w: self }
    }
    #[doc = "Bits 8:10 - 10:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_at_x_int_rd(&mut self) -> VIN_AT_X_INT_RD_W {
        VIN_AT_X_INT_RD_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_mode_sel_ext_rd(&mut self) -> STANDBY_MODE_SEL_EXT_RD_W {
        STANDBY_MODE_SEL_EXT_RD_W { w: self }
    }
    #[doc = "Bits 5:6 - 6:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_pw_sel_ext_rd(&mut self) -> STANDBY_PW_SEL_EXT_RD_W {
        STANDBY_PW_SEL_EXT_RD_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_standby_ext_rd(&mut self) -> DIS_STANDBY_EXT_RD_W {
        DIS_STANDBY_EXT_RD_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_idle_ext_rd(&mut self) -> DIS_IDLE_EXT_RD_W {
        DIS_IDLE_EXT_RD_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_at_x_ext_rd(&mut self) -> VIN_AT_X_EXT_RD_W {
        VIN_AT_X_EXT_RD_W { w: self }
    }
}
