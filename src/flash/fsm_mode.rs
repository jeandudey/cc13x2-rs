#[doc = "Reader of register FSM_MODE"]
pub type R = crate::R<u32, super::FSM_MODE>;
#[doc = "Writer for register FSM_MODE"]
pub type W = crate::W<u32, super::FSM_MODE>;
#[doc = "Register FSM_MODE `reset()`'s with value 0"]
impl crate::ResetValue for super::FSM_MODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `RDV_SUBMODE`"]
pub type RDV_SUBMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDV_SUBMODE`"]
pub struct RDV_SUBMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RDV_SUBMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `PGM_SUBMODE`"]
pub type PGM_SUBMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PGM_SUBMODE`"]
pub struct PGM_SUBMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PGM_SUBMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `ERA_SUBMODE`"]
pub type ERA_SUBMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ERA_SUBMODE`"]
pub struct ERA_SUBMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERA_SUBMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `SUBMODE`"]
pub type SUBMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SUBMODE`"]
pub struct SUBMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `SAV_PGM_CMD`"]
pub type SAV_PGM_CMD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SAV_PGM_CMD`"]
pub struct SAV_PGM_CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> SAV_PGM_CMD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Reader of field `SAV_ERA_MODE`"]
pub type SAV_ERA_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SAV_ERA_MODE`"]
pub struct SAV_ERA_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAV_ERA_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `CMD`"]
pub type CMD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMD`"]
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
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
    #[doc = "Bits 18:19 - 19:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rdv_submode(&self) -> RDV_SUBMODE_R {
        RDV_SUBMODE_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgm_submode(&self) -> PGM_SUBMODE_R {
        PGM_SUBMODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn era_submode(&self) -> ERA_SUBMODE_R {
        ERA_SUBMODE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn submode(&self) -> SUBMODE_R {
        SUBMODE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 9:11 - 11:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sav_pgm_cmd(&self) -> SAV_PGM_CMD_R {
        SAV_PGM_CMD_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 6:8 - 8:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sav_era_mode(&self) -> SAV_ERA_MODE_R {
        SAV_ERA_MODE_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 20:31 - 31:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved20(&mut self) -> RESERVED20_W {
        RESERVED20_W { w: self }
    }
    #[doc = "Bits 18:19 - 19:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rdv_submode(&mut self) -> RDV_SUBMODE_W {
        RDV_SUBMODE_W { w: self }
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgm_submode(&mut self) -> PGM_SUBMODE_W {
        PGM_SUBMODE_W { w: self }
    }
    #[doc = "Bits 14:15 - 15:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn era_submode(&mut self) -> ERA_SUBMODE_W {
        ERA_SUBMODE_W { w: self }
    }
    #[doc = "Bits 12:13 - 13:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn submode(&mut self) -> SUBMODE_W {
        SUBMODE_W { w: self }
    }
    #[doc = "Bits 9:11 - 11:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sav_pgm_cmd(&mut self) -> SAV_PGM_CMD_W {
        SAV_PGM_CMD_W { w: self }
    }
    #[doc = "Bits 6:8 - 8:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sav_era_mode(&mut self) -> SAV_ERA_MODE_W {
        SAV_ERA_MODE_W { w: self }
    }
    #[doc = "Bits 3:5 - 5:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W { w: self }
    }
}
