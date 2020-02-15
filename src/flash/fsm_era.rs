#[doc = "Reader of register FSM_ERA"]
pub type R = crate::R<u32, super::FSM_ERA>;
#[doc = "Writer for register FSM_ERA"]
pub type W = crate::W<u32, super::FSM_ERA>;
#[doc = "Register FSM_ERA `reset()`'s with value 0"]
impl crate::ResetValue for super::FSM_ERA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED26`"]
pub type RESERVED26_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED26`"]
pub struct RESERVED26_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED26_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | (((value as u32) & 0x3f) << 26);
        self.w
    }
}
#[doc = "Reader of field `ERA_BANK`"]
pub type ERA_BANK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ERA_BANK`"]
pub struct ERA_BANK_W<'a> {
    w: &'a mut W,
}
impl<'a> ERA_BANK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 23)) | (((value as u32) & 0x07) << 23);
        self.w
    }
}
#[doc = "Reader of field `ERA_ADDR`"]
pub type ERA_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ERA_ADDR`"]
pub struct ERA_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERA_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x007f_ffff) | ((value as u32) & 0x007f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 26:31 - 31:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved26(&self) -> RESERVED26_R {
        RESERVED26_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
    #[doc = "Bits 23:25 - 25:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn era_bank(&self) -> ERA_BANK_R {
        ERA_BANK_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bits 0:22 - 22:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn era_addr(&self) -> ERA_ADDR_R {
        ERA_ADDR_R::new((self.bits & 0x007f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 26:31 - 31:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved26(&mut self) -> RESERVED26_W {
        RESERVED26_W { w: self }
    }
    #[doc = "Bits 23:25 - 25:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn era_bank(&mut self) -> ERA_BANK_W {
        ERA_BANK_W { w: self }
    }
    #[doc = "Bits 0:22 - 22:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn era_addr(&mut self) -> ERA_ADDR_W {
        ERA_ADDR_W { w: self }
    }
}
