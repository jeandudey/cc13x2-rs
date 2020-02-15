#[doc = "Reader of register HWVER0"]
pub type R = crate::R<u32, super::HWVER0>;
#[doc = "Writer for register HWVER0"]
pub type W = crate::W<u32, super::HWVER0>;
#[doc = "Register HWVER0 `reset()`'s with value 0x0200_b44b"]
impl crate::ResetValue for super::HWVER0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200_b44b
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
#[doc = "Reader of field `EIP_NUM_COMPL`"]
pub type EIP_NUM_COMPL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EIP_NUM_COMPL`"]
pub struct EIP_NUM_COMPL_W<'a> {
    w: &'a mut W,
}
impl<'a> EIP_NUM_COMPL_W<'a> {
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
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved28(&self) -> RESERVED28_R {
        RESERVED28_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
4 bits binary encoding of the major hardware revision number."]
    #[inline(always)]
    pub fn hw_major_ver(&self) -> HW_MAJOR_VER_R {
        HW_MAJOR_VER_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
4 bits binary encoding of the minor hardware revision number."]
    #[inline(always)]
    pub fn hw_minor_ver(&self) -> HW_MINOR_VER_R {
        HW_MINOR_VER_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
4 bits binary encoding of the hardware patch level, initial release will carry value zero."]
    #[inline(always)]
    pub fn hw_patch_lvl(&self) -> HW_PATCH_LVL_R {
        HW_PATCH_LVL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Bit-by-bit logic complement of bits \\[7:0\\]. This TRNG gives 0xB4."]
    #[inline(always)]
    pub fn eip_num_compl(&self) -> EIP_NUM_COMPL_R {
        EIP_NUM_COMPL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
8 bits binary encoding of the module number. This TRNG gives 0x4B."]
    #[inline(always)]
    pub fn eip_num(&self) -> EIP_NUM_R {
        EIP_NUM_R::new((self.bits & 0xff) as u8)
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
4 bits binary encoding of the major hardware revision number."]
    #[inline(always)]
    pub fn hw_major_ver(&mut self) -> HW_MAJOR_VER_W {
        HW_MAJOR_VER_W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\]
4 bits binary encoding of the minor hardware revision number."]
    #[inline(always)]
    pub fn hw_minor_ver(&mut self) -> HW_MINOR_VER_W {
        HW_MINOR_VER_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\]
4 bits binary encoding of the hardware patch level, initial release will carry value zero."]
    #[inline(always)]
    pub fn hw_patch_lvl(&mut self) -> HW_PATCH_LVL_W {
        HW_PATCH_LVL_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Bit-by-bit logic complement of bits \\[7:0\\]. This TRNG gives 0xB4."]
    #[inline(always)]
    pub fn eip_num_compl(&mut self) -> EIP_NUM_COMPL_W {
        EIP_NUM_COMPL_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
8 bits binary encoding of the module number. This TRNG gives 0x4B."]
    #[inline(always)]
    pub fn eip_num(&mut self) -> EIP_NUM_W {
        EIP_NUM_W { w: self }
    }
}
