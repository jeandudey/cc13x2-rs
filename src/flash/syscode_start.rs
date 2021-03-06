#[doc = "Reader of register SYSCODE_START"]
pub type R = crate::R<u32, super::SYSCODE_START>;
#[doc = "Writer for register SYSCODE_START"]
pub type W = crate::W<u32, super::SYSCODE_START>;
#[doc = "Register SYSCODE_START `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCODE_START {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED5`"]
pub type RESERVED5_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED5`"]
pub struct RESERVED5_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff_ffff << 6)) | (((value as u32) & 0x03ff_ffff) << 6);
        self.w
    }
}
#[doc = "Reader of field `SYSCODE_START`"]
pub type SYSCODE_START_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYSCODE_START`"]
pub struct SYSCODE_START_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCODE_START_W<'a> {
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
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 6) & 0x03ff_ffff) as u32)
    }
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn syscode_start(&self) -> SYSCODE_START_R {
        SYSCODE_START_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 6:31 - 31:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved5(&mut self) -> RESERVED5_W {
        RESERVED5_W { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn syscode_start(&mut self) -> SYSCODE_START_W {
        SYSCODE_START_W { w: self }
    }
}
