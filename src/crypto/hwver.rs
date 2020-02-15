#[doc = "Reader of register HWVER"]
pub type R = crate::R<u32, super::HWVER>;
#[doc = "Writer for register HWVER"]
pub type W = crate::W<u32, super::HWVER>;
#[doc = "Register HWVER `reset()`'s with value 0x9200_8778"]
impl crate::ResetValue for super::HWVER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x9200_8778
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
#[doc = "Reader of field `HW_MAJOR_VER`"]
pub type HW_MAJOR_VER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HW_MAJOR_VER`"]
pub struct HW_MAJOR_VER_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_MAJOR_VER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `HW_MINOR_VER`"]
pub type HW_MINOR_VER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HW_MINOR_VER`"]
pub struct HW_MINOR_VER_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_MINOR_VER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `HW_PATCH_LVL`"]
pub type HW_PATCH_LVL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HW_PATCH_LVL`"]
pub struct HW_PATCH_LVL_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_PATCH_LVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `VER_NUM_COMPL`"]
pub type VER_NUM_COMPL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VER_NUM_COMPL`"]
pub struct VER_NUM_COMPL_W<'a> {
    w: &'a mut W,
}
impl<'a> VER_NUM_COMPL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `VER_NUM`"]
pub type VER_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VER_NUM`"]
pub struct VER_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> VER_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - 31:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved28(&self) -> RESERVED28_R {
        RESERVED28_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Major version number"]
    #[inline(always)]
    pub fn hw_major_ver(&self) -> HW_MAJOR_VER_R {
        HW_MAJOR_VER_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Minor version number"]
    #[inline(always)]
    pub fn hw_minor_ver(&self) -> HW_MINOR_VER_R {
        HW_MINOR_VER_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Patch level Starts at 0 at first delivery of this version"]
    #[inline(always)]
    pub fn hw_patch_lvl(&self) -> HW_PATCH_LVL_R {
        HW_PATCH_LVL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
These bits simply contain the complement of bits \\[7:0\\]
(0x87), used by a driver to ascertain that the EIP-120t register is indeed read."]
    #[inline(always)]
    pub fn ver_num_compl(&self) -> VER_NUM_COMPL_R {
        VER_NUM_COMPL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
These bits encode the EIP number for the EIP-120t, this field contains the value 120 (decimal) or 0x78."]
    #[inline(always)]
    pub fn ver_num(&self) -> VER_NUM_R {
        VER_NUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - 31:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved28(&mut self) -> RESERVED28_W {
        RESERVED28_W { w: self }
    }
    #[doc = "Bits 24:27 - 27:24\\]
Major version number"]
    #[inline(always)]
    pub fn hw_major_ver(&mut self) -> HW_MAJOR_VER_W {
        HW_MAJOR_VER_W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\]
Minor version number"]
    #[inline(always)]
    pub fn hw_minor_ver(&mut self) -> HW_MINOR_VER_W {
        HW_MINOR_VER_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\]
Patch level Starts at 0 at first delivery of this version"]
    #[inline(always)]
    pub fn hw_patch_lvl(&mut self) -> HW_PATCH_LVL_W {
        HW_PATCH_LVL_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
These bits simply contain the complement of bits \\[7:0\\]
(0x87), used by a driver to ascertain that the EIP-120t register is indeed read."]
    #[inline(always)]
    pub fn ver_num_compl(&mut self) -> VER_NUM_COMPL_W {
        VER_NUM_COMPL_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
These bits encode the EIP number for the EIP-120t, this field contains the value 120 (decimal) or 0x78."]
    #[inline(always)]
    pub fn ver_num(&mut self) -> VER_NUM_W {
        VER_NUM_W { w: self }
    }
}
