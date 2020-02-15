#[doc = "Reader of register MPU_TYPE"]
pub type R = crate::R<u32, super::MPU_TYPE>;
#[doc = "Writer for register MPU_TYPE"]
pub type W = crate::W<u32, super::MPU_TYPE>;
#[doc = "Register MPU_TYPE `reset()`'s with value 0x0800"]
impl crate::ResetValue for super::MPU_TYPE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0800
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
#[doc = "Reader of field `IREGION`"]
pub type IREGION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IREGION`"]
pub struct IREGION_W<'a> {
    w: &'a mut W,
}
impl<'a> IREGION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DREGION`"]
pub type DREGION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DREGION`"]
pub struct DREGION_W<'a> {
    w: &'a mut W,
}
impl<'a> DREGION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `RESERVED1`"]
pub type RESERVED1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u32) & 0x7f) << 1);
        self.w
    }
}
#[doc = "Reader of field `SEPARATE`"]
pub type SEPARATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEPARATE`"]
pub struct SEPARATE_W<'a> {
    w: &'a mut W,
}
impl<'a> SEPARATE_W<'a> {
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
    #[doc = "Bits 24:31 - 31:24\\]
Reads 0."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
The processor core uses only a unified MPU, this field always reads 0x0."]
    #[inline(always)]
    pub fn iregion(&self) -> IREGION_R {
        IREGION_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Number of supported MPU regions field. This field reads 0x08 indicating eight MPU regions."]
    #[inline(always)]
    pub fn dregion(&self) -> DREGION_R {
        DREGION_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Reads 0."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 0 - 0:0\\]
The processor core uses only a unified MPU, thus this field is always 0."]
    #[inline(always)]
    pub fn separate(&self) -> SEPARATE_R {
        SEPARATE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Reads 0."]
    #[inline(always)]
    pub fn reserved24(&mut self) -> RESERVED24_W {
        RESERVED24_W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\]
The processor core uses only a unified MPU, this field always reads 0x0."]
    #[inline(always)]
    pub fn iregion(&mut self) -> IREGION_W {
        IREGION_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Number of supported MPU regions field. This field reads 0x08 indicating eight MPU regions."]
    #[inline(always)]
    pub fn dregion(&mut self) -> DREGION_W {
        DREGION_W { w: self }
    }
    #[doc = "Bits 1:7 - 7:1\\]
Reads 0."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
The processor core uses only a unified MPU, thus this field is always 0."]
    #[inline(always)]
    pub fn separate(&mut self) -> SEPARATE_W {
        SEPARATE_W { w: self }
    }
}
