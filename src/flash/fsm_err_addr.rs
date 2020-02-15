#[doc = "Reader of register FSM_ERR_ADDR"]
pub type R = crate::R<u32, super::FSM_ERR_ADDR>;
#[doc = "Writer for register FSM_ERR_ADDR"]
pub type W = crate::W<u32, super::FSM_ERR_ADDR>;
#[doc = "Register FSM_ERR_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::FSM_ERR_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FSM_ERR_ADDR`"]
pub type FSM_ERR_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FSM_ERR_ADDR`"]
pub struct FSM_ERR_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> FSM_ERR_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Reader of field `RESERVED4`"]
pub type RESERVED4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED4`"]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `FSM_ERR_BANK`"]
pub type FSM_ERR_BANK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FSM_ERR_BANK`"]
pub struct FSM_ERR_BANK_W<'a> {
    w: &'a mut W,
}
impl<'a> FSM_ERR_BANK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_err_addr(&self) -> FSM_ERR_ADDR_R {
        FSM_ERR_ADDR_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_err_bank(&self) -> FSM_ERR_BANK_R {
        FSM_ERR_BANK_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_err_addr(&mut self) -> FSM_ERR_ADDR_W {
        FSM_ERR_ADDR_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_err_bank(&mut self) -> FSM_ERR_BANK_W {
        FSM_ERR_BANK_W { w: self }
    }
}
