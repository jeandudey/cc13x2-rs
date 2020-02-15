#[doc = "Reader of register REMAP"]
pub type R = crate::R<u32, super::REMAP>;
#[doc = "Writer for register REMAP"]
pub type W = crate::W<u32, super::REMAP>;
#[doc = "Register REMAP `reset()`'s with value 0x2000_0000"]
impl crate::ResetValue for super::REMAP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2000_0000
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
#[doc = "Reader of field `REMAP`"]
pub type REMAP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `REMAP`"]
pub struct REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> REMAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 5)) | (((value as u32) & 0x00ff_ffff) << 5);
        self.w
    }
}
#[doc = "Reader of field `RESERVED0`"]
pub type RESERVED0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED0`"]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 29:31 - 31:29\\]
This field always reads 3'b001. Writing to this field is ignored."]
    #[inline(always)]
    pub fn reserved29(&self) -> RESERVED29_R {
        RESERVED29_R::new(((self.bits >> 29) & 0x07) as u8)
    }
    #[doc = "Bits 5:28 - 28:5\\]
Remap base address field."]
    #[inline(always)]
    pub fn remap(&self) -> REMAP_R {
        REMAP_R::new(((self.bits >> 5) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:4 - 4:0\\]
This field always reads 0. Writing to this field is ignored."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 29:31 - 31:29\\]
This field always reads 3'b001. Writing to this field is ignored."]
    #[inline(always)]
    pub fn reserved29(&mut self) -> RESERVED29_W {
        RESERVED29_W { w: self }
    }
    #[doc = "Bits 5:28 - 28:5\\]
Remap base address field."]
    #[inline(always)]
    pub fn remap(&mut self) -> REMAP_W {
        REMAP_W { w: self }
    }
    #[doc = "Bits 0:4 - 4:0\\]
This field always reads 0. Writing to this field is ignored."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
}
