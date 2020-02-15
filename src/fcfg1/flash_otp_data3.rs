#[doc = "Reader of register FLASH_OTP_DATA3"]
pub type R = crate::R<u32, super::FLASH_OTP_DATA3>;
#[doc = "Writer for register FLASH_OTP_DATA3"]
pub type W = crate::W<u32, super::FLASH_OTP_DATA3>;
#[doc = "Register FLASH_OTP_DATA3 `reset()`'s with value 0x0011_0003"]
impl crate::ResetValue for super::FLASH_OTP_DATA3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0011_0003
    }
}
#[doc = "Reader of field `EC_STEP_SIZE`"]
pub type EC_STEP_SIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EC_STEP_SIZE`"]
pub struct EC_STEP_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> EC_STEP_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 23)) | (((value as u32) & 0x01ff) << 23);
        self.w
    }
}
#[doc = "Reader of field `DO_PRECOND`"]
pub type DO_PRECOND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DO_PRECOND`"]
pub struct DO_PRECOND_W<'a> {
    w: &'a mut W,
}
impl<'a> DO_PRECOND_W<'a> {
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
#[doc = "Reader of field `MAX_EC_LEVEL`"]
pub type MAX_EC_LEVEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAX_EC_LEVEL`"]
pub struct MAX_EC_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_EC_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | (((value as u32) & 0x0f) << 18);
        self.w
    }
}
#[doc = "Reader of field `TRIM_1P7`"]
pub type TRIM_1P7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIM_1P7`"]
pub struct TRIM_1P7_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_1P7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `FLASH_SIZE`"]
pub type FLASH_SIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FLASH_SIZE`"]
pub struct FLASH_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `WAIT_SYSCODE`"]
pub type WAIT_SYSCODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WAIT_SYSCODE`"]
pub struct WAIT_SYSCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_SYSCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 23:31 - 31:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ec_step_size(&self) -> EC_STEP_SIZE_R {
        EC_STEP_SIZE_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn do_precond(&self) -> DO_PRECOND_R {
        DO_PRECOND_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 18:21 - 21:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn max_ec_level(&self) -> MAX_EC_LEVEL_R {
        MAX_EC_LEVEL_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim_1p7(&self) -> TRIM_1P7_R {
        TRIM_1P7_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn flash_size(&self) -> FLASH_SIZE_R {
        FLASH_SIZE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn wait_syscode(&self) -> WAIT_SYSCODE_R {
        WAIT_SYSCODE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 23:31 - 31:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ec_step_size(&mut self) -> EC_STEP_SIZE_W {
        EC_STEP_SIZE_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn do_precond(&mut self) -> DO_PRECOND_W {
        DO_PRECOND_W { w: self }
    }
    #[doc = "Bits 18:21 - 21:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn max_ec_level(&mut self) -> MAX_EC_LEVEL_W {
        MAX_EC_LEVEL_W { w: self }
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim_1p7(&mut self) -> TRIM_1P7_W {
        TRIM_1P7_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn flash_size(&mut self) -> FLASH_SIZE_W {
        FLASH_SIZE_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn wait_syscode(&mut self) -> WAIT_SYSCODE_W {
        WAIT_SYSCODE_W { w: self }
    }
}
