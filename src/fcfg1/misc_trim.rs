#[doc = "Reader of register MISC_TRIM"]
pub type R = crate::R<u32, super::MISC_TRIM>;
#[doc = "Writer for register MISC_TRIM"]
pub type W = crate::W<u32, super::MISC_TRIM>;
#[doc = "Register MISC_TRIM `reset()`'s with value 0xfffe_003b"]
impl crate::ResetValue for super::MISC_TRIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xfffe_003b
    }
}
#[doc = "Reader of field `TRIM_RECHARGE_COMP_OFFSET`"]
pub type TRIM_RECHARGE_COMP_OFFSET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIM_RECHARGE_COMP_OFFSET`"]
pub struct TRIM_RECHARGE_COMP_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_RECHARGE_COMP_OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 12)) | (((value as u32) & 0x1f) << 12);
        self.w
    }
}
#[doc = "Reader of field `TRIM_RECHARGE_COMP_REFLEVEL`"]
pub type TRIM_RECHARGE_COMP_REFLEVEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIM_RECHARGE_COMP_REFLEVEL`"]
pub struct TRIM_RECHARGE_COMP_REFLEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_RECHARGE_COMP_REFLEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `TEMPVSLOPE`"]
pub type TEMPVSLOPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TEMPVSLOPE`"]
pub struct TEMPVSLOPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEMPVSLOPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:16 - 16:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim_recharge_comp_offset(&self) -> TRIM_RECHARGE_COMP_OFFSET_R {
        TRIM_RECHARGE_COMP_OFFSET_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim_recharge_comp_reflevel(&self) -> TRIM_RECHARGE_COMP_REFLEVEL_R {
        TRIM_RECHARGE_COMP_REFLEVEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Signed byte value representing the TEMP slope with battery voltage, in degrees C / V, with four fractional bits."]
    #[inline(always)]
    pub fn tempvslope(&self) -> TEMPVSLOPE_R {
        TEMPVSLOPE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 12:16 - 16:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim_recharge_comp_offset(&mut self) -> TRIM_RECHARGE_COMP_OFFSET_W {
        TRIM_RECHARGE_COMP_OFFSET_W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim_recharge_comp_reflevel(&mut self) -> TRIM_RECHARGE_COMP_REFLEVEL_W {
        TRIM_RECHARGE_COMP_REFLEVEL_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Signed byte value representing the TEMP slope with battery voltage, in degrees C / V, with four fractional bits."]
    #[inline(always)]
    pub fn tempvslope(&mut self) -> TEMPVSLOPE_W {
        TEMPVSLOPE_W { w: self }
    }
}
