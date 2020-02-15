#[doc = "Reader of register MPU_RASR"]
pub type R = crate::R<u32, super::MPU_RASR>;
#[doc = "Writer for register MPU_RASR"]
pub type W = crate::W<u32, super::MPU_RASR>;
#[doc = "Register MPU_RASR `reset()`'s with value 0"]
impl crate::ResetValue for super::MPU_RASR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED29`"]
pub type RESERVED29_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED29`"]
pub struct RESERVED29_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED29_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
#[doc = "Reader of field `XN`"]
pub type XN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XN`"]
pub struct XN_W<'a> {
    w: &'a mut W,
}
impl<'a> XN_W<'a> {
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
#[doc = "Reader of field `RESERVED27`"]
pub type RESERVED27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED27`"]
pub struct RESERVED27_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED27_W<'a> {
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
#[doc = "Reader of field `AP`"]
pub type AP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AP`"]
pub struct AP_W<'a> {
    w: &'a mut W,
}
impl<'a> AP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `RESERVED22`"]
pub type RESERVED22_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED22`"]
pub struct RESERVED22_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED22_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `TEX`"]
pub type TEX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TEX`"]
pub struct TEX_W<'a> {
    w: &'a mut W,
}
impl<'a> TEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
#[doc = "Reader of field `S`"]
pub type S_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S`"]
pub struct S_W<'a> {
    w: &'a mut W,
}
impl<'a> S_W<'a> {
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
#[doc = "Reader of field `C`"]
pub type C_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C`"]
pub struct C_W<'a> {
    w: &'a mut W,
}
impl<'a> C_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `B`"]
pub type B_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B`"]
pub struct B_W<'a> {
    w: &'a mut W,
}
impl<'a> B_W<'a> {
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
#[doc = "Reader of field `SRD`"]
pub type SRD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SRD`"]
pub struct SRD_W<'a> {
    w: &'a mut W,
}
impl<'a> SRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `RESERVED6`"]
pub type RESERVED6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED6`"]
pub struct RESERVED6_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `SIZE`"]
pub type SIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIZE`"]
pub struct SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 1)) | (((value as u32) & 0x1f) << 1);
        self.w
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
    #[doc = "Bits 29:31 - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved29(&self) -> RESERVED29_R {
        RESERVED29_R::new(((self.bits >> 29) & 0x07) as u8)
    }
    #[doc = "Bit 28 - 28:28\\]
Instruction access disable: 0: Enable instruction fetches 1: Disable instruction fetches"]
    #[inline(always)]
    pub fn xn(&self) -> XN_R {
        XN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved27(&self) -> RESERVED27_R {
        RESERVED27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Data access permission: 0x0: Priviliged permissions: No access. User permissions: No access. 0x1: Priviliged permissions: Read-write. User permissions: No access. 0x2: Priviliged permissions: Read-write. User permissions: Read-only. 0x3: Priviliged permissions: Read-write. User permissions: Read-write. 0x4: Reserved 0x5: Priviliged permissions: Read-only. User permissions: No access. 0x6: Priviliged permissions: Read-only. User permissions: Read-only. 0x7: Priviliged permissions: Read-only. User permissions: Read-only."]
    #[inline(always)]
    pub fn ap(&self) -> AP_R {
        AP_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved22(&self) -> RESERVED22_R {
        RESERVED22_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 19:21 - 21:19\\]
Type extension"]
    #[inline(always)]
    pub fn tex(&self) -> TEX_R {
        TEX_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 18 - 18:18\\]
Shareable bit: 0: Not shareable 1: Shareable"]
    #[inline(always)]
    pub fn s(&self) -> S_R {
        S_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Cacheable bit: 0: Not cacheable 1: Cacheable"]
    #[inline(always)]
    pub fn c(&self) -> C_R {
        C_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Bufferable bit: 0: Not bufferable 1: Bufferable"]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Sub-Region Disable field: Setting a bit in this field disables the corresponding sub-region. Regions are split into eight equal-sized sub-regions. Sub-regions are not supported for region sizes of 128 bytes and less."]
    #[inline(always)]
    pub fn srd(&self) -> SRD_R {
        SRD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 1:5 - 5:1\\]
MPU Protection Region Size Field: 0x04: 32B 0x05: 64B 0x06: 128B 0x07: 256B 0x08: 512B 0x09: 1KB 0x0A: 2KB 0x0B: 4KB 0x0C: 8KB 0x0D: 16KB 0x0E: 32KB 0x0F: 64KB 0x10: 128KB 0x11: 256KB 0x12: 512KB 0x13: 1MB 0x14: 2MB 0x15: 4MB 0x16: 8MB 0x17: 16MB 0x18: 32MB 0x19: 64MB 0x1A: 128MB 0x1B: 256MB 0x1C: 512MB 0x1D: 1GB 0x1E: 2GB 0x1F: 4GB"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 0 - 0:0\\]
Region enable bit: 0: Disable region 1: Enable region"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 29:31 - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved29(&mut self) -> RESERVED29_W {
        RESERVED29_W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\]
Instruction access disable: 0: Enable instruction fetches 1: Disable instruction fetches"]
    #[inline(always)]
    pub fn xn(&mut self) -> XN_W {
        XN_W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved27(&mut self) -> RESERVED27_W {
        RESERVED27_W { w: self }
    }
    #[doc = "Bits 24:26 - 26:24\\]
Data access permission: 0x0: Priviliged permissions: No access. User permissions: No access. 0x1: Priviliged permissions: Read-write. User permissions: No access. 0x2: Priviliged permissions: Read-write. User permissions: Read-only. 0x3: Priviliged permissions: Read-write. User permissions: Read-write. 0x4: Reserved 0x5: Priviliged permissions: Read-only. User permissions: No access. 0x6: Priviliged permissions: Read-only. User permissions: Read-only. 0x7: Priviliged permissions: Read-only. User permissions: Read-only."]
    #[inline(always)]
    pub fn ap(&mut self) -> AP_W {
        AP_W { w: self }
    }
    #[doc = "Bits 22:23 - 23:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved22(&mut self) -> RESERVED22_W {
        RESERVED22_W { w: self }
    }
    #[doc = "Bits 19:21 - 21:19\\]
Type extension"]
    #[inline(always)]
    pub fn tex(&mut self) -> TEX_W {
        TEX_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
Shareable bit: 0: Not shareable 1: Shareable"]
    #[inline(always)]
    pub fn s(&mut self) -> S_W {
        S_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Cacheable bit: 0: Not cacheable 1: Cacheable"]
    #[inline(always)]
    pub fn c(&mut self) -> C_W {
        C_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Bufferable bit: 0: Not bufferable 1: Bufferable"]
    #[inline(always)]
    pub fn b(&mut self) -> B_W {
        B_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Sub-Region Disable field: Setting a bit in this field disables the corresponding sub-region. Regions are split into eight equal-sized sub-regions. Sub-regions are not supported for region sizes of 128 bytes and less."]
    #[inline(always)]
    pub fn srd(&mut self) -> SRD_W {
        SRD_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&mut self) -> RESERVED6_W {
        RESERVED6_W { w: self }
    }
    #[doc = "Bits 1:5 - 5:1\\]
MPU Protection Region Size Field: 0x04: 32B 0x05: 64B 0x06: 128B 0x07: 256B 0x08: 512B 0x09: 1KB 0x0A: 2KB 0x0B: 4KB 0x0C: 8KB 0x0D: 16KB 0x0E: 32KB 0x0F: 64KB 0x10: 128KB 0x11: 256KB 0x12: 512KB 0x13: 1MB 0x14: 2MB 0x15: 4MB 0x16: 8MB 0x17: 16MB 0x18: 32MB 0x19: 64MB 0x1A: 128MB 0x1B: 256MB 0x1C: 512MB 0x1D: 1GB 0x1E: 2GB 0x1F: 4GB"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W {
        SIZE_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Region enable bit: 0: Disable region 1: Enable region"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
