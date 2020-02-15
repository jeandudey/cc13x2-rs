#[doc = "Reader of register EFUSE"]
pub type R = crate::R<u32, super::EFUSE>;
#[doc = "Writer for register EFUSE"]
pub type W = crate::W<u32, super::EFUSE>;
#[doc = "Register EFUSE `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSE {
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
#[doc = "Reader of field `INSTRUCTION`"]
pub type INSTRUCTION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INSTRUCTION`"]
pub struct INSTRUCTION_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTRUCTION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
#[doc = "Reader of field `RESERVED16`"]
pub type RESERVED16_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED16`"]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DUMPWORD`"]
pub type DUMPWORD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DUMPWORD`"]
pub struct DUMPWORD_W<'a> {
    w: &'a mut W,
}
impl<'a> DUMPWORD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 29:31 - 31:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved29(&self) -> RESERVED29_R {
        RESERVED29_R::new(((self.bits >> 29) & 0x07) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn instruction(&self) -> INSTRUCTION_R {
        INSTRUCTION_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dumpword(&self) -> DUMPWORD_R {
        DUMPWORD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 29:31 - 31:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved29(&mut self) -> RESERVED29_W {
        RESERVED29_W { w: self }
    }
    #[doc = "Bits 24:28 - 28:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn instruction(&mut self) -> INSTRUCTION_W {
        INSTRUCTION_W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dumpword(&mut self) -> DUMPWORD_W {
        DUMPWORD_W { w: self }
    }
}
