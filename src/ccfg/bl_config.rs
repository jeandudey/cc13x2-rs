#[doc = "Reader of register BL_CONFIG"]
pub type R = crate::R<u32, super::BL_CONFIG>;
#[doc = "Writer for register BL_CONFIG"]
pub type W = crate::W<u32, super::BL_CONFIG>;
#[doc = "Register BL_CONFIG `reset()`'s with value 0xc5ff_ffff"]
impl crate::ResetValue for super::BL_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc5ff_ffff
    }
}
#[doc = "Reader of field `BOOTLOADER_ENABLE`"]
pub type BOOTLOADER_ENABLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BOOTLOADER_ENABLE`"]
pub struct BOOTLOADER_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOTLOADER_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `BL_LEVEL`"]
pub type BL_LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BL_LEVEL`"]
pub struct BL_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BL_LEVEL_W<'a> {
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
#[doc = "Reader of field `BL_PIN_NUMBER`"]
pub type BL_PIN_NUMBER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BL_PIN_NUMBER`"]
pub struct BL_PIN_NUMBER_W<'a> {
    w: &'a mut W,
}
impl<'a> BL_PIN_NUMBER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `BL_ENABLE`"]
pub type BL_ENABLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BL_ENABLE`"]
pub struct BL_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BL_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Bootloader enable. Boot loader can be accessed if IMAGE_VALID_CONF.IMAGE_VALID is non-zero or BL_ENABLE is enabled (and conditions for boot loader backdoor are met). 0xC5: Boot loader is enabled. Any other value: Boot loader is disabled."]
    #[inline(always)]
    pub fn bootloader_enable(&self) -> BOOTLOADER_ENABLE_R {
        BOOTLOADER_ENABLE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the active level of the selected DIO number BL_PIN_NUMBER if boot loader backdoor is enabled by the BL_ENABLE field. 0: Active low. 1: Active high."]
    #[inline(always)]
    pub fn bl_level(&self) -> BL_LEVEL_R {
        BL_LEVEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DIO number that is level checked if the boot loader backdoor is enabled by the BL_ENABLE field."]
    #[inline(always)]
    pub fn bl_pin_number(&self) -> BL_PIN_NUMBER_R {
        BL_PIN_NUMBER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Enables the boot loader backdoor. 0xC5: Boot loader backdoor is enabled. Any other value: Boot loader backdoor is disabled. NOTE! Boot loader must be enabled (see BOOTLOADER_ENABLE) if boot loader backdoor is enabled."]
    #[inline(always)]
    pub fn bl_enable(&self) -> BL_ENABLE_R {
        BL_ENABLE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Bootloader enable. Boot loader can be accessed if IMAGE_VALID_CONF.IMAGE_VALID is non-zero or BL_ENABLE is enabled (and conditions for boot loader backdoor are met). 0xC5: Boot loader is enabled. Any other value: Boot loader is disabled."]
    #[inline(always)]
    pub fn bootloader_enable(&mut self) -> BOOTLOADER_ENABLE_W {
        BOOTLOADER_ENABLE_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the active level of the selected DIO number BL_PIN_NUMBER if boot loader backdoor is enabled by the BL_ENABLE field. 0: Active low. 1: Active high."]
    #[inline(always)]
    pub fn bl_level(&mut self) -> BL_LEVEL_W {
        BL_LEVEL_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
DIO number that is level checked if the boot loader backdoor is enabled by the BL_ENABLE field."]
    #[inline(always)]
    pub fn bl_pin_number(&mut self) -> BL_PIN_NUMBER_W {
        BL_PIN_NUMBER_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Enables the boot loader backdoor. 0xC5: Boot loader backdoor is enabled. Any other value: Boot loader backdoor is disabled. NOTE! Boot loader must be enabled (see BOOTLOADER_ENABLE) if boot loader backdoor is enabled."]
    #[inline(always)]
    pub fn bl_enable(&mut self) -> BL_ENABLE_W {
        BL_ENABLE_W { w: self }
    }
}
