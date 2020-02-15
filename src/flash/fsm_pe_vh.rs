#[doc = "Reader of register FSM_PE_VH"]
pub type R = crate::R<u32, super::FSM_PE_VH>;
#[doc = "Writer for register FSM_PE_VH"]
pub type W = crate::W<u32, super::FSM_PE_VH>;
#[doc = "Register FSM_PE_VH `reset()`'s with value 0x0100"]
impl crate::ResetValue for super::FSM_PE_VH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100
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
#[doc = "Reader of field `PGM_VH`"]
pub type PGM_VH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PGM_VH`"]
pub struct PGM_VH_W<'a> {
    w: &'a mut W,
}
impl<'a> PGM_VH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `ERA_VH`"]
pub type ERA_VH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ERA_VH`"]
pub struct ERA_VH_W<'a> {
    w: &'a mut W,
}
impl<'a> ERA_VH_W<'a> {
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
    pub fn pgm_vh(&self) -> PGM_VH_R {
        PGM_VH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn era_vh(&self) -> ERA_VH_R {
        ERA_VH_R::new((self.bits & 0xff) as u8)
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
    pub fn pgm_vh(&mut self) -> PGM_VH_W {
        PGM_VH_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn era_vh(&mut self) -> ERA_VH_W {
        ERA_VH_W { w: self }
    }
}
