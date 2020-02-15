#[doc = "Reader of register EFUSERELEASE"]
pub type R = crate::R<u32, super::EFUSERELEASE>;
#[doc = "Writer for register EFUSERELEASE"]
pub type W = crate::W<u32, super::EFUSERELEASE>;
#[doc = "Register EFUSERELEASE `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSERELEASE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ODPYEAR`"]
pub type ODPYEAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ODPYEAR`"]
pub struct ODPYEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> ODPYEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | (((value as u32) & 0x7f) << 25);
        self.w
    }
}
#[doc = "Reader of field `ODPMONTH`"]
pub type ODPMONTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ODPMONTH`"]
pub struct ODPMONTH_W<'a> {
    w: &'a mut W,
}
impl<'a> ODPMONTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 21)) | (((value as u32) & 0x0f) << 21);
        self.w
    }
}
#[doc = "Reader of field `ODPDAY`"]
pub type ODPDAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ODPDAY`"]
pub struct ODPDAY_W<'a> {
    w: &'a mut W,
}
impl<'a> ODPDAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `EFUSEYEAR`"]
pub type EFUSEYEAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSEYEAR`"]
pub struct EFUSEYEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSEYEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 9)) | (((value as u32) & 0x7f) << 9);
        self.w
    }
}
#[doc = "Reader of field `EFUSEMONTH`"]
pub type EFUSEMONTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSEMONTH`"]
pub struct EFUSEMONTH_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSEMONTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | (((value as u32) & 0x0f) << 5);
        self.w
    }
}
#[doc = "Reader of field `EFUSEDAY`"]
pub type EFUSEDAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSEDAY`"]
pub struct EFUSEDAY_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSEDAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:31 - 31:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn odpyear(&self) -> ODPYEAR_R {
        ODPYEAR_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
    #[doc = "Bits 21:24 - 24:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn odpmonth(&self) -> ODPMONTH_R {
        ODPMONTH_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn odpday(&self) -> ODPDAY_R {
        ODPDAY_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efuseyear(&self) -> EFUSEYEAR_R {
        EFUSEYEAR_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 5:8 - 8:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efusemonth(&self) -> EFUSEMONTH_R {
        EFUSEMONTH_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efuseday(&self) -> EFUSEDAY_R {
        EFUSEDAY_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:31 - 31:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn odpyear(&mut self) -> ODPYEAR_W {
        ODPYEAR_W { w: self }
    }
    #[doc = "Bits 21:24 - 24:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn odpmonth(&mut self) -> ODPMONTH_W {
        ODPMONTH_W { w: self }
    }
    #[doc = "Bits 16:20 - 20:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn odpday(&mut self) -> ODPDAY_W {
        ODPDAY_W { w: self }
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efuseyear(&mut self) -> EFUSEYEAR_W {
        EFUSEYEAR_W { w: self }
    }
    #[doc = "Bits 5:8 - 8:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efusemonth(&mut self) -> EFUSEMONTH_W {
        EFUSEMONTH_W { w: self }
    }
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efuseday(&mut self) -> EFUSEDAY_W {
        EFUSEDAY_W { w: self }
    }
}
