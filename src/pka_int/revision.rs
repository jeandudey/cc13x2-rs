#[doc = "Reader of register REVISION"]
pub type R = crate::R<u32, super::REVISION>;
#[doc = "Writer for register REVISION"]
pub type W = crate::W<u32, super::REVISION>;
#[doc = "Register REVISION `reset()`'s with value 0x0200_6996"]
impl crate::ResetValue for super::REVISION {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200_6996
    }
}
#[doc = "Reader of field `RESERVED28`"]
pub type RESERVED28_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED28`"]
pub struct RESERVED28_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED28_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `MAJOR_REVISION`"]
pub type MAJOR_REVISION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAJOR_REVISION`"]
pub struct MAJOR_REVISION_W<'a> {
    w: &'a mut W,
}
impl<'a> MAJOR_REVISION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `MINOR_REVISION`"]
pub type MINOR_REVISION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MINOR_REVISION`"]
pub struct MINOR_REVISION_W<'a> {
    w: &'a mut W,
}
impl<'a> MINOR_REVISION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `PATCH_LEVEL`"]
pub type PATCH_LEVEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PATCH_LEVEL`"]
pub struct PATCH_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PATCH_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `COMP_EIP_NUM`"]
pub type COMP_EIP_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMP_EIP_NUM`"]
pub struct COMP_EIP_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_EIP_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `EIP_NUM`"]
pub type EIP_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EIP_NUM`"]
pub struct EIP_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> EIP_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - 31:28\\]
These bits should be ignored on read"]
    #[inline(always)]
    pub fn reserved28(&self) -> RESERVED28_R {
        RESERVED28_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
These bits encode the major version number for this module"]
    #[inline(always)]
    pub fn major_revision(&self) -> MAJOR_REVISION_R {
        MAJOR_REVISION_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
These bits encode the minor version number for this module"]
    #[inline(always)]
    pub fn minor_revision(&self) -> MINOR_REVISION_R {
        MINOR_REVISION_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
These bits encode the hardware patch level for this module they start at value 0 on the first release"]
    #[inline(always)]
    pub fn patch_level(&self) -> PATCH_LEVEL_R {
        PATCH_LEVEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
These bits simply contain the complement of bits \\[7:0\\], used by a driver to ascertain that the EIP150 revision register is indeed read"]
    #[inline(always)]
    pub fn comp_eip_num(&self) -> COMP_EIP_NUM_R {
        COMP_EIP_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
These bits encode the AuthenTec EIP number for the EIP150"]
    #[inline(always)]
    pub fn eip_num(&self) -> EIP_NUM_R {
        EIP_NUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - 31:28\\]
These bits should be ignored on read"]
    #[inline(always)]
    pub fn reserved28(&mut self) -> RESERVED28_W {
        RESERVED28_W { w: self }
    }
    #[doc = "Bits 24:27 - 27:24\\]
These bits encode the major version number for this module"]
    #[inline(always)]
    pub fn major_revision(&mut self) -> MAJOR_REVISION_W {
        MAJOR_REVISION_W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\]
These bits encode the minor version number for this module"]
    #[inline(always)]
    pub fn minor_revision(&mut self) -> MINOR_REVISION_W {
        MINOR_REVISION_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\]
These bits encode the hardware patch level for this module they start at value 0 on the first release"]
    #[inline(always)]
    pub fn patch_level(&mut self) -> PATCH_LEVEL_W {
        PATCH_LEVEL_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
These bits simply contain the complement of bits \\[7:0\\], used by a driver to ascertain that the EIP150 revision register is indeed read"]
    #[inline(always)]
    pub fn comp_eip_num(&mut self) -> COMP_EIP_NUM_W {
        COMP_EIP_NUM_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
These bits encode the AuthenTec EIP number for the EIP150"]
    #[inline(always)]
    pub fn eip_num(&mut self) -> EIP_NUM_W {
        EIP_NUM_W { w: self }
    }
}
