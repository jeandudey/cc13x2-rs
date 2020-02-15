#[doc = "Reader of register FBPRDY"]
pub type R = crate::R<u32, super::FBPRDY>;
#[doc = "Writer for register FBPRDY"]
pub type W = crate::W<u32, super::FBPRDY>;
#[doc = "Register FBPRDY `reset()`'s with value 0x00ff_00fe"]
impl crate::ResetValue for super::FBPRDY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x00ff_00fe
    }
}
#[doc = "Reader of field `RESERVED17`"]
pub type RESERVED17_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED17`"]
pub struct RESERVED17_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 17)) | (((value as u32) & 0x7fff) << 17);
        self.w
    }
}
#[doc = "Reader of field `BANKBUSY`"]
pub type BANKBUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BANKBUSY`"]
pub struct BANKBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BANKBUSY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `PUMPRDY`"]
pub type PUMPRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PUMPRDY`"]
pub struct PUMPRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> PUMPRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `RESERVED1`"]
pub type RESERVED1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 1)) | (((value as u32) & 0x3fff) << 1);
        self.w
    }
}
#[doc = "Reader of field `BANKRDY`"]
pub type BANKRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BANKRDY`"]
pub struct BANKRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> BANKRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 17:31 - 31:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved17(&self) -> RESERVED17_R {
        RESERVED17_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankbusy(&self) -> BANKBUSY_R {
        BANKBUSY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pumprdy(&self) -> PUMPRDY_R {
        PUMPRDY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 1:14 - 14:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x3fff) as u16)
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankrdy(&self) -> BANKRDY_R {
        BANKRDY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 17:31 - 31:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved17(&mut self) -> RESERVED17_W {
        RESERVED17_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankbusy(&mut self) -> BANKBUSY_W {
        BANKBUSY_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pumprdy(&mut self) -> PUMPRDY_W {
        PUMPRDY_W { w: self }
    }
    #[doc = "Bits 1:14 - 14:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankrdy(&mut self) -> BANKRDY_W {
        BANKRDY_W { w: self }
    }
}
