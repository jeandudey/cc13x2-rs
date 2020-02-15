#[doc = "Reader of register FSM_PGM_MAXPUL"]
pub type R = crate::R<u32, super::FSM_PGM_MAXPUL>;
#[doc = "Writer for register FSM_PGM_MAXPUL"]
pub type W = crate::W<u32, super::FSM_PGM_MAXPUL>;
#[doc = "Register FSM_PGM_MAXPUL `reset()`'s with value 0"]
impl crate::ResetValue for super::FSM_PGM_MAXPUL {
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
#[doc = "Reader of field `FSM_PGM_MAXPUL`"]
pub type FSM_PGM_MAXPUL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FSM_PGM_MAXPUL`"]
pub struct FSM_PGM_MAXPUL_W<'a> {
    w: &'a mut W,
}
impl<'a> FSM_PGM_MAXPUL_W<'a> {
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
    pub fn fsm_pgm_maxpul(&self) -> FSM_PGM_MAXPUL_R {
        FSM_PGM_MAXPUL_R::new((self.bits & 0x0fff) as u16)
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
    pub fn fsm_pgm_maxpul(&mut self) -> FSM_PGM_MAXPUL_W {
        FSM_PGM_MAXPUL_W { w: self }
    }
}
