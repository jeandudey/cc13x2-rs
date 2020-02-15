#[doc = "Reader of register FWREV"]
pub type R = crate::R<u32, super::FWREV>;
#[doc = "Writer for register FWREV"]
pub type W = crate::W<u32, super::FWREV>;
#[doc = "Register FWREV `reset()`'s with value 0x2150_0000"]
impl crate::ResetValue for super::FWREV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2150_0000
    }
}
#[doc = "Reader of field `FW_CAPABILITIES`"]
pub type FW_CAPABILITIES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FW_CAPABILITIES`"]
pub struct FW_CAPABILITIES_W<'a> {
    w: &'a mut W,
}
impl<'a> FW_CAPABILITIES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `MAJOR_FW_REVISION`"]
pub type MAJOR_FW_REVISION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAJOR_FW_REVISION`"]
pub struct MAJOR_FW_REVISION_W<'a> {
    w: &'a mut W,
}
impl<'a> MAJOR_FW_REVISION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `MINOR_FW_REVISION`"]
pub type MINOR_FW_REVISION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MINOR_FW_REVISION`"]
pub struct MINOR_FW_REVISION_W<'a> {
    w: &'a mut W,
}
impl<'a> MINOR_FW_REVISION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `FW_PATCH_LEVEL`"]
pub type FW_PATCH_LEVEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FW_PATCH_LEVEL`"]
pub struct FW_PATCH_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FW_PATCH_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `RESERVED0`"]
pub type RESERVED0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED0`"]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - 31:28\\]
Firmware Capabilities 4-bit binary encoding for the functionality implemented in the firmware. 0x0: indicates basic ModExp with/without CRT. 0x1: adds Modular Inversion, 0x2: value 2 adds Modular Inversion and ECC operations. 0x3-0xF : Reserved."]
    #[inline(always)]
    pub fn fw_capabilities(&self) -> FW_CAPABILITIES_R {
        FW_CAPABILITIES_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
4-bit binary encoding of the major firmware revision number"]
    #[inline(always)]
    pub fn major_fw_revision(&self) -> MAJOR_FW_REVISION_R {
        MAJOR_FW_REVISION_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
4-bit binary encoding of the minor firmware revision number"]
    #[inline(always)]
    pub fn minor_fw_revision(&self) -> MINOR_FW_REVISION_R {
        MINOR_FW_REVISION_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
4-bit binary encoding of the firmware patch level, initial release will carry value zero Patches are used to remove bugs without changing the functionality or interface of a module."]
    #[inline(always)]
    pub fn fw_patch_level(&self) -> FW_PATCH_LEVEL_R {
        FW_PATCH_LEVEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 0:15 - 15:0\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 28:31 - 31:28\\]
Firmware Capabilities 4-bit binary encoding for the functionality implemented in the firmware. 0x0: indicates basic ModExp with/without CRT. 0x1: adds Modular Inversion, 0x2: value 2 adds Modular Inversion and ECC operations. 0x3-0xF : Reserved."]
    #[inline(always)]
    pub fn fw_capabilities(&mut self) -> FW_CAPABILITIES_W {
        FW_CAPABILITIES_W { w: self }
    }
    #[doc = "Bits 24:27 - 27:24\\]
4-bit binary encoding of the major firmware revision number"]
    #[inline(always)]
    pub fn major_fw_revision(&mut self) -> MAJOR_FW_REVISION_W {
        MAJOR_FW_REVISION_W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\]
4-bit binary encoding of the minor firmware revision number"]
    #[inline(always)]
    pub fn minor_fw_revision(&mut self) -> MINOR_FW_REVISION_W {
        MINOR_FW_REVISION_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\]
4-bit binary encoding of the firmware patch level, initial release will carry value zero Patches are used to remove bugs without changing the functionality or interface of a module."]
    #[inline(always)]
    pub fn fw_patch_level(&mut self) -> FW_PATCH_LEVEL_W {
        FW_PATCH_LEVEL_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
}
