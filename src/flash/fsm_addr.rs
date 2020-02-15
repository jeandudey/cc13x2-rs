#[doc = "Reader of register FSM_ADDR"]
pub type R = crate::R<u32, super::FSM_ADDR>;
#[doc = "Writer for register FSM_ADDR"]
pub type W = crate::W<u32, super::FSM_ADDR>;
#[doc = "Register FSM_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::FSM_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED31`"]
pub type RESERVED31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED31`"]
pub struct RESERVED31_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `BANK`"]
pub type BANK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BANK`"]
pub struct BANK_W<'a> {
    w: &'a mut W,
}
impl<'a> BANK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Reader of field `CUR_ADDR`"]
pub type CUR_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CUR_ADDR`"]
pub struct CUR_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> CUR_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff_ffff) | ((value as u32) & 0x0fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved31(&self) -> RESERVED31_R {
        RESERVED31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bank(&self) -> BANK_R {
        BANK_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 0:27 - 27:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cur_addr(&self) -> CUR_ADDR_R {
        CUR_ADDR_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved31(&mut self) -> RESERVED31_W {
        RESERVED31_W { w: self }
    }
    #[doc = "Bits 28:30 - 30:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bank(&mut self) -> BANK_W {
        BANK_W { w: self }
    }
    #[doc = "Bits 0:27 - 27:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cur_addr(&mut self) -> CUR_ADDR_W {
        CUR_ADDR_W { w: self }
    }
}
