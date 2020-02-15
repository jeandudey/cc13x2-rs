#[doc = "Reader of register CPUID"]
pub type R = crate::R<u32, super::CPUID>;
#[doc = "Writer for register CPUID"]
pub type W = crate::W<u32, super::CPUID>;
#[doc = "Register CPUID `reset()`'s with value 0x410f_c241"]
impl crate::ResetValue for super::CPUID {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x410f_c241
    }
}
#[doc = "Reader of field `IMPLEMENTER`"]
pub type IMPLEMENTER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IMPLEMENTER`"]
pub struct IMPLEMENTER_W<'a> {
    w: &'a mut W,
}
impl<'a> IMPLEMENTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `VARIANT`"]
pub type VARIANT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VARIANT`"]
pub struct VARIANT_W<'a> {
    w: &'a mut W,
}
impl<'a> VARIANT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `CONSTANT`"]
pub type CONSTANT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CONSTANT`"]
pub struct CONSTANT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONSTANT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `PARTNO`"]
pub type PARTNO_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PARTNO`"]
pub struct PARTNO_W<'a> {
    w: &'a mut W,
}
impl<'a> PARTNO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 4)) | (((value as u32) & 0x0fff) << 4);
        self.w
    }
}
#[doc = "Reader of field `REVISION`"]
pub type REVISION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REVISION`"]
pub struct REVISION_W<'a> {
    w: &'a mut W,
}
impl<'a> REVISION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Implementor code."]
    #[inline(always)]
    pub fn implementer(&self) -> IMPLEMENTER_R {
        IMPLEMENTER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Implementation defined variant number."]
    #[inline(always)]
    pub fn variant(&self) -> VARIANT_R {
        VARIANT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Reads as 0xF"]
    #[inline(always)]
    pub fn constant(&self) -> CONSTANT_R {
        CONSTANT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Number of processor within family."]
    #[inline(always)]
    pub fn partno(&self) -> PARTNO_R {
        PARTNO_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Implementation defined revision number."]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Implementor code."]
    #[inline(always)]
    pub fn implementer(&mut self) -> IMPLEMENTER_W {
        IMPLEMENTER_W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\]
Implementation defined variant number."]
    #[inline(always)]
    pub fn variant(&mut self) -> VARIANT_W {
        VARIANT_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\]
Reads as 0xF"]
    #[inline(always)]
    pub fn constant(&mut self) -> CONSTANT_W {
        CONSTANT_W { w: self }
    }
    #[doc = "Bits 4:15 - 15:4\\]
Number of processor within family."]
    #[inline(always)]
    pub fn partno(&mut self) -> PARTNO_W {
        PARTNO_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Implementation defined revision number."]
    #[inline(always)]
    pub fn revision(&mut self) -> REVISION_W {
        REVISION_W { w: self }
    }
}
