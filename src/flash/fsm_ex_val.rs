#[doc = "Reader of register FSM_EX_VAL"]
pub type R = crate::R<u32, super::FSM_EX_VAL>;
#[doc = "Writer for register FSM_EX_VAL"]
pub type W = crate::W<u32, super::FSM_EX_VAL>;
#[doc = "Register FSM_EX_VAL `reset()`'s with value 0x0301"]
impl crate::ResetValue for super::FSM_EX_VAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0301
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
#[doc = "Reader of field `REP_VSU`"]
pub type REP_VSU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REP_VSU`"]
pub struct REP_VSU_W<'a> {
    w: &'a mut W,
}
impl<'a> REP_VSU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `EXE_VALD`"]
pub type EXE_VALD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXE_VALD`"]
pub struct EXE_VALD_W<'a> {
    w: &'a mut W,
}
impl<'a> EXE_VALD_W<'a> {
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
    pub fn rep_vsu(&self) -> REP_VSU_R {
        REP_VSU_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn exe_vald(&self) -> EXE_VALD_R {
        EXE_VALD_R::new((self.bits & 0xff) as u8)
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
    pub fn rep_vsu(&mut self) -> REP_VSU_W {
        REP_VSU_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn exe_vald(&mut self) -> EXE_VALD_W {
        EXE_VALD_W { w: self }
    }
}
