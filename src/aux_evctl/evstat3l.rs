#[doc = "Reader of register EVSTAT3L"]
pub type R = crate::R<u32, super::EVSTAT3L>;
#[doc = "Writer for register EVSTAT3L"]
pub type W = crate::W<u32, super::EVSTAT3L>;
#[doc = "Register EVSTAT3L `reset()`'s with value 0"]
impl crate::ResetValue for super::EVSTAT3L {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED8`"]
pub type RESERVED8_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED8`"]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Reader of field `ALIAS_EV`"]
pub type ALIAS_EV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALIAS_EV`"]
pub struct ALIAS_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> ALIAS_EV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Alias of EVSTAT3 event 7 down to 0."]
    #[inline(always)]
    pub fn alias_ev(&self) -> ALIAS_EV_R {
        ALIAS_EV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Alias of EVSTAT3 event 7 down to 0."]
    #[inline(always)]
    pub fn alias_ev(&mut self) -> ALIAS_EV_W {
        ALIAS_EV_W { w: self }
    }
}
