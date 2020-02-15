#[doc = "Reader of register FCFG_BANK"]
pub type R = crate::R<u32, super::FCFG_BANK>;
#[doc = "Writer for register FCFG_BANK"]
pub type W = crate::W<u32, super::FCFG_BANK>;
#[doc = "Register FCFG_BANK `reset()`'s with value 0x0801"]
impl crate::ResetValue for super::FCFG_BANK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0801
    }
}
#[doc = "Reader of field `EE_BANK_WIDTH`"]
pub type EE_BANK_WIDTH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EE_BANK_WIDTH`"]
pub struct EE_BANK_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> EE_BANK_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 20)) | (((value as u32) & 0x0fff) << 20);
        self.w
    }
}
#[doc = "Reader of field `EE_NUM_BANK`"]
pub type EE_NUM_BANK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EE_NUM_BANK`"]
pub struct EE_NUM_BANK_W<'a> {
    w: &'a mut W,
}
impl<'a> EE_NUM_BANK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `MAIN_BANK_WIDTH`"]
pub type MAIN_BANK_WIDTH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MAIN_BANK_WIDTH`"]
pub struct MAIN_BANK_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> MAIN_BANK_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 4)) | (((value as u32) & 0x0fff) << 4);
        self.w
    }
}
#[doc = "Reader of field `MAIN_NUM_BANK`"]
pub type MAIN_NUM_BANK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAIN_NUM_BANK`"]
pub struct MAIN_NUM_BANK_W<'a> {
    w: &'a mut W,
}
impl<'a> MAIN_NUM_BANK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:31 - 31:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ee_bank_width(&self) -> EE_BANK_WIDTH_R {
        EE_BANK_WIDTH_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ee_num_bank(&self) -> EE_NUM_BANK_R {
        EE_NUM_BANK_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn main_bank_width(&self) -> MAIN_BANK_WIDTH_R {
        MAIN_BANK_WIDTH_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn main_num_bank(&self) -> MAIN_NUM_BANK_R {
        MAIN_NUM_BANK_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:31 - 31:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ee_bank_width(&mut self) -> EE_BANK_WIDTH_W {
        EE_BANK_WIDTH_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ee_num_bank(&mut self) -> EE_NUM_BANK_W {
        EE_NUM_BANK_W { w: self }
    }
    #[doc = "Bits 4:15 - 15:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn main_bank_width(&mut self) -> MAIN_BANK_WIDTH_W {
        MAIN_BANK_WIDTH_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn main_num_bank(&mut self) -> MAIN_NUM_BANK_W {
        MAIN_NUM_BANK_W { w: self }
    }
}
