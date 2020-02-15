#[doc = "Reader of register BATMONP1"]
pub type R = crate::R<u32, super::BATMONP1>;
#[doc = "Writer for register BATMONP1"]
pub type W = crate::W<u32, super::BATMONP1>;
#[doc = "Register BATMONP1 `reset()`'s with value 0"]
impl crate::ResetValue for super::BATMONP1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED6`"]
pub type RESERVED6_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED6`"]
pub struct RESERVED6_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff_ffff << 6)) | (((value as u32) & 0x03ff_ffff) << 6);
        self.w
    }
}
#[doc = "Reader of field `CFG`"]
pub type CFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFG`"]
pub struct CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:31 - 31:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 0x03ff_ffff) as u32)
    }
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cfg(&self) -> CFG_R {
        CFG_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 6:31 - 31:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved6(&mut self) -> RESERVED6_W {
        RESERVED6_W { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cfg(&mut self) -> CFG_W {
        CFG_W { w: self }
    }
}
