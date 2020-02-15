#[doc = "Reader of register FSM_PRG_PUL"]
pub type R = crate::R<u32, super::FSM_PRG_PUL>;
#[doc = "Writer for register FSM_PRG_PUL"]
pub type W = crate::W<u32, super::FSM_PRG_PUL>;
#[doc = "Register FSM_PRG_PUL `reset()`'s with value 0x0004_0032"]
impl crate::ResetValue for super::FSM_PRG_PUL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0004_0032
    }
}
#[doc = "Reader of field `RESERVED20`"]
pub type RESERVED20_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED20`"]
pub struct RESERVED20_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 20)) | (((value as u32) & 0x0fff) << 20);
        self.w
    }
}
#[doc = "Reader of field `BEG_EC_LEVEL`"]
pub type BEG_EC_LEVEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BEG_EC_LEVEL`"]
pub struct BEG_EC_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BEG_EC_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
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
#[doc = "Reader of field `MAX_PRG_PUL`"]
pub type MAX_PRG_PUL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MAX_PRG_PUL`"]
pub struct MAX_PRG_PUL_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_PRG_PUL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:31 - 31:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved20(&self) -> RESERVED20_R {
        RESERVED20_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn beg_ec_level(&self) -> BEG_EC_LEVEL_R {
        BEG_EC_LEVEL_R::new(((self.bits >> 16) & 0x0f) as u8)
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
    pub fn max_prg_pul(&self) -> MAX_PRG_PUL_R {
        MAX_PRG_PUL_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 20:31 - 31:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved20(&mut self) -> RESERVED20_W {
        RESERVED20_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn beg_ec_level(&mut self) -> BEG_EC_LEVEL_W {
        BEG_EC_LEVEL_W { w: self }
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
    pub fn max_prg_pul(&mut self) -> MAX_PRG_PUL_W {
        MAX_PRG_PUL_W { w: self }
    }
}
