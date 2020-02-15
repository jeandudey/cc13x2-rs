#[doc = "Reader of register FSM_SAV_ERA_PUL"]
pub type R = crate::R<u32, super::FSM_SAV_ERA_PUL>;
#[doc = "Writer for register FSM_SAV_ERA_PUL"]
pub type W = crate::W<u32, super::FSM_SAV_ERA_PUL>;
#[doc = "Register FSM_SAV_ERA_PUL `reset()`'s with value 0"]
impl crate::ResetValue for super::FSM_SAV_ERA_PUL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED12`"]
pub type RESERVED12_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED12`"]
pub struct RESERVED12_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 12)) | (((value as u32) & 0x000f_ffff) << 12);
        self.w
    }
}
#[doc = "Reader of field `SAV_ERA_PUL`"]
pub type SAV_ERA_PUL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SAV_ERA_PUL`"]
pub struct SAV_ERA_PUL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAV_ERA_PUL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sav_era_pul(&self) -> SAV_ERA_PUL_R {
        SAV_ERA_PUL_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&mut self) -> RESERVED12_W {
        RESERVED12_W { w: self }
    }
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sav_era_pul(&mut self) -> SAV_ERA_PUL_W {
        SAV_ERA_PUL_W { w: self }
    }
}
