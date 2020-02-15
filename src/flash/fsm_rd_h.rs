#[doc = "Reader of register FSM_RD_H"]
pub type R = crate::R<u32, super::FSM_RD_H>;
#[doc = "Writer for register FSM_RD_H"]
pub type W = crate::W<u32, super::FSM_RD_H>;
#[doc = "Register FSM_RD_H `reset()`'s with value 0x5a"]
impl crate::ResetValue for super::FSM_RD_H {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x5a
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
#[doc = "Reader of field `RD_H`"]
pub type RD_H_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RD_H`"]
pub struct RD_H_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rd_h(&self) -> RD_H_R {
        RD_H_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rd_h(&mut self) -> RD_H_W {
        RD_H_W { w: self }
    }
}
