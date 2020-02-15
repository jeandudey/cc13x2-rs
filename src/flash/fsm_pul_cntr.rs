#[doc = "Reader of register FSM_PUL_CNTR"]
pub type R = crate::R<u32, super::FSM_PUL_CNTR>;
#[doc = "Writer for register FSM_PUL_CNTR"]
pub type W = crate::W<u32, super::FSM_PUL_CNTR>;
#[doc = "Register FSM_PUL_CNTR `reset()`'s with value 0"]
impl crate::ResetValue for super::FSM_PUL_CNTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED25`"]
pub type RESERVED25_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED25`"]
pub struct RESERVED25_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED25_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | (((value as u32) & 0x7f) << 25);
        self.w
    }
}
#[doc = "Reader of field `CUR_EC_LEVEL`"]
pub type CUR_EC_LEVEL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CUR_EC_LEVEL`"]
pub struct CUR_EC_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CUR_EC_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RESERVED12`"]
pub type RESERVED12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED12`"]
pub struct RESERVED12_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `PUL_CNTR`"]
pub type PUL_CNTR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PUL_CNTR`"]
pub struct PUL_CNTR_W<'a> {
    w: &'a mut W,
}
impl<'a> PUL_CNTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:31 - 31:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved25(&self) -> RESERVED25_R {
        RESERVED25_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
    #[doc = "Bits 16:24 - 24:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cur_ec_level(&self) -> CUR_EC_LEVEL_R {
        CUR_EC_LEVEL_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pul_cntr(&self) -> PUL_CNTR_R {
        PUL_CNTR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 25:31 - 31:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved25(&mut self) -> RESERVED25_W {
        RESERVED25_W { w: self }
    }
    #[doc = "Bits 16:24 - 24:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cur_ec_level(&mut self) -> CUR_EC_LEVEL_W {
        CUR_EC_LEVEL_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&mut self) -> RESERVED12_W {
        RESERVED12_W { w: self }
    }
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pul_cntr(&mut self) -> PUL_CNTR_W {
        PUL_CNTR_W { w: self }
    }
}
