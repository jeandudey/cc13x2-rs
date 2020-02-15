#[doc = "Reader of register LPMBIAS"]
pub type R = crate::R<u8, super::LPMBIAS>;
#[doc = "Writer for register LPMBIAS"]
pub type W = crate::W<u8, super::LPMBIAS>;
#[doc = "Register LPMBIAS `reset()`'s with value 0"]
impl crate::ResetValue for super::LPMBIAS {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPARE6`"]
pub type SPARE6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPARE6`"]
pub struct SPARE6_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u8) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `LPM_TRIM_IOUT`"]
pub type LPM_TRIM_IOUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPM_TRIM_IOUT`"]
pub struct LPM_TRIM_IOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_TRIM_IOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u8) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn spare6(&self) -> SPARE6_R {
        SPARE6_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_trim_iout(&self) -> LPM_TRIM_IOUT_R {
        LPM_TRIM_IOUT_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn spare6(&mut self) -> SPARE6_W {
        SPARE6_W { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_trim_iout(&mut self) -> LPM_TRIM_IOUT_W {
        LPM_TRIM_IOUT_W { w: self }
    }
}
