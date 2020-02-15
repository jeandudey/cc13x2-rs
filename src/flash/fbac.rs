#[doc = "Reader of register FBAC"]
pub type R = crate::R<u32, super::FBAC>;
#[doc = "Writer for register FBAC"]
pub type W = crate::W<u32, super::FBAC>;
#[doc = "Register FBAC `reset()`'s with value 0x0f"]
impl crate::ResetValue for super::FBAC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f
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
#[doc = "Reader of field `OTPPROTDIS`"]
pub type OTPPROTDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OTPPROTDIS`"]
pub struct OTPPROTDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OTPPROTDIS_W<'a> {
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
#[doc = "Reader of field `BAGP`"]
pub type BAGP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BAGP`"]
pub struct BAGP_W<'a> {
    w: &'a mut W,
}
impl<'a> BAGP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `VREADS`"]
pub type VREADS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VREADS`"]
pub struct VREADS_W<'a> {
    w: &'a mut W,
}
impl<'a> VREADS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
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
    pub fn otpprotdis(&self) -> OTPPROTDIS_R {
        OTPPROTDIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bagp(&self) -> BAGP_R {
        BAGP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vreads(&self) -> VREADS_R {
        VREADS_R::new((self.bits & 0xff) as u8)
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
    pub fn otpprotdis(&mut self) -> OTPPROTDIS_W {
        OTPPROTDIS_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bagp(&mut self) -> BAGP_W {
        BAGP_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vreads(&mut self) -> VREADS_W {
        VREADS_W { w: self }
    }
}
