#[doc = "Reader of register FSM_PE_VSU"]
pub type R = crate::R<u32, super::FSM_PE_VSU>;
#[doc = "Writer for register FSM_PE_VSU"]
pub type W = crate::W<u32, super::FSM_PE_VSU>;
#[doc = "Register FSM_PE_VSU `reset()`'s with value 0"]
impl crate::ResetValue for super::FSM_PE_VSU {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED16`"]
pub type RESERVED16_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED16`"]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `PGM_VSU`"]
pub type PGM_VSU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PGM_VSU`"]
pub struct PGM_VSU_W<'a> {
    w: &'a mut W,
}
impl<'a> PGM_VSU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `ERA_VSU`"]
pub type ERA_VSU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ERA_VSU`"]
pub struct ERA_VSU_W<'a> {
    w: &'a mut W,
}
impl<'a> ERA_VSU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgm_vsu(&self) -> PGM_VSU_R {
        PGM_VSU_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn era_vsu(&self) -> ERA_VSU_R {
        ERA_VSU_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgm_vsu(&mut self) -> PGM_VSU_W {
        PGM_VSU_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn era_vsu(&mut self) -> ERA_VSU_W {
        ERA_VSU_W { w: self }
    }
}
