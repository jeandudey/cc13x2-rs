#[doc = "Reader of register FSM_EXECUTE"]
pub type R = crate::R<u32, super::FSM_EXECUTE>;
#[doc = "Writer for register FSM_EXECUTE"]
pub type W = crate::W<u32, super::FSM_EXECUTE>;
#[doc = "Register FSM_EXECUTE `reset()`'s with value 0x000a_000a"]
impl crate::ResetValue for super::FSM_EXECUTE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x000a_000a
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
#[doc = "Reader of field `SUSPEND_NOW`"]
pub type SUSPEND_NOW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SUSPEND_NOW`"]
pub struct SUSPEND_NOW_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPEND_NOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `RESERVED5`"]
pub type RESERVED5_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED5`"]
pub struct RESERVED5_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 5)) | (((value as u32) & 0x07ff) << 5);
        self.w
    }
}
#[doc = "Reader of field `FSMEXECUTE`"]
pub type FSMEXECUTE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FSMEXECUTE`"]
pub struct FSMEXECUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> FSMEXECUTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
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
    pub fn suspend_now(&self) -> SUSPEND_NOW_R {
        SUSPEND_NOW_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 5:15 - 15:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 5) & 0x07ff) as u16)
    }
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsmexecute(&self) -> FSMEXECUTE_R {
        FSMEXECUTE_R::new((self.bits & 0x1f) as u8)
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
    pub fn suspend_now(&mut self) -> SUSPEND_NOW_W {
        SUSPEND_NOW_W { w: self }
    }
    #[doc = "Bits 5:15 - 15:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved5(&mut self) -> RESERVED5_W {
        RESERVED5_W { w: self }
    }
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsmexecute(&mut self) -> FSMEXECUTE_W {
        FSMEXECUTE_W { w: self }
    }
}
