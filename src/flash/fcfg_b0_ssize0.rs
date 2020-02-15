#[doc = "Reader of register FCFG_B0_SSIZE0"]
pub type R = crate::R<u32, super::FCFG_B0_SSIZE0>;
#[doc = "Writer for register FCFG_B0_SSIZE0"]
pub type W = crate::W<u32, super::FCFG_B0_SSIZE0>;
#[doc = "Register FCFG_B0_SSIZE0 `reset()`'s with value 0x002c_0008"]
impl crate::ResetValue for super::FCFG_B0_SSIZE0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x002c_0008
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
#[doc = "Reader of field `B0_NUM_SECTORS`"]
pub type B0_NUM_SECTORS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `B0_NUM_SECTORS`"]
pub struct B0_NUM_SECTORS_W<'a> {
    w: &'a mut W,
}
impl<'a> B0_NUM_SECTORS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RESERVED4`"]
pub type RESERVED4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED4`"]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 4)) | (((value as u32) & 0x0fff) << 4);
        self.w
    }
}
#[doc = "Reader of field `B0_SECT_SIZE`"]
pub type B0_SECT_SIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `B0_SECT_SIZE`"]
pub struct B0_SECT_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> B0_SECT_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved28(&self) -> RESERVED28_R {
        RESERVED28_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b0_num_sectors(&self) -> B0_NUM_SECTORS_R {
        B0_NUM_SECTORS_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b0_sect_size(&self) -> B0_SECT_SIZE_R {
        B0_SECT_SIZE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved28(&mut self) -> RESERVED28_W {
        RESERVED28_W { w: self }
    }
    #[doc = "Bits 16:27 - 27:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b0_num_sectors(&mut self) -> B0_NUM_SECTORS_W {
        B0_NUM_SECTORS_W { w: self }
    }
    #[doc = "Bits 4:15 - 15:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b0_sect_size(&mut self) -> B0_SECT_SIZE_W {
        B0_SECT_SIZE_W { w: self }
    }
}
